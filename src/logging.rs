use env_logger::Env;
pub use log::{debug, error, info, trace, warn};

pub fn init_logging() {
    let env = Env::default().filter_or("MULINK_LOG_LVL", "info");
    env_logger::init_from_env(env);
}

pub fn init_tracing() {
    let env = Env::default().filter_or("MULINK_LOG_LVL", "trace");
    env_logger::init_from_env(env);
}