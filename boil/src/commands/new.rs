use std::{env, fs, path::PathBuf};

use clap::Parser;
use color_eyre::{Result, eyre::eyre};
use git2::{FetchOptions, Repository, build::RepoBuilder};
use toml;
use tracing::info;

use crate::config;

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

// TODO: see if it's possible to do a sparse checkout with git2
fn make_template_root_dir(repo_root: &PathBuf, cmd: &New) -> PathBuf {
    match &cmd.subdir {
        Some(subdir) => repo_root.join(subdir),
        None => repo_root.to_owned(),
    }
}

#[tracing::instrument]
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

#[derive(Debug)]
pub struct TemplateContext {
    lang: String,
    repo_root: PathBuf,
    template_root: PathBuf,
    template_files_path: PathBuf,
}

#[tracing::instrument]
pub fn get_template(_sys_config: &toml::Value, cmd: &New) -> Result<TemplateContext> {
    let repo_root = env::temp_dir().join(&cmd.name);
    let _repo = clone_to_repo_root(&repo_root, cmd)?;

    let template_root = make_template_root_dir(&repo_root, cmd);

    let cfg_path = template_root.join("boilermaker.toml");
    let cfg = config::get_template_config(cfg_path.as_path())?;

    let lang = if let Some(lang_option) = &cmd.lang {
        info!("Using `--lang` from command line: {}", lang_option);
        lang_option.clone()
    } else if let Some(default_lang) = cfg.boilermaker.project.default_lang {
        info!("Using `default_lang` from template config: {default_lang}");
        default_lang
    } else {
        return Err(eyre!(
            "Can't find language. Pass `--lang` option or add `default_lang` to `boilermaker.toml`."
        ));
    };

    let template_files_path = template_root.join(&lang);

    Ok(TemplateContext {
        lang: lang.clone(),
        repo_root,
        template_root,
        template_files_path,
    })
}

#[tracing::instrument]
pub fn create_new(sys_config: &toml::Value, cmd: &New) -> Result<()> {
    info!("Creating new project...");
    info!("Name: {}", cmd.name);
    info!("Template: {}", cmd.template);

    let ctx = get_template(sys_config, &cmd)?;
    println!("---->  ctx: {:#?}", ctx);

    Ok(())
}
