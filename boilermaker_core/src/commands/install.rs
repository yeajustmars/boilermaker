use clap::Parser;
use color_eyre::{Result, eyre::eyre};
use tracing::info;

use crate::db::TemplateRow;
use crate::state::AppState;
use crate::template::{
    CloneContext, clean_dir_if_overwrite, clone_repo, create_template_dir, get_lang,
    get_template_config, install_template, make_name_from_url, make_tmp_dir_from_url,
    remove_git_dir,
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
    #[arg(short = 'O', long, default_value_t = false)]
    pub overwrite: bool,
}

#[tracing::instrument]
pub async fn install(app_state: &AppState, cmd: &Install) -> Result<()> {
    let name = if let Some(name) = &cmd.name {
        name.to_owned()
    } else {
        make_name_from_url(&cmd.template)
    };

    info!("Installing template: {name}");

    let repo_ctx = CloneContext::from(cmd);
    let clone_dir = repo_ctx.dest.as_ref().unwrap();
    let work_dir = if let Some(subdir) = &cmd.subdir {
        clone_dir.join(subdir)
    } else {
        clone_dir.to_path_buf()
    };

    if let Err(err) = clean_dir_if_overwrite(clone_dir, cmd.overwrite) {
        return Err(eyre!("💥 Failed setting up work dir: {}", err));
    }

    if let Err(err) = clone_repo(&repo_ctx).await {
        return Err(eyre!("💥 Failed to clone template: {}", err));
    }

    let cnf = get_template_config(work_dir.as_path())?;
    let lang = get_lang(&cnf, &cmd.lang)?;
    let template_dir = create_template_dir(name.as_str())?;
    let row = TemplateRow {
        name,
        lang,
        template_dir: template_dir.to_str().unwrap().to_string(),
        repo: cmd.template.to_owned(),
        branch: cmd.branch.to_owned(),
        subdir: cmd.subdir.to_owned(),
    };
    let new_id = add_template_to_cache(app_state, row, cmd.overwrite).await?;

    info!("Template added with ID: {}", new_id);

    match install_template(&work_dir, &template_dir, cmd.overwrite).await {
        Ok(_) => info!(
            "Template installed successfully to: {}",
            template_dir.display()
        ),
        Err(e) => {
            return Err(eyre!("💥 Failed to install template: {}", e));
        }
    }

    _ = remove_git_dir(&template_dir);

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

#[tracing::instrument]
async fn add_template_to_cache(
    app_state: &AppState,
    row: TemplateRow,
    overwrite: bool,
) -> Result<i64> {
    let cache = app_state
        .template_db
        .write()
        .map_err(|e| eyre!("💥 Failed to acquire write lock: {}", e))?;

    if !cache.template_table_exists().await? {
        cache.create_template_table().await?;
    }

    if let Some(existing_template) = cache.check_unique(&row).await? {
        if overwrite {
            return Ok(existing_template.id);
        }
        return Err(eyre!(
            "💥 Template with the same name/lang/repo already exists: {:?}.",
            existing_template
        ));
    }

    let new_id = cache.create_template(row).await?;

    Ok(new_id)
}
