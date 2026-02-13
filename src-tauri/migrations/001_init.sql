CREATE TABLE IF NOT EXISTS app_settings (
  id INTEGER PRIMARY KEY CHECK (id = 1),
  locale TEXT NOT NULL DEFAULT 'en-US',
  launch_on_login INTEGER NOT NULL DEFAULT 0,
  theme TEXT NOT NULL DEFAULT 'system'
);
