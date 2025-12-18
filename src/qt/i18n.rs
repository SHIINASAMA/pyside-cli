use std::{fs, path::Path, process::Command};

use crate::{
    errcode::{Errcode, I18nErrorKind},
    files::Files,
};

pub fn generate_i18n_ts_files(
    root: &Path,
    lupdate: &Path,
    files: &Files,
    languages: Vec<String>,
) -> Result<(), Errcode> {
    let i18n_dir = root.join("i18n");
    fs::create_dir_all(&i18n_dir).map_err(|_| Errcode::I18nError(I18nErrorKind::CreateFailed))?;

    let langs = languages;
    for lang in langs {
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
            .map_err(|_| Errcode::I18nError(I18nErrorKind::LUpdateFailed))?;
        cmd.wait()
            .map_err(|_| Errcode::I18nError(I18nErrorKind::LUpdateFailed))?;
        log::info!("Generated translation file: {}", ts_file.display())
    }

    Ok(())
}
