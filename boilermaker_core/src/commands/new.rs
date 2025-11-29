use std::{collections::HashMap, path::PathBuf};

use clap::Parser;
use color_eyre::{Result, eyre::eyre};
use tabled::{Table, Tabled, settings::Style};
use tracing::{debug, error, info};

use crate::db::{TemplateFindParams, TemplateResult};
use crate::state::AppState;
use crate::template as tpl;

#[derive(Debug, Parser)]
pub struct New {
    #[arg(required = true)]
    pub name: String,
    #[arg(short, long)]
    pub lang: Option<String>,
    #[arg(short, long)]
    pub rename: Option<String>,
    #[arg(short, long)]
    pub dir: Option<String>,
    #[arg(short = 'P', long = "output-path")]
    pub output_path: Option<String>,
    #[arg(short = 'O', long, default_value_t = false)]
    pub overwrite: bool,
    #[arg(short = 'v', long = "var", value_name = "KEY=VALUE")]
    pub vars: Vec<String>,
}

#[tracing::instrument]
pub async fn new(app_state: &AppState, cmd: &New) -> Result<()> {
    let project_name = cmd.rename.as_deref().unwrap_or(&cmd.name);

    info!("Creating new project: {project_name}");

    let existing_templates = get_existing_templates(app_state, cmd).await?;
    match existing_templates.len() {
        0 => {
            return Err(eyre!("💥 Cannot find template: {}.", cmd.name));
        }
        2.. => {
            print_multiple_template_results_help(&existing_templates);
            return Ok(());
        }
        _ => {}
    }

    // Read template config. to get the default context & variables.
    let t = existing_templates.first().unwrap();
    let base_dir = PathBuf::from(&t.template_dir);
    let tpl_config = tpl::get_template_config(&base_dir)?;
    let mut context = tpl_config
        .variables
        .as_ref()
        .map(|vars| vars.as_map().clone())
        .unwrap_or_default();

    // Validate extra variables from CLI or app.
    if !cmd.vars.is_empty() {
        let user_vars = vec_to_hashmap(&cmd.vars)?;
        extend_template_context(&mut context, &t.template_dir, user_vars)?;
    }
    debug!("Template context: {:?}", context);

    // Copy template to work-dir before rendering.
    let template_dir = base_dir.join(&t.lang);
    let work_dir = tpl::create_work_dir_clean(&t.name)?;
    tpl::copy_dir(&template_dir, &work_dir).await?;
    let template_paths: Vec<PathBuf> = tpl::list_dir(&work_dir)
        .await?
        .iter()
        .filter(|p| p.is_file())
        .map(|p| p.to_path_buf())
        .collect();
    if let Err(e) = tpl::render_template_files(template_paths, context).await {
        return Err(eyre!("💥 Failed to render template files: {e}"));
    }

    let out_dir = tpl::create_project_dir(project_name, cmd.dir.as_deref(), cmd.overwrite).await?;

    if let Err(e) = tpl::move_file(&work_dir, &out_dir).await {
        return Err(eyre!("💥 Failed to move project to output directory: {e}"));
    }

    info!("Project created at: {}", out_dir.display());
    info!("All set. Happy hacking! 🚀");

    Ok(())
}

async fn get_existing_templates(app_state: &AppState, cmd: &New) -> Result<Vec<TemplateResult>> {
    let find_params = TemplateFindParams {
        name: Some(cmd.name.to_owned()),
        lang: cmd.lang.clone(),
        repo: None,
        branch: None,
        subdir: None,
        sha256_hash: None,
    };

    let cache = app_state.template_db.clone();

    let existing_templates = { cache.find_templates(find_params).await? };

    Ok(existing_templates)
}

#[derive(Tabled)]
struct MultipleResultsRow {
    #[tabled(rename = "Template")]
    template: String,
    #[tabled(rename = "Lang")]
    lang: String,
}

fn print_multiple_template_results_help(template_rows: &Vec<TemplateResult>) {
    let help_line = "Multiple templates found. (You need to provide --lang)";
    let mut help_rows = Vec::new();
    for t in template_rows {
        help_rows.push(MultipleResultsRow {
            template: t.name.clone(),
            lang: t.lang.clone(),
        });
    }

    let mut table = Table::new(&help_rows);
    table.with(Style::psql());
    error!("{}\n\n{table}\n", help_line);
}

// Turn a vec like ["foo=bar", "baz=quux"] into a HashMap
fn vec_to_hashmap(vec: &Vec<String>) -> Result<HashMap<String, String>> {
    let mut hm: HashMap<String, String> = HashMap::new();

    for var in vec {
        match var.split_once('=') {
            Some((key, value)) => {
                hm.insert(key.to_string(), value.to_string());
            }
            None => {
                return Err(eyre!("💥 Invalid variable format: {var}"));
            }
        }
    }
    Ok(hm)
}

fn extend_template_context(
    template_context: &mut HashMap<String, String>,
    template_dir: &str,
    user_vars: HashMap<String, String>,
) -> Result<()> {
    let allowed_vars = tpl::static_analysis::find_variables_in_path(template_dir)?;
    let bad_vars: Vec<_> = user_vars
        .keys()
        .filter(|var| !allowed_vars.contains(*var))
        .map(|s| s.as_str())
        .collect();

    if !bad_vars.is_empty() {
        return Err(eyre!(
            "💥 Some variables aren't available in template: {}.\nKnown variables: {:?}",
            bad_vars.join(", "),
            allowed_vars,
        ));
    }
    template_context.extend(user_vars);

    return Ok(());
}
