use env_logger;
use log::{info, error, warn};

fn setup() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace")).init();
}

fn main() {
    setup();
}
