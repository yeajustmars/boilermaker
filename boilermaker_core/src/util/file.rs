use std::{fs, path::PathBuf};

use color_eyre::{Result, eyre::eyre};
use fs_extra::{copy_items, dir::CopyOptions};
use walkdir::WalkDir;

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
