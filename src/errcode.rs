use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum GeneralErrorKind {
    #[error("Failed to change working directory to {path:?}")]
    WorkDirNotFound {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("Target not found: {target:?}")]
    TargetNotFound { target: String },
    #[error("Failed to create file at {path:?}")]
    CreateFileFailed {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("Failed to remove file at {path:?}")]
    RemoveFileFailed {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("Failed to read file at {path:?}")]
    ReadFileFailed {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("Failed to write file at {path:?}")]
    WriteFileFailed {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("Failed to move file from {from:?} to {to:?}")]
    MoveFileFailed {
        from: PathBuf,
        to: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("File name is invalid: {name:?}")]
    FileNameInvalid { name: PathBuf },
    #[error("Unsupported platform: {msg:?}")]
    UnsupportedPlatform { msg: String },
}

#[derive(Debug, Error)]
pub enum PyProjectErrorKind {
    #[error("Failed to parse TOML file")]
    ParseFailed {
        #[source]
        source: toml::de::Error,
    },
    #[error("Failed to parse TOML file")]
    TomlEditParseFailed {
        #[source]
        source: toml_edit::TomlError,
    },
    #[error("Field not found")]
    FieldNotFound { field: String },
    #[error("Field is invalid")]
    FieldInvalid { field: String },
}

#[derive(Debug, Error)]
pub enum ToolchainErrorKind {
    #[error("LRelease update not found")]
    LReleaseUpdateNotFound,
    #[error("Uic not found")]
    UicNotFound,
    #[error("Rcc not found")]
    RccNotFound,
    #[error("Git not found")]
    GitNotFound,
    #[error("Nuitka not found")]
    NuitkaNotFound,
    #[error("PyInstaller not found")]
    PyInstallerNotFound,
    #[error("PyInstaller unsupported bundle")]
    PyInstallerUnsupportedBundle,
    #[error("PyTest not found")]
    PyTestNotFound,

    #[error("{execution_name} execution failed")]
    ExecutionFailed {
        execution_name: String,
        #[source]
        source: std::io::Error,
    },

    #[error("{execution_name} execution failed with non-zero exit status")]
    NonZeroExit {
        execution_name: String,
        exit_status: std::process::ExitStatus,
    },
}

#[derive(Debug)]
#[allow(unused)]
pub enum Errcode {
    GeneralError(GeneralErrorKind),
    PyProjectConfigError(PyProjectErrorKind),
    ToolchainError(ToolchainErrorKind),
}

pub fn exit_with_error(result: Result<(), Errcode>) {
    match result {
        Ok(()) => {}
        Err(err) => {
            log::error!("{:?}", err);
            std::process::exit(1);
        }
    }
}
