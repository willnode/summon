use std::os::raw::c_void;

use ngx::{ffi::{in_port_t, ngx_str_t, ngx_uint_t, ngx_variable_value_t}, ngx_null_string};
use ngx::core;

pub struct Module;

#[derive(Debug, Default)]
pub struct ModuleConfig {
    pub command: String,
    pub root: String,
    pub user: String,
    pub log: String,
    pub idle_timeout: String,
    pub min_instance: String,
    pub use_port: String,
    pub show_crash: String,
}

#[derive(Debug)]
pub struct ModuleCtx {
    pub pid: ngx_uint_t,
    pub port: ngx_str_t,
}

#[derive(Debug)]
pub struct SafeModuleCtx {
    pub pid: usize,
    pub port: u16,
}


impl Default for ModuleCtx {
    fn default() -> ModuleCtx {
        ModuleCtx {
            pid: 0,
            port: ngx_null_string!(),
        }
    }
}


impl ModuleCtx {
    pub fn save(&mut self, pid: usize, port: in_port_t, pool: &mut core::Pool) -> core::Status {
        self.pid = pid;
        let port_str = port.to_string();
        let port_data = pool.alloc(port_str.len());
        if port_data.is_null() {
            return core::Status::NGX_ERROR;
        }
        unsafe { libc::memcpy(port_data, port_str.as_bytes().as_ptr() as *const c_void, port_str.len()) };
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