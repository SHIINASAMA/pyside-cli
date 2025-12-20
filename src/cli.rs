use clap::{Parser, Subcommand, ValueEnum};
use log::LevelFilter;
// use std::io::Write;

use crate::errcode::Errcode;

#[derive(Parser, Debug)]
#[command(
    name = "pyside-cli",
    about = "Test and build your app",
    arg_required_else_help = true,
    version = env!("CARGO_PKG_VERSION"),
    long_version = env!("CARGO_PKG_VERSION"),
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
    /// Build the app
    Build(BuildOptions),

    /// Generate translation files (.ts) for all languages
    I18n(I18nOptions),

    /// Run tests
    Test(TestOptions),

    /// List all available build targets
    Targets,

    /// Create your project with name
    Create { name: String },
}

#[derive(Parser, Debug, Clone)]
pub struct BuildOptions {
    /// Select a stage to build (default: All)
    #[arg(long, value_enum, default_value_t = BuildStage::All)]
    pub stage: BuildStage,

    /// Create a single executable file.
    #[arg(long, conflicts_with = "onedir")]
    pub onefile: bool,

    /// Create a directory with the executable and all dependencies.
    #[arg(long, conflicts_with = "onefile")]
    pub onedir: bool,

    /// Build target (default: App).
    #[arg(short, long, value_name = "TARGET", default_value_t = String::from("App"))]
    pub target: String,

    /// Backend to use.
    #[arg(long, value_enum, default_value_t = Backend::Nuitka)]
    pub backend: Backend,

    /// Ignore existing caches.
    #[arg(long)]
    pub no_cache: bool,

    /// Additional arguments for the build backend.
    #[arg(last = true)]
    pub backend_args: Vec<String>,
}

#[derive(ValueEnum, Debug, Clone)]
pub enum BuildStage {
    /// Only compile translation files (.ts) to .qm files.
    I18n,
    /// Only convert .ui files to .py files.
    Ui,
    /// Only convert assets files into resource.py.
    Assets,
    /// Only build the app.
    Build,
    /// Build all stages.
    All,
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
pub struct TestOptions {
    /// Additional arguments for the pytest
    #[arg(last = true)]
    pub backend_args: Vec<String>,
}

pub fn init_logger(debug: bool) {
    let mut logger_mode = LevelFilter::Info;
    if debug {
        logger_mode = LevelFilter::Debug;
    }

    env_logger::Builder::from_default_env()
        .filter(None, logger_mode)
        // .format(move |buf, record| writeln!(buf, "{:<5} {}", record.level(), record.args()))
        .init();
}

pub fn parse_cli() -> Result<Args, Errcode> {
    let cli = Args::parse();
    Ok(cli)
}
