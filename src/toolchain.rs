use std::{env, path::PathBuf, process::Command};

use serde_json;
use which::which;

/// Check if the current Python interpreter is running in a virtual environment.
fn in_virtual_env() -> bool {
    let output = Command::new("python")
        .args(["-c", "import sys; print(sys.prefix != sys.base_prefix)"])
        .output();

    match output {
        Ok(out) if out.status.success() => String::from_utf8_lossy(&out.stdout).trim() == "True",
        _ => false,
    }
}

/// Query python site-packages via interpreter
fn python_site_packages() -> Vec<PathBuf> {
    let output = Command::new("python")
        .args([
            "-c",
            "import site, json; print(json.dumps(site.getsitepackages()))",
        ])
        .output();

    let Ok(output) = output else {
        return vec![];
    };

    if !output.status.success() {
        return vec![];
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    serde_json::from_str::<Vec<String>>(&stdout)
        .unwrap_or_default()
        .into_iter()
        .map(PathBuf::from)
        .collect()
}

/// Append PySide6 directory to PATH (process-local)
fn add_pyside6_to_path() {
    let site_packages = python_site_packages();

    let mut appended = Vec::new();

    for site in site_packages {
        let pyside = site.join("PySide6");
        if pyside.exists() && pyside.is_dir() {
            appended.push(pyside);
        }
    }

    if appended.is_empty() {
        return;
    }

    let old = env::var_os("PATH").unwrap_or_default();
    let mut paths: Vec<PathBuf> = env::split_paths(&old).collect();

    paths.extend(appended);

    if let Ok(new_path) = env::join_paths(paths) {
        unsafe {
            env::set_var("PATH", new_path);
        }
    }
}

#[derive(Debug)]
pub struct Toolchain {
    pub git: Option<PathBuf>,
    pub uic: Option<PathBuf>,
    pub rcc: Option<PathBuf>,
    pub lupdate: Option<PathBuf>,
    pub lrelease: Option<PathBuf>,
    pub nuitka: Option<PathBuf>,
    pub pyinstaller: Option<PathBuf>,
    pub pytest: Option<PathBuf>,
}

impl Toolchain {
    pub fn new() -> Self {
        if !in_virtual_env() {
            log::warn!("Not running in a virtual environment, missing tools may not be found.");
        }
        add_pyside6_to_path();

        Self {
            git: which("git").ok(),
            uic: which("pyside6-uic").ok(),
            rcc: which("pyside6-rcc").ok(),
            lupdate: which("lupdate").ok(),
            lrelease: which("lrelease").ok(),
            nuitka: which("nuitka").ok(),
            pyinstaller: which("pyinstaller").ok(),
            pytest: which("pytest").ok(),
        }
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_toolchain_new() -> Result<(), String> {
        let toolchain = Toolchain::new();
        let mut errors = Vec::new();

        if toolchain.git.is_none() {
            errors.push("git missing");
        }
        if toolchain.uic.is_none() {
            errors.push("uic missing");
        }
        if toolchain.rcc.is_none() {
            errors.push("rcc missing");
        }
        if toolchain.lupdate.is_none() {
            errors.push("lupdate missing");
        }
        if toolchain.lrelease.is_none() {
            errors.push("lrelease missing");
        }
        if toolchain.nuitka.is_none() {
            errors.push("nuitka missing");
        }
        if toolchain.pyinstaller.is_none() {
            errors.push("pyinstaller missing");
        }
        if toolchain.pytest.is_none() {
            errors.push("pytest missing");
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors.join(", "))
        }
    }
}
