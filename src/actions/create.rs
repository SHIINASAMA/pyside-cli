use std::{env::current_dir, path::Path, process::Command};

use toml_edit::{DocumentMut, value};

use crate::{
    errcode::{Errcode, GeneralErrorKind, ToolchainErrorKind},
    run_and_wait,
    toolchain::Toolchain,
};

pub fn action(name: String) -> Result<(), Errcode> {
    let toolchain = Toolchain::new();
    let git = match toolchain.git {
        Some(git) => git,
        None => {
            return Err(Errcode::ToolchainError(
                crate::errcode::ToolchainErrorKind::GitNotFound,
            ));
        }
    };

    let (project_name, dst) = if name == "." {
        (
            current_dir().unwrap().to_string_lossy().to_string(),
            ".".into(),
        )
    } else {
        (name.clone(), name.clone())
    };

    log::info!("Creating project: {}.", project_name);

    run_and_wait!(
        Command::new(&git)
            .arg("clone")
            .arg("https://github.com/SHIINASAMA/pyside_template.git")
            .arg(&dst),
        Errcode::ToolchainError(ToolchainErrorKind::GitFailed)
    )?;

    let project_path = Path::new(&project_name);
    let pyproject_file = project_path.join("pyproject.toml");
    let toml_text = std::fs::read_to_string(&pyproject_file).map_err(|e| {
        Errcode::GeneralError(GeneralErrorKind::ReadFileFailed {
            path: pyproject_file.clone(),
            source: e,
        })
    })?;
    let mut doc = toml_text
        .parse::<DocumentMut>()
        .map_err(|e| Errcode::GeneralError(GeneralErrorKind::TomlParseFailed { source: e }))?;

    doc["project"]["name"] = value(&project_name);

    std::fs::write(&pyproject_file, doc.to_string()).map_err(|e| {
        Errcode::GeneralError(GeneralErrorKind::WriteFileFailed {
            path: pyproject_file.clone(),
            source: e,
        })
    })?;

    log::debug!("Remove old .git directory.");
    let git_dir = project_path.join(".git");
    std::fs::remove_dir_all(&git_dir).map_err(|e| {
        Errcode::GeneralError(GeneralErrorKind::RemoveFileFailed {
            path: git_dir,
            source: e,
        })
    })?;

    log::info!("Initializing new git repository.");
    run_and_wait!(
        Command::new(&git).arg("init").current_dir(&project_path),
        Errcode::ToolchainError(ToolchainErrorKind::GitFailed)
    )?;

    log::info!("Project created successfully.");

    Ok(())
}
