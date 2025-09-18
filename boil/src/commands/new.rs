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

#[tracing::instrument]
pub async fn new(sys_config: &toml::Value, cmd: &New) -> Result<()> {
    info!("Creating new project...");
    info!("Name: {}", cmd.name);
    info!("Template: {}", cmd.template);

    let cmd = TemplateCommand::from(cmd);
    let ctx = get_template(sys_config, &cmd).await?;

    if let Err(e) = render_template_files(ctx.template_files.clone(), &ctx).await {
        return Err(eyre!("💥 Failed to render template files: {e}"));
    }

    let _ = move_to_output_dir(&ctx).await?;

    info!("All set. Happy hacking! 🚀");
    Ok(())
}
