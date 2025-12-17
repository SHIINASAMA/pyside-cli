use crate::{
    cli::Args,
    context::{pyproject::PyProjectConfig, toolchain::Toolchain},
    errcode::Errcode,
};

pub struct Context {
    pub args: Args,
    pub toolchain: Toolchain,
    pub pyconfig: PyProjectConfig,
}

impl Context {
    pub fn new(args: Args) -> Result<Self, Errcode> {
        let toolchain = Toolchain::new();
        log::debug!("Toolchain: {:?}", toolchain);

        let pyconfig = PyProjectConfig::new("pyproject.toml".into())?;
        Ok(Self {
            args: args,
            toolchain: toolchain,
            pyconfig: pyconfig,
        })
    }
}
