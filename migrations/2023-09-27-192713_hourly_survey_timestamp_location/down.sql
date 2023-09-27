-- This file should undo anything in `up.sql`
ALTER TABLE hourly_surveys
DROP COLUMN location;

ALTER TABLE hourly_surveys
DROP COLUMN timestamp;