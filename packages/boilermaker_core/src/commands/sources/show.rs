use clap::Parser;
use color_eyre::Result;
//use tracing::info;

//use crate::db::TabledSourceRow;
use crate::state::AppState;
//use crate::util::output::print_table;

#[derive(Debug, Parser)]
pub struct Show {
    #[arg(required = true)]
    pub id_or_name: String,
}

#[tracing::instrument]
pub async fn show(app_state: &AppState, cmd: &Show) -> Result<()> {
    println!("---> SHOW SOURCE");
    Ok(())
}
