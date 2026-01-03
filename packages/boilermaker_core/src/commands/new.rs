use std::{collections::HashMap, path::PathBuf};

use clap::Parser;
use color_eyre::{Result, eyre::eyre};
use minijinja::{
    context,
    value::{Value as JinjaValue, merge_maps},
};
use tabled::{Table, Tabled, settings::Style};
use tracing::{error, info};

use crate::db::{TemplateFindParams, TemplateResult};
use crate::state::AppState;
use crate::template as tpl;
use crate::util::file::{copy_dir, list_dir, move_file};

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
    #[arg(short = 'D', long, default_value_t = false)]
    pub debug: bool,
}

#[tracing::instrument]
pub async fn new(app_state: &AppState, cmd: &New) -> Result<()> {
    let project_name = cmd.rename.as_deref().unwrap_or(&cmd.name);

    info!("Creating new project: {project_name}");

    // Validate there's only one template for arguments.
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

    if cmd.debug {
        info!("existing_templates: {existing_templates:#?}");
    }

    // Read template config. to get the default context & variables.
    let t = existing_templates.first().unwrap();
    let base_dir = PathBuf::from(&t.template_dir);
    let tpl_config = tpl::get_template_config(&base_dir)?;

    if cmd.debug {
        info!("t: {t:#?}");
        info!("base_dir: {base_dir:#?}");
        info!("tpl_config: {tpl_config:#?}");
    }

    // Build template context
    let template_context = if tpl_config.variables.is_none() {
        context! {}
    } else {
        let mut template_context = tpl_config.variables.unwrap();
        let user_context = cmdline_vars_to_hashmap(&cmd.vars)?;
        if let Some(user_context) = user_context {
            template_context =
                extend_template_context(vec![template_context, user_context], &t.template_dir)?;
        }
        template_context
    };

    if cmd.debug {
        info!("template_context: {template_context:#?}");
    }

    // Copy template to work-dir before rendering.
    let work_dir = tpl::create_work_dir_clean(&t.name)?;
    let template_base_dir = PathBuf::from(&t.template_dir);
    let template_dir = template_base_dir.join(&t.lang);

    if cmd.debug {
        info!("work_dir: {work_dir:#?}");
        info!("template_base_dir: {template_base_dir:#?}");
        info!("template_dir: {template_dir:#?}");
    }

    copy_dir(&template_dir, &work_dir).await?;

    let template_paths: Vec<PathBuf> = list_dir(&work_dir)
        .await?
        .iter()
        .filter(|p| p.is_file())
        .map(|p| p.to_path_buf())
        .collect();

    if cmd.debug {
        info!("template_paths: {template_paths:#?}");
    }

    if let Err(e) = tpl::render_template_files(template_paths, template_context).await {
        return Err(eyre!("💥 Failed to render template files: {e}"));
    }

    let out_dir = tpl::create_project_dir(project_name, cmd.dir.as_deref(), cmd.overwrite).await?;

    if cmd.debug {
        info!("out_dir: {out_dir:#?}");
    }

    if let Err(e) = move_file(&work_dir, &out_dir).await {
        return Err(eyre!("💥 Failed to move project to output directory: {e}"));
    }

    info!("Project created at: {}", out_dir.display());
    info!("All set. Happy hacking! 🚀");

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

/// Turn a vec like ["foo=bar", "baz=quux"] into a `HashMap<String, String>`.
/// Note: aggregate types are not (yet) supported.
#[tracing::instrument]
fn cmdline_vars_to_hashmap(vars_vec: &[String]) -> Result<Option<JinjaValue>> {
    let vars_map: Result<HashMap<String, String>> = vars_vec
        .iter()
        .map(|mapping| {
            mapping
                .split_once("=")
                .map(|(x, y)| (x.to_owned(), y.to_owned()))
                .ok_or(eyre!("💥 Invalid variable format: {mapping}"))
        })
        .collect();

    match vars_map {
        Err(e) => Err(eyre!("Failed to parse command line vars: {e}")),
        Ok(vars_map) => {
            if vars_map.is_empty() {
                Ok(None)
            } else {
                let context = JinjaValue::from_serialize(vars_map);
                Ok(Some(context))
            }
        }
    }
}

#[tracing::instrument]
fn extend_template_context(contexts: Vec<JinjaValue>, template_dir: &str) -> Result<JinjaValue> {
    Ok(merge_maps(contexts))
    /*
    //println!("template_context: {template_context:#?}");
    //println!("user_context: {user_context:#?}");
    let allowed_vars = tpl::static_analysis::find_variables_in_path(template_dir)?;
    // println!("allowed_vars: {allowed_vars:#?}");
    let bad_vars: Vec<_> = user_context
        .keys()
        .filter(|var| !allowed_vars.contains(*var))
        .map(|s| s.as_str())
        .collect();
    //println!("bad_vars: {bad_vars:#?}");

    if !bad_vars.is_empty() {
        return Err(eyre!(
            "💥 Some variables aren't available in template: {}.\nKnown variables: {:?}",
            bad_vars.join(", "),
            allowed_vars,
        ));
    }

    for (k, v) in user_context {
        template_context.insert(k, Box::new(v));
    }

    Ok(())
     */
}
