use std::fmt;

#[derive(Debug, Copy, Clone)]
pub enum GeneralErrorKind {
    WorkDirNotFound,
    TargetNotFound,
    CreateFileFailed,
    RemoveFileFailed,
    ReadFileFailed,
    MoveFileFailed,
    FileNameInvaild,
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
