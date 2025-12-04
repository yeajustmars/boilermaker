use std::collections::HashMap;

use clap::{Parser, Subcommand};
use color_eyre::Result;
use serde::Deserialize;
use tabled::{Table, Tabled, settings::Style};
use tracing::info;

use crate::state::AppState;
use crate::util::string;

#[derive(Subcommand)]
pub enum Sources {
    #[command(about = "Add a source")]
    Add(Add),
    #[command(about = "List configured sources")]
    List(List),
}

#[derive(Debug, Parser)]
pub struct Add {
    #[arg(required = true, help = "Source URL or file path")]
    coordinate: String,
}

pub async fn add(_app_state: &AppState, _cmd: &Add) -> Result<()> {
    println!("> > > > > > > > > > > > Add a source!");
    Ok(())
}

#[derive(Debug, Parser)]
pub struct List {
    #[arg(short = 'l', long, help = "List only local sources")]
    pub local: bool,
}

#[derive(Debug, Deserialize, Tabled)]
pub struct SourceMap {
    pub name: String,
    pub backend: String,
    pub description: String,
}

impl From<&HashMap<String, String>> for SourceMap {
    fn from(m: &HashMap<String, String>) -> Self {
        let description = m.get("description").cloned().unwrap_or_default();
        let description = if description.len() > 50 {
            string::truncate_to_char_count(&description, 50) + "..."
        } else {
            description
        };

        SourceMap {
            name: m.get("name").cloned().unwrap_or_default(),
            backend: m.get("backend").cloned().unwrap_or_default(),
            description,
        }
    }
}

pub fn print_sources_table(sources: Vec<HashMap<String, String>>) -> Result<()> {
    let rows = sources.iter().map(SourceMap::from).collect::<Vec<_>>();

    let mut table = Table::new(&rows);
    table.with(Style::psql());

    print!("\n\n{table}\n\n");

    Ok(())
}

// TODO: add filters/options
pub async fn list(app_state: &AppState, _cmd: &List) -> Result<()> {
    if let Some(sources) = &app_state.sys_config.sources {
        print_sources_table(sources.to_vec())?;
        Ok(())
    } else {
        info!("No sources configured.");
        Ok(())
    }
}
