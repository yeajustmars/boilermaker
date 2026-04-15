-- docs up

CREATE TABLE IF NOT EXISTS doc (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  rel_path TEXT NOT NULL,
  content TEXT NOT NULL,
  title TEXT,
  created_at TIMESTAMP NOT NULL
);

CREATE VIRTUAL TABLE IF NOT EXISTS doc_fts USING fts5(rel_path, title, content);

-- after insert
CREATE TRIGGER IF NOT EXISTS doc_after_insert
AFTER INSERT ON doc
BEGIN
  INSERT INTO doc_fts(rowid, rel_path, title, content) VALUES (new.id, new.rel_path, new.title, new.content);
END;

-- after update
CREATE TRIGGER IF NOT EXISTS doc_after_update
AFTER UPDATE ON doc
BEGIN
  INSERT INTO doc_fts(doc_fts, rowid) VALUES('delete', old.id);

  INSERT INTO doc_fts(rowid, rel_path, content) VALUES (new.id, new.rel_path, new.title, new.content);
END;

-- after delete
CREATE TRIGGER IF NOT EXISTS doc_after_delete
AFTER DELETE ON doc
BEGIN
  INSERT INTO doc_fts(doc_fts, rowid) VALUES('delete', old.id);
END;
