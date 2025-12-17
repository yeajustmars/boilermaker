use std::sync::Arc;

use clap::Parser;
use color_eyre::{Result, eyre::eyre};
use tabled::Table;
use tabled::settings::Style;
use tracing::{debug, info};

use crate::db::{SearchResult, TabledSearchResult, TemplateDb};
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

    // TODO: for now you MUST pick between templates installed locally (-l), or
    //       templates from a remote source (-s): search w/o either flag will
    //       error out.
    let scope = if cmd.local {
        SearchScope::Local
    } else if let Some(source_name) = cmd.src.clone() {
        SearchScope::Source(source_name)
    } else {
        return Err(eyre!(
            "Use -l or -s to search for local templates, or sources"
        ));
    };
    let search_results = search_templates(cache.clone(), &term, scope).await?;
    debug!("Search results: {:?}", search_results);
    if search_results.is_empty() {
        info!("No results found for {term}.");
        return Ok(());
    }

    let tabled = search_results
        .into_iter()
        .map(TabledSearchResult::from)
        .collect::<Vec<_>>();
    let mut table = Table::new(tabled);
    table.with(Style::psql());
    print!("\n\n{table}\n\n");

    Ok(())
}

pub enum SearchScope {
    Local,
    Source(String),
}

pub async fn search_templates(
    cache: Arc<dyn TemplateDb>,
    term: &str,
    scope: SearchScope,
) -> Result<Vec<SearchResult>> {
    match scope {
        SearchScope::Local => Ok(cache.search_templates(term).await?),
        SearchScope::Source(name) => Ok(cache.search_sources(&name, term).await?),
    }
}
