use std::{env, fs, path::PathBuf};

use clap::Parser;
use color_eyre::Result;
// use gix::{attrs::search::Outcome, clone, progress::Discard, remote::fetch::Outcome};
use git2::{FetchOptions, Repository, build::RepoBuilder};
use tracing::info;

#[derive(Debug, Parser)]
pub(crate) struct New {
    #[arg(required = true)]
    pub name: String,

    #[arg(short, long)]
    pub template: String,

    #[arg(short, long)]
    pub branch: Option<String>,

    #[arg(short = 'd', long)]
    pub subdir: Option<String>,
}

pub fn clone_repo(
    template: &str,
    name: &str,
    branch: Option<&str>,
    subdir: Option<&str>,
) -> Result<(PathBuf, Repository)> {
    let mut local_path = env::temp_dir().join(name);
    println!("Cloning into temporary directory: {}", local_path.display());

    if local_path.exists() {
        fs::remove_dir_all(&local_path)?;
    }

    let mut fetch_opts = FetchOptions::new();
    fetch_opts.depth(1);

    let mut repo_builder = RepoBuilder::new();
    repo_builder.fetch_options(fetch_opts);

    if let Some(branch) = branch {
        repo_builder.branch(branch);
    }

    let repo = repo_builder.clone(template, &local_path)?;

    // TODO: see if it's possible to do a sparse checkout with git2
    if let Some(subdir) = subdir {
        local_path = local_path.join(subdir);
    }
    println!("Final local path: {}", local_path.display());

    Ok((local_path, repo))
}

#[tracing::instrument]
pub fn create_new(cmd: &New) -> Result<()> {
    info!("Creating new project...");
    info!("Name: {}", cmd.name);
    info!("Template: {}", cmd.template);

    if let Some(branch) = &cmd.branch {
        info!("Branch: {}", branch);
    }

    let _ = clone_repo(
        &cmd.template,
        &cmd.name,
        cmd.branch.as_deref(),
        cmd.subdir.as_deref(),
    )?;

    Ok(())
}
