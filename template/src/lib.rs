use std::{collections::HashMap, env, fs, path::PathBuf};

use color_eyre::{Result, eyre::eyre};
use dirs;
use fs_extra::{copy_items, dir::CopyOptions};
use git2::{FetchOptions, Repository, build::RepoBuilder};
use minijinja;
use walkdir::WalkDir;

use config::TemplateConfig;
pub use config::get_template_config;

#[derive(Debug)]
pub struct CloneContext {
    pub url: String,
    pub dest: Option<PathBuf>,
    pub branch: Option<String>,
}

#[tracing::instrument]
pub async fn clone_repo(ctx: &CloneContext) -> Result<Repository> {
    let mut fetch_opts = FetchOptions::new();
    fetch_opts.depth(1);

    let mut repo_builder = RepoBuilder::new();
    repo_builder.fetch_options(fetch_opts);

    if let Some(branch) = &ctx.branch {
        repo_builder.branch(branch);
    }

    let dir = match &ctx.dest {
        Some(d) => d.into(),
        None => env::temp_dir(),
    };

    let repo = repo_builder.clone(&ctx.url, &dir)?;

    Ok(repo)
}

#[tracing::instrument]
pub fn make_name_from_url(url: &str) -> String {
    url.split('/')
        .last()
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

    return Err(eyre!(
        "💥 Can't find language. Pass `--lang` option or add `default_lang` to `boilermaker.toml`."
    ));
}

#[tracing::instrument]
pub fn clean_dir(work_dir: &PathBuf) -> Result<()> {
    if work_dir.as_path().exists() {
        fs::remove_dir_all(work_dir)?;
    }
    Ok(())
}

//TODO: move to a more generic loc like util::file
#[tracing::instrument]
pub fn clean_dir_if_overwrite(work_dir: &PathBuf, overwrite: bool) -> Result<()> {
    if overwrite {
        clean_dir(work_dir)?;
    }
    Ok(())
}

//TODO: move to a more generic loc like util::file
#[tracing::instrument]
pub fn remove_git_dir(work_dir: &PathBuf) -> Result<()> {
    let git_dir = work_dir.join(".git");
    if git_dir.exists() {
        fs::remove_dir_all(git_dir)?;
    }
    Ok(())
}

#[tracing::instrument]
pub fn make_work_dir_path(name: &str) -> Result<PathBuf> {
    let work_dir = env::temp_dir().join("boilermaker").join(name);
    Ok(work_dir)
}

#[tracing::instrument]
pub fn create_work_dir(name: &str) -> Result<PathBuf> {
    let work_dir = make_work_dir_path(&name)?;
    if !work_dir.exists() {
        fs::create_dir_all(&work_dir)?;
    }
    Ok(work_dir)
}

#[tracing::instrument]
pub fn create_work_dir_clean(name: &str) -> Result<PathBuf> {
    let work_dir = make_work_dir_path(&name)?;
    if work_dir.exists() {
        fs::remove_dir_all(&work_dir)?;
    }
    fs::create_dir_all(&work_dir)?;
    Ok(work_dir)
}

#[tracing::instrument]
fn make_template_dir_path(name: &str) -> Result<PathBuf> {
    let home_dir = dirs::home_dir().ok_or_else(|| eyre!("💥 Can't find home directory"))?;
    let templates_dir = home_dir.join(".boilermaker").join("templates").join(name);
    Ok(templates_dir)
}

#[tracing::instrument]
pub fn create_template_dir(name: &str) -> Result<PathBuf> {
    let template_dir = make_template_dir_path(name)?;
    if !template_dir.exists() {
        fs::create_dir_all(&template_dir)?;
    }
    Ok(template_dir)
}

#[tracing::instrument]
pub fn get_template_dir(name: &str) -> Result<PathBuf> {
    let template_dir = make_template_dir_path(name)?;
    if !template_dir.exists() {
        Err(eyre!("💥 Cannot find template directory for {name}"))
    } else {
        Ok(template_dir)
    }
}

#[tracing::instrument]
pub async fn install_template(
    src_path: &PathBuf,
    dest_path: &PathBuf,
    overwrite: bool,
) -> Result<()> {
    if dest_path.exists() {
        if overwrite {
            if let Err(e) = fs::remove_dir_all(&dest_path) {
                return Err(eyre!("💥 Failed to remove existing output directory: {e}"));
            }
        } else {
            return Err(eyre!(
                "💥 Output dir path exists: {}. (Pass --overwrite to force.)",
                dest_path.display()
            ));
        }
    } else {
        if let Err(e) = fs::create_dir_all(&dest_path) {
            return Err(eyre!("💥 Failed to create output directory: {e}"));
        }
    }

    if let Err(e) = fs::rename(&src_path, &dest_path) {
        return Err(eyre!("💥 Failed to move project to output directory: {e}"));
    }

    Ok(())
}

#[tracing::instrument]
pub async fn get_or_create_project_dir(project_name: &str, dir: Option<&str>) -> Result<PathBuf> {
    let project_dir = if let Some(dir) = dir {
        PathBuf::from(dir).join(project_name)
    } else {
        env::current_dir()?.join(project_name)
    };

    if !project_dir.exists() {
        if let Err(e) = fs::create_dir_all(&project_dir) {
            return Err(eyre!("💥 Failed to create project directory: {e}"));
        }
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
pub async fn render_template_files(
    paths: Vec<PathBuf>,
    ctx: HashMap<String, String>,
) -> Result<()> {
    let mut jinja = minijinja::Environment::new();
    let ctx = minijinja::context! { ..ctx.to_owned() };

    for path in paths {
        if path.is_file() {
            let name = path.file_name().unwrap().to_str().unwrap().to_string();
            let content = fs::read_to_string(&path)?;
            jinja.add_template_owned(name.clone(), content)?;

            let template = jinja.get_template(&name)?;
            let rendered = template.render(&ctx)?;

            fs::write(&path, rendered)?;
        }
    }

    Ok(())
}

//TODO: move to a more generic loc like util::file
#[tracing::instrument]
pub async fn list_dir(path: &PathBuf) -> Result<Vec<PathBuf>> {
    let paths = WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.path().to_path_buf())
        .collect::<Vec<_>>();
    Ok(paths)
}

//TODO: move to a more generic loc like util::file
#[tracing::instrument]
pub async fn copy_dir(src_dir: &PathBuf, dest_dir: &PathBuf) -> Result<()> {
    let files = fs::read_dir(src_dir)?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .collect::<Vec<_>>();

    let options = CopyOptions::new();

    if let Err(e) = copy_items(&files, dest_dir, &options) {
        return Err(eyre!("💥 Failed to copy template files: {e}"));
    }

    Ok(())
}

//TODO: move to a more generic loc like util::file
#[tracing::instrument]
pub async fn move_file(src: &PathBuf, dest: &PathBuf) -> Result<()> {
    if let Err(e) = fs::rename(src, dest) {
        return Err(eyre!("💥 Failed to move file: {e}"));
    }
    Ok(())
}
