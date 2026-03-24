use clap::Parser;
use color_eyre::{Result, eyre::eyre};
use tabled::Tabled;

use crate::{
    db::source::{SourceFindParams, SourceResult},
    state::AppState,
    util::{help, output::print_table},
};

#[derive(Debug, Parser)]
pub struct Show {
    #[arg(required = true)]
    pub id_or_name: String,
}

#[tracing::instrument]
async fn get_source(app_state: &AppState, cmd: &Show) -> Result<SourceResult> {
    let db = app_state.local_db.clone();

    if let Ok(id) = cmd.id_or_name.parse::<i64>() {
        Ok(db
            .get_source(id)
            .await?
            .expect("Failed to unwrap source result"))
    } else {
        let find_params = SourceFindParams {
            ids: None,
            name: Some(cmd.id_or_name.to_owned()),
            coordinate: None,
            description: None,
            sha256_hash: None,
        };
        let results = db.find_sources(find_params).await?;

        match results.len() {
            0 => Err(eyre!("💥 Cannot find source: {}.", cmd.id_or_name))?,
            1 => Ok(results
                .first()
                .unwrap_or(Err(eyre!(
                    "💥 Cannot retrieve source: {}.",
                    cmd.id_or_name
                ))?)
                .to_owned()),
            2.. => {
                help::print_multiple_source_results_help(&results);
                Err(eyre!(
                    "💥 Found multiple results matching source: {}.",
                    cmd.id_or_name
                ))?
            }
        }
    }
}

#[tracing::instrument]
pub async fn show(app_state: &AppState, cmd: &Show) -> Result<()> {
    let source = get_source(app_state, cmd).await?;

    #[rustfmt::skip]
    let rows = vec![
        row("ID", source.id.to_string()),
        row("Name", source.name),
        row("Coordinate", source.coordinate),
        row("Description", source.description.unwrap_or("-".to_string())),
        row("SHA256 Hash", source.sha256_hash.unwrap_or("-".to_string())),
    ];

    print_table(rows);

    Ok(())
}

#[tracing::instrument]
pub fn row(key: &str, value: String) -> ShowResult {
    ShowResult {
        key: key.to_string(),
        value,
    }
}

#[derive(Tabled)]
pub struct ShowResult {
    #[tabled(rename = "Key")]
    key: String,
    #[tabled(rename = "Value")]
    value: String,
}
