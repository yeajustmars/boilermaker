use clap::{Parser, Subcommand};
use color_eyre::Result;
use serde::Deserialize;
use tabled::{Table, Tabled, settings::Style};
use toml::Value::Array as TomlArray;
use tracing::info;

use crate::state::AppState;

#[derive(Subcommand)]
pub enum Sources {
    #[command(about = "List configured sources")]
    List(List),
}

#[derive(Debug, Parser)]
pub struct List {
    #[arg(short = 'l', long, help = "List only local sources")]
    pub local: bool,
}

// TODO: move into Core.Sources
pub fn print_sources_table(rows: Vec<SourceMap>) -> Result<()> {
    let mut table = Table::new(&rows);
    table.with(Style::psql());

    print!("\n\n{table}\n\n");

    Ok(())
}

// TODO: add created_at, updated_at fields
#[derive(Debug, Deserialize, Tabled)]
pub struct SourceMap {
    pub name: String,
    pub backend: String,
    pub description: String,
}

// TODO: fix Serde deserialization to avoid manual mapping
impl From<&toml::map::Map<String, toml::Value>> for SourceMap {
    fn from(table: &toml::map::Map<String, toml::Value>) -> Self {
        let name = table
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap()
            .to_string();
        let backend = table
            .get("backend")
            .and_then(|v| v.as_str())
            .unwrap()
            .to_string();
        let description = table
            .get("description")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        Self {
            name,
            backend,
            description,
        }
    }
}

// TODO: add filters/options
pub async fn list(app_state: &AppState, _cmd: &List) -> Result<()> {
    println!("sys_config: {:?}", app_state.sys_config);
    if let Some(TomlArray(raw_sources)) = app_state.sys_config.get("sources") {
        let rows = raw_sources
            .iter()
            .filter_map(|s| {
                if let toml::Value::Table(table) = s {
                    Some(SourceMap::from(table))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        println!("rows: {rows:?}");

        print_sources_table(rows)?;

        Ok(())
    } else {
        info!("No sources configured.");
        Ok(())
    }
}
