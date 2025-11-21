use clap::Parser;
use color_eyre::Result;

use crate::state::AppState;

#[derive(Parser)]
pub struct Search {
    #[arg(required = true, help = "Search query (regex supported)")]
    pub query: String,
    #[arg(short = 'l', long, help = "Search only installed templates")]
    pub local: bool,
    #[arg(short = 's', long, help = "Search a specific source")]
    pub src: Option<String>,
}

pub async fn search(_app_state: &AppState, _cmd: &Search) -> Result<()> {
    println!("TODO: PICKUP HERE - search command not yet implemented");
    Ok(())
}
