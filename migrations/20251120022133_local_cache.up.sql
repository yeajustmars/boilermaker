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
