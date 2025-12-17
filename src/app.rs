use std::io::{self, Write};

use comfy_table::{Table, presets::UTF8_FULL};

use crate::{
    cache::load_cache,
    cli::{Command, parse_cli},
    context::context::Context,
    errcode::{Errcode, InvalidArgumentKind},
    qt::i18n::generate_i18n_ts_files,
};

pub fn run() -> Result<(), Errcode> {
    let args = parse_cli()?;

    if let Some(path) = &args.work_dir {
        log::info!("Working directory set to {}", path);
        let _ = std::env::set_current_dir(path)
            .map_err(|_| Errcode::InvalidArgument(InvalidArgumentKind::WorkDirNotFound));
    };

    let mut context = Context::new(args)?;
    let command = context.args.command.clone();

    match command {
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
        Command::I18n(opt) => {
            let lupdate = match &context.toolchain.lupdate {
                Some(lupdate) => lupdate.clone(),
                None => {
                    log::warn!("PySide6 lupdate not found, skipping i18n generation");
                    return Ok(());
                }
            };
            let target = &opt.target;
            let root = &context.pyconfig.scripts[target].clone();
            context.glob_files(root);

            log::info!("Generating i18n files...");
            generate_i18n_ts_files(&context, &lupdate, root)?;
        }
        _ => {}
    }

    Ok(())
}
