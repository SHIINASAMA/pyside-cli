use std::{fmt, path::PathBuf};

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
    #[error("Failed to parse TOML file")]
    TomlParseFailed {
        #[source]
        source: toml_edit::TomlError,
    },
}

#[derive(Debug, Copy, Clone)]
pub enum PyProjectErrorKind {
    ReadFaild,
    ParseFailed,
    FieldNotFound,
}

#[derive(Debug, Copy, Clone)]
pub enum CacheErrorKind {
    SaveFailed,
}

#[derive(Debug, Copy, Clone)]
pub enum ToolchainErrorKind {
    LReleaseUpdateNotFound,
    UicNotFound,
    RccNotFound,
    GitNotFound,
    NuitkaNotFound,
    PyInstallerNotFound,
    PyTestNotFound,
    LUpdateFailed,
    LReleaseFailed,
    UicFailed,
    RccFailed,
    GitFailed,
    NuitkaFailed,
    PyInstallerFailed,
    PyTestFailed,
}

#[derive(Debug)]
#[allow(unused)]
pub enum Errcode {
    GeneralError(GeneralErrorKind),
    PyProjectConfigError(PyProjectErrorKind),
    CacheError(CacheErrorKind),
    ToolchainError(ToolchainErrorKind),
}

impl fmt::Display for Errcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            // Errcode::InvalidArgument(arg) => write!(f, "Invalid argument: {}", arg),
            _ => write!(f, "{:?}", self),
        }
    }
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
