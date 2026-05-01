use std::{env, fs, path::PathBuf};

use color_eyre::{Result, eyre::eyre};
use fs_extra::{copy_items, dir::CopyOptions};
use walkdir::WalkDir;

// TODO: remove this function, it's redundant with fs::read_to_string and is only used in one place
#[tracing::instrument]
pub fn read_file_to_string(path: &PathBuf) -> Result<String> {
    let content = fs::read_to_string(path)?;
    Ok(content)
}

#[tracing::instrument]
pub fn remove_git_dir(dir: &PathBuf) -> Result<()> {
    let git_dir = dir.join(".git");
    if git_dir.exists() {
        fs::remove_dir_all(git_dir)?;
    }
    Ok(())
}

#[tracing::instrument]
pub async fn list_dir(path: &PathBuf) -> Result<Vec<PathBuf>> {
    let paths = WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.path().to_path_buf())
        .collect::<Vec<_>>();
    Ok(paths)
}

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

#[tracing::instrument]
pub async fn move_file(src: &PathBuf, dest: &PathBuf) -> Result<()> {
    if let Err(e) = fs::rename(src, dest) {
        return Err(eyre!("💥 Failed to move file: {e}"));
    }
    Ok(())
}

// TODO: replace all calls to .boilermaker dir with this function
#[tracing::instrument]
pub fn get_boilermaker_dir() -> Result<PathBuf> {
    let home_dir = dirs::home_dir().ok_or_else(|| eyre!("💥 Could not find home directory"))?;
    let boilermaker_dir = home_dir.join(".boilermaker");
    if !boilermaker_dir.exists() {
        return Err(eyre!(
            "💥 Could not find .boilermaker directory at {}",
            boilermaker_dir.display()
        ));
    }
    Ok(boilermaker_dir)
}

#[tracing::instrument]
pub fn get_docs_dir() -> Result<PathBuf> {
    let boilermaker_dir = get_boilermaker_dir()?;
    let docs_dir = boilermaker_dir.join("docs");
    if !docs_dir.exists() {
        return Err(eyre!(
            "💥 Could not find docs directory at {}",
            docs_dir.display()
        ));
    }
    Ok(docs_dir)
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
