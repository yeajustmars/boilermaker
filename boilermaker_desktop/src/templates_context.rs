use dioxus::prelude::*;
use tracing::error;

use crate::APP_STATE;
use boilermaker_core::db::{ListTemplateOptions, TemplateResult};

// This handles a list of templates as a Dioxus context. Currently, loads all templates from DB, but
// eventually it could be extended to load templates from sources & search.
#[derive(Clone, Copy)]
pub struct TemplatesContext {
    pub templates: Signal<Vec<TemplateResult>>,

    // Used in refresh() to trigger, well yes, a refresh.
    refresh_trigger: Signal<usize>,
}

impl TemplatesContext {
    pub fn refresh(&mut self) {
        let current = *self.refresh_trigger.read();
        self.refresh_trigger.set(current + 1);
    }

    pub fn is_empty(&self) -> bool {
        self.templates.read().is_empty()
    }
}

// Initialize once at the root (in App).
pub fn init_templates_context() {
    let templates = use_signal::<Vec<TemplateResult>>(Vec::new);
    let refresh_trigger = use_signal(|| 0usize);

    let _ = use_resource(move || async move {
        let trigger = refresh_trigger.read();
        // Drop immediately to avoid holding the read-references over an await call.
        drop(trigger);
        if let Err(e) = load_templates(templates).await {
            error!("Error loading templates: {}", e);
        }
    });

    // Shares the context to all child elements.
    use_context_provider(|| TemplatesContext {
        templates,
        refresh_trigger,
    });
}

async fn load_templates(
    mut templates: Signal<Vec<TemplateResult>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let cache = &APP_STATE
        .get()
        .ok_or("APP_STATE not initialized")?
        .template_db;
    let list_opts = Some(ListTemplateOptions {
        order_by: Some("created_at DESC, name ASC".to_string()),
        limit: Some(10),
        offset: None,
    });

    let rows = cache.list_templates(list_opts).await?;
    templates.set(rows);
    Ok(())
}
