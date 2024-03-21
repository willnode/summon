use chashmap::CHashMap;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use crate::SafeModuleCtx;

type ThreadControl = Arc<Mutex<Option<JoinHandle<()>>>>;

lazy_static! {
    pub static ref GLOBAL_PROCESSES: CHashMap<String, SafeModuleCtx> =
        CHashMap::new();
    pub static ref THREAD_CONTROL: ThreadControl = Arc::new(Mutex::new(None));
}
