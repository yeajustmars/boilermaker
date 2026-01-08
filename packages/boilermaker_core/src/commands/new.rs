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
//use crate::template::static_analysis as analyzer;
use crate::util::file::{copy_dir, move_file};

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
    #[arg(short = 'p', long = "use-profile", value_name = "PROFILE")]
    pub use_profile: Option<String>,
    #[arg(short = 'v', long = "var", value_name = "KEY=VALUE")]
    pub vars: Vec<String>,
    #[arg(short = 'O', long, default_value_t = false)]
    pub overwrite: bool,
}

#[tracing::instrument]
async fn setup_template(app_state: &AppState, cmd: &New) -> Result<(TemplateResult, bool)> {
    match cmd.name.parse::<i64>() {
        Ok(id) => Ok((get_template_by_id(app_state, id).await?, true)),
        Err(_) => {
            let existing_templates = get_existing_templates(app_state, cmd).await?;

            match existing_templates.len() {
                0 => Err(eyre!("💥 Cannot find template: {}.", cmd.name))?,
                1 => Ok((
                    existing_templates
                        .first()
                        .unwrap_or(Err(eyre!("💥 Cannot retrieve template: {}.", cmd.name))?)
                        .to_owned(),
                    false,
                )),
                2.. => {
                    print_multiple_template_results_help(&existing_templates);
                    Err(eyre!(
                        "💥 Found multiple results matching template: {}.",
                        cmd.name
                    ))?
                }
            }
        }
    }
}

#[tracing::instrument]
fn make_project_name(cmd: &New, t: &TemplateResult, by_id: bool) -> Result<String> {
    let project_name = if by_id {
        t.name.as_str()
    } else {
        cmd.rename.as_deref().unwrap_or(&cmd.name)
    };

    info!("Creating new project: {project_name}");

    Ok(project_name.to_string())
}

// TODO: refactor for readability (multiple functions?)
#[tracing::instrument]
pub async fn new(app_state: &AppState, cmd: &New) -> Result<()> {
    let (t, by_id) = setup_template(app_state, cmd).await?;
    let project_name = make_project_name(cmd, &t, by_id)?;
    let base_dir = PathBuf::from(&t.template_dir);
    let work_dir = tpl::create_work_dir_clean(&t.name)?;
    let template_base_dir = PathBuf::from(&t.template_dir);
    let template_dir = template_base_dir.join(&t.lang);
    let template_paths = tpl::get_template_paths(&template_dir).await?;
    let tpl_config = tpl::get_template_config(&base_dir)?;

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
        ctx = extend_template_context(vec![ctx, user_ctx], &template_paths)?;
    }

    copy_dir(&template_dir, &work_dir).await?;
    let new_template_paths = tpl::get_template_paths(&template_dir).await?;

    if let Err(e) = tpl::render_template_files(&work_dir, new_template_paths, ctx).await {
        return Err(eyre!("💥 Failed to render template files: {e}"));
    }

    let out_dir = tpl::create_project_dir(&project_name, cmd.dir.as_deref(), cmd.overwrite).await?;
    if let Err(e) = move_file(&work_dir, &out_dir).await {
        return Err(eyre!("💥 Failed to move project to output directory: {e}"));
    }

    info!("Project created at: {}", out_dir.display());
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

// TODO: finish validating vars
#[tracing::instrument]
fn extend_template_context(
    contexts: Vec<JinjaValue>,
    template_paths: &Vec<PathBuf>,
) -> Result<JinjaValue> {
    /*
    println!("&contexts[0] = {:#?}", &contexts[0]);
    let aaa = contexts.first().unwrap();
    println!("aaa = {:#?}", aaa);
    let bbb: HashMap<String, JinjaValue> = aaa.downcast_object_ref();
    println!("bbb = {:#?}", bbb);

    let config_vars = contexts
        .first()
        .unwrap()
        .downcast_object_ref::<HashMap<String, JinjaValue>>();
    .unwrap()
    .keys()
    .map(|s| s.to_string())
    .collect::<HashSet<String>>()
     */
    //println!("config_vars = {:#?}", config_vars);

    /*
    let file_vars = analyzer::get_minijinja_vars(template_paths)?;

    let allowed_vars = config_vars
        .union(&file_vars)
        .cloned()
        .collect::<HashSet<String>>();

    let user_contexts = &contexts[1..];

    let mut bad_vars: HashSet<String> = HashSet::new();
    for ctx in user_contexts {
        let map = ctx
            .downcast_object_ref::<HashMap<String, JinjaValue>>()
            .unwrap();
        for key in map.keys() {
            if !allowed_vars.contains(key) {
                bad_vars.insert(key.to_owned());
            }
        }
    }

    if !bad_vars.is_empty() {
        return Err(eyre!(
            "💥 Some variables aren't available in template: {:#?}.\nKnown variables: {:#?}",
            bad_vars,
            allowed_vars
        ));
    }
         */

    Ok(merge_maps(contexts))
}
