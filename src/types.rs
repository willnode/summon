use std::os::raw::c_void;

use ngx::core;
use ngx::{
    ffi::{in_port_t, ngx_str_t, ngx_variable_value_t},
    ngx_null_string,
};

pub struct Module;

#[derive(Debug, Clone)]
pub struct ModuleConfig {
    pub command: String,
    pub root: String,
    pub user: String,
    pub log: String,
    pub idle_timeout: u64,
    pub min_instance: u64,
    pub use_port: u16,
    pub show_crash: bool,
}

impl Default for ModuleConfig {
    fn default() -> ModuleConfig {
        ModuleConfig {
            command: "".to_string(),
            root: "".to_string(),
            user: "".to_string(),
            log: "".to_string(),
            idle_timeout: 15 * 10,
            min_instance: 1,
            use_port: 0,
            show_crash: false,
        }
    }
}

#[derive(Debug)]
pub struct NgxModuleCtx {
    pub port: ngx_str_t,
}

#[derive(Debug, Default, Clone)]
pub struct SafeModuleCtx {
    pub pid: usize,
    pub port: u16,
    pub timeout: u64,
    pub lastreq: u64,
}

impl Default for NgxModuleCtx {
    fn default() -> NgxModuleCtx {
        NgxModuleCtx {
            port: ngx_null_string!(),
        }
    }
}

impl NgxModuleCtx {
    pub fn save(&mut self, port: in_port_t, pool: &mut core::Pool) -> core::Status {
        let port_str = port.to_string();
        let port_data = pool.alloc(port_str.len());
        if port_data.is_null() {
            return core::Status::NGX_ERROR;
        }
        unsafe {
            libc::memcpy(
                port_data,
                port_str.as_bytes().as_ptr() as *const c_void,
                port_str.len(),
            )
        };
        self.port.len = port_str.len();
        self.port.data = port_data as *mut u8;

        core::Status::NGX_OK
    }

    pub unsafe fn bind_port(&self, v: *mut ngx_variable_value_t) {
        if self.port.len == 0 {
            (*v).set_not_found(1);
            return;
        }

        (*v).set_valid(1);
        (*v).set_no_cacheable(0);
        (*v).set_not_found(0);
        (*v).set_len(self.port.len as u32);
        (*v).data = self.port.data;
    }
}
