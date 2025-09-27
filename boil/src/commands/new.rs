use std::{collections::HashMap, path::PathBuf};

use clap::Parser;
use color_eyre::{Result, eyre::eyre};
use tabled::{Table, Tabled, settings::Style};
use tracing::{error, info};

use crate::AppState;
use db::{TemplateFindParams, TemplateResult};
use template as tpl;

/*
use clap::Parser;
use color_eyre::{Result, eyre::eyre};
use toml;
use tracing::info;

use crate::template::{TemplateCommand, get_template, move_to_output_dir, render_template_files};

#[derive(Debug, Parser)]
pub(crate) struct New {
    #[arg(required = true)]
    pub name: String,
    #[arg(short, long)]
    pub template: String,
    #[arg(short, long)]
    pub lang: Option<String>,
    #[arg(short, long)]
    pub branch: Option<String>,
    #[arg(short = 'd', long)]
    pub subdir: Option<String>,
    #[arg(short, long = "output-dir")]
    pub output_dir: Option<String>,
    #[arg(short = 'O', long, default_value_t = false)]
    pub overwrite: bool,
}

impl From<&New> for TemplateCommand {
    #[tracing::instrument]
    fn from(cmd: &New) -> Self {
        Self {
            name: cmd.name.to_owned(),
            template: cmd.template.to_owned(),
            lang: cmd.lang.to_owned(),
            branch: cmd.branch.to_owned(),
            subdir: cmd.subdir.to_owned(),
            output_dir: cmd.output_dir.to_owned(),
            overwrite: cmd.overwrite,
        }
    }
}

/*
pub struct TemplateContext {
    pub lang: String,
    pub repo_root: PathBuf,
    pub src_root: PathBuf,
    pub target_root: PathBuf,
    pub target_dir: PathBuf,
    pub output_dir: PathBuf,
    pub template_files: Vec<PathBuf>,
    pub vars: HashMap<String, String>,
    pub overwrite: bool,
}
 */

#[tracing::instrument]
pub async fn new(sys_config: &toml::Value, cmd: &New) -> Result<()> {
    info!("Creating new project...");
    info!("Name: {}", cmd.name);
    info!("Template: {}", cmd.template);

    // 1. get name + lang
    // 2. check if template exists in local cache
    // 3. if not, clone template repo to local cache

    let cmd = TemplateCommand::from(cmd);

    // TODO: move cache and other global state to a passed state struct
    let local_cache_path = BOILERMAKER_LOCAL_CACHE_PATH.to_str().unwrap();
    let local_cache = LocalCache::new(local_cache_path).await?;

    let ctx = get_template(sys_config, &cmd).await?;

    if let Err(e) = render_template_files(ctx.template_files.clone(), &ctx).await {
        return Err(eyre!("💥 Failed to render template files: {e}"));
    }

    let _ = move_to_output_dir(&ctx).await?;

    info!("All set. Happy hacking! 🚀");
    Ok(())
}
 */

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
}

#[tracing::instrument]
pub async fn new(app_state: &AppState, cmd: &New) -> Result<()> {
    let project_name = if let Some(rename) = &cmd.rename {
        rename
    } else {
        &cmd.name
    };

    info!("Creating new project: {project_name}");

    let existing_templates = get_existing_templates(&app_state, &cmd).await?;
    match existing_templates.len() {
        0 => {
            return Err(eyre!("Cannot find template: {}.", cmd.name));
        }
        2.. => {
            print_multiple_template_results_help(&existing_templates);
            return Ok(());
        }
        _ => {}
    }

    let t = existing_templates.first().unwrap();

    let work_dir = tpl::create_work_dir_clean(&t.name)?;

    //TODO: clean up and refactor
    let template_base_dir = PathBuf::from(&t.template_dir);
    let template_dir = template_base_dir.join(&t.lang);
    let _ = tpl::copy_dir(&template_dir, &work_dir).await?;

    let template_paths: Vec<PathBuf> = tpl::list_dir(&work_dir)
        .await?
        .iter()
        .filter(|p| p.is_file())
        .map(|p| p.to_path_buf())
        .collect();

    let template_config = tpl::get_template_config(&template_base_dir)?;
    let template_context = if let Some(vars) = template_config.variables {
        vars.as_map().clone()
    } else {
        let ctx: HashMap<String, String> = HashMap::new();
        ctx
    };

    if let Err(e) = tpl::render_template_files(template_paths, template_context).await {
        return Err(eyre!("💥 Failed to render template files: {e}"));
    }

    let out_dir = tpl::get_or_create_project_dir(&project_name, cmd.dir.as_deref()).await?;

    if out_dir.exists() {
        if cmd.overwrite {
            tpl::clean_dir(&out_dir)?;
        } else {
            return Err(eyre!(
                "💥 Output directory already exists: {}. (Use --overwrite to force.)",
                out_dir.display()
            ));
        }
    }

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
    };

    let cache = app_state
        .template_db
        .read()
        .map_err(|e| eyre!("Failed to acquire template_db read lock: {e}"))?;

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
