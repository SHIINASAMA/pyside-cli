use crate::context::{pyproject::{PyProjectConfig, read_pyconfig}, toolchain::Toolchain};

pub struct Context {
    pub toolchain: Toolchain,
    pub pyconfig: PyProjectConfig,
}

impl Context {
    pub fn new() -> Self {
        let toolchain = Toolchain::new();
        log::debug!("Toolchain: {:?}", toolchain);

        let pyconfig = read_pyconfig("pyproject.toml".into());
        Self {
            toolchain: toolchain,
            pyconfig: pyconfig,
        }
    }
}
