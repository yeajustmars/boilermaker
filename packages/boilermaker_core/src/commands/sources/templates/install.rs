use clap::Parser;
use color_eyre::{Result, eyre::eyre};

use crate::{
    commands::install::{Install as CoreCmdInstall, install as core_cmd_install},
    db::{SourceTemplateFindParams, SourceTemplateResult},
    state::AppState,
    util::help,
};

#[derive(Debug, Parser)]
pub struct Install {
    #[arg(required = true, help = "Source template to install")]
    pub id_or_name: String,
    #[arg(short = 'n', long, help = "Rename")]
    pub rename: Option<String>,
}

#[tracing::instrument]
async fn get_source_template(app_state: &AppState, cmd: &Install) -> Result<SourceTemplateResult> {
    let cache = app_state.local_db.clone();

    if let Ok(id) = cmd.id_or_name.parse::<i64>() {
        Ok(cache
            .get_source_template(id)
            .await?
            .expect("Failed to unwrap template result"))
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
            0 => Err(eyre!("💥 Cannot find source template: {}.", cmd.id_or_name))?,
            1 => Ok(results
                .first()
                .unwrap_or(Err(eyre!(
                    "💥 Cannot retrieve source template: {}.",
                    cmd.id_or_name
                ))?)
                .to_owned()),
            2.. => {
                help::print_multiple_source_template_results_help(&results);
                Err(eyre!(
                    "💥 Found multiple results matching source template: {}.",
                    cmd.id_or_name
                ))?
            }
        }
    }
}

#[tracing::instrument]
pub async fn install(app_state: &AppState, cmd: &Install) -> Result<()> {
    let st = get_source_template(app_state, cmd).await?;

    let cmd = CoreCmdInstall {
        template: st.repo,
        rename: Some(st.name),
        lang: Some(st.lang),
        branch: st.branch,
        subdir: st.subdir,
        local: false,
    };

    if let Err(e) = core_cmd_install(app_state, &cmd).await {
        return Err(eyre!("Could not install template from source: {e}"));
    }

    Ok(())
}
