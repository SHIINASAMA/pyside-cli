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

    #[serde(flatten)]
    pub platforms: HashMap<String, toml::Value>,
}

#[derive(Debug, Deserialize)]
struct I18n {
    pub languages: Option<Vec<String>>,
}

pub struct PyProjectConfig {
    pub languages: Vec<String>,
    pub extra_nuitka_options_list: Vec<String>,
}

impl PyProjectConfig {
    pub fn new(cfg: &PyProject) -> Self {
        let platform = std::env::consts::OS;
        Self {
            languages: Self::get_languages(cfg).unwrap_or_default().to_vec(),
            extra_nuitka_options_list: Self::get_extra_nuitka_options_for_platform(cfg, platform),
        }
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

    fn get_extra_nuitka_options_for_platform(config: &PyProject, platform: &str) -> Vec<String> {
        let platforms = match &config.tool.as_ref().and_then(|t| t.pyside_cli.as_ref()) {
            Some(cli) => &cli.platforms,
            None => return Vec::new(),
        };

        let key = match platform {
            "windows" => "win32",
            "linux" => "linux",
            "macos" => "darwin",
            other => other,
        };

        let mut opts = Vec::new();

        let non_tables: Vec<(&String, &toml::Value)> = platforms
            .iter()
            .filter(|(_, v)| v.as_table().is_none())
            .collect();
        opts.append(&mut Self::flatten_backend_options(&non_tables));

        if let Some(toml::Value::Table(table)) = platforms.get(key) {
            let platform_entries: Vec<(&String, &toml::Value)> = table.iter().collect();
            opts.append(&mut Self::flatten_backend_options(&platform_entries));
        }

        opts
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
}

use std::fs;

pub fn read_pyconfig(path: PathBuf) -> Result<PyProjectConfig, Errcode> {
    let toml_content = fs::read_to_string(path).unwrap();
    let project: PyProject = toml::from_str(&toml_content).unwrap();
    let config = PyProjectConfig::new(&project);
    return Ok(config);
}

mod tests {
    use super::*;

    #[test]
    fn test_parsing_pyproject_i18n() {
        let pyproject = r#"
            [tool.pyside-cli.i18n]
            languages = ["en_US", "zh_CN"]
        "#;

        let project: PyProject = toml::from_str(pyproject).unwrap();
        let languages = PyProjectConfig::get_languages(&project).unwrap_or_default();
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
        let windows_options =
            PyProjectConfig::get_extra_nuitka_options_for_platform(&config, "windows");
        assert!(
            windows_options.contains(&"--windows-flag".to_string()),
            "Windows options missing"
        );

        let linux_options =
            PyProjectConfig::get_extra_nuitka_options_for_platform(&config, "linux");
        assert!(
            linux_options.contains(&"--linux-flag".to_string()),
            "Linux options missing"
        );

        let macos_options =
            PyProjectConfig::get_extra_nuitka_options_for_platform(&config, "darwin");
        assert!(
            macos_options.contains(&"--macos-flag".to_string()),
            "MacOS options missing"
        );
    }
}
