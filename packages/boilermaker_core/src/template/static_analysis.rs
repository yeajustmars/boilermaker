use color_eyre::eyre::{Result, eyre};
use regex::Regex;
use std::collections::HashSet;
use std::fs::{File, read_to_string};
use std::io::Read as _;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

// An alpha-numeric string enclosed in {{ }}.
// TODO: re-strengthen this regex to whatever Jinja uses internally
const JINJA_VAR_REGEX: &str = r"\{\{\s*([\w_-]+)\s*\}\}";

// TODO: Is this deprecated after adding get_minijinja_vars?
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

// TODO: Is this deprecated after adding get_minijinja_vars?
#[tracing::instrument]
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

#[tracing::instrument]
pub fn get_minijinja_vars(paths: &Vec<PathBuf>) -> Result<HashSet<String>> {
    let mut vars: HashSet<String> = HashSet::new();

    for path in paths {
        let template_name = path.file_name().unwrap().to_str().unwrap();
        let template_source = read_to_string(path)?;

        let mut jinja = minijinja::Environment::new();
        jinja.add_template(template_name, &template_source)?;

        let template = jinja.get_template(template_name)?;
        vars.extend(template.undeclared_variables(true));
    }

    Ok(vars)
}
