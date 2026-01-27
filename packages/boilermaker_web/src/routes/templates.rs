use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::Html};
use axum_template::TemplateEngine;
use color_eyre::eyre::{eyre, Result};
use minijinja::context;

use crate::{make_context, WebAppState};
use boilermaker_core::db::SourceTemplateResult;

// TODO: paginate templates
// TODO: add proper error page
pub async fn templates(State(app): State<Arc<WebAppState>>) -> Result<Html<String>, StatusCode> {
    let (sources, templates) = {
        let db = app.db.clone();

        let sources = db
            .list_sources()
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        println!("sources: {sources:#?}");

        let mut templates: Vec<SourceTemplateResult> = vec![];
        for source in &sources {
            let ts = db
                .list_source_templates(source.id, None)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            templates.extend(ts);
        }
        println!("templates: {templates:#?}");

        (sources, templates)
    };

    let ctx = make_context(context! {
        title => "Templates",
        sources => sources,
        templates => templates,
    });
    let out = app
        .template
        .render("templates.html", ctx)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Html(out))
}
