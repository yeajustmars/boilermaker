use color_eyre::Result;

use template::TemplateResult;

#[async_trait::async_trait]
pub trait TemplateCache: Send + Sync {
    async fn add_template(&self) -> i64;
    async fn update_template(&self) -> Result<TemplateResult>;
    async fn get_template(&self) -> Result<TemplateResult>;
    async fn list_templates(&self) -> Vec<TemplateResult>;
    async fn search_templates(&self) -> Vec<TemplateResult>;
    async fn delete_template(&self) -> Result<i64>;
    async fn template_table_exists(&self) -> Result<bool>;
}
