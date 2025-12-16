use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(
    name = "pyside-cli",
    about = "Test and build your app",
    arg_required_else_help = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Convert rc files and build the app
    All(BuildOptions),

    /// Build the app
    Build(BuildOptions),

    /// Generate translation files (.ts) for all languages
    I18n,

    /// Convert rc files to python files
    Rc,

    /// Run tests
    Test,

    /// List all available build targets
    Targets,

    /// Create your project with name
    Create { name: String },
}

#[derive(Parser, Debug)]
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

    /// Use low performance mode
    #[arg(long)]
    pub low_perf: bool,

    /// Ignore existing caches
    #[arg(long)]
    pub no_cache: bool,

    /// Enable debug mode
    #[arg(long)]
    pub debug: bool,

    /// Additional arguments for the build backend
    #[arg(last = true)]
    pub backend_args: Vec<String>,
}

#[derive(ValueEnum, Debug, Clone)]
pub enum Backend {
    Nuitka,
    Pyinstaller,
}
