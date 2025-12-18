use std::{fs, path::Path, process::Command};

use crate::errcode::{Errcode, GeneralErrorKind, ToolchainErrorKind};
use crate::utils::get_file_mtime;
use crate::{cache::Cache, files::Files};

pub fn generate_i18n_ts_files(
    root: &Path,
    lupdate: &Path,
    files: &Files,
    languages: Vec<String>,
) -> Result<(), Errcode> {
    let i18n_dir = root.join("i18n");
    fs::create_dir_all(&i18n_dir)
        .map_err(|_| Errcode::GeneralError(GeneralErrorKind::CreateFileFailed))?;

    for lang in languages {
        let ts_file = i18n_dir.join(format!("{}.ts", lang));
        log::info!("Generating {} ...", ts_file.display());
        let mut cmd = Command::new(lupdate)
            .arg("-silent")
            .arg("-locations")
            .arg("absolute")
            .arg("-extensions")
            .arg("-ui")
            .args(&files.source_list)
            .args(&files.ui_list)
            .arg("-ts")
            .arg(ts_file.clone())
            .spawn()
            .map_err(|_| Errcode::ToolchainError(ToolchainErrorKind::LUpdateFailed))?;
        cmd.wait()
            .map_err(|_| Errcode::ToolchainError(ToolchainErrorKind::LUpdateFailed))?;
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

        let ts_mtime = get_file_mtime(ts_file);

        let key = ts_file
            .strip_prefix(root)
            .unwrap()
            .to_string_lossy()
            .to_string();
        let pre_ts_time = match cache.i18n.get(&key) {
            Some(t) => t.clone(),
            None => 0.0,
        };

        if pre_ts_time >= ts_mtime {
            log::info!("{} is up to date.", key);
            continue;
        }
        let qm_file = qm_root.join(format!("{}.qm", qm_filename.to_string_lossy()));
        log::info!("Compiling {} to {}.", ts_file.display(), qm_file.display());
        let mut cmd = Command::new(&lrelease)
            .arg(ts_file)
            .arg("-qm")
            .arg(&qm_file)
            .spawn()
            .map_err(|_| Errcode::ToolchainError(ToolchainErrorKind::LReleaseFailed))?;
        cmd.wait()
            .map_err(|_| Errcode::ToolchainError(ToolchainErrorKind::LReleaseFailed))?;
        log::info!("Compiled .qm file: {}.", qm_file.display());

        cache.i18n.insert(key, ts_mtime);
    }

    Ok(())
}
