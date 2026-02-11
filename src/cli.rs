use clap::{
    Parser, Subcommand, ValueEnum,
    builder::{
        Styles,
        styling::{AnsiColor, Effects},
    },
};
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
    color = clap::ColorChoice::Always,
    styles = Styles::styled()
            .header(AnsiColor::BrightBlue.on_default().effects(Effects::BOLD))
            .usage(AnsiColor::BrightGreen.on_default().effects(Effects::BOLD))
            .literal(AnsiColor::BrightCyan.on_default())
            .placeholder(AnsiColor::BrightYellow.on_default())
            .error(AnsiColor::BrightRed.on_default().effects(Effects::BOLD))
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

    /// Generate shell completions
    #[command(hide = true)]
    Completions {
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    },
}

#[derive(Parser, Debug, Clone)]
pub struct BuildOptions {
    /// Select a stage to build (default: All)
    #[arg(long, value_enum, default_value_t = BuildStage::All)]
    pub stage: BuildStage,

    /// Compatibility flag. Equivalent to `--build-type onefile`
    #[arg(long, conflicts_with = "onedir")]
    pub onefile: bool,

    /// Compatibility flag. Equivalent to `--build-type onedir`
    #[arg(long, conflicts_with = "onefile")]
    pub onedir: bool,

    /// Final build output type
    #[arg(long, value_enum)]
    pub build_type: Option<BuildType>,

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

impl BuildOptions {
    /// Resolves the final build type based on user input.
    ///
    /// Priority order:
    /// 1. Compatibility flags (`--onefile` / `--onedir`)
    /// 2. Explicit `--build-type` argument
    /// 3. Fallback to default (Onedir)
    ///
    /// This design allows us to:
    /// - Distinguish between "user did not specify" and "user explicitly chose"
    /// - Keep backward compatibility with legacy flags
    /// - Centralize decision logic in one place
    pub fn resolve_build_type(&self) -> BuildType {
        // Compatibility flags take highest priority
        if self.onefile {
            return BuildType::Onefile;
        }

        if self.onedir {
            return BuildType::Onedir;
        }

        // Explicit --build-type argument
        if let Some(bt) = &self.build_type {
            return bt.clone();
        }

        // Default fallback when nothing is specified
        BuildType::Onedir
    }
}


#[derive(ValueEnum, Debug, Clone)]
pub enum BuildType {
    /// Build as a single executable file.
    Onefile,

    /// Build into a directory.
    Onedir,

    /// Build as a macOS app bundle.
    Bundle,
}


#[derive(ValueEnum, Debug, Clone)]
pub enum BuildStage {
    /// Only compile translation files (.ts) to .qm files.
    I18n,
    /// Only convert .ui files to .py files.
    Ui,
    /// Only convert assets files into resource.py.
    Assets,
    /// Including I18n, UI, Assets stages.
    Rc,
    /// Only build the app.
    Build,
    /// Build all stages.
    All,
}

#[derive(ValueEnum, Debug, Clone)]
pub enum Backend {
    /// Use Nuitka as the build backend.
    /// Ref: https://nuitka.net/
    Nuitka,
    /// Use PyInstaller as the build backend.
    /// Ref: https://pyinstaller.org/
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
