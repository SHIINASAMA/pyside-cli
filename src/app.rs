use crate::actions;
use crate::cli::{Command, parse_cli};
use crate::errcode::{Errcode, InvalidArgumentKind};

pub fn run() -> Result<(), Errcode> {
    let args = parse_cli()?;

    if let Some(path) = &args.work_dir {
        log::info!("Working directory set to {} .", path);
        let _ = std::env::set_current_dir(path)
            .map_err(|_| Errcode::InvalidArgument(InvalidArgumentKind::WorkDirNotFound));
    };

    match &args.command {
        Command::Targets => actions::targets::action()?,
        Command::I18n(opt) => actions::i18n::action(opt)?,
        Command::Build(opt) => actions::build::action(opt)?,
        _ => {}
    }

    Ok(())
}
