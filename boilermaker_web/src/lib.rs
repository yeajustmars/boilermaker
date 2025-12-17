use std::{fmt, sync::Arc};

use axum::{routing::get, serve::Serve, Router};
use color_eyre::eyre::Result;
use tower_http::services::ServeDir;

use boilermaker_core::{
    config::{DEFAULT_LOCAL_CACHE_PATH_STRING, DEFAULT_WEBSITE_DATABASE_PATH_STRING},
    db::{LocalCache, TemplateDb},
    state::TemplateDbType,
    util::env::is_dev_env,
};

pub mod routes;

pub struct WebAppState {
    pub db: TemplateDbType,
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

        Ok(WebAppState {
            db,
            log_level,
            is_dev_env,
        })
    }
}

pub struct WebApp {
    server: Serve<tokio::net::TcpListener, Router, Router>,
    pub address: String,
}

// TODO: solve: ERROR tokio-runtime-worker ThreadId(03) dioxus_document: Unable to find a document in the renderer. Using the default no-op document.
impl WebApp {
    #[tracing::instrument]
    pub async fn build(address: &str, app_state: Arc<WebAppState>) -> Result<Self> {
        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();

        let router = Router::new()
            .route("/", get(routes::home))
            .route("/docs", get(routes::docs))
            .route("/get-involved", get(routes::get_involved))
            .route("/settings", get(routes::settings))
            .route("/templates", get(routes::templates))
            .nest_service("/assets", ServeDir::new("assets"))
            .with_state(app_state);

        let server = axum::serve(listener, router);

        Ok(WebApp { server, address })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}
