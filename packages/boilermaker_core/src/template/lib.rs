use std::{env, fs, path::PathBuf};

use auth_git2::GitAuthenticator;
use color_eyre::{Result, eyre::eyre};
use dirs;
use fs_extra::dir::{CopyOptions, copy};
use git2::{Config, FetchOptions, RemoteCallbacks, Repository, build::RepoBuilder};
use minijinja::{Environment as JinjaEnv, value::Value as JinjaValue};
use tracing::info;
use walkdir::WalkDir;

use crate::config::TemplateConfig;
pub use crate::config::get_template_config;
use crate::constants::TEMPLATE_FILEPATH_VAR_PATTERN as FILEPATH_VARS;
use crate::util::file::{list_dir, move_file};

#[derive(Debug)]
pub struct CloneContext {
    pub url: String,
    pub dest: Option<PathBuf>,
    pub branch: Option<String>,
}

impl CloneContext {
    pub fn new(url: &str, dest: Option<PathBuf>, branch: Option<String>) -> Self {
        CloneContext {
            url: url.to_owned(),
            branch,
            dest,
        }
    }
}

// TODO: add optional depth parameter in CloneContext
// TODO: check if repo exists locally, and if so, just update it
#[tracing::instrument]
pub async fn clone_repo(ctx: &CloneContext) -> Result<Repository> {
    let auth = GitAuthenticator::default();
    let git_config = Config::open_default()?;
    let mut repo_builder = RepoBuilder::new();
    let mut fetch_opts = FetchOptions::new();
    let mut remote_callbacks = RemoteCallbacks::new();

    remote_callbacks.credentials(auth.credentials(&git_config));
    fetch_opts.remote_callbacks(remote_callbacks);
    fetch_opts.depth(1);
    repo_builder.fetch_options(fetch_opts);

    if let Some(branch) = &ctx.branch {
        repo_builder.branch(branch);
    }

    let dir = match &ctx.dest {
        Some(d) => d.into(),
        None => env::temp_dir(),
    };

    let repo = repo_builder.clone(&ctx.url, &dir);
    if let Err(e) = repo {
        if e.message().contains("404") {
            return Err(eyre!(
                "💥 Repository not found (404): {}: Check the URL and your access rights.",
                ctx.url
            ));
        }
        return Err(eyre!("💥 Failed to clone repository: {}", e));
    }
    Ok(repo?)
}

#[tracing::instrument]
pub fn make_name_from_url(url: &str) -> String {
    url.split('/')
        .next_back()
        .unwrap()
        .split('.')
        .next()
        .unwrap()
        .to_string()
}

#[tracing::instrument]
pub fn make_tmp_dir_from_url(url: &str) -> PathBuf {
    env::temp_dir().join(make_name_from_url(url))
}

#[tracing::instrument]
pub fn get_lang(tpl_cnf: &TemplateConfig, option: &Option<String>) -> Result<String> {
    if let Some(lang_option) = option {
        return Ok(lang_option.clone());
    }

    if let Some(default_lang) = &tpl_cnf.project.default_lang {
        return Ok(default_lang.clone());
    }

    Err(eyre!(
        "💥 Can't find language. Pass `--lang` option or add `default_lang` to `boilermaker.toml`."
    ))
}

#[tracing::instrument]
pub fn dir_exists(dir: &PathBuf) -> bool {
    dir.as_path().exists()
}

#[tracing::instrument]
pub fn remove_dir_if_exists(dir: &PathBuf) -> Result<()> {
    if dir.as_path().exists() {
        fs::remove_dir_all(dir)?;
    }
    Ok(())
}

#[tracing::instrument]
pub fn clean_dir(dir: &PathBuf) -> Result<()> {
    remove_dir_if_exists(dir)?;
    Ok(())
}

#[tracing::instrument]
pub fn make_work_dir_path(name: &str) -> Result<PathBuf> {
    let work_dir = env::temp_dir().join("boilermaker").join(name);
    Ok(work_dir)
}

#[tracing::instrument]
pub fn create_work_dir(name: &str) -> Result<PathBuf> {
    let work_dir = make_work_dir_path(name)?;
    if !work_dir.exists() {
        fs::create_dir_all(&work_dir)?;
    }
    Ok(work_dir)
}

#[tracing::instrument]
pub fn create_work_dir_clean(name: &str) -> Result<PathBuf> {
    let work_dir = make_work_dir_path(name)?;
    if work_dir.exists() {
        fs::remove_dir_all(&work_dir)?;
    }
    fs::create_dir_all(&work_dir)?;
    Ok(work_dir)
}

#[tracing::instrument]
pub fn get_template_dir_path(name: &str) -> Result<PathBuf> {
    let home_dir = dirs::home_dir().ok_or_else(|| eyre!("💥 Can't find home directory"))?;
    let templates_dir = home_dir.join(".boilermaker").join("templates").join(name);
    Ok(templates_dir)
}

#[tracing::instrument]
pub fn create_template_dir(name: &str) -> Result<PathBuf> {
    let template_dir = get_template_dir_path(name)?;
    if !template_dir.exists() {
        fs::create_dir_all(&template_dir)?;
    }
    Ok(template_dir)
}

#[tracing::instrument]
pub async fn install_template(src_path: &PathBuf, dest_path: &PathBuf) -> Result<()> {
    if dest_path.exists() {
        return Err(eyre!(
            "💥 Template dir path exists: {}",
            dest_path.display()
        ));
    }

    if let Err(e) = fs::create_dir_all(dest_path) {
        return Err(eyre!("💥 Failed to create template directory: {e}"));
    }

    let mut options = CopyOptions::new();
    options.content_only = true;

    let src = src_path
        .clone()
        .into_os_string()
        .into_string()
        .map_err(|e| eyre!("💥 Invalid source path: {:?}", e))?;

    let dest = dest_path
        .clone()
        .into_os_string()
        .into_string()
        .map_err(|e| eyre!("💥 Invalid destination path: {:?}", e))?;

    if let Err(e) = copy(src, dest, &options) {
        return Err(eyre!(
            "💥 Failed to move project to template directory: {e}"
        ));
    }

    // TODO: discuss keeping the tmp dir after install for any purpose, otherwise burn it

    Ok(())
}

#[tracing::instrument]
pub async fn create_project_dir(
    project_name: &str,
    dir: Option<&str>,
    overwrite: bool,
) -> Result<PathBuf> {
    let project_dir = if let Some(dir) = dir {
        PathBuf::from(dir).join(project_name)
    } else {
        env::current_dir()?.join(project_name)
    };

    if project_dir.exists() {
        if overwrite {
            clean_dir(&project_dir)?;
        } else {
            return Err(eyre!(
                "💥 Project directory already exists: {}. (Use --overwrite to force.)",
                project_dir.display()
            ));
        }
    }

    if let Err(e) = fs::create_dir_all(&project_dir) {
        return Err(eyre!("💥 Failed to create project directory: {e}"));
    }

    if !project_dir.is_dir() {
        return Err(eyre!(
            "💥 Project path is not a directory: {}",
            project_dir.display()
        ));
    }

    Ok(project_dir)
}

//TODO: add setting to warn from sys_config on directory in paths vec
//NOTE: for now, just skip
#[tracing::instrument]
pub async fn render_template_files(dir: &PathBuf, ctx: JinjaValue) -> Result<()> {
    info!("Rendering template content...");

    let mut jinja = minijinja::Environment::new();

    for path in get_template_paths(dir).await? {
        if path.is_file() {
            let name = path.file_name().unwrap().to_str().unwrap().to_string();
            let content = fs::read_to_string(&path)?;
            jinja.add_template_owned(name.clone(), content)?;

            let template = jinja.get_template(&name)?;
            let rendered = template.render(&ctx)?;

            fs::write(&path, rendered)?;
        }
    }

    info!("Checking for vars in file paths...");
    interpolate_template_filepaths(dir, &ctx).await?;

    Ok(())
}

#[tracing::instrument]
pub async fn list_template_files(dir: &PathBuf) -> Result<Vec<PathBuf>> {
    let files = list_dir(dir)
        .await?
        .into_iter()
        .filter(|p| p.is_file() && !p.to_str().unwrap_or("").contains(".git"))
        .collect::<Vec<_>>();
    Ok(files)
}

#[tracing::instrument]
pub async fn get_template_paths(template_dir: &PathBuf) -> Result<Vec<PathBuf>> {
    let paths: Vec<PathBuf> = list_dir(template_dir)
        .await?
        .iter()
        .filter(|p| p.is_file())
        .map(|p| p.to_path_buf())
        .collect();
    Ok(paths)
}

#[tracing::instrument]
pub async fn interpolate_template_filepaths(
    template_dir: &PathBuf,
    ctx: &JinjaValue,
) -> Result<()> {
    let mut env = JinjaEnv::new();

    for entry in WalkDir::new(template_dir).contents_first(true) {
        let entry = entry.unwrap();
        let path = entry.path().to_path_buf();
        let file_name = path.file_name().unwrap().to_str().unwrap().to_owned();
        let mut caps_iter = FILEPATH_VARS.captures_iter(&file_name).peekable();

        if caps_iter.peek().is_none() {
            continue;
        }

        // TODO: double-check all vars are interpolated into filename
        let mut new_path: PathBuf = path.clone();
        for cap in caps_iter {
            let var_path_str = if let Some(underscore) = cap.name("underscore") {
                underscore.as_str()
            } else if let Some(dash) = cap.name("dash") {
                dash.as_str()
            } else {
                continue;
            };
            let target = var_path_str;
            let var_path_str = var_path_str.trim_matches(['-', '_']);

            let s = format!("{{{{{}}}}}", var_path_str);
            let template = match env.get_template(&s) {
                Ok(t) => t,
                _ => {
                    env.add_template_owned(s.clone(), s.clone())?;
                    env.get_template(&s)?
                }
            };

            let var_value = template.render(ctx)?;
            let new_file_name = file_name.replace(target, &var_value);
            new_path = new_path.with_file_name(new_file_name);
        }

        move_file(&path, &new_path).await?;
    }

    Ok(())
}

/// Render a single variable using minijinja.
///
/// Note: this function create a new Jinja Environment each time it's called.
/// If you're doing anything serious, use minimjinja directly, and set up an
/// environment once and add templates to it.
///
/// # Example
///
/// ```rust
/// use minijinja::{context, Environment as JinjaEnv};
///
/// use boilermaker::tpl::render_var;
///
/// let ctx = context! { a => context! { b => "Hello, World!" } };
/// let rendered = render_var("a.b", &ctx).unwrap();
/// assert_eq!(rendered, "Hello, World!");
/// ```
// TODO: make a global JinjaEnv to avoid recreating it each time
#[tracing::instrument]
pub fn render_var(path: &str, ctx: &JinjaValue) -> Result<String> {
    Ok(JinjaEnv::new().render_str(&format!("{{{{ {} }}}}", path), ctx)?)
}
