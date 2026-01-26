use std::{fs, path::PathBuf};

use clap::Parser;
use color_eyre::{Result, eyre::eyre};
use git2::Repository;
use tracing::{info, warn};

use crate::{
    db::{HashableTemplateValues, TemplateRow},
    state::AppState,
    template::{
        CloneContext, clean_dir, clone_repo, get_lang, get_template_config, get_template_dir_path,
        install_template, make_name_from_url, make_tmp_dir_from_url, open_repo,
    },
    util::{crypto::sha256_hash_string, file::remove_git_dir},
};

#[derive(Debug, Parser)]
pub struct Install {
    #[arg(required = true)]
    pub template: String,
    #[arg(short = 'n', long, help = "Rename")]
    pub rename: Option<String>,
    #[arg(short, long)]
    pub lang: Option<String>,
    #[arg(short, long)]
    pub branch: Option<String>,
    #[arg(short = 'd', long)]
    pub subdir: Option<String>,
    #[arg(short = 'f', long, default_value_t = false)]
    pub local: bool,
}

#[tracing::instrument]
async fn get_local_work_dir(
    repo_ctx: &CloneContext,
    cmd: &Install,
) -> Result<(Repository, PathBuf)> {
    let dir = if let Some(subdir) = &cmd.subdir {
        PathBuf::from(&cmd.template).join(subdir)
    } else {
        PathBuf::from(&cmd.template)
    };

    if !dir.exists() {
        return Err(eyre!(
            "💥 Local template path does not exist: {}",
            dir.display()
        ));
    }

    let repo = open_repo(repo_ctx).await?;

    Ok((repo, dir))
}

#[tracing::instrument]
async fn clone_remote_to_local_work_dir(
    repo_ctx: &CloneContext,
    cmd: &Install,
) -> Result<(Repository, PathBuf)> {
    let clone_dir = repo_ctx.dest.as_ref().unwrap();

    if let Err(err) = clean_dir(clone_dir) {
        return Err(eyre!("💥 Failed setting up clone dir: {}", err));
    }

    info!("Cloning template");
    let repo = match clone_repo(repo_ctx).await {
        Ok(repo) => repo,
        Err(err) => {
            return Err(eyre!("💥 Failed to clone template: {}", err));
        }
    };

    let dir = if let Some(subdir) = &cmd.subdir {
        clone_dir.join(subdir)
    } else {
        clone_dir.to_path_buf()
    };

    Ok((repo, dir))
}

#[tracing::instrument]
async fn configure_install(cmd: &Install) -> Result<InstallConfig> {
    let name = if let Some(name) = &cmd.rename {
        name.to_owned()
    } else {
        make_name_from_url(&cmd.template)
    };

    let repo_ctx = CloneContext::from(cmd);
    let (repo, work_dir) = if cmd.local {
        get_local_work_dir(&repo_ctx, cmd).await?
    } else {
        clone_remote_to_local_work_dir(&repo_ctx, cmd).await?
    };

    let cnf = get_template_config(work_dir.as_path())?;
    let lang = get_lang(&cnf, &cmd.lang)?;

    let branch = if let Some(branch) = &cmd.branch {
        branch.to_owned()
    } else {
        repo.head()?.shorthand().unwrap_or("unknown").to_string()
    };

    let subdir = cmd.subdir.to_owned();

    let mut install = InstallConfig {
        name,
        lang,
        repo: repo_ctx.url.to_owned(),
        branch,
        subdir,
        work_dir,
        sha256_hash: None,
        template_dir: None,
    };
    install.set_hash_string();
    install.set_template_dir();
    Ok(install)
}

// TODO: add default_branch, default_subdir to config
#[tracing::instrument]
pub async fn install(app_state: &AppState, cmd: &Install) -> Result<()> {
    let install = configure_install(cmd).await?;
    let template_dir = install.template_dir.clone().unwrap();
    // TODO: clean up InstallConfig + TemplateRow duplication
    let row = TemplateRow {
        name: install.name.to_owned(),
        lang: install.lang.to_owned(),
        repo: install.repo.to_owned(),
        branch: Some(install.branch.to_owned()),
        subdir: install.subdir.to_owned(),
        sha256_hash: Some(install.sha256_hash.to_owned().unwrap()),
        template_dir: install
            .template_dir
            .clone()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap(),
    };

    let cache = app_state.local_db.clone();

    if !cache.template_table_exists().await? {
        cache.create_schema().await?;
    }

    let existing_db_entry = cache.check_unique(&row).await?;

    if let Some(t) = existing_db_entry {
        if template_dir.exists() {
            return Err(eyre!(
                "💥 Template with the same name/lang/repo already exists: {}, {}, {}",
                t.name,
                t.lang,
                t.repo
            ));
        } else {
            info!(
                "Template entry exists in DB but directory is missing. Reininstalling: {}.",
                t.name
            );
            cache.delete_template(t.id).await?;
        }
    }

    let new_id = cache.create_template(row).await?;

    info!("Template added to cache with ID: {}", new_id);

    if !cmd.local {
        remove_other_langs(&install)?;
    }

    match install_template(&install.work_dir, &template_dir).await {
        Ok(_) => info!(
            "Template installed successfully to: {}",
            template_dir.display()
        ),
        Err(e) => {
            return Err(eyre!("💥 Failed to install template: {}", e));
        }
    }

    cache.index_template(new_id).await?;

    if !cmd.local {
        let path = install.work_dir.clone();
        remove_git_dir(&path)?;
    }

    Ok(())
}

impl From<&Install> for CloneContext {
    #[tracing::instrument]
    fn from(cmd: &Install) -> Self {
        Self {
            url: cmd.template.to_owned(),
            branch: cmd.branch.to_owned(),
            dest: Some(make_tmp_dir_from_url(&cmd.template)),
        }
    }
}

#[derive(Debug, Clone)]
struct InstallConfig {
    name: String,
    lang: String,
    repo: String,
    branch: String,
    subdir: Option<String>,
    work_dir: PathBuf,
    sha256_hash: Option<String>,
    template_dir: Option<PathBuf>,
}

impl InstallConfig {
    #[tracing::instrument]
    pub fn set_hash_string(&mut self) {
        self.sha256_hash = Some(self.hash_values());
    }

    #[tracing::instrument]
    pub fn set_template_dir(&mut self) {
        let hash = self.sha256_hash.as_ref().unwrap();
        let dir = get_template_dir_path(hash).expect("Failed to get template dir path");
        self.template_dir = Some(dir);
    }
}

impl HashableTemplateValues for InstallConfig {
    fn hash_values(&self) -> String {
        let input = format!(
            "{}~~{}~~{}~~{}~~{}",
            self.repo,
            self.name,
            self.lang,
            self.branch,
            self.subdir.as_deref().unwrap_or(""),
        );
        sha256_hash_string(&input)
    }
}

#[tracing::instrument]
fn remove_other_langs(install: &InstallConfig) -> Result<()> {
    let keep = install.lang.as_str();
    for entry in fs::read_dir(&install.work_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let dir_name = path.file_name().unwrap().to_string_lossy();
            if keep == dir_name.as_ref() {
                continue;
            }
            std::fs::remove_dir_all(&path)?;
        }
    }

    Ok(())
}
