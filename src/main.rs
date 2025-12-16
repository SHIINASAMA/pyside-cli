mod cli;
mod context;

use clap::Parser;
use env_logger;

use crate::{cli::Cli, context::toolchain::Toolchain};

fn main() {
    env_logger::Builder::from_default_env()
        .filter(None, log::LevelFilter::Debug)
        .init();
    let _args = Cli::parse();
    let _toolchain = Toolchain::new();
    log::info!("{:?}", _toolchain);
}
