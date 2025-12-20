use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    thread,
};

use crate::{
    builder::builder::Builder,
    errcode::{Errcode, GeneralErrorKind, ToolchainErrorKind},
    run_and_wait,
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
        let build_dir = Path::new("build");
        let new_target = build_dir.join(&self.target_name);
        if new_target.exists() {
            if new_target.is_dir() {
                log::debug!("Removing old target directory.");
                fs::remove_dir_all(&new_target)
                    .map_err(|_| Errcode::GeneralError(GeneralErrorKind::RemoveFileFailed))?;
            } else {
                log::debug!("Removing old target file.");
                fs::remove_file(&new_target)
                    .map_err(|_| Errcode::GeneralError(GeneralErrorKind::RemoveFileFailed))?;
            }
        }
        Ok(())
    }

    fn build(&self) -> Result<(), Errcode> {
        run_and_wait!(
            Command::new(&self.exec).args(&self.options),
            Errcode::ToolchainError(ToolchainErrorKind::NuitkaFailed)
        )?;
        Ok(())
    }

    fn post_build(&self) -> Result<(), Errcode> {
        let build_dir = Path::new("build");
        let old_target = build_dir.join(format!("{}.dist", &self.target_dir));
        let new_target = build_dir.join(&self.target_name);
        if !self.onefile {
            log::debug!(
                "Renaming {} to {}.",
                old_target.display(),
                new_target.display()
            );
            fs::rename(old_target, new_target)
                .map_err(|_| Errcode::GeneralError(GeneralErrorKind::MoveFileFailed))?;
        }
        Ok(())
    }
}
