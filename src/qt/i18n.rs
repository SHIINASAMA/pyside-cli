use std::{fs, path::Path, process::Command};

use crate::errcode::{Errcode, GeneralErrorKind, ToolchainErrorKind};
use crate::run_and_wait;
use crate::{cache::Cache, files::Files};

pub fn generate_i18n_ts_files(
    root: &Path,
    lupdate: &Path,
    files: &Files,
    languages: Vec<String>,
) -> Result<(), Errcode> {
    if languages.is_empty() {
        log::info!("No languages specified, skipping.");
        return Ok(());
    }

    let i18n_dir = root.join("i18n");
    fs::create_dir_all(&i18n_dir)
        .map_err(|_| Errcode::GeneralError(GeneralErrorKind::CreateFileFailed))?;

    for lang in languages {
        let ts_file = i18n_dir.join(format!("{}.ts", lang));
        log::info!("Generating {} ...", ts_file.display());
        run_and_wait!(
            Command::new(lupdate)
                .arg("-silent")
                .arg("-locations")
                .arg("absolute")
                .arg("-extensions")
                .arg("-ui")
                .args(&files.source_list)
                .args(&files.ui_list)
                .arg("-ts")
                .arg(ts_file.clone()),
            Errcode::ToolchainError(ToolchainErrorKind::LUpdateFailed)
        )?;

        log::info!("Generated translation file: {}", ts_file.display())
    }

    Ok(())
}

pub fn compile_i18n_ts_files(
    root: &Path,
    lrelease: &Path,
    files: &Files,
    cache: &mut Cache,
) -> Result<(), Errcode> {
    let qm_root = root.join("assets").join("i18n");
    fs::create_dir_all(&qm_root)
        .map_err(|_| Errcode::GeneralError(GeneralErrorKind::CreateFileFailed))?;

    for ts_file in &files.i18n_list {
        let Some(qm_filename) = ts_file.file_stem() else {
            return Err(Errcode::GeneralError(GeneralErrorKind::FileNameInvaild));
        };

        let key = ts_file
            .strip_prefix(root)
            .unwrap()
            .to_string_lossy()
            .to_string();

        if !cache.check_i18n_file(&key) {
            log::info!("{} is up to date.", key);
            continue;
        }
        let qm_file = qm_root.join(format!("{}.qm", qm_filename.to_string_lossy()));
        log::info!("Compiling {} to {}.", ts_file.display(), qm_file.display());

        run_and_wait!(
            Command::new(&lrelease)
                .arg(ts_file)
                .arg("-qm")
                .arg(&qm_file),
            Errcode::ToolchainError(ToolchainErrorKind::LReleaseFailed)
        )?;

        log::info!("Compiled .qm file: {}.", qm_file.display());
    }

    Ok(())
}
