use std::io::{self, Write};

use comfy_table::{Table, presets::UTF8_FULL};

use crate::{errcode::Errcode, pyproject::PyProjectConfig};

pub fn action() -> Result<(), Errcode> {
    let pyproject_config = PyProjectConfig::new("pyproject.toml".into())?;
    log::info!("Available targets");
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_header(vec!["Target Name", "Path"]);

    for (key, value) in &pyproject_config.scripts {
        table.add_row(vec![key.as_str(), value.display().to_string().as_str()]);
    }

    let mut out = io::stdout().lock();
    writeln!(out, "{table}").unwrap();
    Ok(())
}
