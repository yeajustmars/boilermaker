use clap::Parser;
use color_eyre::Result;
use tabled::{Table, settings::Style};
use tracing::info;

use crate::db::local_db::DisplayableTemplateListResult;
use crate::state::AppState;

#[derive(Parser)]
pub struct List {
    #[arg(short = 'u', long)]
    pub public: bool,
    #[arg(short = 'p', long)]
    pub private: bool,
}

pub async fn list(app_state: &AppState, _cmd: &List) -> Result<()> {
    let cache = app_state.local_db.clone();

    let result = cache.list_templates(None).await?;

    let rows = result
        .into_iter()
        .map(DisplayableTemplateListResult::to_std_row)
        .collect::<Vec<_>>();

    if rows.is_empty() {
        info!("No templates found in the cache.");
        info!("💡 Have a look at `boil install`");
        return Ok(());
    }

    let mut table = Table::new(&rows);
    table.with(Style::psql());

    print!("\n\n{table}\n\n");

    Ok(())
}
