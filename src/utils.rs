use std::{
    fs,
    path::Path,
    time::{Duration, UNIX_EPOCH},
};

pub fn get_file_mtime(path: &Path) -> f64 {
    match fs::metadata(&path) {
        Ok(meta) => match meta.modified() {
            Ok(time) => match time.duration_since(UNIX_EPOCH) {
                Ok(dur) => dur.as_secs() as f64 + dur.subsec_nanos() as f64 * 1e-9,
                Err(e) => {
                    log::warn!("Failed to get `{}` duration time: {}", path.display(), e);
                    0.0
                }
            },
            Err(e) => {
                log::warn!(
                    "Failed to get `{}` modification time: {}",
                    path.display(),
                    e
                );
                0.0
            }
        },
        Err(e) => {
            log::warn!("Failed to get `{}` metadata: {}", path.display(), e);
            0.0
        }
    }
}

pub fn format_duration(d: Duration) -> String {
    let ms = d.as_millis();
    let secs = d.as_secs();
    let mins = secs / 60;
    let hours = mins / 60;

    if hours > 0 {
        format!("{}h{}m", hours, mins % 60)
    } else if mins > 0 {
        format!("{}m{}s", mins, secs % 60)
    } else if secs > 0 {
        format!("{}s", secs)
    } else {
        format!("{}ms", ms)
    }
}

#[macro_export]
macro_rules! run_tool {
    ($name:expr, $cmd:expr) => {{
        let mut child = $cmd.spawn().map_err(|e| {
            Errcode::ToolchainError(ToolchainErrorKind::ExecutionFailed {
                execution_name: $name.to_string_lossy().to_string(),
                source: e,
            })
        })?;

        let status = child.wait().map_err(|e| {
            Errcode::ToolchainError(ToolchainErrorKind::ExecutionFailed {
                execution_name: $name.to_string_lossy().to_string(),
                source: e,
            })
        })?;

        if !status.success() {
            return Err(Errcode::ToolchainError(ToolchainErrorKind::NonZeroExit {
                execution_name: $name.to_string_lossy().to_string(),
                exit_status: status,
            }));
        }
    }};
}
