ALTER TABLE friends
RENAME TO oldFriends;

CREATE TABLE friends (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE,
    freq_weeks INTEGER NOT NULL,
    last_seen TEXT
);

INSERT INTO friends
(id, name, freq_weeks, last_seen)
SELECT id, name, freq_weeks, last_seen
FROM oldFriends;

DROP TABLE oldFriends;
