use std::{fs, path::Path, process::Command};

use crate::errcode::{Errcode, GeneralErrorKind, ToolchainErrorKind};
use crate::run_tool;
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
    fs::create_dir_all(&i18n_dir).map_err(|e| {
        Errcode::GeneralError(GeneralErrorKind::CreateFileFailed {
            path: i18n_dir.clone(),
            source: e,
        })
    })?;

    for lang in languages {
        let ts_file = i18n_dir.join(format!("{}.ts", lang));
        log::info!("Generating {} ...", ts_file.display());
        run_tool!(
            &lupdate,
            Command::new(lupdate)
                .arg("-silent")
                .arg("-locations")
                .arg("absolute")
                .arg("-extensions")
                .arg("-ui")
                .args(&files.source_list)
                .args(&files.ui_list)
                .arg("-ts")
                .arg(ts_file.clone())
        );

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
    fs::create_dir_all(&qm_root).map_err(|e| {
        Errcode::GeneralError(GeneralErrorKind::CreateFileFailed {
            path: qm_root.clone(),
            source: e,
        })
    })?;

    for ts_file in &files.i18n_list {
        let Some(qm_filename) = ts_file.file_stem() else {
            return Err(Errcode::GeneralError(GeneralErrorKind::FileNameInvalid {
                name: ts_file.clone(),
            }));
        };

        let key = ts_file.to_string_lossy();

        if !cache.check_i18n_file(&key) {
            log::info!("{} is up to date.", key);
            continue;
        }
        let qm_file = qm_root.join(format!("{}.qm", qm_filename.to_string_lossy()));
        log::info!("Compiling {} to {}.", ts_file.display(), qm_file.display());

        run_tool!(
            &lrelease,
            Command::new(lrelease).arg(ts_file).arg("-qm").arg(&qm_file)
        );

        log::info!("Compiled .qm file: {}.", qm_file.display());
    }

    Ok(())
}
