use std::path::Path;

use clap::Parser;
use color_eyre::{Result, eyre::eyre};

use crate::state::AppState;
use crate::util::validation::supported_langs;

#[derive(Debug, Parser)]
pub struct Blank {
    #[arg(required = true)]
    pub name: String,
    #[arg(required = true)]
    pub lang: Vec<String>,
    #[arg(short, long)]
    pub dir: Option<String>,
}

#[tracing::instrument]
pub fn blank_boilermaker_config_toml(name: &str) -> String {
    format!(
        r#"
[project]
name = \"{name}\"
description = \"\"
version = \"0.1.0\"
default_lang = \"bash\"
repository = \"\"
authors = []
keywords = []
website = \"\"
license = \"\"

[variables]
    "#,
    )
}

#[tracing::instrument]
pub async fn blank(app_state: &AppState, cmd: &Blank) -> Result<()> {
    println!("{cmd:#?}");

    let supported_langs = supported_langs();
    for lang in &cmd.lang {
        if !supported_langs.contains(lang.as_str()) {
            return Err(eyre!(
                "💥 Invalid lang provided: {lang}. See `docs/supported-languages`."
            ));
        }
    }

    println!("TODO: PICKUP HERE");

    Ok(())
}
