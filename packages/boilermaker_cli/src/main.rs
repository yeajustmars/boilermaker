use std::path::PathBuf;
use std::sync::Arc;

use clap::{Parser, Subcommand};
use color_eyre::eyre::Result;
use tracing::info;

use boilermaker_core::{
    commands,
    commands::{Sources, sources, sources::templates::Templates as SourceTemplates},
    config::{DEFAULT_LOCAL_CACHE_PATH_STRING, get_system_config},
    db::LocalCache,
    logging,
    state::AppState,
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(
        short,
        long,
        value_name = "FILE",
        help = "Path to config file (default: ~/.config/boilermaker/boilermaker.toml)"
    )]
    config: Option<PathBuf>,

    #[arg(
        short = 'D',
        long,
        action = clap::ArgAction::Count,
        help = "Turn on debug logging (use -D[DDD] for more verbosity)"
    )]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Install a template locally")]
    Install(commands::Install),
    #[command(about = "List all templates in the local cache")]
    List(commands::List),
    #[command(about = "Create a new project from a template")]
    New(commands::New),
    #[command(name = "rm", about = "Remove templates or local cache itself")]
    Remove(commands::Remove),
    #[command(about = "Search for templates")]
    Search(commands::Search),
    #[command(about = "Show template details")]
    Show(commands::Show),
    #[command(subcommand, about = "Manage Sources")]
    Sources(commands::Sources),
    #[command(about = "Update an installed template")]
    Update(commands::Update),
}

#[tokio::main]
#[tracing::instrument]
async fn main() -> Result<()> {
    color_eyre::install().expect("Failed to set up error handling");

    let cli = Cli::parse();

    logging::init_tracing(cli.debug)?;

    let cache_path = DEFAULT_LOCAL_CACHE_PATH_STRING.as_str();

    // TODO: decide where a remote db should be allowed vs just searching remote and installing
    // locally
    // TODO: If yes, check global boilermaker config for local vs remote db option
    let app_state = AppState {
        sys_config: get_system_config(cli.config.as_deref())?,
        log_level: cli.debug,
        local_db: Arc::new(LocalCache::new(cache_path).await?),
    };

    {
        let cache = app_state.local_db.clone();
        if !cache.template_table_exists().await? {
            cache.create_schema().await?;
        }
    }

    let Some(command) = cli.command else {
        println!("🔨 Boilermaker - Making boilerplate more sane!");
        info!("No command provided. Use --help for usage.");
        return Ok(());
    };

    // TODO: clean this up with aliases or direct imports.
    match command {
        Commands::Install(cmd) => commands::install(&app_state, &cmd).await,
        Commands::List(cmd) => commands::list(&app_state, &cmd).await,
        Commands::New(cmd) => commands::new(&app_state, &cmd).await,
        Commands::Remove(cmd) => commands::remove(&app_state, &cmd).await,
        Commands::Search(cmd) => commands::search(&app_state, &cmd).await,
        Commands::Show(cmd) => commands::show(&app_state, &cmd).await,
        Commands::Sources(subcmd) => match subcmd {
            Sources::Add(cmd) => sources::add(&app_state, &cmd).await,
            Sources::List(cmd) => sources::list(&app_state, &cmd).await,
            Sources::Templates(subcmd) => match subcmd {
                SourceTemplates::Install(cmd) => {
                    sources::templates::install(&app_state, &cmd).await
                }
                SourceTemplates::List(cmd) => sources::templates::list(&app_state, &cmd).await,
                SourceTemplates::Show(cmd) => sources::templates::show(&app_state, &cmd).await,
            },
        },
        Commands::Update(cmd) => commands::update(&app_state, &cmd).await,
    }
}
