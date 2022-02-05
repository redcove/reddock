-- Add up migration script here
CREATE TABLE IF NOT EXISTS reports (
    id            INTEGER    PRIMARY KEY AUTOINCREMENT,
    name          TEXT       NOT NULL,
    scope         TEXT       NOT NULL,
    description   TEXT,
    updated       INT        NOT NULL,
    created       INT        NOT NULL
)
