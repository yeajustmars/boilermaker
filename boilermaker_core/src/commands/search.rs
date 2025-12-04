use clap::Parser;
use color_eyre::Result;

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
    let cache = app_state.cache_db.clone();

    let results = if cmd.local {
        cache.search_templates(&term).await?
    } else {
        // TODO: implement remote search
        cache.search_templates(&term).await?
    };

    println!("Search results for '{}':", &cmd.term);
    println!("{:#?}", &results);

    Ok(())
}
