use env_logger;

use std::env;
use log::{info, error, warn};

mod core;
mod arguments;

pub mod lib;

fn setup() -> () {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace")).init();
}

fn main() {
    let mut arguments_manager: arguments::Arguments = arguments::Arguments::new(
        env::args().skip(1).collect(),
        vec!["--directory"]
    );

    setup();

    arguments_manager.load();
}
