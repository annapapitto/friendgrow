ALTER TABLE friends
RENAME COLUMN freq_weeks TO freq_days;

UPDATE friends
SET freq_days = (
    SELECT (f_copy.freq_days * 7)
    FROM friends as f_copy
    WHERE friends.id = f_copy.id
);

UPDATE friends SET freq_days = 1 WHERE freq_days = 0;
