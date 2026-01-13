use std::{fs, path::PathBuf};

use clap::Parser;
use color_eyre::{Result, eyre::eyre};
use tracing::info;

use crate::{
    config::DEFAULT_LOCAL_CACHE_PATH,
    state::AppState,
    template::remove_dir_if_exists,
    util::{io::prompt_confirm, math::rand_i32_between},
};

#[derive(Parser, Debug, Clone)]
pub struct Remove {
    #[arg()]
    pub id: Option<i64>,
    #[arg(short = 'a', long = "all", help = "Remove all templates")]
    pub all: bool,
    #[arg(
        short = 'A',
        long = "apocalyptic",
        help = "Removes all installed templates then destroys local Sqlite DB file"
    )]
    pub destroy_db: bool,
}

#[tracing::instrument]
async fn remove_one(app_state: &AppState, cmd: &Remove) -> Result<()> {
    let id = match cmd.id {
        Some(id) => id,
        None => {
            return Err(eyre!(
                "💥 Template ID must be provided unless --all or --destroy-db is specified."
            ));
        }
    };

    let cache = app_state.local_db.clone();

    let template = match cache.get_template(id).await? {
        Some(template) => template,
        None => {
            return Err(eyre!("💥 No template found with ID: {}", id));
        }
    };

    let template_dir = PathBuf::from(&template.template_dir);

    if let Err(err) = remove_dir_if_exists(&template_dir) {
        return Err(eyre!("💥 Failed to remove template directory: {}", err));
    }

    let removed_id = cache.delete_template(id).await?;

    info!("Removed template: {} ({})", removed_id, template.name);

    Ok(())
}

#[tracing::instrument]
async fn remove_all(app_state: &AppState, cmd: &Remove, confirm_action: bool) -> Result<()> {
    if confirm_action {
        tracing::warn!("About to remove **ALL** templates from local cache and filesystem!");
        if !confirm()? {
            info!("Aborting removal of all templates.");
            return Ok(());
        }
    }

    let templates = {
        let cache = app_state.local_db.clone();
        let templates = match cache.list_templates(None).await {
            Ok(templates) => templates,
            Err(err) => {
                return Err(eyre!("💥 Failed to list templates: {}", err));
            }
        };
        cache.delete_templates_all().await?;
        templates
    };

    let mut removed: Vec<(i64, &str)> = Vec::new();
    for t in &templates {
        let dir = PathBuf::from(&t.template_dir);
        if let Err(err) = remove_dir_if_exists(&dir) {
            return Err(eyre!(
                "💥 Failed to remove template directory {:?}: {}",
                t.template_dir,
                err
            ));
        }
        removed.push((t.id, &t.name));
    }

    let out_msg = removed
        .iter()
        .map(|(id, name)| format!("{id} ({name})"))
        .collect::<Vec<String>>()
        .join("\n\t- ");
    info!("Removed templates:\n\t- {}", out_msg);

    Ok(())
}

// TODO: check for custom DB path vs DEFAULT_LOCAL_CACHE_PATH
#[tracing::instrument]
async fn destroy_local_db(app_state: &AppState, cmd: &Remove) -> Result<()> {
    if !DEFAULT_LOCAL_CACHE_PATH.exists() {
        info!(
            "Local DB file does not exist at {:?}, nothing to destroy.",
            DEFAULT_LOCAL_CACHE_PATH.display()
        );
        return Ok(());
    }

    tracing::warn!("About to destroy the local DB!");
    if !confirm()? {
        info!("Aborting deletion of local DB.");
        return Ok(());
    }

    remove_all(app_state, cmd, false).await?;

    match fs::remove_file(DEFAULT_LOCAL_CACHE_PATH.as_path()) {
        Ok(_) => {
            info!(
                "Successfully destroyed local DB at {:?}",
                DEFAULT_LOCAL_CACHE_PATH.display()
            );
        }
        Err(err) => {
            return Err(eyre!(
                "💥 Failed to destroy local DB at {:?}: {}",
                DEFAULT_LOCAL_CACHE_PATH.display(),
                err
            ));
        }
    }

    Ok(())
}

#[tracing::instrument]
pub async fn remove(app_state: &AppState, cmd: &Remove) -> Result<()> {
    if cmd.destroy_db {
        destroy_local_db(app_state, cmd).await
    } else if cmd.all {
        remove_all(app_state, cmd, true).await
    } else {
        remove_one(app_state, cmd).await
    }
}

#[tracing::instrument]
fn confirm() -> Result<bool> {
    let rand_3_digit_int = rand_i32_between(100, 999);
    prompt_confirm(
        &format!(
            "To confirm, please type '{}' and hit ENTER: ",
            rand_3_digit_int
        ),
        &rand_3_digit_int.to_string(),
    )
}
