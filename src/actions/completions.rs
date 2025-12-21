use clap::CommandFactory;
use clap_complete::generate;

use crate::cli::Args;

pub fn action(shell: clap_complete::Shell) {
    let mut cmd = Args::command();
    generate(shell, &mut cmd, "pyside-cli", &mut std::io::stdout());
}
