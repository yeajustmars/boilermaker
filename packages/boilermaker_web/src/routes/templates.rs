use std::sync::Arc;

use axum::{
    extract::{rejection::QueryRejection, Query, State},
    http::StatusCode,
    response::Html,
};
use axum_template::TemplateEngine;
use color_eyre::Result;
use minijinja::context;
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::{make_context, WebAppState};
use boilermaker_core::db::{ListTemplateOptions, SourceTemplateResult};

enum TemplateError {
    MalformedQuery,
}

// TODO: impl custom error details w/o EVER revealing the underlying logged error
// TODO: make error.html pretty
fn template_error(app: Arc<WebAppState>, cause: TemplateError) -> Result<Html<String>, StatusCode> {
    match cause {
        TemplateError::MalformedQuery => {
            let ctx = make_context(context! {
                title => "Template Error",
                status => 400,
                error_msg => "400 Bad Request",
                error_details => "Invalid query string",
            });
            let err_page = app
                .template
                .render("error.html", ctx)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            Ok(Html(err_page))
        }
    }
}

// TODO: add proper error page
pub async fn templates(
    State(app): State<Arc<WebAppState>>,
    query: Result<Query<PageQuery>, QueryRejection>,
) -> Result<Html<String>, StatusCode> {
    let q = match query {
        Ok(Query(query)) => query,
        Err(rejection) => {
            error!("Malformed template query: {rejection}");
            return template_error(app, TemplateError::MalformedQuery);
        }
    };
    let options = ListTemplateOptions::from(q);

    let (sources, templates) = {
        let db = app.db.clone();
        let sources = db
            .list_sources()
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let mut templates: Vec<SourceTemplateResult> = vec![];
        for source in &sources {
            let more_templates = db
                .list_source_templates(source.id, Some(&options))
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            templates.extend(more_templates);
        }
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

#[derive(Deserialize, Debug)]
pub enum Order {
    #[serde(rename = "asc")]
    ASC,
    #[serde(rename = "desc")]
    DESC,
}

#[derive(Deserialize, Debug)]
pub struct PageQuery {
    pub sort_by: String,
    pub offset: usize,
    pub limit: usize,
    pub order: Order,
}

impl From<PageQuery> for ListTemplateOptions {
    fn from(q: PageQuery) -> Self {
        let sort_by = if q.sort_by.is_empty() {
            "name".to_string()
        } else {
            q.sort_by
        };
        let direction = match q.order {
            Order::ASC => "ASC",
            Order::DESC => "DESC",
        };
        let order_by = Some(format!("{sort_by} {direction}"));
        let limit = if q.limit > 0 {
            Some(q.limit as u16)
        } else {
            None
        };
        let offset = if q.offset > 0 {
            Some(q.offset as u16)
        } else {
            None
        };

        Self {
            order_by,
            limit,
            offset,
        }
    }
}
