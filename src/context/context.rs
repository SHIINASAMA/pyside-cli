use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::{
    cli::Args,
    context::{pyproject::PyProjectConfig, toolchain::Toolchain},
    errcode::Errcode,
};

pub struct Context {
    pub source_list: Vec<PathBuf>,
    pub ui_list: Vec<PathBuf>,
    pub asset_list: Vec<PathBuf>,
    pub i18n_list: Vec<PathBuf>,

    pub args: Args,
    pub toolchain: Toolchain,
    pub pyconfig: PyProjectConfig,
}

impl Context {
    pub fn new(args: Args) -> Result<Self, Errcode> {
        let toolchain = Toolchain::new();
        log::debug!("Toolchain: {:?}", toolchain);

        let pyconfig = PyProjectConfig::new("pyproject.toml".into())?;

        Ok(Self {
            source_list: Vec::new(),
            ui_list: Vec::new(),
            asset_list: Vec::new(),
            i18n_list: Vec::new(),
            args: args,
            toolchain: toolchain,
            pyconfig: pyconfig,
        })
    }

    pub fn glob_files(&mut self, root: &Path) {
        let assets_dir = root.join("assets");
        let i18n_dir = root.join("i18n");

        let exclude_dirs = [root.join("resources"), root.join("test")];

        for entry in WalkDir::new(root).into_iter().filter_map(Result::ok) {
            let path = entry.path();

            if exclude_dirs.iter().any(|ex| path.starts_with(ex)) {
                continue;
            }

            if !path.is_file() {
                continue;
            }

            // assets
            if path.starts_with(&assets_dir) {
                self.asset_list.push(path.to_path_buf());
                continue;
            }

            // i18n
            if path.starts_with(&i18n_dir) {
                self.i18n_list.push(path.to_path_buf());
                continue;
            }

            // source / ui
            match path.extension().and_then(|s| s.to_str()) {
                Some("py") => self.source_list.push(path.to_path_buf()),
                Some("ui") => self.ui_list.push(path.to_path_buf()),
                _ => {}
            }
        }
    }
}
