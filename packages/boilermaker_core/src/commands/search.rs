use std::sync::Arc;

use clap::Parser;
use color_eyre::{Result, eyre::eyre};
use tracing::info;

use crate::db::{SearchResult, TabledSearchResult, TemplateDb};
use crate::state::AppState;
use crate::util::output::print_table;

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
        return Err(eyre!(
            "No search? Use \"list\" to see all installed templates."
        ));
    }

    let scope = SearchScope::from(cmd);
    let search_results = search_templates(cache, &term, scope).await?;
    if search_results.is_empty() {
        info!("No results found for {term}.");
        return Ok(());
    }

    let tabled: Vec<TabledSearchResult> = search_results
        .into_iter()
        .map(TabledSearchResult::from)
        .collect();
    print_table(tabled);

    Ok(())
}

pub enum SearchScope {
    Local,
    Source(String),
    All,
}

impl SearchScope {
    pub fn from(cmd: &Search) -> Self {
        if cmd.local {
            SearchScope::Local
        } else if let Some(source_name) = cmd.src.clone() {
            SearchScope::Source(source_name)
        } else {
            SearchScope::All
        }
    }
}

pub async fn search_templates(
    cache: Arc<dyn TemplateDb>,
    term: &str,
    scope: SearchScope,
) -> Result<Vec<SearchResult>> {
    match scope {
        SearchScope::Local => Ok(cache.search_templates(term).await?),
        SearchScope::Source(name) => Ok(cache.search_sources(Some(name), term).await?),
        SearchScope::All => {
            let mut all_results = Vec::new();
            let local = cache.search_templates(term).await?;
            all_results.extend(local);

            let sources = cache.search_sources(None, term).await?;
            all_results.extend(sources);
            Ok(all_results)
        }
    }
}
