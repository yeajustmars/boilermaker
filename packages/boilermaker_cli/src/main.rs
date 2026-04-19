use std::path::PathBuf;
use std::sync::Arc;

use clap::{CommandFactory, Parser, Subcommand};
use color_eyre::eyre::Result;
use tracing::info;

use boilermaker_core::{
    commands,
    commands::{
        Completion, Config, Docs, Sources, completion, config, docs, sources,
        sources::templates::Templates as SourceTemplates,
    },
    config::{get_system_config, get_system_config_path},
    db::{IndexDocsOptions, LocalDb},
    logging,
    state::AppState,
    util::env::is_dev_env,
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(
        short,
        long,
        value_name = "FILE",
        help = "Path to config file (default: ~/.config/boilermaker/boilermaker.toml)"
    )]
    config: Option<PathBuf>,

    #[arg(
        short = 'D',
        long,
        action = clap::ArgAction::Count,
        help = "Turn on debug logging (use -D[DDD] for more verbosity)"
    )]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(subcommand, about = "Get and set system config")]
    Config(commands::Config),
    #[command(subcommand, about = "Documentation")]
    Docs(commands::Docs),
    #[command(subcommand, about = "Manage CLI completion")]
    Completion(commands::Completion),
    #[command(about = "Install a template locally")]
    Install(commands::Install),
    #[command(about = "List all templates in the local DB")]
    List(commands::List),
    #[command(about = "Create a new project from a template")]
    New(commands::New),
    #[command(name = "rm", about = "Remove templates or local DB itself")]
    Remove(commands::Remove),
    #[command(about = "Search for templates")]
    Search(commands::Search),
    #[command(about = "Show template details")]
    Show(commands::Show),
    #[command(subcommand, about = "Manage Sources")]
    Sources(commands::Sources),
    #[command(about = "Update an installed template")]
    Update(commands::Update),
}

#[tokio::main]
#[tracing::instrument]
async fn main() -> Result<()> {
    color_eyre::install().expect("Failed to set up error handling");

    let cli = Cli::parse();

    logging::init_tracing(cli.debug)?;

    // TODO: decide where a remote db should be allowed vs just searching remote and installing
    // locally
    // TODO: If yes, check global boilermaker config for local vs remote db option

    let is_dev_env = is_dev_env();
    let config_path = cli.config.map(|p| p.as_path().to_owned());
    let sys_config = get_system_config(config_path.as_deref())?;
    let app_state = AppState {
        config_path: get_system_config_path(config_path.as_deref())?
            .map(|p| p.to_string_lossy().into_owned()),
        db_path: sys_config.db_path.clone(),
        log_level: cli.debug,
        local_db: Arc::new(LocalDb::new(&sys_config.db_path).await?),
        sys_config,
    };

    {
        let db = app_state.local_db.clone();
        if !db.template_table_exists().await? {
            db.create_schema().await?;

            let idx_docs_opts = Some(IndexDocsOptions { dev: is_dev_env });
            db.index_docs(idx_docs_opts).await?;
        }
    }

    let Some(command) = cli.command else {
        println!("🔨 Boilermaker - Make. Boilerplate. Sane!");
        info!("No command provided. Use --help for usage.");
        return Ok(());
    };

    // TODO: clean this up with aliases or direct imports.
    match command {
        Commands::Config(subcmd) => match subcmd {
            Config::Get(cmd) => config::get(&app_state, &cmd).await,
        },
        Commands::Docs(subcmd) => match subcmd {
            Docs::List(cmd) => docs::list(&app_state, &cmd).await,
            Docs::View(cmd) => docs::view(&app_state, &cmd).await,
        },
        Commands::Completion(subcmd) => match subcmd {
            Completion::GenBash(cmd) => {
                let mut clap_cli = Cli::command();
                completion::gen_bash(&app_state, &cmd, &mut clap_cli).await
            }
        },
        Commands::Install(cmd) => commands::install(&app_state, &cmd).await,
        Commands::List(cmd) => commands::list(&app_state, &cmd).await,
        Commands::New(cmd) => commands::new(&app_state, &cmd).await,
        Commands::Remove(cmd) => commands::remove(&app_state, &cmd).await,
        Commands::Search(cmd) => commands::search(&app_state, &cmd).await,
        Commands::Show(cmd) => commands::show(&app_state, &cmd).await,
        Commands::Sources(subcmd) => match subcmd {
            Sources::Add(cmd) => sources::add(&app_state, &cmd).await,
            Sources::List(cmd) => sources::list(&app_state, &cmd).await,
            Sources::Show(cmd) => sources::show::show(&app_state, &cmd).await,
            Sources::Templates(subcmd) => match subcmd {
                SourceTemplates::Install(cmd) => {
                    sources::templates::install(&app_state, &cmd).await
                }
                SourceTemplates::List(cmd) => sources::templates::list(&app_state, &cmd).await,
                SourceTemplates::Show(cmd) => {
                    sources::templates::show::show(&app_state, &cmd).await
                }
            },
        },
        Commands::Update(cmd) => commands::update(&app_state, &cmd).await,
    }
}
