use crate::{builder::builder::Builder, errcode::Errcode};

pub struct PyInstallerBuilder {}

impl PyInstallerBuilder {
    pub fn new() -> Self {
        PyInstallerBuilder {}
    }
}

impl Builder for PyInstallerBuilder {
    fn pre_build(&self) -> Result<(), Errcode> {
        Ok(())
    }

    fn build(&self) -> Result<(), Errcode> {
        Ok(())
    }

    fn post_build(&self) -> Result<(), Errcode> {
        Ok(())
    }
}
