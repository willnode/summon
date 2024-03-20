use crate::ModuleConfig;
use std::os::raw::{c_char, c_void};
use ngx::ffi::{ngx_command_t, ngx_conf_t, ngx_str_t};

#[no_mangle]
pub extern "C" fn ngx_http_summon_app_set_command(
    cf: *mut ngx_conf_t,
    _cmd: *mut ngx_command_t,
    conf: *mut c_void,
) -> *mut c_char {
    unsafe {
        let conf = &mut *(conf as *mut ModuleConfig);
        let args = (*(*cf).args).elts as *mut ngx_str_t;

        let val = (*args.add(1)).to_str();

        // set default value optionally
        conf.command = val.to_string();
    };

    std::ptr::null_mut()
}


#[no_mangle]
pub extern "C" fn ngx_http_summon_app_set_root(
    cf: *mut ngx_conf_t,
    _cmd: *mut ngx_command_t,
    conf: *mut c_void,
) -> *mut c_char {
    unsafe {
        let conf = &mut *(conf as *mut ModuleConfig);
        let args = (*(*cf).args).elts as *mut ngx_str_t;

        let val = (*args.add(1)).to_str();

        // set default value optionally
        conf.root = val.to_string();
    };

    std::ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn ngx_http_summon_app_set_user(
    cf: *mut ngx_conf_t,
    _cmd: *mut ngx_command_t,
    conf: *mut c_void,
) -> *mut c_char {
    unsafe {
        let conf = &mut *(conf as *mut ModuleConfig);
        let args = (*(*cf).args).elts as *mut ngx_str_t;

        let val = (*args.add(1)).to_str();

        conf.user = val.to_string();
    };

    std::ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn ngx_http_summon_app_set_log(
    cf: *mut ngx_conf_t,
    _cmd: *mut ngx_command_t,
    conf: *mut c_void,
) -> *mut c_char {
    unsafe {
        let conf = &mut *(conf as *mut ModuleConfig);
        let args = (*(*cf).args).elts as *mut ngx_str_t;

        let val = (*args.add(1)).to_str();

        conf.log = val.to_string();
    };

    std::ptr::null_mut()
}


#[no_mangle]
pub extern "C" fn ngx_http_summon_app_set_idle_timeout(
    cf: *mut ngx_conf_t,
    _cmd: *mut ngx_command_t,
    conf: *mut c_void,
) -> *mut c_char {
    unsafe {
        let conf = &mut *(conf as *mut ModuleConfig);
        let args = (*(*cf).args).elts as *mut ngx_str_t;

        let val = (*args.add(1)).to_str();

        conf.idle_timeout = val.to_string();
    };

    std::ptr::null_mut()
}


#[no_mangle]
pub extern "C" fn ngx_http_summon_app_set_min_instance(
    cf: *mut ngx_conf_t,
    _cmd: *mut ngx_command_t,
    conf: *mut c_void,
) -> *mut c_char {
    unsafe {
        let conf = &mut *(conf as *mut ModuleConfig);
        let args = (*(*cf).args).elts as *mut ngx_str_t;

        let val = (*args.add(1)).to_str();

        conf.min_instance = val.to_string();
    };

    std::ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn ngx_http_summon_app_set_use_port(
    cf: *mut ngx_conf_t,
    _cmd: *mut ngx_command_t,
    conf: *mut c_void,
) -> *mut c_char {
    unsafe {
        let conf = &mut *(conf as *mut ModuleConfig);
        let args = (*(*cf).args).elts as *mut ngx_str_t;

        let val = (*args.add(1)).to_str();

        conf.use_port = val.to_string();
    };

    std::ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn ngx_http_summon_app_set_show_crash(
    cf: *mut ngx_conf_t,
    _cmd: *mut ngx_command_t,
    conf: *mut c_void,
) -> *mut c_char {
    unsafe {
        let conf = &mut *(conf as *mut ModuleConfig);
        let args = (*(*cf).args).elts as *mut ngx_str_t;

        let val = (*args.add(1)).to_str();

        conf.show_crash = val.to_string();
    };

    std::ptr::null_mut()
}
