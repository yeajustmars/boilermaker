use std::fs::File;
use std::io;

use clap::{Command, Parser};
use clap_complete::{Shell, generate};
use color_eyre::Result;

use crate::state::AppState;

#[derive(Debug, Parser)]
pub struct GenBash {
    #[arg(short, long)]
    pub file: Option<String>,
}

#[tracing::instrument]
pub async fn gen_bash(app_state: &AppState, cmd: &GenBash, clap_cli: &mut Command) -> Result<()> {
    let bin_name = "boil";

    if let Some(path) = &cmd.file {
        let mut file = File::create(path)?;
        generate(Shell::Bash, clap_cli, bin_name, &mut file)
    } else {
        generate(Shell::Bash, clap_cli, bin_name, &mut io::stdout());
    }

    Ok(())
}
