mod app;
mod cli;
mod context;
mod errcode;

use crate::{app::run, errcode::exit_with_error};

fn main() {
    exit_with_error(run());
}
