use std::collections::HashSet;

use clap::Parser;
use color_eyre::{eyre::eyre, Result};
use tabled::settings::Style;
use tabled::Table;
use tracing::{debug, info};

use crate::db::{DisplayableTemplateListResult, TemplateFindParams};
use crate::state::AppState;

#[derive(Debug, Parser)]
pub struct Search {
    #[arg(required = true, help = "Search term")]
    pub term: String,
    #[arg(short = 'l', long, help = "Search only installed templates")]
    pub local: bool,
    #[arg(short = 's', long, help = "Search a specific source")]
    pub src: Option<String>,
}

pub async fn search(app_state: &AppState, cmd: &Search) -> Result<()> {
    let term = cmd.term.trim().to_owned();
    let cache = app_state.local_db.clone();

    if term.is_empty() {
        return Err(eyre!("Empty search"));
    }
    debug!("Searching : {term}");

    // Search templates
    let search_results = cache.search_templates(&term).await?;
    debug!("Search results: {:?}", search_results);
    if search_results.is_empty() {
        info!("No results found for {term}.");
        return Ok(());
    }

    // Load matching templates
    let template_ids: HashSet<i64> = search_results.iter().map(|res| res.template_id).collect();
    let find_params = TemplateFindParams {
        ids: Some(template_ids.into_iter().collect()),
        ..Default::default()
    };
    let templates = cache.find_templates(find_params).await?;

    // Display
    let rows = templates
        .into_iter()
        .map(DisplayableTemplateListResult::to_std_row)
        .collect::<Vec<_>>();
    if rows.is_empty() {
        info!("No results found for {term}.");
        return Ok(());
    }

    let mut table = Table::new(&rows);
    table.with(Style::psql());
    print!("\n\n{table}\n\n");

    Ok(())
}
