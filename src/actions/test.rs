use std::process::Command;

use crate::{
    cli::TestOptions,
    errcode::{Errcode, ToolchainErrorKind},
    toolchain::Toolchain,
};

pub fn action(opt: TestOptions) -> Result<(), Errcode> {
    let toolchain = Toolchain::new();
    let pytest = match &toolchain.pytest {
        Some(pytest) => pytest.clone(),
        None => {
            return Err(Errcode::ToolchainError(ToolchainErrorKind::PyTestNotFound));
        }
    };

    let mut cmd = Command::new(pytest)
        .args(opt.backend_args)
        .spawn()
        .map_err(|_| Errcode::ToolchainError(ToolchainErrorKind::PyTestFailed))?;
    cmd.wait()
        .map_err(|_| Errcode::ToolchainError(ToolchainErrorKind::PyTestFailed))?;

    Ok(())
}
