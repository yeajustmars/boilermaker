use clap::Parser;
use color_eyre::Result;
use tabled::{Table, settings::Style};
use tracing::info;

use crate::{commands::ListResult, db::TemplateResult, state::AppState};

#[derive(Parser, Debug)]
pub struct List {
    #[arg(short = 'u', long)]
    pub public: bool,
    #[arg(short = 'p', long)]
    pub private: bool,
}

#[tracing::instrument]
pub async fn list(app_state: &AppState, _cmd: &List) -> Result<()> {
    let cache = app_state.local_db.clone();
    let result = cache.list_templates(None).await?;
    let rows = result.into_iter().map(ListResult::from).collect::<Vec<_>>();

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

impl From<TemplateResult> for ListResult {
    fn from(row: TemplateResult) -> Self {
        Self {
            id: row.id,
            name: row.name,
            lang: row.lang,
            repo: row.repo,
            branch: row.branch.unwrap_or("-".to_string()),
            subdir: row.subdir.unwrap_or("-".to_string()),
        }
    }
}
