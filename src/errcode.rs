use std::fmt;

#[derive(Debug, Copy, Clone)]
pub enum InvalidArgumentKind {
    WorkDirNotFound,
}

#[derive(Debug, Copy, Clone)]
pub enum PyProjectErrorKind {
    ReadFaild,
    ParseFailed,
    MissingField,
}

#[derive(Debug)]
#[allow(unused)]
pub enum Errcode {
    InvalidArgument(InvalidArgumentKind),
    PyProjectConfigError(PyProjectErrorKind),
    IoError(std::io::Error),
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
