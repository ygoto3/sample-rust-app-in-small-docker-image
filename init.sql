CREATE TABLE IF NOT EXISTS samples (
  id INTEGER PRIMARY KEY NOT NULL,
  name TEXT NOT NULL
);

INSERT INTO samples (name) VALUES ('sample1'), ('sample2'), ('sample3');
