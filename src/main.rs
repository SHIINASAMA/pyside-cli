mod cli;
mod context;
mod errcode;

use crate::{cli::parse_cli, context::context::Context};

fn main() {
    let _args = parse_cli();
    let _context = Context::new();
}
