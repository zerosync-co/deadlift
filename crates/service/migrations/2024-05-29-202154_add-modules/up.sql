CREATE TABLE IF NOT EXISTS modules (
    hash TEXT PRIMARY KEY NOT NULL,
    binary BLOB NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    subject TEXT NOT NULL UNIQUE
);
