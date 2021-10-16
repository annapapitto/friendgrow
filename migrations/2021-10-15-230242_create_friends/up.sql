CREATE TABLE friends (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE,
    freq_days INTEGER NOT NULL,
    last_seen TEXT
);