use crate::utils::{parse_duration_to_seconds, parse_on_off_to_bool};
use crate::ModuleConfig;
use std::os::raw::{c_char, c_void};
use ngx::{ngx_null_command, ngx_string};
use ngx::ffi::{
    ngx_command_t, ngx_conf_t, ngx_str_t, ngx_uint_t, NGX_CONF_TAKE1, NGX_HTTP_LOC_CONF, NGX_RS_HTTP_LOC_CONF_OFFSET
};

#[no_mangle]
pub static mut ngx_http_summonapp_commands: [ngx_command_t; 9] = [
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

        let val = (*args.add(1)).to_str() as &str;

        conf.idle_timeout = parse_duration_to_seconds(val).unwrap_or(60 * 15);
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

        let val = (*args.add(1)).to_str() as &str;

        conf.min_instance = val.parse().unwrap_or(1);
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

        let val = (*args.add(1)).to_str() as &str;

        conf.use_port = val.parse().unwrap_or(0);
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

        let val = (*args.add(1)).to_str() as &str;

        conf.show_crash = parse_on_off_to_bool(val).unwrap_or(false);
    };

    std::ptr::null_mut()
}

