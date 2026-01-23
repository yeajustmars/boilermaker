use clap::Parser;
use color_eyre::{Result, eyre::eyre};
use tabled::Tabled;

use crate::{
    db::{TemplateFindParams, TemplateResult},
    state::AppState,
    util::{help, output::print_table, time::timestamp_to_iso8601},
};

#[derive(Parser, Debug)]
pub struct Show {
    #[arg(required = true)]
    pub id_or_name: String,
}

#[tracing::instrument]
async fn get_template(app_state: &AppState, cmd: &Show) -> Result<TemplateResult> {
    let cache = app_state.local_db.clone();

    if let Ok(id) = cmd.id_or_name.parse::<i64>() {
        Ok(cache
            .get_template(id)
            .await?
            .expect("Failed to unwrap template result"))
    } else {
        let find_params = TemplateFindParams {
            ids: None,
            name: Some(cmd.id_or_name.to_owned()),
            lang: None,
            repo: None,
            branch: None,
            subdir: None,
            sha256_hash: None,
        };
        let results = cache.find_templates(find_params).await?;

        match results.len() {
            0 => Err(eyre!("💥 Cannot find template: {}.", cmd.id_or_name))?,
            1 => Ok(results
                .first()
                .unwrap_or(Err(eyre!(
                    "💥 Cannot retrieve template: {}.",
                    cmd.id_or_name
                ))?)
                .to_owned()),
            2.. => {
                help::print_multiple_template_results_help(&results);
                Err(eyre!(
                    "💥 Found multiple results matching template: {}.",
                    cmd.id_or_name
                ))?
            }
        }
    }
}

#[tracing::instrument]
pub fn row(key: &str, value: String) -> ShowResult {
    ShowResult {
        key: key.to_string(),
        value,
    }
}

#[tracing::instrument]
pub async fn show(app_state: &AppState, cmd: &Show) -> Result<()> {
    let template = get_template(app_state, cmd).await?;

    #[rustfmt::skip]
    let rows = vec![
        row("ID", template.id.to_string()),
        row("Name", template.name),
        row("Lang", template.lang),
        row("Repo", template.repo),
        row("Branch", template.branch.unwrap_or("-".to_string())),
        row("Subdir", template.subdir.unwrap_or("-".to_string())),
        row("SHA256 Hash", template.sha256_hash.unwrap_or("-".to_string())),
        row("Created At", template.created_at
            .map(|v| timestamp_to_iso8601(v as i64))
            .unwrap_or("-".to_string())),
        row("Updated At", template.updated_at
            .map(|v| timestamp_to_iso8601(v as i64))
            .unwrap_or("-".to_string())),
    ];

    print_table(rows);

    Ok(())
}

#[derive(Tabled)]
pub struct ShowResult {
    #[tabled(rename = "Key")]
    key: String,
    #[tabled(rename = "Value")]
    value: String,
}
