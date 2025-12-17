mod app;
mod cache;
mod cli;
mod context;
mod errcode;
mod qt;

use crate::{app::run, errcode::exit_with_error};

fn main() {
    exit_with_error(run());
}
