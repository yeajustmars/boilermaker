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

fn clone_to_repo_root(repo_root: &PathBuf, cmd: &New) -> Result<Repository> {
    info!("Cloning into temporary directory: {}", repo_root.display());

    if repo_root.exists() {
        fs::remove_dir_all(&repo_root)?;
    }

    let mut fetch_opts = FetchOptions::new();
    fetch_opts.depth(1);

    let mut repo_builder = RepoBuilder::new();
    repo_builder.fetch_options(fetch_opts);

    if let Some(branch) = &cmd.branch {
        repo_builder.branch(branch);
    }

    let repo = repo_builder.clone(&cmd.template, &repo_root)?;
    Ok(repo)
}

// TODO: see if it's possible to do a sparse checkout with git2
fn make_template_root_dir(repo_root: &PathBuf, cmd: &New) -> PathBuf {
    match &cmd.subdir {
        Some(subdir) => repo_root.join(subdir),
        None => repo_root.to_owned(),
    }
}

// TODO: should we be using PathBuf or just Path here?
pub fn clone_git_repo(
    _sys_config: &toml::Value,
    cmd: &New,
) -> Result<(PathBuf, toml::Value, Repository)> {
    let repo_root = env::temp_dir().join(&cmd.name);
    println!("repo_root: {}", repo_root.display());
    let tpl_root = make_template_root_dir(&repo_root, cmd);
    println!("tpl_root: {}", tpl_root.display());
    let tpl_config_path = &tpl_root.join("boilermaker.toml");
    let tpl_config = get_template_config(tpl_config_path.as_path())?;

    println!("============================================================= x");
    let x = tpl_config.get("boilermaker");
    println!("tpl_config.boilermaker: {:#?}", x);
    println!("------------------------------------------------------------- x");

    let repo = clone_to_repo_root(&repo_root, cmd)?;

    /*
    let lang = if let Some(lang) = &cmd.lang {
        lang.clone()
    } else if let Some(default_lang) = tpl_config.get("default_language").and_then(|v| v.as_str()) {
        default_lang.to_string()
    } else {
        return Err(color_eyre::eyre::eyre!(
            "Language not specified and no default_language in config"
        ));
    };

    let tpl_files_path = false;
    let tpl_files = false;
     */

    Ok((tpl_root, tpl_config, repo))
}

#[tracing::instrument]
pub fn create_new(sys_config: &toml::Value, cmd: &New) -> Result<()> {
    info!("Creating new project...");
    info!("Name: {}", cmd.name);
    info!("Template: {}", cmd.template);

    let (tpl_root, tpl_config, repo) = clone_git_repo(sys_config, &cmd)?;
    println!("----> tpl_root: {}", tpl_root.display());
    println!("----> repo path: {}", repo.path().display());
    // println!("----> tpl_config: {:#?}", tpl_config);

    Ok(())
}
