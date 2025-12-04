use std::collections::HashSet;
use std::fs::File;
use std::io::Read as _;
use std::path::Path;

use color_eyre::eyre::{Result, eyre};
use regex::Regex;
use walkdir::WalkDir;

// An alpha-numeric string enclosed in {{ }}.
const JINJA_VAR_REGEX: &str = r"\{\{\s*([\w_-]+)\s*\}\}";

// Find all template variables in files under `root`.
#[tracing::instrument]
pub fn find_variables_in_path(root: &str) -> Result<HashSet<String>> {
    let mut vars: HashSet<String> = HashSet::new();
    let re = Regex::new(JINJA_VAR_REGEX).unwrap();

    for entry in WalkDir::new(root) {
        match entry {
            Ok(entry) => {
                if entry.file_type().is_dir() {
                    continue;
                }

                let file_vars = find_vars_in_file(&re, entry.path())?;
                for name in file_vars {
                    vars.insert(name);
                }
            }
            Err(e) => return Err(eyre!("Error walking {}: {}", root, e)),
        }
    }
    Ok(vars)
}

fn find_vars_in_file(re: &Regex, path: &Path) -> Result<Vec<String>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;
    let vars: Vec<String> = re
        .captures_iter(&contents)
        .flat_map(|captures| {
            captures
                .iter()
                .skip(1) // Index 0 is the full match.
                .flatten()
                .map(|cap| cap.as_str().to_string())
                .collect::<Vec<_>>()
        })
        .collect();

    Ok(vars)
}
