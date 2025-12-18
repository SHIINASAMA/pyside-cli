use std::{fs, path::Path, time::UNIX_EPOCH};

pub fn get_file_mtime(path: &Path) -> f64 {
    match fs::metadata(&path) {
        Ok(meta) => match meta.modified() {
            Ok(time) => match time.duration_since(UNIX_EPOCH) {
                Ok(dur) => dur.as_secs() as f64 + dur.subsec_nanos() as f64 * 1e-9,
                Err(_) => 0.0,
            },
            Err(_) => 0.0,
        },
        Err(_) => 0.0,
    }
}
