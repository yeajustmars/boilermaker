use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use clap::Parser;
use color_eyre::{Result, eyre::eyre};
use minijinja::{
    context,
    value::{Value as JinjaValue, merge_maps},
};
use serde::Deserialize;
use tabled::{Table, Tabled, settings::Style};
use tracing::{error, info, warn};

use crate::db::{TemplateFindParams, TemplateResult};
use crate::state::AppState;
use crate::template as tpl;
//use crate::template::static_analysis as ana;
use crate::util::file::{copy_dir, move_file};

#[derive(Debug, Parser)]
pub struct New {
    #[arg(required = true)]
    pub id_or_name: String,
    #[arg(short, long)]
    pub lang: Option<String>,
    #[arg(short, long)]
    pub rename: Option<String>,
    #[arg(short, long)]
    pub dir: Option<String>,
    #[arg(short = 'p', long = "use-profile", value_name = "PROFILE")]
    pub use_profile: Option<String>,
    #[arg(short = 'v', long = "var", value_name = "KEY=VALUE")]
    pub vars: Vec<String>,
    #[arg(short = 'O', long, default_value_t = false)]
    pub overwrite: bool,
    #[arg(short = 'S', long = "strict-vars", default_value_t = false)]
    pub strict_vars: bool,
}

#[tracing::instrument]
async fn setup_template(app_state: &AppState, cmd: &New) -> Result<(TemplateResult, bool)> {
    match cmd.id_or_name.parse::<i64>() {
        Ok(id) => Ok((get_template_by_id(app_state, id).await?, true)),
        Err(_) => {
            let existing_templates = get_existing_templates(app_state, cmd).await?;

            match existing_templates.len() {
                0 => Err(eyre!("💥 Cannot find template: {}.", cmd.id_or_name))?,
                1 => Ok((
                    existing_templates
                        .first()
                        .unwrap_or(Err(eyre!(
                            "💥 Cannot retrieve template: {}.",
                            cmd.id_or_name
                        ))?)
                        .to_owned(),
                    false,
                )),
                2.. => {
                    print_multiple_template_results_help(&existing_templates);
                    Err(eyre!(
                        "💥 Found multiple results matching template: {}.",
                        cmd.id_or_name
                    ))?
                }
            }
        }
    }
}

#[tracing::instrument]
fn make_project_name(cmd: &New, t: &TemplateResult, by_id: bool) -> Result<String> {
    let project_name = if let Some(rename) = &cmd.rename {
        rename.to_string()
    } else if by_id {
        t.name.clone()
    } else {
        cmd.id_or_name.to_string()
    };

    Ok(project_name)
}

// Copies template from template_dir to temporary work_dir, renders it with context,
// and if nothing fails, moves it to final project_dir.
// TODO: refactor for readability (multiple functions?)
// TODO: add --strict-vars flag to fail on unknown vars
#[tracing::instrument]
pub async fn new(app_state: &AppState, cmd: &New) -> Result<()> {
    let (t, by_id) = setup_template(app_state, cmd).await?;
    let tpl_base_dir = PathBuf::from(&t.template_dir);
    let tpl_dir = tpl_base_dir.join(&t.lang);
    let tpl_config = tpl::get_template_config(&tpl_base_dir)?;

    let tmp_work_dir = tpl::create_work_dir_clean(t.sha256_hash.as_ref().unwrap())?;
    copy_dir(&tpl_dir, &tmp_work_dir).await?;

    let mut ctx = if tpl_config.variables.is_none() {
        context! {}
    } else {
        tpl_config.variables.unwrap()
    };
    if let Some(profile_name) = &cmd.use_profile {
        let Ok(profile_ctx) = ctx.get_attr("profiles") else {
            return Err(eyre!("Cannot find profiles key in template context"));
        };
        let Ok(profile_ctx) = profile_ctx.get_attr(profile_name) else {
            return Err(eyre!("Cannot find profile: {}", profile_name));
        };
        // TODO: discuss deep merge (not initially obvious in minijinja)
        ctx = merge_maps(vec![ctx, profile_ctx]);
    }
    if let Some(user_ctx) = cmdline_vars_to_hashmap(&cmd.vars)? {
        let from_paths = tpl::get_template_paths(&tpl_dir).await?;
        ctx = extend_template_context(vec![ctx, user_ctx], &from_paths, cmd)?;
    }

    if let Err(e) = tpl::render_template_files(&tmp_work_dir, ctx).await {
        return Err(eyre!("💥 Failed to render template files: {e}"));
    }

    let project_name = make_project_name(cmd, &t, by_id)?;

    let project_dir =
        tpl::create_project_dir(&project_name, cmd.dir.as_deref(), cmd.overwrite).await?;
    move_file(&tmp_work_dir, &project_dir).await?;

    info!("Project created at: {}", project_dir.display());
    info!("All set. Happy hacking! 🚀");

    Ok(())
}

#[tracing::instrument]
async fn get_template_by_id(app_state: &AppState, id: i64) -> Result<TemplateResult> {
    app_state
        .local_db
        .clone()
        .get_template(id)
        .await?
        .ok_or(eyre!("💥 Cannot find template with ID: {}.", id))
}

#[tracing::instrument]
async fn get_existing_templates(app_state: &AppState, cmd: &New) -> Result<Vec<TemplateResult>> {
    let find_params = TemplateFindParams {
        ids: None,
        name: Some(cmd.id_or_name.to_owned()),
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

/// Alias for HashSet<String>
type StringSet = HashSet<String>;

/// Allows user-provided vars.
/// Note: Only top-level vars are allowed to be passed (for now).
// TODO: add better error handling instead of propagating deserialization errors
// TODO: Disuss when/where to use static analysis to enforce var checks.
#[tracing::instrument]
fn extend_template_context(
    contexts: Vec<JinjaValue>,
    template_paths: &Vec<PathBuf>,
    cmd: &New,
) -> Result<JinjaValue> {
    let strict_mode = cmd.strict_vars;

    let config_map = deserialize_jinja_value_map(&contexts[0])?;
    let config_vars = config_map.keys().cloned().collect::<StringSet>();

    let mut user_vars: StringSet = HashSet::new();
    for ctx in &contexts[1..] {
        let m = deserialize_jinja_value_map(ctx)?;
        let vars = m.keys().cloned().collect::<StringSet>();
        user_vars.extend(vars);
    }

    let is_config_superset = config_vars.is_superset(&user_vars);

    let unknown_vars = user_vars
        .difference(&config_vars)
        .cloned()
        .collect::<StringSet>();

    if strict_mode {
        info!("🔒Strict vars mode enabled.");

        if !is_config_superset {
            return Err(eyre!(
                "💥The following vars must be declared in template's boilermaker.toml: {:?}",
                unknown_vars
            ));
        }

        // TODO: Figure out comparator for StringSet vs nested context to diff vars
        // NOTE: Convert all config_vars (in map) to flattened dot-delimited paths?
        /*
        let file_refs = ana::get_minijinja_vars(template_paths)?;
        println!(
            ">>> file_refs: {} {file_refs:#?}",
            std::any::type_name_of_val(&file_refs)
        );
         */
    } else if !is_config_superset {
        warn!(
            "Unknown user vars provided: {:?} (Set --strict-vars to enforce.)",
            unknown_vars
        );
    }

    Ok(merge_maps(contexts))
}

// TODO: add better erro handling instead of propagating deserialization errors
#[tracing::instrument]
fn deserialize_jinja_value_map(v: &JinjaValue) -> Result<HashMap<String, JinjaValue>> {
    Ok(HashMap::<String, JinjaValue>::deserialize(v)?)
}
