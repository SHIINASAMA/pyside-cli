use std::{
    path::{Path, PathBuf},
    process::Command,
};

use crate::{
    builder::builder::Builder,
    errcode::{Errcode, GeneralErrorKind, ToolchainErrorKind},
    run_and_wait,
};

pub struct PyInstallerBuilder {
    target_name: String,
    _target_dir: String,
    exec: PathBuf,
    options: Vec<String>,
}

impl PyInstallerBuilder {
    pub fn new(
        target_name: &str,
        target_dir: &str,
        pyinstaller_exec: &Path,
        onefile: bool,
        extra_options: Vec<String>,
    ) -> Self {
        let work_dir = if onefile {
            "build/pyinstaller_onefile_build"
        } else {
            "build/pyinstaller_onedir_build"
        };

        let mut options = vec![
            if onefile {
                "--onefile".into()
            } else {
                "--onedir".into()
            },
            "--distpath".into(),
            "build".into(),
            "--workpath".into(),
            work_dir.to_string(),
            "--noconfirm".into(),
            // "--log-level".into(),
            // if debug { "DEBUG" } else { "INFO" }.into(),
            "--name".into(),
            target_name.to_string(),
            format!("{}/__main__.py", target_dir),
        ];

        options.extend(extra_options);

        log::debug!("Build options: {:?}", options);

        PyInstallerBuilder {
            target_name: target_name.to_string(),
            _target_dir: target_dir.to_string(),
            exec: pyinstaller_exec.to_path_buf(),
            options: options,
        }
    }
}

impl Builder for PyInstallerBuilder {
    fn pre_build(&self) -> Result<(), Errcode> {
        Ok(())
    }

    fn build(&self) -> Result<(), Errcode> {
        run_and_wait!(
            Command::new(&self.exec).args(&self.options),
            Errcode::ToolchainError(ToolchainErrorKind::PyInstallerFailed)
        )?;
        Ok(())
    }

    fn post_build(&self) -> Result<(), Errcode> {
        let build_dir = Path::new("build");
        let target_spec_file = build_dir.join(format!("{}.spec", self.target_name));
        if target_spec_file.exists() {
            log::debug!("Removing old target spec file.");
            std::fs::remove_file(target_spec_file)
                .map_err(|_| Errcode::GeneralError(GeneralErrorKind::RemoveFileFailed))?;
        }
        Ok(())
    }
}
