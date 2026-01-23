use clap::Parser;
use color_eyre::Result;

use crate::state::AppState;

#[derive(Debug, Parser)]
pub struct Show {
    #[arg(required = true)]
    pub id_or_name: String,
}

#[tracing::instrument]
pub async fn show(app_state: &AppState, cmd: &Show) -> Result<()> {
    println!("Sources > Templates > Show");
    Ok(())
}
