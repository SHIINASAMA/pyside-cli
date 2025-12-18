use std::path::Path;

use crate::{cache::Cache, errcode::Errcode, files::Files};

pub fn convert_ui_files(
    root: &Path,
    uic: &Path,
    files: &Files,
    cache: &mut Cache,
) -> Result<(), Errcode> {
    Ok(())
}
