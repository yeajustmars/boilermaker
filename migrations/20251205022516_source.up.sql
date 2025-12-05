
-- ------------------------------------------------ source

CREATE TABLE IF NOT EXISTS source (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL UNIQUE,
  backend TEXT NOT NULL,
  config TEXT,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP
);

CREATE VIRTUAL TABLE IF NOT EXISTS source_fts USING fts5(
  name,
  backend,
  config,
  content='source',
  content_rowid='id'
);

-- .................. template after insert
CREATE TRIGGER IF NOT EXISTS source_after_insert AFTER INSERT ON source BEGIN
    INSERT INTO template_fts
      (rowid, name, lang, template_dir, repo, branch, subdir, sha256_hash)
    VALUES
      (new.id, new.name, new.lang, new.template_dir, new.repo, new.branch, new.subdir, new.sha256_hash);
END;

-- .................. template after update
CREATE TRIGGER IF NOT EXISTS source_after_update AFTER UPDATE ON source BEGIN
    INSERT INTO template_fts
      (template_fts, rowid, name, lang, template_dir, repo, branch, subdir, sha256_hash)
    VALUES
      ('delete', old.rowid, old.name, old.lang, old.template_dir, old.repo, old.branch, old.subdir, old.sha256_hash);

    INSERT INTO template_fts
      (rowid, name, lang, template_dir, repo, branch, subdir, sha256_hash)
    VALUES
      (new.id, new.name, new.lang, new.template_dir, new.repo, new.branch, new.subdir, new.sha256_hash);
END;

-- .................. template after delete
CREATE TRIGGER IF NOT EXISTS source_after_delete AFTER DELETE ON source BEGIN
    INSERT INTO template_fts
      (template_fts, rowid, name, lang, template_dir, repo, branch, subdir, sha256_hash)
    VALUES
      ('delete', old.rowid, old.name, old.lang, old.template_dir, old.repo, old.branch, old.subdir, old.sha256_hash);
END;

-- ------------------------------------------------ source_template

CREATE TABLE IF NOT EXISTS source_template (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  source_id INTEGER NOT NULL,
  name TEXT NOT NULL,
  lang TEXT,
  template_dir TEXT,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP,
  repo TEXT,
  branch TEXT,
  subdir TEXT,
  sha256_hash TEXT NOT NULL UNIQUE,
  UNIQUE (name, repo, branch, subdir)
  FOREIGN KEY (source_id) REFERENCES source(id)
);

CREATE VIRTUAL TABLE IF NOT EXISTS source_template_fts USING fts5(
  name,
  lang,
  template_dir,
  repo,
  branch,
  subdir,
  sha256_hash,
  content='template',
  content_rowid='id'
);

-- .................. template after insert
CREATE TRIGGER IF NOT EXISTS source_template_after_insert AFTER INSERT ON source_template BEGIN
    INSERT INTO template_fts
      (rowid, name, lang, template_dir, repo, branch, subdir, sha256_hash)
    VALUES
      (new.id, new.name, new.lang, new.template_dir, new.repo, new.branch, new.subdir, new.sha256_hash);
END;

-- .................. template after update
CREATE TRIGGER IF NOT EXISTS source_template_after_update AFTER UPDATE ON source_template BEGIN
    INSERT INTO template_fts
      (template_fts, rowid, name, lang, template_dir, repo, branch, subdir, sha256_hash)
    VALUES
      ('delete', old.rowid, old.name, old.lang, old.template_dir, old.repo, old.branch, old.subdir, old.sha256_hash);

    INSERT INTO template_fts
      (rowid, name, lang, template_dir, repo, branch, subdir, sha256_hash)
    VALUES
      (new.id, new.name, new.lang, new.template_dir, new.repo, new.branch, new.subdir, new.sha256_hash);
END;

-- .................. template after delete
CREATE TRIGGER IF NOT EXISTS source_template_after_delete AFTER DELETE ON source_template BEGIN
    INSERT INTO template_fts
      (template_fts, rowid, name, lang, template_dir, repo, branch, subdir, sha256_hash)
    VALUES
      ('delete', old.rowid, old.name, old.lang, old.template_dir, old.repo, old.branch, old.subdir, old.sha256_hash);
END;








-- ------------------------------------------------ source_template_content

CREATE TABLE IF NOT EXISTS source_template_content (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  template_id INTEGER NOT NULL,
  file_path TEXT NOT NULL,
  content TEXT,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP,
  FOREIGN KEY (template_id) REFERENCES template(id) ON DELETE CASCADE,
  UNIQUE (template_id, file_path)
);

CREATE VIRTUAL TABLE IF NOT EXISTS source_template_content_fts USING fts5(
  file_path,
  content,
  content='template_content',
  content_rowid='id'
);

-- .................. template after insert
CREATE TRIGGER IF NOT EXISTS source_template_content_after_insert AFTER INSERT ON source_template_content BEGIN
    INSERT INTO template_content_fts
      (rowid, file_path, content)
    VALUES
      (new.id, new.file_path, new.content);
END;

-- .................. template after update
CREATE TRIGGER IF NOT EXISTS source_template_content_after_update AFTER UPDATE ON source_template_content BEGIN
    INSERT INTO template_content_fts
      (template_content_fts, rowid, file_path, content)
    VALUES
      ('delete', old.rowid, old.file_path, old.content);

    INSERT INTO template_content_fts
      (rowid, file_path, content)
    VALUES
      (new.rowid, new.file_path, new.content);
END;

-- .................. template after delete
CREATE TRIGGER IF NOT EXISTS source_template_content_after_delete AFTER DELETE ON source_template_content BEGIN
    INSERT INTO template_content_fts
      (template_content_fts, rowid, file_path, content)
    VALUES
      ('delete', old.rowid, old.file_path, old.content);
END;
