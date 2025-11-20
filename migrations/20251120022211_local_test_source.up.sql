
-- ------------------------------------------------ template

CREATE TABLE IF NOT EXISTS template (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  uuid TEXT NOT NULL UNIQUE,
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

CREATE VIRTUAL TABLE template_fts USING fts5(
  uuid,
  name,
  lang,
  template_dir,
  repo,
  branch,
  subdir,
  content='template',
  content_rowid='id'
);

-- .................. template after insert
CREATE TRIGGER IF NOT EXISTS template_after_insert AFTER INSERT ON template BEGIN
    INSERT INTO template_fts (rowid, uuid, name, lang, template_dir, repo, branch, subdir)
    VALUES (new.id, new.uuid, new.name, new.lang, new.template_dir, new.repo, new.branch, new.subdir);
END;

-- .................. template after update
CREATE TRIGGER IF NOT EXISTS template_after_update AFTER UPDATE ON template BEGIN
    INSERT INTO template_fts (template_fts, rowid, uuid, name, lang, template_dir, repo, branch, subdir)
    VALUES('delete', old.uuid, old.name, old.lang, old.template_dir, old.repo, old.branch, old.subdir);

    INSERT INTO template_fts (rowid, uuid, name, lang, template_dir, repo, branch, subdir)
    VALUES (new.id, new.uuid, new.name, new.lang, new.template_dir, new.repo, new.branch, new.subdir);
END;

-- .................. template after delete
CREATE TRIGGER IF NOT EXISTS template_after_delete AFTER DELETE ON template BEGIN
    INSERT INTO template_fts (template_fts, rowid, uuid, name, lang, template_dir, repo, branch, subdir)
    VALUES('delete', old.rowid, old.uuid, old.name, old.lang, old.template_dir, old.repo, old.branch, old.subdir);
END;








-- ------------------------------------------------ template_content

CREATE TABLE IF NOT EXISTS template_content (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  template_id INTEGER NOT NULL,
  file_path TEXT NOT NULL,
  content TEXT,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP,
  FOREIGN KEY (template_id) REFERENCES template(id) ON DELETE CASCADE,
  UNIQUE (template_id, file_path)
);

CREATE VIRTUAL TABLE template_content_fts USING fts5(
  file_path,
  content,
  content='template_content',
  content_rowid='id'
);

-- .................. template after insert
CREATE TRIGGER IF NOT EXISTS template_content_after_insert AFTER INSERT ON template_content BEGIN
    INSERT INTO template_content_fts (rowid, file_path, content)
    VALUES (new.id, new.file_path, new.content);
END;

-- .................. template after update
CREATE TRIGGER IF NOT EXISTS template_content_after_update AFTER UPDATE ON template_content BEGIN
    INSERT INTO template_content_fts (template_content_fts, rowid, file_path, content)
    VALUES('delete', old.rowid, old.file_path, old.content);

    INSERT INTO template_content_fts (rowid, file_path, content)
    VALUES (new.doc_id, new.file_path, new.content);
END;

-- .................. template after delete
CREATE TRIGGER IF NOT EXISTS template_content_after_delete AFTER DELETE ON template_content BEGIN
    INSERT INTO template_content_fts (template_content_fts, rowid, file_path, content)
    VALUES('delete', old.doc_id, old.file_path, old.content);
END;

-- ------------------------------------------------ sample queries

-- INSERT INTO template (uuid, name, lang, template_dir, created_at, updated_at, repo, branch, subdir) VALUES ('123e4567-e89b-12d3-a456-426614174000', 'Default Template', 'en', '/templates/default', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 'some_repo_url', 'main', '');

-- INSERT INTO template_content (template_id, file_path, content, created_at, updated_at) VALUES (1, 'index.html', '<html><body><h1>Welcome to the Default Template</h1></body></html>', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

