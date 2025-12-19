use std::{
    fs::{self, File},
    io::Write,
    path::Path,
    process::Command,
};

use walkdir::WalkDir;

use crate::{
    cache::Cache,
    errcode::{Errcode, GeneralErrorKind, ToolchainErrorKind},
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

fn touch_init_py(resources_dir: &Path) -> Result<(), Errcode> {
    let init_file = resources_dir.join("__init__.py");
    if !init_file.exists() {
        fs::File::create(&init_file)
            .map_err(|_| Errcode::GeneralError(GeneralErrorKind::CreateFileFailed))?;
    }

    // Walk all subdirectories
    for entry in WalkDir::new(resources_dir)
        .into_iter()
        .filter_map(Result::ok)
    {
        if entry.file_type().is_dir() {
            let init_file = entry.path().join("__init__.py");
            if !init_file.exists() {
                fs::File::create(init_file)
                    .map_err(|_| Errcode::GeneralError(GeneralErrorKind::CreateFileFailed))?;
            }
        }
    }
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

    if !cache.check_all_assets(files) {
        log::info!("Assets are up to date, skipping.");
        return Ok(());
    }

    generate_assets_qrc(root, files)?;

    let res_dir = root.join("resources");
    let py_res_file = res_dir.join("resource.py");
    if !res_dir.exists() {
        fs::create_dir_all(&res_dir)
            .map_err(|_| Errcode::GeneralError(GeneralErrorKind::CreateFileFailed))?;
    }

    // TODO: Write version.py

    let mut cmd = Command::new(&rcc)
        .arg(root.join("assets").join("assets.qrc"))
        .arg("-o")
        .arg(py_res_file)
        .spawn()
        .map_err(|_| Errcode::ToolchainError(ToolchainErrorKind::RccFailed))?;
    cmd.wait()
        .map_err(|_| Errcode::ToolchainError(ToolchainErrorKind::RccFailed))?;

    touch_init_py(&res_dir)?;

    Ok(())
}
