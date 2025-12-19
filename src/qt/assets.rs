use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use crate::{
    cache::Cache,
    errcode::{Errcode, GeneralErrorKind},
    files::Files,
};

macro_rules! my_write {
    ($file:expr, $($arg:tt)*) => {
        write!($file, $($arg)*)
            .map_err(|_| Errcode::GeneralError(GeneralErrorKind::CreateFileFailed))
    };
}

fn generate_assets_qrc(root: &Path, files: &Files) -> Result<(), Errcode> {
    let res_dir = root.join("resources");
    let qrc_file = res_dir.join("assets.qrc");

    fs::create_dir_all(&res_dir)
        .map_err(|_| Errcode::GeneralError(GeneralErrorKind::CreateFileFailed))?;

    let mut f = File::create(qrc_file)
        .map_err(|_| Errcode::GeneralError(GeneralErrorKind::CreateFileFailed))?;

    my_write!(
        f,
        "<!DOCTYPE RCC>
<RCC version=\"1.0\">
  <qresource>"
    )?;

    let assets_root = root.join("assets");
    for asset in &files.asset_list {
        // alias = path relative to assets/
        let alias = asset
            .strip_prefix(&assets_root)
            .unwrap_or(asset)
            .to_string_lossy()
            .replace('\\', "/");
        // rel_path = ../assets/xxx/yyy
        let rel_path = Path::new("..")
            .join("assets")
            .join(&alias)
            .to_string_lossy()
            .replace('\\', "/");
        my_write!(f, "  <file alias=\"{}\">{}</file>", alias, rel_path)?;
    }

    my_write!(
        f,
        "</qresource>
</RCC>"
    )?;

    Ok(())
}

pub fn compile_resources(
    root: &Path,
    rcc: &Path,
    files: &Files,
    cache: &mut Cache,
) -> Result<(), Errcode> {
    if files.asset_list.is_empty() {
        log::info!("No assets found, skipping.");
        return Ok(());
    }

    let res_dir = root.join("resources");
    let py_res_file = res_dir.join("resources.py");

    Ok(())
}
