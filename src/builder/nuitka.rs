use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    thread,
};

use crate::{
    builder::builder::Builder,
    cli::BuildType,
    errcode::{Errcode, GeneralErrorKind, ToolchainErrorKind},
    run_tool,
};

pub struct NuitkaBuilder {
    target_name: String,
    target_dir: String,
    exec: PathBuf,
    build_type: BuildType,
    options: Vec<String>,
}

impl NuitkaBuilder {
    pub fn new(
        target_name: &str,
        target_dir: &str,
        nuitka_exe: &Path,
        build_type: BuildType,
        extra_options: Vec<String>,
    ) -> Result<Self, Errcode> {
        let n = thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1);
        let mut options = vec![
            "--output-dir=build".into(),
            "--output-filename=App".into(),
            target_dir.to_string(),
            format!("--jobs={}", n),
        ];

        match build_type {
            BuildType::Onedir => {
                options.push("--onefile".into());
            }
            BuildType::Onefile => {
                options.push("--standalone".into());
            }
            BuildType::Bundle => {
                #[cfg(not(target_os = "macos"))]
                {
                    // never reach here
                    return Err(Errcode::GeneralError(GeneralErrorKind::UnsupportedPlatform));
                }
            }
        };

        options.extend(extra_options);

        log::debug!("Build options: {:?}", options);

        Ok(NuitkaBuilder {
            target_name: target_name.to_string(),
            target_dir: target_dir.to_string(),
            exec: nuitka_exe.to_path_buf(),
            build_type: build_type,
            options: options,
        })
    }
}

impl Builder for NuitkaBuilder {
    fn pre_build(&self) -> Result<(), Errcode> {
        let build_dir = Path::new("build");
        let new_target = build_dir.join(&self.target_name);
        if new_target.exists() {
            if new_target.is_dir() {
                log::debug!("Removing old target directory.");
                fs::remove_dir_all(&new_target).map_err(|e| {
                    Errcode::GeneralError(GeneralErrorKind::RemoveFileFailed {
                        path: new_target.clone(),
                        source: e,
                    })
                })?;
            } else {
                log::debug!("Removing old target file.");
                fs::remove_file(&new_target).map_err(|e| {
                    Errcode::GeneralError(GeneralErrorKind::RemoveFileFailed {
                        path: new_target.clone(),
                        source: e,
                    })
                })?;
            }
        }
        Ok(())
    }

    fn build(&self) -> Result<(), Errcode> {
        run_tool!(&self.exec, Command::new(&self.exec).args(&self.options));
        Ok(())
    }

    fn post_build(&self) -> Result<(), Errcode> {
        let build_dir = Path::new("build");
        let old_target = build_dir.join(format!("{}.dist", &self.target_dir));
        let new_target = build_dir.join(&self.target_name);
        if self.build_type != BuildType::Onefile {
            log::debug!(
                "Renaming {} to {}.",
                old_target.display(),
                new_target.display()
            );
            fs::rename(&old_target, &new_target).map_err(|e| {
                Errcode::GeneralError(GeneralErrorKind::MoveFileFailed {
                    from: old_target,
                    to: new_target,
                    source: e,
                })
            })?;
        }
        Ok(())
    }
}
