use clap::Parser;
use color_eyre::Result;
use tabled::{Table, settings::Style};
use tracing::info;

use crate::db::TabledSourceRow;
use crate::state::AppState;

#[derive(Debug, Parser)]
pub struct List {
    #[arg(short = 'l', long, help = "List only local sources")]
    pub local: bool,
}

#[tracing::instrument]
pub async fn list(app_state: &AppState, _cmd: &List) -> Result<()> {
    let sources = app_state.local_db.list_sources().await?;
    if sources.is_empty() {
        info!("No sources found.");
        info!("💡 Have a look at `boil sources add`");
        return Ok(());
    }

    let table_rows = sources
        .into_iter()
        .map(TabledSourceRow::from)
        .collect::<Vec<_>>();
    let mut table = Table::new(&table_rows);
    table.with(Style::psql());
    print!("\n\n{table}\n\n");

    Ok(())
}
