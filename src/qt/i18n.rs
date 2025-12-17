use std::{fs, path::Path, process::Command};

use crate::{context::context::Context, errcode::Errcode};

pub fn generate_i18n_ts_files(ctx: &Context, lupdate: &Path, root: &Path) -> Result<(), Errcode> {
    let i18n_dir = root.join("i18n");
    fs::create_dir_all(&i18n_dir).map_err(|e| Errcode::IoError(e))?;

    let langs = &ctx.pyconfig.languages;
    for lang in langs {
        let ts_file = i18n_dir.join(format!("{}.ts", lang));
        log::info!("Generating {} ...", ts_file.display());
        let mut cmd = Command::new(lupdate)
            .arg("-silent")
            .arg("-locations")
            .arg("absolute")
            .arg("-extensions")
            .arg("-ui")
            .args(&ctx.source_list)
            .args(&ctx.ui_list)
            .arg("-ts")
            .arg(ts_file.clone())
            .spawn()
            .map_err(|e| Errcode::IoError(e))?;
        cmd.wait().map_err(|e| Errcode::IoError(e))?;
        log::info!("Generated translation file: {}", ts_file.display())
    }

    Ok(())
}
