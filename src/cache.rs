use std::{collections::HashMap, fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::{files::Files, utils::get_file_mtime};

type CacheMap = HashMap<String, f64>;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Cache {
    #[serde(default)]
    pub ui: CacheMap,
    #[serde(default)]
    pub i18n: CacheMap,
    #[serde(default)]
    pub assets: CacheMap,
}

impl Cache {
    pub fn is_empty(&self) -> bool {
        self.ui.is_empty() && self.i18n.is_empty() && self.assets.is_empty()
    }

    fn check_outdated(file: &str, cache_map: &mut CacheMap) -> bool {
        let mtime = get_file_mtime(Path::new(file));

        match cache_map.get(file) {
            Some(&cached) if cached >= mtime => false,
            _ => {
                cache_map.insert(file.to_owned(), mtime);
                true
            }
        }
    }

    pub fn check_ui_file(&mut self, file: &str) -> bool {
        Self::check_outdated(file, &mut self.ui)
    }

    pub fn check_i18n_file(&mut self, file: &str) -> bool {
        Self::check_outdated(file, &mut self.i18n)
    }

    pub fn check_all_assets(&mut self, files: &Files) -> bool {
        let mut is_outdated = false;

        for asset in &files.asset_list {
            let asset_str = asset.to_string_lossy().to_string();
            if Self::check_outdated(&asset_str, &mut self.assets) {
                is_outdated = true;
            }
        }

        is_outdated
    }
}

pub fn load_cache() -> Cache {
    let cache_dir = Path::new(".cache");
    let cache_file = cache_dir.join("assets.json");

    if let Err(e) = fs::create_dir_all(cache_dir) {
        log::warn!("Failed to create cache dir: {e}");
        return Cache::default();
    }

    if cache_file.exists() {
        log::info!("Cache found.");

        match fs::read_to_string(&cache_file) {
            Ok(content) => match serde_json::from_str::<Cache>(&content) {
                Ok(cache) => {
                    if cache.is_empty() {
                        log::info!("No cache found.");
                    }
                    return cache;
                }
                Err(e) => log::warn!("Failed to parse cache: {e}"),
            },
            Err(e) => log::warn!("Failed to read cache file: {e}"),
        }
    }

    log::info!("No cache found.");
    Cache::default()
}

pub fn save_cache(cache: &Cache) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(cache)?;
    fs::write(".cache/assets.json", json)?;
    log::info!("Cache saved.");
    Ok(())
}
