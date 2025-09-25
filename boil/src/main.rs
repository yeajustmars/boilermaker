use std::path::PathBuf;

use clap::{Parser, Subcommand};
use color_eyre::eyre::Result;
use tracing::warn;

mod commands;
mod config;
mod local_cache;
mod logging;
mod template;

use config::get_system_config;

//TODO: 1. [ ] add custom macro for logging to reduce icon/symbol duplication, etc (possibly just a function?)
//TODO: 2. [ ] add ability to use YAML for config files as well as TOML
//TODO: 3. [ ] move all (or most) main logic into lib.rs

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
    Add(commands::add::Add),
    #[command(about = "List all templates in the local cache")]
    List(commands::list::List),
    #[command(about = "Create a new project from a Git repository template")]
    New(commands::new::New),
    #[command(about = "Remove a template from the local cache")]
    // Remove(remove::Remove),
    #[command(about = "Update an existing template in the cache")]
    Update(commands::update::Update),
}

#[tokio::main]
#[tracing::instrument]
async fn main() -> Result<()> {
    color_eyre::install().expect("Failed to install color_eyre");

    let cli = Cli::parse();

    logging::init_tracing(cli.debug)?;

    let sys_config = get_system_config(cli.config.as_deref())?;

    if let Some(command) = cli.command {
        match command {
            Commands::Add(cmd) => commands::add::add(&sys_config, &cmd).await?,
            Commands::List(cmd) => commands::list::list(&sys_config, &cmd).await?,
            Commands::New(cmd) => commands::new::new(&sys_config, &cmd).await?,
            Commands::Update(cmd) => commands::update::update(&sys_config, &cmd).await?,
        }
    } else {
        warn!("❗ No command provided. Use --help for usage.");
    }

    Ok(())
}
