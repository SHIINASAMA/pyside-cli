use std::time::Instant;

use crate::{
    cli::I18nOptions,
    errcode::{Errcode, GeneralErrorKind},
    files::Files,
    pyproject::PyProjectConfig,
    qt::i18n::generate_i18n_ts_files,
    toolchain::Toolchain,
    utils::format_duration,
};

pub fn action(opt: I18nOptions) -> Result<(), Errcode> {
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
        return Err(Errcode::GeneralError(GeneralErrorKind::TargetNotFound));
    };

    let files = Files::new(root);

    log::info!("Generating i18n files...");
    let start = Instant::now();
    generate_i18n_ts_files(root, &lupdate, &files, pyproject_config.languages)?;
    log::info!(
        "I18n files generated in {}.",
        format_duration(start.elapsed())
    );

    Ok(())
}
