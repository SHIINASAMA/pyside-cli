use std::io::{self, Write};

use comfy_table::{Table, presets::UTF8_FULL};

use crate::{
    cli::Command, cli::parse_cli, context::context::Context, errcode::Errcode,
    errcode::InvalidArgumentKind,
};

pub fn run() -> Result<(), Errcode> {
    let args = parse_cli()?;

    if let Some(path) = &args.work_dir {
        log::info!("Working directory set to {}", path);
        std::env::set_current_dir(path)
            .map_err(|_| Errcode::InvalidArgument(InvalidArgumentKind::WorkDirNotFound));
    };

    let context = Context::new(args)?;

    match context.args.command {
        Command::Targets => {
            log::info!("Available targets");
            let mut table = Table::new();
            table
                .load_preset(UTF8_FULL)
                .set_header(vec!["Target Name", "Path"]);

            for (key, value) in &context.pyconfig.scripts {
                table.add_row(vec![key.as_str(), value.display().to_string().as_str()]);
            }

            let mut out = io::stdout().lock();
            writeln!(out, "{table}").unwrap();
        }
        _ => {}
    }

    Ok(())
}
