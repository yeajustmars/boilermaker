use std::{env, fs, path::PathBuf};

use clap::Parser;
use color_eyre::{Result, eyre::eyre};
// use colored::Colorize;
use fs_extra::copy_items_with_progress;
use git2::{FetchOptions, Repository, build::RepoBuilder};
// use nu_ansi_term::Style; // TODO: possibly replace nu_ansi_term with colored
use toml;
use tracing::info;

use crate::config::{BoilermakerConfig, get_template_config};

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

// TODO: add local .cache dir that doesn't need to copy every time (maybe 10 minutes?)
#[tracing::instrument]
fn clone_repo(src_root: &PathBuf, cmd: &New) -> Result<Repository> {
    info!("Cloning into temporary directory: {}", src_root.display());

    let mut fetch_opts = FetchOptions::new();
    fetch_opts.depth(1);

    let mut repo_builder = RepoBuilder::new();
    repo_builder.fetch_options(fetch_opts);

    if let Some(branch) = &cmd.branch {
        repo_builder.branch(branch);
    }

    let repo = repo_builder.clone(&cmd.template, &src_root)?;
    Ok(repo)
}

#[derive(Debug)]
pub struct TemplateContext {
    lang: String,
    repo_root: PathBuf,
    src_root: PathBuf,
    target_root: PathBuf,
    target_dir: PathBuf,
}

#[tracing::instrument]
fn get_lang(cmd: &New, cfg: &BoilermakerConfig) -> Result<String> {
    if let Some(lang_option) = &cmd.lang {
        info!("Using `--lang` from command line: {}", lang_option);
        return Ok(lang_option.clone());
    }

    if let Some(default_lang) = &cfg.boilermaker.project.default_lang {
        info!("Using `default_lang` from template config: {default_lang}");
        return Ok(default_lang.clone());
    }

    return Err(eyre!(
        "Can't find language. Pass `--lang` option or add `default_lang` to `boilermaker.toml`."
    ));
}

#[tracing::instrument]
fn copy_files_to_target(
    template_files_path: &PathBuf,
    lang: &str,
    target_root: &PathBuf,
    target_dir: &PathBuf,
) -> Result<()> {
    match fs::create_dir(&target_root) {
        Ok(_) => info!("Created target directory: {}", target_root.display()),
        Err(e) => return Err(eyre!("Failed to create target directory: {e}")),
    }

    match fs::create_dir(&target_dir) {
        Ok(_) => info!("Created target directory: {}", target_dir.display()),
        Err(e) => return Err(eyre!("Failed to create target directory: {e}")),
    }

    let files_iter = fs::read_dir(&template_files_path)?;
    let files: Vec<PathBuf> = files_iter
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .collect();

    info!("Copying template files for language '{}'...", lang);
    match copy_items_with_progress(
        &files,
        &target_dir,
        &fs_extra::dir::CopyOptions::new(),
        |progress| {
            info!("\tCopied {} bytes", progress.copied_bytes);
            fs_extra::dir::TransitProcessResult::ContinueOrAbort
        },
    ) {
        Ok(_) => info!(
            "Copied template files to target directory: {}",
            target_root.display()
        ),
        Err(e) => return Err(eyre!("Failed to copy template files: {e}")),
    }

    Ok(())
}

#[tracing::instrument]
pub fn get_template(_sys_config: &toml::Value, cmd: &New) -> Result<TemplateContext> {
    let repo_root = env::temp_dir().join(&cmd.name);
    let src_root = repo_root.join("src");
    let template_root = make_template_root_dir(&src_root, cmd);
    let cfg_path = template_root.join("boilermaker.toml");
    let cfg: BoilermakerConfig = get_template_config(cfg_path.as_path())?;
    let lang = get_lang(&cmd, &cfg)?;
    let target_root = repo_root.join("target");
    let target_dir = target_root.join(&lang);
    let template_files_path = template_root.join(&lang);

    if repo_root.exists() {
        fs::remove_dir_all(&repo_root)?;
    }
    let _repo = clone_repo(&src_root, cmd)?;
    let _ = copy_files_to_target(&template_files_path, &lang, &target_root, &target_dir)?;

    // let target_root = target_root.join(&lang);

    Ok(TemplateContext {
        lang: lang.clone(),
        repo_root,
        src_root,
        target_root,
        target_dir,
    })
}

#[tracing::instrument]
pub fn new(sys_config: &toml::Value, cmd: &New) -> Result<()> {
    info!("Creating new project...");
    info!("Name: {}", cmd.name);
    info!("Template: {}", cmd.template);

    let ctx = get_template(sys_config, &cmd)?;
    println!("---->  ctx: {:#?}", ctx);

    Ok(())
}
