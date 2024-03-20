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
