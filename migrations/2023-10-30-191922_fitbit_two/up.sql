-- Your SQL goes here
CREATE TABLE user_fitbit_two (
    id SERIAL PRIMARY KEY,
    userId SERIAL NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    verificationCode VARCHAR NOT NULL,
    UNIQUE(userId)
);