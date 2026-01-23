use clap::Parser;
use color_eyre::{Result, eyre::eyre};

use crate::{db::SourceTemplateFindParams, state::AppState, util::help};

#[derive(Debug, Parser)]
pub struct Install {
    #[arg(required = true, help = "Source template to install")]
    pub id_or_name: String,
    #[arg(short = 'n', long, help = "Rename")]
    pub rename: Option<String>,
}

#[tracing::instrument]
pub async fn install(app_state: &AppState, cmd: &Install) -> Result<()> {
    /*
    let st = if let Ok(id) = cmd.id_or_name.parse::<i64>() {
        let cache = app_state.local_db.clone();
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
                help::print_multiple_template_results_help(&results);
                Err(eyre!(
                    "💥 Found multiple results matching template: {}.",
                    cmd.id_or_name
                ))?
            }
        }
    };
     */

    Ok(())
}
