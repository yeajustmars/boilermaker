use std::{env, fs, path::PathBuf};

use clap::Parser;
use color_eyre::{Result, eyre::eyre};
use git2::{FetchOptions, Repository, build::RepoBuilder};
use toml;
use tracing::info;

use crate::config::get_template_config;

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
}

pub fn clone_git_repo(
    _sys_config: &toml::Value,
    cmd: &New,
) -> Result<(PathBuf, toml::Value, Repository)> {
    let base_path = env::temp_dir().join(&cmd.name);
    info!("Cloning into temporary directory: {}", base_path.display());

    if base_path.exists() {
        fs::remove_dir_all(&base_path)?;
    }

    let mut fetch_opts = FetchOptions::new();
    fetch_opts.depth(1);

    let mut repo_builder = RepoBuilder::new();
    repo_builder.fetch_options(fetch_opts);

    if let Some(branch) = &cmd.branch {
        repo_builder.branch(branch);
    }

    let repo = repo_builder.clone(&cmd.template, &base_path)?;

    // TODO: see if it's possible to do a sparse checkout with git2
    let local_path = if let Some(subdir) = cmd.subdir.as_deref() {
        base_path.join(subdir).to_owned()
    } else {
        base_path.to_owned()
    };
    println!("Final local path: {}", local_path.display());

    let tpl_config_path = &local_path.join("boilermaker.toml");
    let tpl_config = get_template_config(tpl_config_path.as_path())?;

    let lang = if let Some(lang) = &cmd.lang {
        lang.clone()
    } else if let Some(default_lang) = tpl_config.get("default_language").and_then(|v| v.as_str()) {
        default_lang.to_string()
    } else {
        return Err(color_eyre::eyre::eyre!(
            "Language not specified and no default_language in config"
        ));
    };

    Ok((local_path, tpl_config, repo))
}

#[tracing::instrument]
pub fn create_new(sys_config: &toml::Value, cmd: &New) -> Result<()> {
    info!("Creating new project...");
    info!("Name: {}", cmd.name);
    info!("Template: {}", cmd.template);

    let (local_path, tpl_config, repo) = clone_git_repo(sys_config, &cmd)?;
    println!("----> local_path: {}", local_path.display());
    println!("----> repo path: {}", repo.path().display());
    println!("----> tpl_config: {:#?}", tpl_config);

    Ok(())
}
