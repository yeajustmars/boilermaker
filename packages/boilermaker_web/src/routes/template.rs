use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Html,
};
use axum_template::TemplateEngine;
use color_eyre::Result;
use minijinja::context;
use pulldown_cmark::{html, Options, Parser};
use tracing::error;

use crate::{make_context, WebAppState};

enum TemplateDetailsError {
    InvalidTemplateId,
    UnknownTemplate,
}

// TODO: impl custom error details w/o EVER revealing the underlying logged error
// TODO: make error.html pretty
fn template_error(
    app: Arc<WebAppState>,
    cause: TemplateDetailsError,
) -> Result<Html<String>, StatusCode> {
    match cause {
        TemplateDetailsError::InvalidTemplateId => {
            let ctx = make_context(context! {
                title => "Template Details Error",
                status => 400,
                error_msg => "400 Bad Request",
                error_details => "The provided template ID is invalid.",
            });
            let err_page = app
                .template
                .render("error.html", ctx)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            Ok(Html(err_page))
        }
        TemplateDetailsError::UnknownTemplate => {
            let ctx = make_context(context! {
                title => "Template Details Error",
                status => 400,
                error_msg => "400 Bad Request",
                error_details => "Couldn't find the requested template.",
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
pub async fn template(
    State(app): State<Arc<WebAppState>>,
    Path(template_id): Path<String>,
) -> Result<Html<String>, StatusCode> {
    let template_id = match template_id.trim().parse::<i64>() {
        Ok(id) => id,
        Err(_) => {
            return template_error(app.clone(), TemplateDetailsError::InvalidTemplateId);
        }
    };

    let (template, files) = {
        let db = app.db.clone();
        let template = match db.get_source_template(template_id).await.map_err(|e| {
            error!("DB error retrieving source template {}: {}", template_id, e);
            StatusCode::INTERNAL_SERVER_ERROR
        })? {
            Some(t) => t,
            None => {
                return template_error(app.clone(), TemplateDetailsError::UnknownTemplate);
            }
        };

        // TODO: decide on pulling all of this. Maybe put it as an option?
        let files = db
            .get_source_template_content_readme_boilermaker(template_id)
            .await
            .map_err(|e| {
                error!(
                    "DB error retrieving template files for template {}: {}",
                    template_id, e
                );
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
        (template, files)
    };

    let readme_rendered = {
        match &files.readme {
            None => "No README found".to_string(),
            Some(readme) => {
                let options = Options::empty();
                let parser = Parser::new_ext(&readme.content, options);
                let mut html_output = String::new();
                html::push_html(&mut html_output, parser);
                html_output
            }
        }
    };

    let ctx = make_context(context! {
        title => "Templates",
        template => template,
        readme => files.readme,
        readme_rendered => readme_rendered,
        boilermaker => files.boilermaker,
    });

    let out = app
        .template
        .render("template.html", ctx)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Html(out))
}
