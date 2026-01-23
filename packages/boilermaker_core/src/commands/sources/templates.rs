use clap::{Parser, Subcommand};
use color_eyre::Result;

use crate::state::AppState;

#[derive(Subcommand)]
pub enum Templates {
    #[command(about = "Install from source")]
    Install(Install),
}

#[derive(Debug, Parser)]
pub struct Install {
    #[arg(required = true, help = "Source template to install")]
    pub id_or_name: String,
    #[arg(short, long, help = "Optional rename")]
    pub name: Option<String>,
}

#[tracing::instrument]
pub async fn install(app_state: &AppState, cmd: &Install) -> Result<()> {
    println!("GOT IT!");
    Ok(())
}
