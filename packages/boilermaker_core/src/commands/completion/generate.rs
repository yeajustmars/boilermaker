use std::fs::File;
use std::io;

use clap::{Command, Parser};
use clap_complete::{Shell, generate as generate_clap_complete};
use color_eyre::{Result, eyre::eyre};
use tracing::info;

use crate::state::AppState;

const BIN_NAME: &str = "boil";

#[derive(Debug, Parser)]
pub struct Generate {
    #[arg(required = true)]
    pub shell: String,
    #[arg(short, long)]
    pub file: Option<String>,
}

#[tracing::instrument]
pub async fn generate(app_state: &AppState, cmd: &Generate, clap_cli: &mut Command) -> Result<()> {
    let shell = match cmd.shell.to_lowercase().as_str() {
        "bash" => Shell::Bash,
        "zsh" => Shell::Zsh,
        _ => {
            return Err(eyre!(
                "Unsupported shell: {}. Supported shells are: bash, zsh",
                cmd.shell
            ));
        }
    };

    if let Some(path) = &cmd.file {
        let mut file = File::create(path)?;
        generate_clap_complete(shell, clap_cli, BIN_NAME, &mut file);
        info!(
            "Generated completion script for {} in {}",
            cmd.shell,
            cmd.file.as_deref().unwrap_or("stdout")
        );
    } else {
        generate_clap_complete(shell, clap_cli, BIN_NAME, &mut io::stdout());
    }

    Ok(())
}
