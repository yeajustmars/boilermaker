use std::{any::Any, collections::HashMap, path::PathBuf};

use clap::Parser;
use color_eyre::{Result, eyre::eyre};
use minijinja::value::{Value as JinjaValue, merge_maps};
use tabled::{Table, Tabled, settings::Style};
use tracing::{error, info};

use crate::db::{TemplateFindParams, TemplateResult};
use crate::state::AppState;
use crate::template as tpl;
//use crate::util::file::{copy_dir, list_dir, move_file};

type ContextHashMap = HashMap<String, Box<dyn Any>>;

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

    // Validate extra variables from CLI or app.
    let mut user_vars: ContextHashMap = HashMap::new();
    if !cmd.vars.is_empty() {
        let raw_user_vars = vec_to_hashmap(&cmd.vars)?;
        extend_template_context(&mut user_vars, &t.template_dir, raw_user_vars)?;
    }

    if let Some(tpl_vars) = tpl_config.variables {
        let a = JinjaValue::from_serialize(tpl_vars);
        let b = JinjaValue::from_serialize(user_vars);
        let x = merge_maps([a, b]);
    }

    // TODO: PICKUP HERE

    /*
    // Copy template to work-dir before rendering.
    let work_dir = tpl::create_work_dir_clean(&t.name)?;
    let template_base_dir = PathBuf::from(&t.template_dir);
    let template_dir = template_base_dir.join(&t.lang);
    copy_dir(&template_dir, &work_dir).await?;

    let template_paths: Vec<PathBuf> = list_dir(&work_dir)
        .await?
        .iter()
        .filter(|p| p.is_file())
        .map(|p| p.to_path_buf())
        .collect();
    if let Err(e) = tpl::render_template_files(template_paths, context).await {
        return Err(eyre!("💥 Failed to render template files: {e}"));
    }

    let out_dir = tpl::create_project_dir(project_name, cmd.dir.as_deref(), cmd.overwrite).await?;

    if let Err(e) = move_file(&work_dir, &out_dir).await {
        return Err(eyre!("💥 Failed to move project to output directory: {e}"));
    }

    info!("Project created at: {}", out_dir.display());
    info!("All set. Happy hacking! 🚀");
     */

    Ok(())
}

#[tracing::instrument]
async fn get_existing_templates(app_state: &AppState, cmd: &New) -> Result<Vec<TemplateResult>> {
    let find_params = TemplateFindParams {
        ids: None,
        name: Some(cmd.name.to_owned()),
        lang: cmd.lang.clone(),
        repo: None,
        branch: None,
        subdir: None,
        sha256_hash: None,
    };

    let cache = app_state.local_db.clone();

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

#[tracing::instrument]
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
#[tracing::instrument]
fn vec_to_hashmap(vec: &[String]) -> Result<HashMap<String, String>> {
    vec.iter()
        .map(|mapping| {
            mapping
                .split_once("=")
                .map(|(x, y)| (x.to_owned(), y.to_owned()))
                .ok_or(eyre!("💥 Invalid variable format: {mapping}"))
        })
        .collect()
}

#[tracing::instrument]
fn extend_template_context(
    template_context: &mut ContextHashMap,
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

    for (k, v) in user_vars {
        template_context.insert(k, Box::new(v));
    }

    Ok(())
}
