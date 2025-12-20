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
    exec: PathBuf,
    onefile: bool,
    options: Vec<String>,
}

impl NuitkaBuilder {
    pub fn new(
        target_name: &str,
        target_dir: &str,
        nuitka_exe: &Path,
        onefile: bool,
        extra_options: Vec<String>,
    ) -> Self {
        let n = thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1);
        let mut options = vec![
            "--output-dir=build".into(),
            "--output-filename=App".into(),
            target_dir.to_string(),
            if onefile {
                "--onefile".into()
            } else {
                "--standalone".into()
            },
            format!("--jobs={}", n),
        ];

        options.extend(extra_options);

        log::debug!("Build options: {:?}", options);

        NuitkaBuilder {
            target_name: target_name.to_string(),
            target_dir: target_dir.to_string(),
            exec: nuitka_exe.to_path_buf(),
            onefile: onefile,
            options: options,
        }
    }
}

impl Builder for NuitkaBuilder {
    fn pre_build(&self) -> Result<(), Errcode> {
        Ok(())
    }

    fn build(&self) -> Result<(), Errcode> {
        let mut cmd = Command::new(&self.exec)
            .args(&self.options)
            .spawn()
            .map_err(|_| Errcode::ToolchainError(ToolchainErrorKind::NuitkaFailed))?;
        cmd.wait()
            .map_err(|_| Errcode::ToolchainError(ToolchainErrorKind::NuitkaFailed))?;
        Ok(())
    }

    fn post_build(&self) -> Result<(), Errcode> {
        let build_dir = Path::new("build");
        let old_target_dir = build_dir.join(format!("{}.dist", &self.target_dir));
        let new_target_dir = build_dir.join(&self.target_name);
        if !self.onefile {
            if new_target_dir.exists() {
                log::debug!("Removing old target directory.");
                fs::remove_dir_all(&new_target_dir)
                    .map_err(|_| Errcode::GeneralError(GeneralErrorKind::RemoveFileFailed))?;
            }
            log::debug!(
                "Renaming {} to {}.",
                old_target_dir.display(),
                new_target_dir.display()
            );
            fs::rename(old_target_dir, new_target_dir)
                .map_err(|_| Errcode::GeneralError(GeneralErrorKind::MoveFileFailed))?;
        }
        Ok(())
    }
}
