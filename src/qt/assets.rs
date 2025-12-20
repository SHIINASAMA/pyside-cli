use std::{
    fs::{self, File},
    io::Write,
    path::Path,
    process::{Command, Stdio},
};

use walkdir::WalkDir;

use crate::{
    cache::Cache,
    errcode::{Errcode, GeneralErrorKind, ToolchainErrorKind},
    files::Files,
    run_and_wait,
};

macro_rules! my_write {
    ($file:expr, $($arg:tt)*) => {
        write!($file, $($arg)*)
            .map_err(|_| Errcode::GeneralError(GeneralErrorKind::CreateFileFailed))
    };
}

fn generate_assets_qrc(root: &Path, files: &Files) -> Result<(), Errcode> {
    let res_dir = root.join("resources");
    let assets_dir = root.join("assets");
    let qrc_file = assets_dir.join("assets.qrc");

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

    for asset in &files.asset_list {
        // alias = path relative to assets/
        let alias = asset
            .strip_prefix(&assets_dir)
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

fn touch_version_py(resources_dir: &Path, git: &Path) -> Result<(), Errcode> {
    let version_py = resources_dir.join("version.py");
    let version = get_last_tag(git, "0.0.0.0");

    let mut f = File::create(version_py)
        .map_err(|_| Errcode::GeneralError(GeneralErrorKind::CreateFileFailed))?;
    my_write!(f, "__version__ = '{}'\n", version)?;

    Ok(())
}

fn get_last_tag(git: &Path, default: &str) -> String {
    let output = Command::new(git)
        .args(["describe", "--tags", "--abbrev=0", "--first-parent"])
        .stderr(Stdio::null())
        .output();

    let output = match output {
        Ok(o) if o.status.success() => o,
        _ => return default.to_string(),
    };

    let tag = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if tag.is_empty() {
        default.to_string()
    } else {
        tag
    }
}

pub fn compile_resources(
    root: &Path,
    rcc: &Path,
    git: &Path,
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

    run_and_wait!(
        Command::new(&rcc)
            .arg(root.join("assets").join("assets.qrc"))
            .arg("-o")
            .arg(py_res_file),
        Errcode::ToolchainError(ToolchainErrorKind::RccFailed)
    )?;

    touch_version_py(&res_dir, git)?;
    touch_init_py(&res_dir)?;

    Ok(())
}
