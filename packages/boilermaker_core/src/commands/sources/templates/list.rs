use clap::Parser;
use color_eyre::{Result, eyre::eyre};

use crate::{
    commands::ListResult,
    db::{SourceFindParams, SourceTemplateResult},
    state::AppState,
    util::{help, output::print_table},
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
                help::print_multiple_source_results_help(&results);
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

    print_table(rows);

    Ok(())
}

impl From<&SourceTemplateResult> for ListResult {
    fn from(row: &SourceTemplateResult) -> Self {
        Self {
            id: row.id,
            name: row.name.clone(),
            lang: row.lang.clone(),
            repo: row.repo.clone(),
            branch: row.branch.clone().unwrap_or("-".to_string()),
            subdir: row.subdir.clone().unwrap_or("-".to_string()),
        }
    }
}
