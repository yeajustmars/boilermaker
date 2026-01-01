use std::time::{SystemTime, UNIX_EPOCH};
use std::{fmt, sync::Arc};

use axum::{routing::get, serve::Serve, Router};
use axum_embed::ServeEmbed;
use axum_template::engine::Engine;
use color_eyre::eyre::Result;
use minijinja::{context, path_loader, value::merge_maps, Environment};
use minijinja_autoreload::AutoReloader;
use rust_embed::RustEmbed;
use tracing::info;

use boilermaker_core::{
    config::{DEFAULT_LOCAL_CACHE_PATH_STRING, DEFAULT_WEBSITE_DATABASE_PATH_STRING},
    db::{LocalCache, TemplateDb, TemplateMethods},
    state::TemplateDbType,
    util::env::is_dev_env,
};
use boilermaker_ui::constants::{
    DROPDOWN_LINK_STYLE, DROPDOWN_MENU_STYLE, FONT_AWESOME_URL, FONT_FIRA_CODE_URL,
    FONT_ROBOTO_URL, INDENTED_DROPDOWN_LINK_STYLE, LAYOUT_STYLE, NAVBAR_STYLE,
};

pub mod routes;

pub struct WebAppState {
    pub db: TemplateDbType,
    pub template: Engine<AutoReloader>,
    pub base_ctx: minijinja::value::Value,
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

        let base_ctx = context! {
            is_dev_env => is_dev_env,
            layout_style => LAYOUT_STYLE,
            nav_style => NAVBAR_STYLE,
            nav_dropdown_menu_style => DROPDOWN_MENU_STYLE,
            nav_dropdown_style => DROPDOWN_LINK_STYLE,
            nav_indented_dropdown_style => INDENTED_DROPDOWN_LINK_STYLE,
            font_awesome_url => FONT_AWESOME_URL,
            font_roboto_url => FONT_ROBOTO_URL,
            font_fira_code_url => FONT_FIRA_CODE_URL,

        };

        Ok(WebAppState {
            db,
            template,
            base_ctx,
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

fn get_unix_timestamp_nanos() -> u128 {
    let duration_since_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    duration_since_epoch.as_nanos()
}

pub fn make_context(
    app: Arc<WebAppState>,
    ctx: minijinja::value::Value,
) -> minijinja::value::Value {
    let time_values = context! {
        timestamp => get_unix_timestamp_nanos(),
    };
    merge_maps([app.base_ctx.clone(), time_values, ctx])
}
