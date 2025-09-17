use std::path::PathBuf;

use clap::{Parser, Subcommand};
use color_eyre::eyre::Result;
use tracing::warn;

mod commands;
mod config;
mod local_cache;
mod logging;
mod template;

use commands::{add, new};
use config::get_system_config;

//TODO: 1. [ ] add custom macro for logging to reduce icon/symbol duplication, etc (possibly just a function?)
//TODO: 2. [ ] add ability to use YAML for config files as well as TOML
//TODO: 3. [ ] move all (or most) main logic into lib.rs

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "~/.config/boilermaker/boilermaker.toml")]
    config: Option<PathBuf>,

    #[arg(short = 'D', long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Add(add::Add),
    New(new::New),
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
            Commands::Add(cmd) => add::add(&sys_config, &cmd).await?,
            Commands::New(cmd) => new::new(&sys_config, &cmd).await?,
        }
    } else {
        //TODO: default to printing help menu if no command is provided
        warn!("❗ No command provided. Use --help for more information.");
    }

    Ok(())
}
