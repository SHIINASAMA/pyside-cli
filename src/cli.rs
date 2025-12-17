use clap::{Parser, Subcommand, ValueEnum};
use log::LevelFilter;

use crate::errcode::Errcode;

#[derive(Parser, Debug)]
#[command(
    name = "pyside-cli",
    about = "Test and build your app",
    arg_required_else_help = true
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,

    /// Enable debug mode
    #[arg(long)]
    pub debug: bool,

    /// Change working directory
    #[arg(long, value_name = "DIR")]
    pub work_dir: Option<String>,

    /// Use low performance mode
    #[arg(long)]
    pub low_perf: bool,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    /// Convert rc files and build the app
    All(BuildOptions),

    /// Build the app
    Build(BuildOptions),

    /// Generate translation files (.ts) for all languages
    I18n(I18nOptions),

    /// Convert rc files to python files
    Rc(RcOptions),

    /// Run tests
    Test(TestOptions),

    /// List all available build targets
    Targets,

    /// Create your project with name
    Create { name: String },
}

#[derive(Parser, Debug, Clone)]
pub struct BuildOptions {
    /// Create a single executable file
    #[arg(long, conflicts_with = "onedir")]
    pub onefile: bool,

    /// Create a directory with the executable and all dependencies
    #[arg(long, conflicts_with = "onefile")]
    pub onedir: bool,

    /// Build target (default: App)
    #[arg(short, long, value_name = "TARGET")]
    pub target: Option<String>,

    /// Backend to use
    #[arg(long, value_enum, default_value_t = Backend::Nuitka)]
    pub backend: Backend,

    /// Ignore existing caches
    #[arg(long)]
    pub no_cache: bool,

    /// Additional arguments for the build backend
    #[arg(last = true)]
    pub backend_args: Vec<String>,
}

#[derive(ValueEnum, Debug, Clone)]
pub enum Backend {
    Nuitka,
    Pyinstaller,
}

#[derive(Parser, Debug, Clone)]
pub struct I18nOptions {
    /// Target to glob i18n files for (default: App)
    #[arg(short, long, value_name = "TARGET", default_value_t = String::from("App"))]
    pub target: String,
}

#[derive(Parser, Debug, Clone)]
pub struct RcOptions {
    /// Target to generate resource files for (default: App)
    #[arg(short, long, value_name = "TARGET", default_value_t = String::from("App"))]
    pub target: String,

    /// Ignore existing caches
    #[arg(long)]
    pub no_cache: bool,
}

#[derive(Parser, Debug, Clone)]
pub struct TestOptions {
    /// Additional arguments for the pytest
    #[arg(last = true)]
    pub backend_args: Vec<String>,
}

pub fn parse_cli() -> Result<Args, Errcode> {
    let cli = Args::parse();
    let mut logger_mode = LevelFilter::Info;
    if cli.debug {
        logger_mode = LevelFilter::Debug;
    }
    env_logger::Builder::from_default_env()
        .filter(None, logger_mode)
        .init();
    Ok(cli)
}
