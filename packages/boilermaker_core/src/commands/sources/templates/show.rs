use clap::Parser;
use color_eyre::{Result, eyre::eyre};

use crate::{
    commands::show::row,
    db::{SourceTemplateFindParams, SourceTemplateResult},
    state::AppState,
    util::{help, output::print_table, time::timestamp_to_iso8601},
};

#[derive(Debug, Parser)]
pub struct Show {
    #[arg(required = true)]
    pub id_or_name: String,
}

#[tracing::instrument]
async fn get_source_template(app_state: &AppState, cmd: &Show) -> Result<SourceTemplateResult> {
    let cache = app_state.local_db.clone();

    if let Ok(id) = cmd.id_or_name.parse::<i64>() {
        Ok(cache
            .get_source_template(id)
            .await?
            .expect("Failed to unwrap source template result"))
    } else {
        let find_params = SourceTemplateFindParams {
            ids: None,
            source_ids: None,
            name: Some(cmd.id_or_name.to_owned()),
            lang: None,
            repo: None,
            branch: None,
            subdir: None,
            sha256_hash: None,
        };
        let results = cache.find_source_templates(find_params).await?;

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
                help::print_multiple_source_template_results_help(&results);
                Err(eyre!(
                    "💥 Found multiple results matching template: {}.",
                    cmd.id_or_name
                ))?
            }
        }
    }
}

// TODO: look at making various show's take a trait object as this isn't DRY
pub async fn show(app_state: &AppState, cmd: &Show) -> Result<()> {
    let template = get_source_template(app_state, cmd).await?;

    #[rustfmt::skip]
    let rows = vec![
        row("ID", template.id.to_string()),
        row("Source ID", template.source_id.to_string()),
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
