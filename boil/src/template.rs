use std::{collections::HashMap, env, fs, path::PathBuf};

use color_eyre::{Result, eyre::eyre};
// use colored::Colorize;
use fs_extra::copy_items_with_progress;
use git2::{FetchOptions, Repository, build::RepoBuilder};
use lazy_static::lazy_static;
use minijinja;
// use nu_ansi_term::Style; // TODO: possibly replace nu_ansi_term with colored
use serde::{Deserialize, Serialize};
use toml;
use tracing::info;
use walkdir::WalkDir;

use crate::{
    config::{BoilermakerConfig, get_template_config},
    local_cache::{BOILERMAKER_LOCAL_CACHE_PATH, LocalCache},
};

// TODO: move to a constants mod
lazy_static! {
    pub static ref BOILERMAKER_TEMPLATES_DIR: PathBuf = make_boilermaker_template_dir().unwrap();
}

#[derive(Debug)]
pub(crate) struct TemplateCommand {
    pub name: String,
    pub template: String,
    pub lang: Option<String>,
    pub branch: Option<String>,
    pub subdir: Option<String>,
    pub output_dir: Option<String>,
    pub overwrite: bool,
}

// TODO: see if it's possible to do a sparse checkout with git2
#[tracing::instrument]
pub fn make_template_root_dir(repo_root: &PathBuf, cmd: &TemplateCommand) -> PathBuf {
    match &cmd.subdir {
        Some(subdir) => repo_root.join(subdir),
        None => repo_root.to_owned(),
    }
}

// TODO: add local .cache dir that doesn't need to copy every time (maybe 10 minutes?)
#[tracing::instrument]
pub async fn clone_repo(src_root: &PathBuf, cmd: &TemplateCommand) -> Result<Repository> {
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

#[tracing::instrument]
pub fn get_lang(cmd: &TemplateCommand, cfg: &BoilermakerConfig) -> Result<String> {
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
pub async fn copy_files_to_target(
    template_files_path: &PathBuf,
    lang: &str,
    target_root: &PathBuf,
    target_dir: &PathBuf,
) -> Result<Vec<PathBuf>> {
    match fs::create_dir(&target_root) {
        Ok(_) => info!("Created target directory: {}", target_root.display()),
        Err(e) => return Err(eyre!("Failed to create target directory: {e}")),
    }

    match fs::create_dir(&target_dir) {
        Ok(_) => info!("Created target directory: {}", target_dir.display()),
        Err(e) => return Err(eyre!("Failed to create target directory: {e}")),
    }

    let files: Vec<PathBuf> = fs::read_dir(&template_files_path)?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .collect();

    info!("Copying template files for language '{}'...", lang);
    match copy_items_with_progress(
        &files,
        &target_dir,
        &fs_extra::dir::CopyOptions::new(),
        |progress| {
            info!(
                "\tCopied {} bytes to {}/{}",
                progress.copied_bytes, progress.dir_name, progress.file_name,
            );
            fs_extra::dir::TransitProcessResult::ContinueOrAbort
        },
    ) {
        Ok(_) => info!(
            "Copied template files to target directory: {}",
            target_root.display()
        ),
        Err(e) => return Err(eyre!("Failed to copy template files: {e}")),
    }

    let template_files: Vec<PathBuf> = WalkDir::new(target_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !e.file_type().is_dir())
        .map(|e| e.path().to_path_buf())
        .collect::<Vec<_>>();

    Ok(template_files)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TemplateContext {
    pub lang: String,
    pub repo_root: PathBuf,
    pub src_root: PathBuf,
    pub target_root: PathBuf,
    pub target_dir: PathBuf,
    pub output_dir: PathBuf,
    pub template_files: Vec<PathBuf>,
    pub vars: HashMap<String, String>,
    pub overwrite: bool,
}

#[tracing::instrument]
pub async fn get_template(
    _sys_config: &toml::Value,
    cmd: &TemplateCommand,
) -> Result<TemplateContext> {
    let output_dir = match &cmd.output_dir {
        Some(dir) => PathBuf::from(dir),
        None => env::current_dir()?.join(&cmd.name),
    };

    // TODO: add option to force overwrite existing output dir
    if output_dir.exists() && !cmd.overwrite {
        return Err(eyre!(
            "💥 Output dir path exists: {}. (Pass --overwrite to force.)",
            output_dir.display()
        ));
    }

    let repo_root = env::temp_dir().join(&cmd.name);
    let src_root = repo_root.join("src");

    if repo_root.exists() {
        fs::remove_dir_all(&repo_root)?;
    }

    let _repo = clone_repo(&src_root, cmd).await?;

    let template_root = make_template_root_dir(&src_root, cmd);
    let cfg_path = template_root.join("boilermaker.toml");
    let cfg: BoilermakerConfig = get_template_config(cfg_path.as_path())?;
    println!(
        "-------------------------------------- Using template config: {:?}",
        cfg
    );
    let lang = get_lang(&cmd, &cfg)?;
    let template_files_path = template_root.join(&lang);
    let target_root = repo_root.join("target");
    let target_dir = target_root.join(&lang);
    let template_files =
        copy_files_to_target(&template_files_path, &lang, &target_root, &target_dir).await?;

    let vars: HashMap<String, String> = match &cfg.boilermaker.variables {
        Some(m) => m.to_owned(),
        None => HashMap::new(),
    };

    Ok(TemplateContext {
        lang: lang.to_owned(),
        repo_root,
        src_root,
        target_root,
        target_dir,
        output_dir,
        template_files,
        vars,
        overwrite: cmd.overwrite,
    })
}

#[tracing::instrument]
pub async fn render_template_files(
    template_files: Vec<PathBuf>,
    ctx: &TemplateContext,
) -> Result<()> {
    let mut jinja = minijinja::Environment::new();

    for file_path in template_files {
        let name = file_path.file_name().unwrap().to_str().unwrap().to_string();
        let content = fs::read_to_string(&file_path)?;
        jinja.add_template_owned(name.clone(), content)?;

        let template = jinja.get_template(&name)?;
        let rendered = template.render(minijinja::context! { ..ctx.vars.to_owned() })?;

        fs::write(&file_path, rendered)?;
        info!("Rendered template file: {}", file_path.display());
    }

    Ok(())
}

#[tracing::instrument]
pub async fn move_to_output_dir(ctx: &TemplateContext) -> Result<()> {
    let output_dir = &ctx.output_dir;

    if output_dir.exists() {
        if ctx.overwrite {
            match fs::remove_dir_all(&output_dir) {
                Ok(_) => info!(
                    "Removed existing output directory: {}",
                    output_dir.display()
                ),
                Err(e) => return Err(eyre!("💥 Failed to remove existing output directory: {e}")),
            }
        } else {
            return Err(eyre!(
                "💥 Output dir path exists: {}. (Pass --overwrite to force.)",
                output_dir.display()
            ));
        }
    } else {
        match fs::create_dir_all(&output_dir) {
            Ok(_) => info!("Created output directory: {}", output_dir.display()),
            Err(e) => return Err(eyre!("💥 Failed to create output directory: {e}")),
        }
    }

    match fs::rename(&ctx.target_dir, &output_dir) {
        Ok(_) => info!(
            "Moved project to output directory: {}",
            output_dir.display()
        ),
        Err(e) => return Err(eyre!("💥 Failed to move project to output directory: {e}")),
    }

    Ok(())
}

#[tracing::instrument]
pub fn make_boilermaker_template_dir() -> Result<PathBuf> {
    let home_dir = dirs::home_dir().ok_or_else(|| eyre!("Can't find home directory"))?;
    let templates_dir = home_dir.join(".boilermaker").join("templates");

    if !templates_dir.exists() {
        fs::create_dir_all(&templates_dir)?;
        info!(
            "Created boilermaker templates directory: {}",
            templates_dir.display()
        );
    }

    Ok(templates_dir)
}
