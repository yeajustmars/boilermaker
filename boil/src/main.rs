use std::path::PathBuf;

use clap::{Parser, Subcommand};
use color_eyre::eyre::Result;
use tracing::{info, warn};
use tracing_error::ErrorLayer;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;

mod commands;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "~/.config/boilermaker/config.toml")]
    config: Option<PathBuf>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    List(commands::list::List),
    Test(commands::test::Test),
}

//TODO: implement config file
const CONFIG_FILE: &str = "~/.config/boilermaker/boilermaker.toml";

fn main() -> Result<()> {
    color_eyre::install().expect("Failed to install color_eyre");
    init_tracing()?;

    let cli = Cli::parse();

    //TODO: add setup from config file (user-provided or default)
    if let Some(config_path) = cli.config.as_deref() {
        info!("ⓘ Value for config: {}", config_path.display());
    } else {
        info!("ⓘ No config file found at `{CONFIG_FILE}`. Using defaults.");
    }

    //TODO: Add tracing for --debug flag
    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => info!("Debug mode is off"),
        1 => info!("Debug mode is kind of on"),
        2 => info!("Debug mode is on"),
        _ => info!("Don't be crazy"),
    }

    if let Some(command) = cli.command {
        match command {
            Commands::List(list_cmd) => {
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
        warn!("No command provided. Use --help for more information.");
        Ok(())
    }
}

pub fn init_tracing() -> Result<()> {
    let fmt_layer = fmt::layer()
        .event_format(fmt::format().compact())
        .without_time()
        .with_target(false)
        .with_level(true);

    //TODO: add ability to configure log level via CLI args and/or config file
    let filter_layer = EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new("info"))?;

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .init();

    Ok(())
}
