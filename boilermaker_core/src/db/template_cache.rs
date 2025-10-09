use color_eyre::Result;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};

#[async_trait::async_trait]
pub trait TemplateDb: Send + Sync {
    async fn check_unique(&self, row: &TemplateRow) -> Result<Option<TemplateResult>>;
    async fn create_template(&self, row: TemplateRow) -> Result<i64>;
    async fn create_template_table(&self) -> Result<()>;
    async fn delete_template(&self, id: i64) -> Result<i64>;
    async fn find_templates(&self, query: TemplateFindParams) -> Result<Vec<TemplateResult>>;
    async fn get_template(&self, id: i64) -> Result<Option<TemplateResult>>;
    async fn list_templates(
        &self,
        opts: Option<ListTemplateOptions>,
    ) -> Result<Vec<TemplateResult>>;
    async fn template_table_exists(&self) -> Result<bool>;
    async fn update_template(&self, row: TemplateRow) -> Result<TemplateResult>;
}

#[derive(Debug)]
pub struct LocalCache {
    pub pool: SqlitePool,
    pub path: String,
}

impl LocalCache {
    #[tracing::instrument]
    pub async fn new(db_path: &str) -> Result<Self> {
        let options = SqliteConnectOptions::new()
            .filename(db_path)
            .create_if_missing(true);
        let pool = SqlitePool::connect_with(options).await?;
        Ok(Self {
            pool,
            path: db_path.to_owned(),
        })
    }
}

#[async_trait::async_trait]
impl TemplateDb for LocalCache {
    #[tracing::instrument]
    async fn check_unique(&self, row: &TemplateRow) -> Result<Option<TemplateResult>> {
        let result = sqlx::query_as::<_, TemplateResult>(
            r#"
            SELECT *
            FROM template
            WHERE
                name = ?1 AND
                lang = ?2 AND
                repo = ?3;
            "#,
        )
        .bind(&row.name)
        .bind(&row.lang)
        .bind(&row.repo)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    #[tracing::instrument]
    async fn create_template(&self, row: TemplateRow) -> Result<i64> {
        let result = sqlx::query(
            r#"
            INSERT INTO template (name, lang, template_dir, created_at, repo, branch, subdir)
            VALUES (?, ?, ?, strftime('%s','now'), ?, ?, ?);
            "#,
        )
        .bind(&row.name)
        .bind(&row.lang)
        .bind(&row.template_dir)
        .bind(&row.repo)
        .bind(&row.branch)
        .bind(&row.subdir)
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    #[tracing::instrument]
    // TODO: move to migration
    // TODO: rewrite with compile-time macros in sqlx
    async fn create_template_table(&self) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS template (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                lang TEXT,
                template_dir TEXT,
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP,
                repo TEXT,
                branch TEXT,
                subdir TEXT,
                UNIQUE (name, repo, branch, subdir)
            );
            "#,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    #[tracing::instrument]
    async fn delete_template(&self, _id: i64) -> Result<i64> {
        todo!()
    }

    //TODO: add regexs, fuzzy matching, predicates, etc
    #[tracing::instrument]
    async fn find_templates(&self, params: TemplateFindParams) -> Result<Vec<TemplateResult>> {
        let mut query = String::from("SELECT * FROM template WHERE 1=1"); //TODO: clean up the 1=1 hack
        let mut bindings: Vec<String> = Vec::new();

        if let Some(name) = params.name {
            query.push_str(" AND name = ?");
            bindings.push(name);
        }
        if let Some(lang) = params.lang {
            query.push_str(" AND lang = ?");
            bindings.push(lang);
        }
        if let Some(repo) = params.repo {
            query.push_str(" AND repo = ?");
            bindings.push(repo);
        }
        if let Some(branch) = params.branch {
            query.push_str(" AND branch = ?");
            bindings.push(branch);
        }
        if let Some(subdir) = params.subdir {
            query.push_str(" AND subdir = ?");
            bindings.push(subdir);
        }

        query.push_str(" ORDER BY name ASC");

        let mut q = sqlx::query_as::<_, TemplateResult>(&query);
        for binding in &bindings {
            q = q.bind(binding);
        }

        let results = q.fetch_all(&self.pool).await?;

        Ok(results)
    }

    // TODO: add options for ordering, pagination, filtering, etc
    #[tracing::instrument]
    async fn list_templates(
        &self,
        _opts: Option<ListTemplateOptions>,
    ) -> Result<Vec<TemplateResult>> {
        let results = sqlx::query_as::<_, TemplateResult>(
            r#"
            SELECT *
            FROM template
            ORDER BY created_at DESC;
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(results)
    }

    #[tracing::instrument]
    async fn get_template(&self, id: i64) -> Result<Option<TemplateResult>> {
        let result = sqlx::query_as::<_, TemplateResult>(
            r#"
            SELECT *
            FROM template
            WHERE id = ?;
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    #[tracing::instrument]
    async fn template_table_exists(&self) -> Result<bool> {
        // TODO: rewrite with compile-time macros in sqlx
        let row: (i64,) = sqlx::query_as(
            "SELECT COUNT(name) FROM sqlite_master WHERE type='table' AND name='template';",
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(row.0 > 0)
    }

    #[tracing::instrument]
    async fn update_template(&self, _row: TemplateRow) -> Result<TemplateResult> {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct TemplateRow {
    pub name: String,
    pub lang: String,
    pub template_dir: String,
    pub repo: String,
    pub branch: Option<String>,
    pub subdir: Option<String>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct TemplateResult {
    pub id: i64,
    pub name: String,
    pub lang: String,
    pub template_dir: String,
    pub repo: String,
    pub branch: Option<String>,
    pub subdir: Option<String>,
    pub created_at: Option<i32>,
    pub updated_at: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct TemplateFindParams {
    pub name: Option<String>,
    pub lang: Option<String>,
    pub repo: Option<String>,
    pub branch: Option<String>,
    pub subdir: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ListTemplateOptions {
    pub order_by: Option<String>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}
