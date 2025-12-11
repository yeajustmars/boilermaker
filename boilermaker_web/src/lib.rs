use std::{fmt, path::PathBuf, sync::Arc};

use axum::{routing::get, serve::Serve, Router};
use color_eyre::eyre::Result;
use minijinja::path_loader;
use minijinja_autoreload::AutoReloader;
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
    pub jinja: minijinja::Environment<'static>,
    pub reloader: Option<AutoReloader>,
    pub is_dev_env: bool,
    pub log_level: u8,
}

impl fmt::Debug for WebAppState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WebAppState {{ db, jinja, ... }}")
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

        let mut jinja = minijinja::Environment::new();

        let reloader = if is_dev_env {
            Some(AutoReloader::new(move |notifier| {
                let template_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("templates");
                let mut env = minijinja::Environment::new();
                env.set_loader(path_loader(&template_path));
                notifier.watch_path(&template_path, true);
                Ok(env)
            }))
        } else {
            None
        };

        if !is_dev_env {
            Self::load_templates(&mut jinja);
        }

        Ok(WebAppState {
            db,
            log_level,
            jinja,
            reloader,
            is_dev_env,
        })
    }

    fn load_templates(jinja: &mut minijinja::Environment) {
        jinja
            .add_template("layout.jinja", include_str!("../templates/layout.jinja"))
            .unwrap();
        jinja
            .add_template("nav.jinja", include_str!("../templates/nav.jinja"))
            .unwrap();
        jinja
            .add_template("help.jinja", include_str!("../templates/help.jinja"))
            .unwrap();
        jinja
            .add_template("home.jinja", include_str!("../templates/home.jinja"))
            .unwrap();
    }

    #[tracing::instrument]
    pub fn render(&self, name: &str, context: minijinja::value::Value) -> String {
        if self.is_dev_env {
            let env = self.reloader.as_ref().unwrap().acquire_env().unwrap();
            let template = env.get_template(name).unwrap();
            template.render(context).unwrap()
        } else {
            let template = self.jinja.get_template(name).unwrap();
            template.render(context).unwrap()
        }
    }
}

pub struct WebApp {
    server: Serve<tokio::net::TcpListener, Router, Router>,
    pub address: String,
}

// TODO: figure out how to use assets directory from boilermaker_views in web crate
// TODO: make sure there is one (and only one) assets/ dir for use by views + web crates
impl WebApp {
    #[tracing::instrument]
    pub async fn build(address: &str, app_state: Arc<WebAppState>) -> Result<Self> {
        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();

        let router = Router::new()
            .route("/", get(routes::home))
            .route("/help", get(routes::help))
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
