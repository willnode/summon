mod prop;
mod types;

use prop::*;
use types::*;

use ngx::core::NgxStr;
use ngx::ffi::{
    nginx_version, ngx_array_push, ngx_command_t, ngx_conf_t, ngx_http_core_module, ngx_http_handler_pt,
    ngx_http_module_t, ngx_http_phases_NGX_HTTP_ACCESS_PHASE, ngx_http_request_t, ngx_int_t, ngx_module_t, ngx_str_t,
    ngx_uint_t, NGX_CONF_TAKE1, NGX_HTTP_LOC_CONF, NGX_HTTP_MODULE, NGX_RS_HTTP_LOC_CONF_OFFSET,
    NGX_RS_MODULE_SIGNATURE,
};
use ngx::http::{MergeConfigError, Request};
use ngx::{core, core::Status, http, http::HTTPModule};
use ngx::{http_request_handler, ngx_log_debug_http, ngx_modules, ngx_null_command, ngx_string};
use std::collections::HashMap;
use std::os::raw::{c_char, c_void};
use std::process::{Child, Command};
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use std::env;


impl http::HTTPModule for Module {
    type MainConf = ();
    type SrvConf = ();
    type LocConf = ModuleConfig;

    unsafe extern "C" fn postconfiguration(cf: *mut ngx_conf_t) -> ngx_int_t {
        let cmcf = http::ngx_http_conf_get_module_main_conf(cf, &ngx_http_core_module);

        let h = ngx_array_push(&mut (*cmcf).phases[ngx_http_phases_NGX_HTTP_ACCESS_PHASE as usize].handlers)
            as *mut ngx_http_handler_pt;
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
static mut ngx_http_summonapp_commands: [ngx_command_t; 9] = [
    ngx_command_t {
        name: ngx_string!("summon_app_command"),
        type_: (NGX_HTTP_LOC_CONF | NGX_CONF_TAKE1) as ngx_uint_t,
        set: Some(ngx_http_summon_app_set_command),
        conf: NGX_RS_HTTP_LOC_CONF_OFFSET,
        offset: 0,
        post: std::ptr::null_mut(),
    },
    ngx_command_t {
        name: ngx_string!("summon_app_root"),
        type_: (NGX_HTTP_LOC_CONF | NGX_CONF_TAKE1) as ngx_uint_t,
        set: Some(ngx_http_summon_app_set_root),
        conf: NGX_RS_HTTP_LOC_CONF_OFFSET,
        offset: 0,
        post: std::ptr::null_mut(),
    },
    ngx_command_t {
        name: ngx_string!("summon_app_user"),
        type_: (NGX_HTTP_LOC_CONF | NGX_CONF_TAKE1) as ngx_uint_t,
        set: Some(ngx_http_summon_app_set_user),
        conf: NGX_RS_HTTP_LOC_CONF_OFFSET,
        offset: 0,
        post: std::ptr::null_mut(),
    },
    ngx_command_t {
        name: ngx_string!("summon_app_log"),
        type_: (NGX_HTTP_LOC_CONF | NGX_CONF_TAKE1) as ngx_uint_t,
        set: Some(ngx_http_summon_app_set_log),
        conf: NGX_RS_HTTP_LOC_CONF_OFFSET,
        offset: 0,
        post: std::ptr::null_mut(),
    },
    ngx_command_t {
        name: ngx_string!("summon_app_idle_timeout"),
        type_: (NGX_HTTP_LOC_CONF | NGX_CONF_TAKE1) as ngx_uint_t,
        set: Some(ngx_http_summon_app_set_idle_timeout),
        conf: NGX_RS_HTTP_LOC_CONF_OFFSET,
        offset: 0,
        post: std::ptr::null_mut(),
    },
    ngx_command_t {
        name: ngx_string!("summon_app_min_instance"),
        type_: (NGX_HTTP_LOC_CONF | NGX_CONF_TAKE1) as ngx_uint_t,
        set: Some(ngx_http_summon_app_set_min_instance),
        conf: NGX_RS_HTTP_LOC_CONF_OFFSET,
        offset: 0,
        post: std::ptr::null_mut(),
    },
    ngx_command_t {
        name: ngx_string!("summon_app_use_port"),
        type_: (NGX_HTTP_LOC_CONF | NGX_CONF_TAKE1) as ngx_uint_t,
        set: Some(ngx_http_summon_app_set_use_port),
        conf: NGX_RS_HTTP_LOC_CONF_OFFSET,
        offset: 0,
        post: std::ptr::null_mut(),
    },
    ngx_command_t {
        name: ngx_string!("summon_app_show_crash"),
        type_: (NGX_HTTP_LOC_CONF | NGX_CONF_TAKE1) as ngx_uint_t,
        set: Some(ngx_http_summon_app_set_show_crash),
        conf: NGX_RS_HTTP_LOC_CONF_OFFSET,
        offset: 0,
        post: std::ptr::null_mut(),
    },
    ngx_null_command!(),
];

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
    fn merge(&mut self, prev: &ModuleConfig) -> Result<(), MergeConfigError> {
        Ok(())
    }
}

pub fn get_host(request: &Request) -> Option<&NgxStr> {
    if !request.get_inner().headers_in.user_agent.is_null() {
        unsafe { Some(NgxStr::from_ngx_str((*request.get_inner().headers_in.host).value)) }
    } else {
        None
    }
}

lazy_static! {
    static ref GLOBAL_PROCESSES: Mutex<HashMap<String, Arc<Mutex<Child>>>> = Mutex::new(HashMap::new());
}

fn get_or_spawn_process(host: &str, command: &str) -> Arc<Mutex<Child>> {
    let mut processes = GLOBAL_PROCESSES.lock().unwrap();
    if let Some(process) = processes.get(host) {
        let mut process_guard = process.lock().unwrap();
        if process_guard.try_wait().ok().flatten().is_none() {
            // Process is still running.
            return process.clone();
        }
        // If process exited, it will fall through to spawning a new one.
    }

    // Command to run should be adjusted according to your actual command.
    let child = Command::new("cmd")
        .args(&["-c", command])
        .spawn()
        .expect("Failed to spawn process");

    let child_arc = Arc::new(Mutex::new(child));
    processes.insert(host.to_string(), child_arc.clone());
    child_arc
}

http_request_handler!(summon_app_access_handler, |request: &mut http::Request| {
    let co = unsafe { request.get_module_loc_conf::<ModuleConfig>(&ngx_http_summonapp_module) };
    let co = co.expect("module config is none");
    if let Some(host) =get_host(&request) {
        if let Ok(host_str) = host.to_str() {
            get_or_spawn_process(host_str, &co.command);
        }
    }

    core::Status::NGX_DECLINED
});
