use std::process::Command;

use crate::{
    cli::TestOptions,
    errcode::{Errcode, ToolchainErrorKind},
    run_and_wait,
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

    run_and_wait!(
        Command::new(pytest).args(opt.backend_args),
        Errcode::ToolchainError(ToolchainErrorKind::PyTestFailed)
    )?;

    Ok(())
}
