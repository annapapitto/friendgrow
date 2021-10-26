ALTER TABLE friends
RENAME COLUMN freq_days TO freq_weeks;

UPDATE friends
SET freq_weeks = (
    SELECT MIN(f_copy.freq_weeks / 7)
    FROM friends as f_copy
    WHERE friends.id = f_copy.id
);

UPDATE friends SET freq_weeks = 1 WHERE freq_weeks = 0;
