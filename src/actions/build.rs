use std::time::Instant;

use crate::{
    cache::{Cache, load_cache, save_cache},
    cli::{BuildOptions, BuildStage},
    errcode::{CacheErrorKind, Errcode, InvalidArgumentKind, ToolchainErrorKind},
    files::Files,
    pyproject::PyProjectConfig,
    qt::{i18n::compile_i18n_ts_files, ui::convert_ui_files},
    toolchain::Toolchain,
};

pub fn action(opt: &BuildOptions) -> Result<(), Errcode> {
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

    // I18N
    if matches!(opt.stage, BuildStage::I18n | BuildStage::All) {
        log::info!("Compiling i18n files...");
        let start = Instant::now();
        compile_i18n_ts_files(root, &lrelease, &files, &mut cache)?;
        log::info!("I18n files compiled in {}ms.", start.elapsed().as_millis());
    }

    // UI
    let uic = match &toolchain.uic {
        Some(uic) => uic.clone(),
        None => {
            return Err(Errcode::ToolchainError(ToolchainErrorKind::UicNotFound));
        }
    };
    if matches!(opt.stage, BuildStage::Ui | BuildStage::All) {
        log::info!("Compiling ui files...");
        let start = Instant::now();
        convert_ui_files(root, &uic, &files, &mut cache)?;
        log::info!("Ui files compiled in {}ms.", start.elapsed().as_millis());
    }

    let _ = save_cache(&cache).map_err(|_| Errcode::CacheError(CacheErrorKind::SaveFailed));
    Ok(())
}
