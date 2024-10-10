use env_logger;

use clap::Parser;
use std::{env};

mod core;

pub mod structs;
pub mod lib;
pub mod constants;

fn setup() -> () {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace")).init();
}

fn main() {
    let arguments: structs::Arguments = structs::Arguments::parse();

    setup();
    core::load(&arguments);
}
