use std::{collections::HashMap, fs, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Cache {
    #[serde(default)]
    pub ui: HashMap<String, f64>,
    #[serde(default)]
    pub i18n: HashMap<String, f64>,
    #[serde(default)]
    pub assets: HashMap<String, f64>,
}

impl Cache {
    pub fn is_empty(&self) -> bool {
        self.ui.is_empty() && self.i18n.is_empty() && self.assets.is_empty()
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
    Ok(())
}
