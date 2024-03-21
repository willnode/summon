mod mem;
mod prop;
mod timer;
mod types;
mod utils;

use prop::*;
use sysinfo::System;
use timer::idling_killer;
use types::*;

use ngx::ffi::{
    nginx_version, ngx_array_push, ngx_conf_t, ngx_http_add_variable, ngx_http_core_module,
    ngx_http_handler_pt, ngx_http_module_t, ngx_http_phases_NGX_HTTP_ACCESS_PHASE,
    ngx_http_request_t, ngx_http_variable_t, ngx_int_t, ngx_module_t, ngx_uint_t,
    ngx_variable_value_t, NGX_HTTP_MODULE, NGX_OK, NGX_RS_MODULE_SIGNATURE,
};
use utils::{get_host, spawn_once};
use ngx::http::MergeConfigError;
use ngx::{core, core::Status, http, http::HTTPModule};
use ngx::{
    http_request_handler, http_variable_get, ngx_http_null_variable, ngx_log_debug_http,
    ngx_modules, ngx_string,
};
use std::os::raw::{c_char, c_void};

use crate::mem::GLOBAL_PROCESSES;
use crate::utils::{find_free_port, now_timestamp, spawn_process, wait_for_connection};

impl http::HTTPModule for Module {
    type MainConf = ();
    type SrvConf = ();
    type LocConf = ModuleConfig;

    // static ngx_int_t ngx_http_orig_dst_add_variables(ngx_conf_t *cf)
    unsafe extern "C" fn preconfiguration(cf: *mut ngx_conf_t) -> ngx_int_t {
        for mut v in ngx_http_summonapp_vars {
            if v.name.len == 0 {
                break;
            }
            let var = ngx_http_add_variable(cf, &mut v.name, v.flags);
            if var.is_null() {
                return core::Status::NGX_ERROR.into();
            }
            (*var).get_handler = v.get_handler;
            (*var).data = v.data;
        }
        core::Status::NGX_OK.into()
    }

    unsafe extern "C" fn postconfiguration(cf: *mut ngx_conf_t) -> ngx_int_t {
        let cmcf = http::ngx_http_conf_get_module_main_conf(cf, &ngx_http_core_module);

        let h = ngx_array_push(
            &mut (*cmcf).phases[ngx_http_phases_NGX_HTTP_ACCESS_PHASE as usize].handlers,
        ) as *mut ngx_http_handler_pt;
        if h.is_null() {
            return core::Status::NGX_ERROR.into();
        }

        // set an Access phase handler
        *h = Some(summon_app_access_handler);

        core::Status::NGX_OK.into()
    }
}

#[no_mangle]
static ngx_http_summonapp_module_ctx: ngx_http_module_t = ngx_http_module_t {
    preconfiguration: Some(Module::preconfiguration),
    postconfiguration: Some(Module::postconfiguration),
    create_main_conf: Some(Module::create_main_conf),
    init_main_conf: Some(Module::init_main_conf),
    create_srv_conf: Some(Module::create_srv_conf),
    merge_srv_conf: Some(Module::merge_srv_conf),
    create_loc_conf: Some(Module::create_loc_conf),
    merge_loc_conf: Some(Module::merge_loc_conf),
};

ngx_modules!(ngx_http_summonapp_module);

#[no_mangle]
pub static mut ngx_http_summonapp_module: ngx_module_t = ngx_module_t {
    ctx_index: ngx_uint_t::max_value(),
    index: ngx_uint_t::max_value(),
    name: std::ptr::null_mut(),
    spare0: 0,
    spare1: 0,
    version: nginx_version as ngx_uint_t,
    signature: NGX_RS_MODULE_SIGNATURE.as_ptr() as *const c_char,

    ctx: &ngx_http_summonapp_module_ctx as *const _ as *mut _,
    commands: unsafe { &ngx_http_summonapp_commands[0] as *const _ as *mut _ },
    type_: NGX_HTTP_MODULE as ngx_uint_t,

    init_master: None,
    init_module: None,
    init_process: None,
    init_thread: None,
    exit_thread: None,
    exit_process: None,
    exit_master: None,

    spare_hook0: 0,
    spare_hook1: 0,
    spare_hook2: 0,
    spare_hook3: 0,
    spare_hook4: 0,
    spare_hook5: 0,
    spare_hook6: 0,
    spare_hook7: 0,
};

impl http::Merge for ModuleConfig {
    fn merge(&mut self, _prev: &ModuleConfig) -> Result<(), MergeConfigError> {
        Ok(())
    }
}


fn is_process_running(d: sysinfo::Pid) -> bool {
    let mut system = System::new_all();
    system.refresh_all();

    system.process(d).is_some()
}


fn get_or_spawn_process(request: &http::Request, host: &str, co: &ModuleConfig) -> u32 {
    let new_ctx = request.pool().allocate::<NgxModuleCtx>(Default::default());

    if new_ctx.is_null() {
        return NGX_OK;
    }

    spawn_once(idling_killer);

    let prockey = format!("{}:{}", host, co.command);
    if let Some(mut ctx) = GLOBAL_PROCESSES.get_mut(&prockey) {
        // eprintln!("Killerfew examiane {:?}, {:?}", GLOBAL_PROCESSES.len(), ctx);
        if is_process_running(ctx.pid.into()) {
            unsafe {
                (*new_ctx).save(ctx.port, &mut request.pool());
                request.set_module_ctx(new_ctx as *mut c_void, &ngx_http_summonapp_module);
            };
            ctx.lastreq = now_timestamp();
            return NGX_OK;
        }
        ngx_log_debug_http!(request, "process is dead {}", ctx.pid);
        drop(ctx);
    }

    let port = find_free_port().expect("Unable to get free port");
    ngx_log_debug_http!(request, "spawning at port {}: '{}'", port, prockey);

    let pid = spawn_process(co, port, request);

    wait_for_connection(port, request, host);

    unsafe {
        (*new_ctx).save(port, &mut request.pool());
        request.set_module_ctx(new_ctx as *mut c_void, &ngx_http_summonapp_module);
    };
    GLOBAL_PROCESSES.insert(prockey, SafeModuleCtx{
        pid,
        port,
        timeout: co.idle_timeout,
        lastreq: now_timestamp(),
    });

    NGX_OK
}

http_request_handler!(summon_app_access_handler, |request: &mut http::Request| {
    let co = unsafe { request.get_module_loc_conf::<ModuleConfig>(&ngx_http_summonapp_module) };
    let co = co.expect("module config is none");
    let host = get_host(request);
    if co.user != "" && co.command != "" {
        if let Some(h) = host {
            get_or_spawn_process(request, h.to_str().unwrap(), co);
            core::Status::NGX_OK
        } else {
            core::Status::NGX_DECLINED
        }
    } else {
        core::Status::NGX_DECLINED
    }
});

#[no_mangle]
static mut ngx_http_summonapp_vars: [ngx_http_variable_t; 2] = [
    ngx_http_variable_t {
        name: ngx_string!("summon_port"),
        set_handler: None,
        get_handler: Some(ngx_http_summonapp_port_variable),
        data: 0,
        flags: 0,
        index: 0,
    },
    ngx_http_null_variable!(),
];

http_variable_get!(
    ngx_http_summonapp_port_variable,
    |request: &mut http::Request, v: *mut ngx_variable_value_t, _: usize| {
        let ctx = unsafe { request.get_module_ctx::<NgxModuleCtx>(&ngx_http_summonapp_module) };

        if let Some(obj) = ctx {
            ngx_log_debug_http!(request, "summon: found port {}", obj.port.to_string());
            obj.bind_port(v);
            return core::Status::NGX_OK;
        }
        ngx_log_debug_http!(request, "summon: no found context");

        core::Status::NGX_ERROR
    }
);

