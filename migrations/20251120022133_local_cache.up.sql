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
  sha256_hash TEXT NOT NULL UNIQUE,
  UNIQUE (name, repo, branch, subdir)
);
