use clap::Parser;
use color_eyre::Result;
use tracing::info;

use crate::AppState;
use template::get_or_make_project_dir;

/*
use clap::Parser;
use color_eyre::{Result, eyre::eyre};
use toml;
use tracing::info;

use crate::template::{TemplateCommand, get_template, move_to_output_dir, render_template_files};

#[derive(Debug, Parser)]
pub(crate) struct New {
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

impl From<&New> for TemplateCommand {
    #[tracing::instrument]
    fn from(cmd: &New) -> Self {
        Self {
            name: cmd.name.to_owned(),
            template: cmd.template.to_owned(),
            lang: cmd.lang.to_owned(),
            branch: cmd.branch.to_owned(),
            subdir: cmd.subdir.to_owned(),
            output_dir: cmd.output_dir.to_owned(),
            overwrite: cmd.overwrite,
        }
    }
}

/*
pub struct TemplateContext {
    pub lang: String,
    pub repo_root: PathBuf,
    pub src_root: PathBuf,
    pub target_root: PathBuf,
    pub target_dir: PathBuf,
    pub output_dir: PathBuf,
    pub template_files: Vec<PathBuf>,
    pub vars: HashMap<String, String>,
    pub overwrite: bool,
}
 */

#[tracing::instrument]
pub async fn new(sys_config: &toml::Value, cmd: &New) -> Result<()> {
    info!("Creating new project...");
    info!("Name: {}", cmd.name);
    info!("Template: {}", cmd.template);

    // 1. get name + lang
    // 2. check if template exists in local cache
    // 3. if not, clone template repo to local cache

    let cmd = TemplateCommand::from(cmd);

    // TODO: move cache and other global state to a passed state struct
    let local_cache_path = BOILERMAKER_LOCAL_CACHE_PATH.to_str().unwrap();
    let local_cache = LocalCache::new(local_cache_path).await?;

    let ctx = get_template(sys_config, &cmd).await?;

    if let Err(e) = render_template_files(ctx.template_files.clone(), &ctx).await {
        return Err(eyre!("💥 Failed to render template files: {e}"));
    }

    let _ = move_to_output_dir(&ctx).await?;

    info!("All set. Happy hacking! 🚀");
    Ok(())
}
 */

#[derive(Debug, Parser)]
pub struct New {
    #[arg(required = true)]
    pub name: String,
    #[arg(short, long)]
    pub rename: Option<String>,
    #[arg(short, long)]
    pub lang: Option<String>,
    #[arg(short, long)]
    pub dir: Option<String>,
    #[arg(short = 'P', long = "output-path")]
    pub output_path: Option<String>,
    #[arg(short = 'O', long, default_value_t = false)]
    pub overwrite: bool,
}

#[tracing::instrument]
pub async fn new(app_state: &AppState, cmd: &New) -> Result<()> {
    let project_name = if let Some(rename) = &cmd.rename {
        rename
    } else {
        &cmd.name
    };
    info!("Creating new project: {project_name}");

    let project_dir = get_or_make_project_dir(&project_name, cmd.dir.as_deref()).await?;
    info!("Using project directory: {}", project_dir.display());

    Ok(())
}
