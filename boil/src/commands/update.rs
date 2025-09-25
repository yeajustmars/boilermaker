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
pub(crate) struct Update {
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
}

impl From<&Update> for TemplateCommand {
    #[tracing::instrument]
    fn from(cmd: &Update) -> Self {
        Self {
            name: cmd.name.to_owned(),
            template: cmd.template.to_owned(),
            branch: cmd.branch.to_owned(),
            subdir: cmd.subdir.to_owned(),
            lang: cmd.lang.to_owned(),
            output_dir: cmd.output_dir.to_owned(),
            overwrite: true,
        }
    }
}

impl From<&Update> for TemplateRow {
    #[tracing::instrument]
    fn from(cmd: &Update) -> Self {
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
pub async fn update(sys_config: &toml::Value, cmd: &Update) -> Result<()> {
    info!("Updating template: {}", &cmd.name);
    Ok(())
}
