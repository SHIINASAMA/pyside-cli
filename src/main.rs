mod app;
mod cache;
mod cli;
mod errcode;
mod files;
mod pyproject;
mod qt;
mod toolchain;

use crate::{app::run, errcode::exit_with_error};

fn main() {
    exit_with_error(run());
}
