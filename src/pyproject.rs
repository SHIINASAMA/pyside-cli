use std::fs::read_to_string;
use std::{collections::HashMap, path::PathBuf};

use serde::Deserialize;

use crate::errcode::Errcode;

#[derive(Debug, Deserialize)]
struct PyProject {
    pub project: Option<Project>,
    pub tool: Option<Tool>,
}

#[derive(Debug, Deserialize)]
struct Project {
    pub scripts: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
struct Tool {
    #[serde(rename = "pyside-cli")]
    pub pyside_cli: Option<PySideCli>,
}

#[derive(Debug, Deserialize)]
struct PySideCli {
    pub i18n: Option<I18n>,

    pub pyinstaller: Option<PyInstaller>,

    #[serde(flatten)]
    pub options: HashMap<String, toml::Value>,
}

#[derive(Debug, Deserialize)]
struct PyInstaller {
    #[serde(flatten)]
    pub options: HashMap<String, toml::Value>,
}

#[derive(Debug, Deserialize)]
struct I18n {
    pub languages: Option<Vec<String>>,
}

pub struct PyProjectConfig {
    pub scripts: HashMap<String, PathBuf>,
    pub languages: Vec<String>,
    pub extra_nuitka_options_list: Vec<String>,
    pub extra_pyinstaller_options_list: Vec<String>,
}

use crate::errcode::PyProjectErrorKind;

impl PyProjectConfig {
    pub fn new(path: PathBuf) -> Result<Self, Errcode> {
        let toml_content = read_to_string(path)
            .map_err(|_| Errcode::PyProjectConfigError(PyProjectErrorKind::ReadFaild))?;
        let cfg: PyProject = toml::from_str(&toml_content)
            .map_err(|_| Errcode::PyProjectConfigError(PyProjectErrorKind::ParseFailed))?;

        let platform = std::env::consts::OS;

        let scripts = parse_scripts(&cfg)?;

        Ok(Self {
            scripts: scripts,
            languages: get_languages(&cfg).unwrap_or_default().to_vec(),
            extra_nuitka_options_list: get_extra_nuitka_options_for_platform(&cfg, platform)
                .unwrap_or_default()
                .to_vec(),
            extra_pyinstaller_options_list: get_extra_pyinstaller_options_for_platform(
                &cfg, platform,
            )
            .unwrap_or_default()
            .to_vec(),
        })
    }
}

fn resolve_package_path(entry_point: &str) -> String {
    // Split the entry point to get the module path (before the colon)
    let module_path = entry_point.split(':').next().unwrap_or("");
    let parts: Vec<&str> = module_path.split('.').collect();

    // Find the deepest package that exists (contains __init__.py)
    for i in (1..=parts.len()).rev() {
        let package_name = parts[..i].join(".");
        let package_dir: PathBuf = package_name.replace('.', "/").into();
        let init_file = package_dir.join("__init__.py");

        if init_file.exists() {
            return package_name;
        }
    }

    // If no package with __init__.py found, return the first part
    parts.first().map(|s| s.to_string()).unwrap_or_default()
}

fn parse_scripts(config: &PyProject) -> Result<HashMap<String, PathBuf>, Errcode> {
    let raw_scripts = match get_scripts(&config) {
        Some(scripts) => scripts,
        None => {
            return Err(Errcode::PyProjectConfigError(
                PyProjectErrorKind::FieldNotFound,
            ));
        }
    };

    let mut result = HashMap::new();

    for (name, entry_point) in raw_scripts {
        // Resolve package name, e.g. "cli.__main__:main" -> "cli"
        let package_name = resolve_package_path(entry_point);

        if package_name.is_empty() {
            return Err(Errcode::PyProjectConfigError(
                PyProjectErrorKind::FieldNotFound,
            ));
        }

        // Convert package name to path: cli.sub -> cli/sub
        let package_path: PathBuf = package_name.replace('.', "/").into();

        result.insert(name.clone(), package_path);
    }

    Ok(result)
}

fn flatten_backend_options(cfg: &[(&String, &toml::Value)]) -> Vec<String> {
    let mut opts = Vec::new();

    for (key, val) in cfg {
        match val {
            toml::Value::Boolean(true) => opts.push(format!("--{}", key)),
            toml::Value::String(s) => opts.push(format!("--{}={}", key, s)),
            toml::Value::Array(arr) => {
                let joined = arr
                    .iter()
                    .filter_map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                opts.push(format!("--{}={}", key, joined));
            }
            _ => {} // ignore false/null/unsupported types
        }
    }

    opts
}

fn get_extra_options_for_platfrom(
    values: &HashMap<String, toml::Value>,
    platform: &str,
) -> Vec<String> {
    let key = match platform {
        "windows" => "win32",
        "linux" => "linux",
        "macos" => "darwin",
        other => other,
    };

    let mut opts = Vec::new();

    let non_tables: Vec<(&String, &toml::Value)> = values
        .iter()
        .filter(|(_, v)| v.as_table().is_none())
        .collect();
    opts.append(&mut flatten_backend_options(&non_tables));

    if let Some(toml::Value::Table(table)) = values.get(key) {
        let platform_entries: Vec<(&String, &toml::Value)> = table.iter().collect();
        opts.append(&mut flatten_backend_options(&platform_entries));
    }

    opts
}

fn get_extra_nuitka_options_for_platform(
    config: &PyProject,
    platform: &str,
) -> Option<Vec<String>> {
    let platforms = &config.tool.as_ref()?.pyside_cli.as_ref()?.options;

    Some(get_extra_options_for_platfrom(platforms, platform))
}

fn get_extra_pyinstaller_options_for_platform(
    config: &PyProject,
    platform: &str,
) -> Option<Vec<String>> {
    let platforms = &config
        .tool
        .as_ref()?
        .pyside_cli
        .as_ref()?
        .pyinstaller
        .as_ref()?
        .options;

    Some(get_extra_options_for_platfrom(&platforms, platform))
}

fn get_languages<'a>(config: &'a PyProject) -> Option<&'a [String]> {
    config
        .tool
        .as_ref()?
        .pyside_cli
        .as_ref()?
        .i18n
        .as_ref()?
        .languages
        .as_deref()
}

fn get_scripts<'a>(config: &'a PyProject) -> Option<&'a HashMap<String, String>> {
    config.project.as_ref()?.scripts.as_ref()
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_parsing_pyproject_i18n() {
        let pyproject = r#"
            [tool.pyside-cli.i18n]
            languages = ["en_US", "zh_CN"]
        "#;

        let project: PyProject = toml::from_str(pyproject).unwrap();
        let languages = get_languages(&project).unwrap_or_default();
        assert_eq!(languages, &["en_US", "zh_CN"]);
    }

    #[test]
    fn test_extra_nuitka_options_platforms() {
        let pyproject_toml = r#"
            [tool.pyside-cli]
            onefile=true
            standalone=true

            [tool.pyside-cli.win32]
            windows-flag=true

            [tool.pyside-cli.linux]
            linux-flag=true

            [tool.pyside-cli.darwin]
            macos-flag=true
        "#;

        let config: PyProject = toml::from_str(pyproject_toml).unwrap();
        let windows_options = get_extra_nuitka_options_for_platform(&config, "windows")
            .unwrap_or_default()
            .to_vec();
        assert!(
            windows_options.contains(&"--windows-flag".to_string()),
            "Windows options missing"
        );

        let linux_options = get_extra_nuitka_options_for_platform(&config, "linux")
            .unwrap_or_default()
            .to_vec();
        assert!(
            linux_options.contains(&"--linux-flag".to_string()),
            "Linux options missing"
        );

        let macos_options = get_extra_nuitka_options_for_platform(&config, "darwin")
            .unwrap_or_default()
            .to_vec();
        assert!(
            macos_options.contains(&"--macos-flag".to_string()),
            "MacOS options missing"
        );
    }
}
