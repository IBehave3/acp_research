-- Your SQL goes here
ALTER TABLE hourly_surveys
ADD location VARCHAR NOT NULL;

ALTER TABLE hourly_surveys
ADD timestamp INT NOT NULL;