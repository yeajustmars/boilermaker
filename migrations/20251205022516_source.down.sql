DROP TABLE IF EXISTS source;

DROP TABLE IF EXISTS source_template;
DROP TABLE IF EXISTS source_template_fts;
DROP TRIGGER IF EXISTS source_template_after_insert;
DROP TRIGGER IF EXISTS source_template_after_update;
DROP TRIGGER IF EXISTS source_template_after_delete;

DROP TABLE IF EXISTS source_template_content;
DROP TABLE IF EXISTS source_template_content_fts;
DROP TRIGGER IF EXISTS source_template_content_after_insert;
DROP TRIGGER IF EXISTS source_template_content_after_update;
DROP TRIGGER IF EXISTS source_template_content_after_delete;
