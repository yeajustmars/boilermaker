use std::{fmt, sync::Arc};

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
    let (query, options) = match query {
        Err(rejection) => {
            error!("Malformed template query: {rejection}");
            return template_error(app, TemplateError::MalformedQuery);
        }
        Ok(Query(pq)) => (pq.clone(), Some(ListTemplateOptions::from(pq))),
    };

    let (sources, templates) = {
        let db = app.db.clone();

        let sources = db
            .list_sources()
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let mut templates: Vec<SourceTemplateResult> = vec![];
        for source in &sources {
            let more_templates = db
                .list_source_templates(source.id, options.as_ref())
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
        query => query,
    });
    let out = app
        .template
        .render("templates.html", ctx)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Html(out))
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Order {
    #[serde(rename = "asc")]
    ASC,
    #[serde(rename = "desc")]
    DESC,
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Order::ASC => write!(f, "ASC"),
            Order::DESC => write!(f, "DESC"),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct PageQuery {
    pub sort_by: Option<String>,
    pub offset: Option<u16>,
    pub limit: Option<u16>,
    pub order: Option<Order>,
}

impl From<PageQuery> for ListTemplateOptions {
    fn from(q: PageQuery) -> Self {
        let sort_by = match q.sort_by {
            Some(s) => s,
            None => "name".to_string(),
        };
        let direction = match q.order {
            Some(order) => &order.to_string(),
            None => "ASC",
        };
        let order_by = Some(format!("{sort_by} {direction}"));

        Self {
            order_by,
            limit: q.limit,
            offset: q.offset,
        }
    }
}
