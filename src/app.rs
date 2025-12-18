use std::io::{self, Write};
use std::time::Instant;

use comfy_table::{Table, presets::UTF8_FULL};

use crate::cache::{Cache, save_cache};
use crate::errcode::{CacheErrorKind, ToolchainErrorKind};
use crate::qt::i18n::compile_i18n_ts_files;
use crate::{
    cache::load_cache,
    cli::{Command, parse_cli},
    errcode::{Errcode, InvalidArgumentKind},
    files::Files,
    pyproject::PyProjectConfig,
    qt::i18n::generate_i18n_ts_files,
    toolchain::Toolchain,
};

pub fn run() -> Result<(), Errcode> {
    let args = parse_cli()?;

    if let Some(path) = &args.work_dir {
        log::info!("Working directory set to {} .", path);
        let _ = std::env::set_current_dir(path)
            .map_err(|_| Errcode::InvalidArgument(InvalidArgumentKind::WorkDirNotFound));
    };

    match &args.command {
        Command::Targets => {
            let pyproject_config = PyProjectConfig::new("pyproject.toml".into())?;
            log::info!("Available targets");
            let mut table = Table::new();
            table
                .load_preset(UTF8_FULL)
                .set_header(vec!["Target Name", "Path"]);

            for (key, value) in &pyproject_config.scripts {
                table.add_row(vec![key.as_str(), value.display().to_string().as_str()]);
            }

            let mut out = io::stdout().lock();
            writeln!(out, "{table}").unwrap();
        }
        Command::I18n(opt) => {
            let toolchain = Toolchain::new();
            let lupdate = match &toolchain.lupdate {
                Some(lupdate) => lupdate.clone(),
                None => {
                    log::warn!("PySide6 lupdate not found, skipping i18n generation.");
                    return Ok(());
                }
            };
            let pyproject_config = PyProjectConfig::new("pyproject.toml".into())?;
            let Some(root) = &pyproject_config.scripts.get(&opt.target) else {
                return Err(Errcode::InvalidArgument(
                    InvalidArgumentKind::TargetNotFound,
                ));
            };

            let files = Files::new(root);

            log::info!("Generating i18n files...");
            let start = Instant::now();
            generate_i18n_ts_files(root, &lupdate, &files, pyproject_config.languages)?;
            log::info!("I18n files generated in {}ms.", start.elapsed().as_millis());
        }
        Command::Rc(opt) => {
            let toolchain = Toolchain::new();
            let lrelease = match &toolchain.lrelease {
                Some(lrelease) => lrelease.clone(),
                None => {
                    return Err(Errcode::ToolchainError(
                        ToolchainErrorKind::LReleaseUpdateNotFound,
                    ));
                }
            };
            let pyproject_config = PyProjectConfig::new("pyproject.toml".into())?;
            let Some(root) = &pyproject_config.scripts.get(&opt.target) else {
                return Err(Errcode::InvalidArgument(
                    InvalidArgumentKind::TargetNotFound,
                ));
            };
            let files = Files::new(root);
            let mut cache: Cache = load_cache();

            log::info!("Compiling i18n files...");
            let start = Instant::now();
            compile_i18n_ts_files(root, &lrelease, &files, &mut cache)?;
            log::info!("I18n files compiled in {}ms.", start.elapsed().as_millis());
            let _ = save_cache(&cache).map_err(|_| Errcode::CacheError(CacheErrorKind::SaveFailed));
        }
        _ => {}
    }

    Ok(())
}
