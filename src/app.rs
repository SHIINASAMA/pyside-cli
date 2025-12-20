use crate::actions;
use crate::cli::{Command, init_logger, parse_cli};
use crate::errcode::{Errcode, GeneralErrorKind};

pub fn run() -> Result<(), Errcode> {
    let args = parse_cli()?;

    init_logger(args.debug.clone());

    if let Some(path) = &args.work_dir {
        log::info!("Working directory set to {} .", path);
        let _ = std::env::set_current_dir(path)
            .map_err(|_| Errcode::GeneralError(GeneralErrorKind::WorkDirNotFound));
    };

    match args.command {
        Command::Targets => actions::targets::action()?,
        Command::I18n(opt) => actions::i18n::action(opt)?,
        Command::Build(opt) => actions::build::action(opt)?,
        Command::Test(opt) => actions::test::action(opt)?,
        Command::Create { name } => actions::create::action(name)?,
    }

    Ok(())
}
