use std::fs;

use clap::Parser;
use color_eyre::{Result, eyre::eyre};
use tracing::info;

use boil::AppState;
use template::{self, CloneContext};

/*
use std::fs;
use std::path::PathBuf;

use clap::Parser;

use color_eyre::Result;
use tracing::info;

use crate::local_cache::{BOILERMAKER_LOCAL_CACHE_PATH, LocalCache, TemplateRow};
use crate::template::{
    BOILERMAKER_TEMPLATES_DIR, TemplateCommand, get_template, move_to_output_dir,
};

#[derive(Debug, Parser)]
pub(crate) struct Add {
    #[arg(required = true)]
    pub name: String,
    #[arg(short, long)]
    pub template: String,
    #[arg(short, long)]
    pub lang: Option<String>,
    #[arg(short, long)]
    pub branch: Option<String>,
    #[arg(short = 'd', long)]
    pub subdir: Option<String>,
    #[arg(short, long = "output-dir")]
    pub output_dir: Option<String>,
    #[arg(short = 'O', long, default_value_t = false)]
    pub overwrite: bool,
}

impl From<&Add> for TemplateCommand {
    #[tracing::instrument]
    fn from(cmd: &Add) -> Self {
        Self {
            name: cmd.name.to_owned(),
            template: cmd.template.to_owned(),
            branch: cmd.branch.to_owned(),
            subdir: cmd.subdir.to_owned(),
            lang: cmd.lang.to_owned(),
            output_dir: cmd.output_dir.to_owned(),
            overwrite: cmd.overwrite,
        }
    }
}

impl From<&Add> for TemplateRow {
    #[tracing::instrument]
    fn from(cmd: &Add) -> Self {
        Self {
            name: cmd.name.to_owned(),
            lang: cmd.lang.to_owned().unwrap_or_default(),
            template_dir: cmd.output_dir.to_owned().unwrap_or_default(),
            repo: cmd.template.to_owned(),
            branch: cmd.branch.to_owned(),
            subdir: cmd.subdir.to_owned(),
        }
    }
}

#[tracing::instrument]
pub async fn add(sys_config: &toml::Value, cmd: &Add) -> Result<()> {
    info!("Adding template: {}", &cmd.name);
    info!("Template: {}", cmd.template);

    // TODO: short-circuit if template already exists in local cache (before cloning)

    let mut cmd = TemplateCommand::from(cmd);
    if cmd.output_dir.is_none() {
        cmd.output_dir = Some(
            BOILERMAKER_TEMPLATES_DIR
                .join(&cmd.name)
                .to_str()
                .unwrap()
                .to_string(),
        );
    }

    let ctx = get_template(sys_config, &cmd).await?;

    // TODO: decide if output_dir should be cleared automatically if exists.
    let output_dir = PathBuf::from(cmd.output_dir.as_ref().unwrap());
    if output_dir.exists() {
        fs::remove_dir_all(&output_dir)?;
    }

    let local_cache_path = BOILERMAKER_LOCAL_CACHE_PATH.to_str().unwrap();
    let local_cache = LocalCache::new(local_cache_path).await?;

    if !local_cache.template_table_exists().await? {
        local_cache.create_template_table().await?;
    }

    // TODO: look at using a single struct for both TemplateCommand and TemplateContext
    if cmd.lang.is_none() {
        cmd.lang = Some(ctx.lang.to_owned());
    }

    let row = TemplateRow::from(&cmd);

    let new_id = local_cache.add_template(row).await?;
    info!("Added template with ID: {}", new_id);

    let _ = move_to_output_dir(&ctx).await?;

    Ok(())
}
 */

#[derive(Debug, Parser)]
pub struct Add {
    #[arg(required = true)]
    pub template: String,
    #[arg(short, long)]
    pub branch: Option<String>,
    #[arg(short = 'O', long, default_value_t = false)]
    pub overwrite: bool,
}

#[tracing::instrument]
pub async fn add(_sys_config: &toml::Value, _app_state: &AppState, cmd: &Add) -> Result<()> {
    info!("Adding template: {}", &cmd.template);

    let repo_ctx = CloneContext::from(cmd);
    let work_dir = repo_ctx.dest.as_ref().unwrap();

    if work_dir.as_path().exists() {
        if cmd.overwrite {
            fs::remove_dir_all(work_dir)?;
        } else {
            return Err(eyre!(
                "Directory {} already exists. Use --overwrite to force.",
                work_dir.display()
            ));
        }
    }

    let _repo = template::clone_repo(&repo_ctx).await?;

    Ok(())
}

impl From<&Add> for CloneContext {
    #[tracing::instrument]
    fn from(cmd: &Add) -> CloneContext {
        CloneContext {
            url: cmd.template.to_owned(),
            branch: cmd.branch.to_owned(),
            dest: Some(template::make_tmp_dir_from_url(&cmd.template)),
        }
    }
}
