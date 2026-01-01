use std::{fmt, sync::Arc};

use axum::{routing::get, serve::Serve, Router};
use axum_embed::ServeEmbed;
use axum_template::engine::Engine;
use color_eyre::eyre::Result;
use minijinja::{path_loader, Environment};
use minijinja_autoreload::AutoReloader;
use rust_embed::RustEmbed;
use tracing::info;

use boilermaker_core::{
    config::{DEFAULT_LOCAL_CACHE_PATH_STRING, DEFAULT_WEBSITE_DATABASE_PATH_STRING},
    db::{LocalCache, TemplateDb, TemplateMethods},
    state::TemplateDbType,
    util::env::is_dev_env,
};

pub mod routes;

pub struct WebAppState {
    pub db: TemplateDbType,
    pub template: Engine<AutoReloader>,
    pub is_dev_env: bool,
    pub log_level: u8,
}

impl fmt::Debug for WebAppState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WebAppState {{ db... }}")
    }
}

impl WebAppState {
    #[tracing::instrument]
    pub async fn new() -> Result<Self> {
        let is_dev_env = is_dev_env();
        let log_level = 1;

        let db_path = if is_dev_env {
            DEFAULT_LOCAL_CACHE_PATH_STRING.as_str()
        } else {
            DEFAULT_WEBSITE_DATABASE_PATH_STRING.as_str()
        };

        let db = Arc::new(LocalCache::new(db_path).await?);
        {
            let db = db.clone();
            if !db.template_table_exists().await? {
                db.create_schema().await?;
            }
        }

        // TODO: remove minijinja autoreloader in production
        let template_dir = "views/";
        let reloader = AutoReloader::new(move |notifier| {
            let mut env = Environment::new();
            env.set_loader(path_loader(&template_dir));
            notifier.watch_path(template_dir, true);
            Ok(env)
        });
        let template = Engine::from(reloader);

        Ok(WebAppState {
            db,
            template,
            log_level,
            is_dev_env,
        })
    }
}

#[derive(RustEmbed, Clone)]
#[folder = "../../packages/boilermaker_ui/assets/"]
struct Assets;

pub struct WebApp {
    server: Serve<tokio::net::TcpListener, Router, Router>,
    pub address: String,
    pub is_dev_env: bool,
}

impl WebApp {
    #[tracing::instrument]
    pub async fn build(address: &str, app_state: Arc<WebAppState>) -> Result<Self> {
        let is_dev_env = app_state.is_dev_env;
        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let serve_assets = ServeEmbed::<Assets>::new();

        let router = Router::new()
            .route("/", get(routes::home))
            .route("/docs", get(routes::docs))
            .route("/get-involved", get(routes::get_involved))
            .route("/settings", get(routes::settings))
            .route("/templates", get(routes::templates))
            //.nest_service("/assets", ServeDir::new("../../boilermaker_ui/assets"))
            .nest_service("/assets", serve_assets)
            .with_state(app_state);

        let server = axum::serve(listener, router);

        Ok(WebApp {
            server,
            address,
            is_dev_env,
        })
    }

    #[tracing::instrument]
    pub async fn run(self) -> Result<(), std::io::Error> {
        info!(
            "Starting web server on: {} (dev_env={})",
            self.address, self.is_dev_env
        );

        self.server.await
    }
}

impl fmt::Debug for WebApp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "WebApp {{ address: {}, is_dev_env: {} }}",
            self.address, self.is_dev_env
        )
    }
}
