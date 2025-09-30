use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use clap::{Parser, Subcommand};
use color_eyre::eyre::Result;
use tracing::warn;

use boil::{AppState, commands, logging};
use core::config::{get_system_config, DEFAULT_LOCAL_CACHE_PATH};
use core::db::LocalCache;

//TODO: 1. [ ] add custom macro for logging to reduce icon/symbol duplication, etc (possibly just a function?)
//TODO: 2. [ ] add ability to use YAML for config files as well as TOML

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
        help = "Turn on debug logging (use -D[DDD] for more verbosity)")]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Add a template to the cache")]
    Add(commands::Add),
    #[command(about = "List all templates in the local cache")]
    List(commands::List),
    #[command(about = "Create a new project from a template")]
    New(commands::new::New),
    //#[command(about = "Remove a template from the local cache")]
    // Remove(remove::Remove),
    //#[command(about = "Update an existing template in the cache")]
    // Update(commands::update::Update),
}

#[tokio::main]
#[tracing::instrument]
async fn main() -> Result<()> {
    color_eyre::install().expect("Failed to set up error handling");

    let cli = Cli::parse();

    logging::init_tracing(cli.debug)?;

    // TODO: check global boilermaker config for local vs remote db option
    let local_cache_path = DEFAULT_LOCAL_CACHE_PATH.as_path().to_str().unwrap();
    let app_state = AppState {
        template_db: Arc::new(RwLock::new(LocalCache::new(local_cache_path).await?)),
        sys_config: get_system_config(cli.config.as_deref())?,
        log_level: cli.debug,
    };

    if let Some(command) = cli.command {
        match command {
            Commands::Add(cmd) => commands::add(&app_state, &cmd).await?,
            Commands::List(cmd) => commands::list(&app_state, &cmd).await?,
            Commands::New(cmd) => commands::new(&app_state, &cmd).await?,
            // Commands::Update(cmd) => commands::update::update(&sys_config, &cmd).await?,
        }
    } else {
        warn!("❗ No command provided. Use --help for usage.");
    }

    Ok(())
}
