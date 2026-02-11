use std::{
    path::{Path, PathBuf},
    process::Command,
};

use crate::{
    builder::builder::Builder,
    cli::BuildType,
    errcode::{Errcode, GeneralErrorKind, ToolchainErrorKind},
    run_tool,
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
        build_type: BuildType,
        extra_options: Vec<String>,
    ) -> Result<Self, Errcode> {
        let (build_type_str, work_dir) = match build_type {
            BuildType::Onefile => ("--onefile", "build/pyinstaller_onefile_build"),
            BuildType::Onedir => ("--onedir", "build/pyinstaller_onedir_build"),
            BuildType::Bundle => {
                return Err(Errcode::ToolchainError(
                    ToolchainErrorKind::PyInstallerUnsupportedBundle,
                ));
            }
        };

        let mut options = vec![
            build_type_str.into(),
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

        Ok(PyInstallerBuilder {
            target_name: target_name.to_string(),
            _target_dir: target_dir.to_string(),
            exec: pyinstaller_exec.to_path_buf(),
            options: options,
        })
    }
}

impl Builder for PyInstallerBuilder {
    fn pre_build(&self) -> Result<(), Errcode> {
        Ok(())
    }

    fn build(&self) -> Result<(), Errcode> {
        run_tool!(&self.exec, Command::new(&self.exec).args(&self.options));
        Ok(())
    }

    fn post_build(&self) -> Result<(), Errcode> {
        let build_dir = Path::new("build");
        let target_spec_file = build_dir.join(format!("{}.spec", self.target_name));
        if target_spec_file.exists() {
            log::debug!("Removing old target spec file.");
            std::fs::remove_file(&target_spec_file).map_err(|e| {
                Errcode::GeneralError(GeneralErrorKind::RemoveFileFailed {
                    path: target_spec_file,
                    source: e,
                })
            })?;
        }
        Ok(())
    }
}
