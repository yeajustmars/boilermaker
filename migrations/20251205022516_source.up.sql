
-- ------------------------------------------------ source

CREATE TABLE IF NOT EXISTS source (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL UNIQUE,
  backend TEXT NOT NULL,
  coordinate TEXT,
  description TEXT,
  sha256_hash TEXT NOT NULL UNIQUE,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP
);

CREATE VIRTUAL TABLE IF NOT EXISTS source_fts USING fts5(
  name,
  backend,
  coordinate,
  description,
  sha256_hash,
  content='source',
  content_rowid='id'
);

-- .................. template after insert
CREATE TRIGGER IF NOT EXISTS source_after_insert AFTER INSERT ON source BEGIN
    INSERT INTO source_fts
      (rowid, name, backend, coordinate, sha256_hash)
    VALUES
      (new.id, new.name, new.backend, new.coordinate, new.sha256_hash);
END;

-- .................. template after update
CREATE TRIGGER IF NOT EXISTS source_after_update AFTER UPDATE ON source BEGIN
    INSERT INTO source_fts
      (source_fts, rowid, name, backend, coordinate, sha256_hash)
    VALUES
      ('delete', old.rowid, old.name, old.backend, old.coordinate, old.sha256_hash);

    INSERT INTO source_fts
      (source_fts, rowid, name, backend, coordinate, sha256_hash)
    VALUES
      (new.id, new.name, new.backend, new.coordinate, new.sha256_hash);
END;

-- .................. template after delete
CREATE TRIGGER IF NOT EXISTS source_after_delete AFTER DELETE ON source BEGIN
    INSERT INTO source_fts
      (source_fts, rowid, name, backend, coordinate, sha256_hash)
    VALUES
      ('delete', old.rowid, old.name, old.backend, old.coordinate, old.sha256_hash);
END;

-- ------------------------------------------------ source_template

CREATE TABLE IF NOT EXISTS source_template (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  source_id INTEGER NOT NULL,
  repo TEXT NOT NULL,
  lang TEXT NOT NULL,
  name TEXT NOT NULL,
  branch TEXT,
  subdir TEXT,
  sha256_hash TEXT NOT NULL UNIQUE,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP,
  UNIQUE (name, repo, branch, subdir)
  FOREIGN KEY (source_id) REFERENCES source(id) DEFERRABLE INITIALLY DEFERRED
);

CREATE VIRTUAL TABLE IF NOT EXISTS source_template_fts USING fts5(
  repo,
  lang,
  name,
  branch,
  subdir,
  sha256_hash,
  content='template',
  content_rowid='id'
);

-- .................. template after insert
CREATE TRIGGER IF NOT EXISTS source_template_after_insert AFTER INSERT ON source_template BEGIN
    INSERT INTO source_template_fts
      (rowid, repo, lang, name, branch, subdir, sha256_hash)
    VALUES
      (new.id, new.repo, new.lang, new.name, new.branch, new.subdir, new.sha256_hash);
END;

-- .................. template after update
CREATE TRIGGER IF NOT EXISTS source_template_after_update AFTER UPDATE ON source_template BEGIN
    INSERT INTO source_template_fts
      (source_template_fts, rowid, repo, lang, name, branch, subdir, sha256_hash)
    VALUES
      ('delete', old.rowid, old.repo, old.lang, old.name, old.branch, old.subdir, old.sha256_hash);

    INSERT INTO source_template_fts
      (rowid, repo, lang, name, branch, subdir, sha256_hash)
    VALUES
      (new.id, new.repo, new.lang, new.name, new.branch, new.subdir, new.sha256_hash);
END;

-- .................. template after delete
CREATE TRIGGER IF NOT EXISTS source_template_after_delete AFTER DELETE ON source_template BEGIN
    INSERT INTO source_template_fts
      (source_template_fts, rowid, repo, lang, name, branch, subdir, sha256_hash)
    VALUES
      ('delete', old.rowid, old.repo, old.lang, old.name, old.branch, old.subdir, old.sha256_hash);
END;








-- ------------------------------------------------ source_template_content

CREATE TABLE IF NOT EXISTS source_template_content (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  source_template_id INTEGER NOT NULL,
  file_path TEXT NOT NULL,
  content TEXT,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP,
  FOREIGN KEY (source_template_id) REFERENCES source_template(id) ON DELETE CASCADE DEFERRABLE INITIALLY DEFERRED,
  UNIQUE (source_template_id, file_path)
);

CREATE VIRTUAL TABLE IF NOT EXISTS source_template_content_fts USING fts5(
  file_path,
  content,
  content='template_content',
  content_rowid='id'
);

-- .................. template after insert
CREATE TRIGGER IF NOT EXISTS source_template_content_after_insert AFTER INSERT ON source_template_content BEGIN
    INSERT INTO source_template_content_fts
      (rowid, file_path, content)
    VALUES
      (new.id, new.file_path, new.content);
END;

-- .................. template after update
CREATE TRIGGER IF NOT EXISTS source_template_content_after_update AFTER UPDATE ON source_template_content BEGIN
    INSERT INTO source_template_content_fts
      (source_template_content_fts, rowid, file_path, content)
    VALUES
      ('delete', old.rowid, old.file_path, old.content);

    INSERT INTO source_template_content_fts
      (rowid, file_path, content)
    VALUES
      (new.rowid, new.file_path, new.content);
END;

-- .................. template after delete
CREATE TRIGGER IF NOT EXISTS source_template_content_after_delete AFTER DELETE ON source_template_content BEGIN
    INSERT INTO source_template_content_fts
      (source_template_content_fts, rowid, file_path, content)
    VALUES
      ('delete', old.rowid, old.file_path, old.content);
END;
