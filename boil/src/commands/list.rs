use clap::Parser;
use color_eyre::{Result, eyre::eyre};
use tabled::{Table, Tabled, settings::Style};

use crate::AppState;
use core::db::TemplateResult;

#[derive(Parser)]
pub struct List {
    #[arg(short = 'u', long)]
    pub public: bool,
    #[arg(short = 'p', long)]
    pub private: bool,
}

pub async fn list(app_state: &AppState, _cmd: &List) -> Result<()> {
    let result = {
        let cache = app_state
            .template_db
            .write()
            .map_err(|e| eyre!("Failed to acquire write lock: {}", e))?;

        cache.list_templates().await?
    };

    let rows = result
        .into_iter()
        .map(DisplayableTemplateListResult::to_std_row)
        .collect::<Vec<_>>();

    let mut table = Table::new(&rows);
    table.with(Style::psql());

    print!("\n\n{table}\n\n");

    Ok(())
}

#[derive(Debug, Tabled)]
struct DisplayableTemplateListResult {
    id: i64,
    name: String,
    lang: String,
    repo: String,
    created_at: String,
    updated_at: String,
}

impl DisplayableTemplateListResult {
    fn to_std_row(row: TemplateResult) -> Self {
        Self {
            id: row.id,
            name: row.name,
            lang: row.lang,
            repo: row.repo,
            created_at: row
                .created_at
                .map(|v| v.to_string())
                .unwrap_or_else(|| "-".to_string()),
            updated_at: row
                .updated_at
                .map(|v| v.to_string())
                .unwrap_or_else(|| "-".to_string()),
        }
    }
}
