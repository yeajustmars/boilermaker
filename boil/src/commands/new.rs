use std::fs;

use clap::Parser;
use color_eyre::{Result, eyre::eyre};
use toml;
use tracing::info;

use crate::template;
use crate::template::TemplateCommand;

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
    #[arg(short, long)]
    pub output: Option<String>,
}

impl From<&New> for TemplateCommand {
    fn from(cmd: &New) -> Self {
        Self {
            name: cmd.name.to_owned(),
            template: cmd.template.to_owned(),
            lang: cmd.lang.to_owned(),
            branch: cmd.branch.to_owned(),
            subdir: cmd.subdir.to_owned(),
            output: cmd.output.to_owned(),
        }
    }
}

#[tracing::instrument]
pub fn new(sys_config: &toml::Value, cmd: &New) -> Result<()> {
    info!("Creating new project...");
    info!("Name: {}", cmd.name);
    info!("Template: {}", cmd.template);

    let cmd = TemplateCommand::from(cmd);
    let ctx = template::get_template(sys_config, &cmd)?;

    if let Err(e) = template::render_template_files(ctx.template_files.clone(), &ctx) {
        return Err(eyre!("💥 Failed to render template files: {e}"));
    }

    let output_dir = &ctx.output_dir;

    if !&output_dir.is_dir() {
        match fs::create_dir_all(&output_dir) {
            Ok(_) => info!("Created output directory: {}", output_dir.display()),
            Err(e) => return Err(eyre!("💥 Failed to create output directory: {e}")),
        }
    } else {
        return Err(eyre!(
            "💥 Output directory already exists: {}",
            output_dir.display()
        ));
    }

    match fs::rename(&ctx.target_dir, &output_dir) {
        Ok(_) => info!(
            "Moved project to output directory: {}",
            output_dir.display()
        ),
        Err(e) => return Err(eyre!("💥 Failed to move project to output directory: {e}")),
    }

    info!("All set. Happy hacking! 🚀");
    Ok(())
}
