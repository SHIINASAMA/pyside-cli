mod actions;
mod app;
mod builder;
mod cache;
mod cli;
mod errcode;
mod files;
mod pyproject;
mod qt;
mod toolchain;
mod utils;

use crate::{app::run, errcode::exit_with_error};

fn main() {
    exit_with_error(run());
}
