use clap::Parser;

use color_eyre::Result;
use tracing::info;

use crate::template;
use crate::template::{BOILERMAKER_TEMPLATES_DIR, TemplateCommand};

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
pub fn add(sys_config: &toml::Value, cmd: &Add) -> Result<()> {
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

    let ctx = template::get_template(sys_config, &cmd)?;

    if cmd.output.exists() {
        fs::remove_dir_all(&cmd.output)?;
    }

    let _ = template::move_to_output_dir(&ctx)?;

    Ok(())
}
