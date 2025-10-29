use clap::Parser;
use color_eyre::{Result, eyre::eyre};
use tabled::{Table, settings::Style};

use crate::db::template_cache::DisplayableTemplateListResult;
use crate::state::AppState;

#[derive(Parser)]
pub struct List {
    #[arg(short = 'u', long)]
    pub public: bool,
    #[arg(short = 'p', long)]
    pub private: bool,
}

pub async fn list(app_state: &AppState, _cmd: &List) -> Result<()> {
    let cache = app_state
        .template_db
        .write()
        .map_err(|e| eyre!("Failed to acquire write lock: {}", e))?;

    let result = cache.list_templates(None).await?;

    let rows = result
        .into_iter()
        .map(DisplayableTemplateListResult::to_std_row)
        .collect::<Vec<_>>();

    let mut table = Table::new(&rows);
    table.with(Style::psql());

    print!("\n\n{table}\n\n");

    Ok(())
}
