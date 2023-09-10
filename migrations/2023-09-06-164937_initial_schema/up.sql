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

CREATE TABLE airthings (
    id SERIAL PRIMARY KEY,
    deviceid VARCHAR NOT NULL,
    userId SERIAL NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    battery DOUBLE PRECISION NOT NULL,
    co2 DOUBLE PRECISION NOT NULL,
    humidity DOUBLE PRECISION NOT NULL,
    pm1 DOUBLE PRECISION NOT NULL,
    pm25 DOUBLE PRECISION NOT NULL,
    pressure DOUBLE PRECISION NOT NULL,
    radonShortTermAvg DOUBLE PRECISION NOT NULL,
    temp DOUBLE PRECISION NOT NULL,
    time INT NOT NULL,
    voc DOUBLE PRECISION NOT NULL,
    relayDeviceType VARCHAR NOT NULL,

    UNIQUE(deviceid, time)
);

CREATE TABLE gray_wolfs (
    id SERIAL PRIMARY KEY,
    userId SERIAL NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    deviceid VARCHAR NOT NULL,
    version DOUBLE PRECISION NOT NULL,
    generator VARCHAR NOT NULL,
    api VARCHAR NOT NULL,
    error VARCHAR NOT NULL,
    battery VARCHAR NOT NULL,
    status VARCHAR NOT NULL,
    serialNumber VARCHAR NOT NULL,
    timeStamp TIMESTAMPTZ NOT NULL,

    UNIQUE(deviceid, timeStamp)
);

CREATE TABLE gray_wolf_sensors (
    id SERIAL PRIMARY KEY,
    grayWolfsId SERIAL NOT NULL REFERENCES gray_wolfs(id) ON DELETE CASCADE,
    sensor VARCHAR NOT NULL,
    unit VARCHAR NOT NULL,
    value DOUBLE PRECISION NOT NULL,
    sensorId INT NOT NULL,
    status VARCHAR NOT NULL
);

CREATE TABLE uhoo_auras (
    id SERIAL PRIMARY KEY,
    userId SERIAL NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    deviceid VARCHAR NOT NULL,
    virusIndex INT NOT NULL,
    temperature DOUBLE PRECISION NOT NULL,
    humidity DOUBLE PRECISION NOT NULL,
    pm25 INT NOT NULL,
    tvoc INT NOT NULL,
    co2 INT NOT NULL,
    airPressure DOUBLE PRECISION NOT NULL,
    ozone INT NOT NULL,
    no2 INT NOT NULL,
    pm1 INT NOT NULL,
    pm4 INT NOT NULL,
    pm10 INT NOT NULL,
    ch2o INT NOT NULL,
    light INT NOT NULL,
    sound INT NOT NULL,
    h2s INT NOT NULL,
    no INT NOT NULL,
    so2 INT NOT NULL,
    nh3 INT NOT NULL,
    oxygen INT NOT NULL,
    timestamp INT NOT NULL,
    temperatureUnit VARCHAR NOT NULL,
    tempUnit VARCHAR NOT NULL,
    humidityUnit VARCHAR NOT NULL,
    pm25Unit VARCHAR NOT NULL,
    dustUnit VARCHAR NOT NULL,
    tvocUnit VARCHAR NOT NULL,
    vocUnit VARCHAR NOT NULL,
    co2Unit VARCHAR NOT NULL,
    coUnit VARCHAR NOT NULL,
    airPressureUnit VARCHAR NOT NULL,
    pressureUnit VARCHAR NOT NULL,
    ozoneUnit VARCHAR NOT NULL,
    no2Unit VARCHAR NOT NULL,
    pm1Unit VARCHAR NOT NULL,
    pm4Unit VARCHAR NOT NULL,
    pm10Unit VARCHAR NOT NULL,
    ch2oUnit VARCHAR NOT NULL,
    lightUnit VARCHAR NOT NULL,
    h2sUnit VARCHAR NOT NULL,
    noUnit VARCHAR NOT NULL,
    so2Unit VARCHAR NOT NULL,
    nh3Unit VARCHAR NOT NULL,
    oxygenUnit VARCHAR NOT NULL,

    UNIQUE(deviceid, timestamp)
);