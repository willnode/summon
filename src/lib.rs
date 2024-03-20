mod prop;
mod types;

use prop::*;
use sysinfo::System;
use types::*;

use lazy_static::lazy_static;
use ngx::core::NgxStr;
use ngx::ffi::{
    nginx_version, ngx_array_push, ngx_conf_t, ngx_http_add_variable, ngx_http_core_module,
    ngx_http_handler_pt, ngx_http_module_t, ngx_http_phases_NGX_HTTP_ACCESS_PHASE,
    ngx_http_request_t, ngx_http_variable_t, ngx_int_t, ngx_module_t, ngx_uint_t,
    ngx_variable_value_t, NGX_DECLINED, NGX_HTTP_MODULE, NGX_OK, NGX_RS_MODULE_SIGNATURE,
};
use ngx::http::{MergeConfigError, Request};
use ngx::{core, core::Status, http, http::HTTPModule};
use ngx::{
    http_request_handler, http_variable_get, ngx_http_null_variable, ngx_log_debug_http,
    ngx_modules, ngx_string,
};
use std::collections::HashMap;
use std::env;
use std::net::{SocketAddr, TcpListener};
use std::os::raw::{c_char, c_void};
use std::process::{Child, Command};
use std::sync::{Arc, Mutex};

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

        env::set_var("RUST_BACKTRACE", "1");

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

pub fn get_host(request: &Request) -> Option<&NgxStr> {
    if !request.get_inner().headers_in.user_agent.is_null() {
        unsafe {
            Some(NgxStr::from_ngx_str(
                (*request.get_inner().headers_in.host).value,
            ))
        }
    } else {
        None
    }
}

lazy_static! {
    static ref GLOBAL_PROCESSES: Mutex<HashMap<String, Arc<Mutex<Child>>>> =
        Mutex::new(HashMap::new());
}

fn is_process_running(d: sysinfo::Pid) -> bool {
    let mut system = System::new_all();
    system.refresh_all();

    system.process(d).is_some()
}

fn get_or_spawn_process(request: &http::Request, co: &ModuleConfig) -> u32 {
    if let Some(ctx) = unsafe { request.get_module_ctx::<ModuleCtx>(&ngx_http_summonapp_module) } {
        if is_process_running(ctx.pid.into()) {
            return NGX_OK;
        }
    }

    let port = find_free_port().expect("Unable to get free port");
    ngx_log_debug_http!(request, "spawning at port {}: '{}'", port, co.command);

    let new_ctx = request.pool().allocate::<ModuleCtx>(Default::default());

    if new_ctx.is_null() {
        return NGX_OK;
    }

    // Command to run should be adjusted according to your actual command.
    let mut childp = Command::new("/bin/bash");

    let fcmd = format!("source /etc/profile ; source ~/.profile ; {}", co.command);

    childp
        .args(&["-c", &fcmd])
        .env("HOME", format!("/home/{}", co.user))
    .env("PORT", port.to_string());

    ngx_log_debug_http!(request, "env cmd: {:#?}", fcmd);

    let child = childp.spawn().expect("Failed to spawn process");

    unsafe {
        (*new_ctx).save(child.id(), port, &mut request.pool());
        request.set_module_ctx(new_ctx as *mut c_void, &ngx_http_summonapp_module);
    };
    NGX_OK
}

http_request_handler!(summon_app_access_handler, |request: &mut http::Request| {
    let co = unsafe { request.get_module_loc_conf::<ModuleConfig>(&ngx_http_summonapp_module) };
    let co = co.expect("module config is none");
    get_or_spawn_process(request, co);

    core::Status::NGX_DONE
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
        let ctx = unsafe { request.get_module_ctx::<ModuleCtx>(&ngx_http_summonapp_module) };

        if let Some(obj) = ctx {
            ngx_log_debug_http!(request, "httporigdst: found context and binding variable");
            obj.bind_port(v);
            return core::Status::NGX_OK;
        }
        core::Status::NGX_OK
    }
);

fn find_free_port() -> Result<u16, std::io::Error> {
    // Bind to port 0; the OS will assign a free port
    let listener = TcpListener::bind("127.0.0.1:0")?;

    // Retrieve the assigned port
    match listener.local_addr()? {
        SocketAddr::V4(addr) => Ok(addr.port()),
        SocketAddr::V6(addr) => Ok(addr.port()),
    }
}
