use clap::Parser;
use color_eyre::{Result, eyre::eyre};
use tabled::{Table, Tabled, settings::Style};
use tracing::error;

use crate::{
    commands::ListResult,
    db::{SourceFindParams, SourceResult, SourceTemplateResult},
    state::AppState,
};

#[derive(Debug, Parser)]
pub struct List {
    #[arg(required = true)]
    pub source_id_or_name: String,
}

#[tracing::instrument]
pub async fn get_source_id(app_state: &AppState, source_id_or_name: &String) -> Result<i64> {
    let cache = app_state.local_db.clone();

    if let Ok(id) = source_id_or_name.parse::<i64>() {
        Ok(id)
    } else {
        let find_params = SourceFindParams {
            ids: None,
            name: Some(source_id_or_name.to_owned()),
            coordinate: None,
            description: None,
            sha256_hash: None,
        };
        let results = cache.find_sources(find_params).await?;

        let result = match results.len() {
            0 => return Err(eyre!("💥 Cannot find source: {}.", source_id_or_name))?,
            1 => results.first(),
            2.. => {
                print_multiple_source_results_help(&results);
                return Err(eyre!(
                    "💥 Found multiple results matching template: {}.",
                    source_id_or_name
                ))?;
            }
        };

        match result {
            Some(source) => Ok(source.id),
            None => Err(eyre!("💥 Cannot find source: {}.", source_id_or_name))?,
        }
    }
}

#[tracing::instrument]
pub async fn list(app_state: &AppState, cmd: &List) -> Result<()> {
    let source_id = get_source_id(app_state, &cmd.source_id_or_name).await?;
    let result = {
        let cache = app_state.local_db.clone();
        cache.list_source_templates(source_id, None).await?
    };
    let rows = result
        .iter()
        .map(ListResult::from)
        .collect::<Vec<ListResult>>();

    let mut table = Table::new(&rows);
    table.with(Style::psql());
    print!("\n\n{table}\n\n");

    Ok(())
}

impl From<&SourceTemplateResult> for ListResult {
    fn from(row: &SourceTemplateResult) -> Self {
        Self {
            id: row.id,
            name: row.name.to_owned(),
            lang: row.lang.to_owned(),
            repo: row.repo.to_owned(),
            branch: row.branch.to_owned().unwrap_or("-".to_string()),
            subdir: row.subdir.to_owned().unwrap_or("-".to_string()),
        }
    }
}

#[derive(Tabled)]
struct MultipleResultsRow {
    #[tabled(rename = "ID")]
    id: i64,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Coordinate")]
    coordinate: String,
    #[tabled(rename = "SHA256 Hash")]
    sha256_hash: String,
}

#[tracing::instrument]
fn print_multiple_source_results_help(source_rows: &Vec<SourceResult>) {
    let help_line = "Multiple sources found matching name. Use ID instead.";
    let mut help_rows = Vec::new();
    for s in source_rows {
        help_rows.push(MultipleResultsRow {
            id: s.id,
            name: s.name.clone(),
            coordinate: s.coordinate.clone(),
            sha256_hash: s.sha256_hash.clone().unwrap(),
        });
    }

    let mut table = Table::new(&help_rows);
    table.with(Style::psql());
    error!("{}\n\n{table}\n", help_line);
}
