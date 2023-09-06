-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    createdAt TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    username VARCHAR NOT NULL UNIQUE,
    passwordHash VARCHAR NOT NULL,
    salt VARCHAR NOT NULL
);

CREATE TABLE user_airthings (
    id SERIAL PRIMARY KEY,
    userId SERIAL NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    clientSecret VARCHAR NOT NULL,
    clientId VARCHAR NOT NULL,
    groupId VARCHAR NOT NULL,
    deviceIds TEXT[],
    UNIQUE(userId)
);

CREATE TABLE user_gray_wolfs (
    id SERIAL PRIMARY KEY,
    userId SERIAL NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    apiKey VARCHAR NOT NULL,
    deviceIds TEXT[],
    UNIQUE(userId)
);

CREATE TABLE user_uhoo_auras (
    id SERIAL PRIMARY KEY,
    userId SERIAL NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    clientSecret VARCHAR NOT NULL,
    deviceIds TEXT[],
    UNIQUE(userId)
);