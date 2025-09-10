use std::path::PathBuf;

use clap::{Parser, Subcommand};
use color_eyre::eyre::Result;
use tracing::{info, warn};
use tracing_error::ErrorLayer;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;

mod commands;
mod config;

use config::get_config;

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

#[tracing::instrument]
fn main() -> Result<()> {
    color_eyre::install().expect("Failed to install color_eyre");

    let cli = Cli::parse();

    init_tracing(cli.debug)?;

    let config = get_config(cli.config.as_deref())?;
    println!("config: {:#?}", config);

    if let Some(command) = cli.command {
        match command {
            Commands::List(list_cmd) => {
                //TODO: move into list::List command implementation
                if list_cmd.public {
                    info!("ⓘ  Listing public items...");
                    Ok(())
                } else if list_cmd.private {
                    info!("ⓘ  Listing private items...");
                    Ok(())
                } else {
                    info!("ⓘ  Listing all items...");
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
        warn!("❗ No command provided. Use --help for more information.");
        Ok(())
    }
}

#[tracing::instrument]
pub fn init_tracing(debug_level: u8) -> Result<()> {
    //TODO: Add more specific formatting for each debug level (0-4)
    let fmt_layer: Box<dyn tracing_subscriber::Layer<_> + Send + Sync> = match debug_level {
        1..=4 => Box::new(
            fmt::layer()
                .event_format(fmt::format().pretty())
                .with_thread_ids(true)
                .with_thread_names(true)
                .with_target(true)
                .with_level(true)
                .with_file(true)
                .with_line_number(true),
        ),
        _ => Box::new(
            fmt::layer()
                .event_format(fmt::format().compact())
                .without_time()
                .with_target(false)
                .with_level(true),
        ),
    };

    //TODO: add ability to configure log level via CLI args and/or config file
    let filter_layer = EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new("info"))?;

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .init();

    Ok(())
}
