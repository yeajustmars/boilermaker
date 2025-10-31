use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use clap::{Parser, Subcommand};
use color_eyre::eyre::Result;
use tracing::warn;

use boilermaker_core::{
    commands,
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
    New(commands::new::New),
    #[command(about = "Remove a template from the local cache")]
    Remove(commands::Remove),
    //#[command(about = "Update an existing template in the cache")]
    //Update(commands::update::Update),
}

#[tokio::main]
#[tracing::instrument]
async fn main() -> Result<()> {
    color_eyre::install().expect("Failed to set up error handling");

    let cli = Cli::parse();

    logging::init_tracing(cli.debug)?;

    let db_path = DEFAULT_LOCAL_CACHE_PATH_STRING.as_str();

    // TODO: check global boilermaker config for local vs remote db option
    let app_state = AppState {
        template_db: Arc::new(RwLock::new(LocalCache::new(db_path).await?)),
        sys_config: get_system_config(cli.config.as_deref())?,
        log_level: cli.debug,
    };

    {
        let cache = app_state.template_db.clone();
        let template_table_exists = cache.read().unwrap().template_table_exists().await?;
        if !template_table_exists {
            cache.write().unwrap().create_template_table().await?;
        }
    }

    if let Some(command) = cli.command {
        match command {
            Commands::Install(cmd) => commands::install(&app_state, &cmd).await?,
            Commands::List(cmd) => commands::list(&app_state, &cmd).await?,
            Commands::New(cmd) => commands::new(&app_state, &cmd).await?,
            Commands::Remove(cmd) => commands::remove(&app_state, &cmd).await?,
            // Commands::Update(cmd) => commands::update::update(&sys_config, &cmd).await?,
        }
    } else {
        warn!("❗ No command provided. Use --help for usage.");
    }

    Ok(())
}
