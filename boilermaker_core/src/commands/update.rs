use clap::Parser;
use color_eyre::Result;
use tracing::info;

use crate::state::AppState;

#[derive(Debug, Parser)]
pub struct Update {
    #[arg(required = true)]
    pub id: i32,
}

#[tracing::instrument]
pub async fn update(app_state: &AppState, cmd: &Update) -> Result<()> {
    info!("Updating template: {}", &cmd.id);
    Ok(())
}
