use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    thread,
};

use crate::{
    builder::builder::Builder,
    errcode::{Errcode, GeneralErrorKind, ToolchainErrorKind},
};

pub struct NuitkaBuilder {
    target_name: String,
    target_dir: String,
    nuitka_exe: PathBuf,
    options: Vec<String>,
}

impl NuitkaBuilder {
    pub fn new(
        target_name: &str,
        target_dir: &str,
        nuitka_exe: &Path,
        onefile: &bool,
        extra_options: Vec<String>,
    ) -> Self {
        let n = thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1);
        let mut options = vec![
            "--output-dir=build".into(),
            "--output-filename=App".into(),
            target_dir.to_string(),
            if *onefile {
                "--onefile".into()
            } else {
                "--standalone".into()
            },
            format!("--jobs={}", n),
        ];

        options.extend(extra_options);

        log::debug!("Full build options: {:?}", options);

        NuitkaBuilder {
            target_name: target_name.to_string(),
            target_dir: target_dir.to_string(),
            nuitka_exe: nuitka_exe.to_path_buf(),
            options: options,
        }
    }
}

impl Builder for NuitkaBuilder {
    fn pre_build(&self) -> Result<(), Errcode> {
        Ok(())
    }

    fn build(&self) -> Result<(), Errcode> {
        let mut cmd = Command::new(&self.nuitka_exe)
            .args(&self.options)
            .spawn()
            .map_err(|_| Errcode::ToolchainError(ToolchainErrorKind::NuitkaFailed))?;
        cmd.wait()
            .map_err(|_| Errcode::ToolchainError(ToolchainErrorKind::NuitkaFailed))?;
        Ok(())
    }

    fn post_build(&self) -> Result<(), Errcode> {
        let build_dir = Path::new("build");
        let target_dir = build_dir.join(&self.target_name);
        if target_dir.is_dir() {
            if target_dir.exists() {
                fs::remove_dir_all(&target_dir)
                    .map_err(|_| Errcode::GeneralError(GeneralErrorKind::RemoveFileFailed))?;
            }
            let raw_target_dir = build_dir.join(format!("{}.dist", &self.target_dir));
            fs::rename(raw_target_dir, &target_dir)
                .map_err(|_| Errcode::GeneralError(GeneralErrorKind::MoveFileFailed))?;
        }
        Ok(())
    }
}
