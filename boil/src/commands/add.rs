use std::fs;
use std::path::PathBuf;

use clap::Parser;

use color_eyre::Result;
use tracing::info;

use crate::local_cache::{BOILERMAKER_LOCAL_CACHE_PATH, LocalCache};
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
    #[arg(short, long)]
    pub output: Option<String>,
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
            output: cmd.output.to_owned(),
        }
    }
}

#[tracing::instrument]
pub async fn add(sys_config: &toml::Value, cmd: &Add) -> Result<()> {
    info!("Adding template: {}", &cmd.name);
    info!("Name: {}", cmd.name);
    info!("Template: {}", cmd.template);

    let mut cmd = TemplateCommand::from(cmd);
    if cmd.output.is_none() {
        cmd.output = Some(
            BOILERMAKER_TEMPLATES_DIR
                .join(&cmd.name)
                .to_str()
                .unwrap()
                .to_string(),
        );
    }

    let ctx = get_template(sys_config, &cmd).await?;

    // TODO: decide if output_dir should be cleared automatically if exists.
    let output_dir = PathBuf::from(cmd.output.as_ref().unwrap());
    if output_dir.exists() {
        fs::remove_dir_all(&output_dir)?;
    }

    let local_cache_path = BOILERMAKER_LOCAL_CACHE_PATH.to_str().unwrap();
    let local_cache = LocalCache::new(local_cache_path).await?;

    if !local_cache.template_table_exists().await? {
        local_cache.create_template_table().await?;
    }

    let new_id = local_cache.add_template(cmd).await?;
    info!("Added template with ID: {}", new_id);

    let _ = move_to_output_dir(&ctx).await?;

    Ok(())
}
