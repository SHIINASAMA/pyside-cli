use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct Files {
    pub source_list: Vec<PathBuf>,
    pub ui_list: Vec<PathBuf>,
    pub asset_list: Vec<PathBuf>,
    pub i18n_list: Vec<PathBuf>,
}

impl Files {
    pub fn new(root: &Path) -> Self {
        let assets_dir = root.join("assets");
        let i18n_dir = root.join("i18n");

        let mut source_list = Vec::new();
        let mut ui_list = Vec::new();
        let mut asset_list = Vec::new();
        let mut i18n_list = Vec::new();

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
                asset_list.push(path.to_path_buf());
                continue;
            }

            // i18n
            if path.starts_with(&i18n_dir) {
                i18n_list.push(path.to_path_buf());
                continue;
            }

            // source / ui
            match path.extension().and_then(|s| s.to_str()) {
                Some("py") => source_list.push(path.to_path_buf()),
                Some("ui") => ui_list.push(path.to_path_buf()),
                _ => {}
            }
        }

        log::debug!("Source list: {:?}", source_list);
        log::debug!("UI list: {:?}", ui_list);
        log::debug!("Asset list: {:?}", asset_list);
        log::debug!("I18n list: {:?}", i18n_list);

        Self {
            source_list: source_list,
            ui_list: ui_list,
            asset_list: asset_list,
            i18n_list: i18n_list,
        }
    }
}
