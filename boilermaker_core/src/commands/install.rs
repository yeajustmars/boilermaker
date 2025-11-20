use clap::Parser;
use color_eyre::{Result, eyre::eyre};
use tracing::{error, info};

use crate::db::TemplateRow;
use crate::state::AppState;
use crate::template::{
    CloneContext, clean_dir, clone_repo, get_lang, get_template_config, get_template_dir_path,
    install_template, make_name_from_url, make_tmp_dir_from_url, remove_git_dir,
};

#[derive(Debug, Parser)]
pub struct Install {
    #[arg(required = true)]
    pub template: String,
    #[arg(short, long)]
    pub name: Option<String>,
    #[arg(short, long)]
    pub lang: Option<String>,
    #[arg(short, long)]
    pub branch: Option<String>,
    #[arg(short = 'd', long)]
    pub subdir: Option<String>,
}

#[tracing::instrument]
pub async fn install(app_state: &AppState, cmd: &Install) -> Result<()> {
    let name = if let Some(name) = &cmd.name {
        name.to_owned()
    } else {
        make_name_from_url(&cmd.template)
    };

    let repo_ctx = CloneContext::from(cmd);
    let clone_dir = repo_ctx.dest.as_ref().unwrap();

    if let Err(err) = clean_dir(clone_dir) {
        return Err(eyre!("💥 Failed setting up clone dir: {}", err));
    }

    info!("Cloning template");
    if let Err(err) = clone_repo(&repo_ctx).await {
        return Err(eyre!("💥 Failed to clone template: {}", err));
    }

    let work_dir = if let Some(subdir) = &cmd.subdir {
        clone_dir.join(subdir)
    } else {
        clone_dir.to_path_buf()
    };

    let cnf = get_template_config(work_dir.as_path())?;
    let lang = get_lang(&cnf, &cmd.lang)?;
    let template_dir = get_template_dir_path(&name)?;
    let row = TemplateRow {
        name,
        lang,
        template_dir: template_dir.to_str().unwrap().to_string(),
        repo: cmd.template.to_owned(),
        branch: cmd.branch.to_owned(),
        subdir: cmd.subdir.to_owned(),
        sha256_hash: None,
    };
    let row = row.set_hash_string();

    let cache = app_state.template_db.clone();

    // TODO: also create template_content + FTS tables
    if !cache.template_table_exists().await? {
        cache.create_template_table().await?;
    }

    let existing_db_entry = cache.check_unique(&row).await?;

    if let Some(t) = existing_db_entry {
        if template_dir.exists() {
            error!(
                "💥 Template with the same name/lang/repo already exists: {}, {}, {}",
                t.name, t.lang, t.repo
            );
            return Ok(());
        } else {
            info!(
                "Template entry exists in DB but directory is missing. Reininstalling: {}.",
                t.name
            );
            cache.delete_template(t.id).await?;
        }
    }

    let new_id = cache.create_template(row).await?;

    info!("Template added to cache with ID: {}", new_id);

    match install_template(&work_dir, &template_dir).await {
        Ok(_) => info!(
            "Template installed successfully to: {}",
            template_dir.display()
        ),
        Err(e) => {
            return Err(eyre!("💥 Failed to install template: {}", e));
        }
    }

    remove_git_dir(&template_dir)?;

    Ok(())
}

impl From<&Install> for CloneContext {
    #[tracing::instrument]
    fn from(cmd: &Install) -> Self {
        Self {
            url: cmd.template.to_owned(),
            branch: cmd.branch.to_owned(),
            dest: Some(make_tmp_dir_from_url(&cmd.template)),
        }
    }
}
