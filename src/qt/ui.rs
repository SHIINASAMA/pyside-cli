use std::{fs, path::Path, process::Command};

use crate::{
    cache::Cache,
    errcode::{Errcode, GeneralErrorKind, ToolchainErrorKind},
    files::Files,
    run_and_wait,
};

pub fn convert_ui_files(
    root: &Path,
    uic: &Path,
    files: &Files,
    cache: &mut Cache,
) -> Result<(), Errcode> {
    let ui_dir = root.join("ui");
    let res_dir = root.join("resources");

    if !ui_dir.exists() || !ui_dir.is_dir() {
        log::info!("No UI files found, skipping.");
        return Ok(());
    }

    if files.ui_list.is_empty() {
        log::info!("No UI files found, skipping.");
        return Ok(());
    }

    if !res_dir.exists() || !res_dir.exists() {
        fs::create_dir_all(&res_dir)
            .map_err(|_| Errcode::GeneralError(GeneralErrorKind::CreateFileFailed))?;
    }

    for input_file in &files.ui_list {
        let rel_path = match input_file
            .parent()
            .and_then(|p| p.strip_prefix(&ui_dir).ok())
        {
            Some(p) => p,
            None => return Err(Errcode::GeneralError(GeneralErrorKind::FileNameInvaild)),
        };

        let output_dir = res_dir.join(rel_path);
        fs::create_dir_all(&output_dir)
            .map_err(|_| Errcode::GeneralError(GeneralErrorKind::CreateFileFailed))?;

        let output_file = output_dir.join(format!(
            "{}_ui.py",
            input_file
                .file_stem()
                .and_then(|s| s.to_str())
                .ok_or(Errcode::GeneralError(GeneralErrorKind::FileNameInvaild))?
        ));

        let key = input_file.to_string_lossy().to_string();

        if !cache.check_ui_file(&key) {
            log::info!("{} is up to date.", key);
            continue;
        }

        run_and_wait!(
            Command::new(uic)
                .arg(input_file)
                .arg("-o")
                .arg(&output_file),
            Errcode::ToolchainError(ToolchainErrorKind::UicFailed)
        )?;

        log::info!(
            "Converted {} to {}.",
            input_file.display(),
            output_file.display()
        );
    }

    Ok(())
}
