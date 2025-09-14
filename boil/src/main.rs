use std::path::PathBuf;

use clap::{Parser, Subcommand};
use color_eyre::eyre::Result;
use tracing::{info, warn};

mod commands;
mod config;
mod logging;

use commands::{list, new, test};
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
    List(list::List),
    New(new::New),
    Test(test::Test),
}

#[tracing::instrument]
fn main() -> Result<()> {
    color_eyre::install().expect("Failed to install color_eyre");

    let cli = Cli::parse();

    logging::init_tracing(cli.debug)?;

    let sys_config = get_system_config(cli.config.as_deref())?;

    if let Some(command) = cli.command {
        match command {
            Commands::List(list_cmd) => {
                //TODO: move into list::List command implementation
                if list_cmd.public {
                    info!("Listing public items...");
                    Ok(())
                } else if list_cmd.private {
                    info!("Listing private items...");
                    Ok(())
                } else {
                    info!("Listing all items...");
                    Ok(())
                }
            }
            Commands::New(cmd) => new::new(&sys_config, &cmd),
            Commands::Test(test_cmd) => {
                if test_cmd.list {
                    info!("Listing tests...");
                    Ok(())
                } else {
                    info!("Running tests...");
                    Ok(())
                }
            }
        }
    } else {
        //TODO: default to printing help menu if no command is provided
        warn!("❗ No command provided. Use --help for more information.");
        Ok(())
    }
}
