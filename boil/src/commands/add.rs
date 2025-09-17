use clap::Parser;

use color_eyre::{Result, eyre::eyre};
use tracing::info;

#[derive(Debug, Parser)]
pub(crate) struct Add {
    #[arg(required = true)]
    pub name: String,
    #[arg(short, long)]
    pub template: String,
    #[arg(short, long)]
    pub branch: Option<String>,
    #[arg(short = 'd', long)]
    pub subdir: Option<String>,
    #[arg(short, long)]
    pub output: Option<String>,
}

#[tracing::instrument]
pub fn add(sys_config: &toml::Value, cmd: &Add) -> Result<()> {
    info!("Adding new template: {}", &cmd.name);
    Ok(())
}
