-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    createdAt TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    username VARCHAR NOT NULL UNIQUE,
    passwordHash VARCHAR NOT NULL,
    salt VARCHAR NOT NULL,
  
    age INT NOT NULL,
    race TEXT[],
    otherRace VARCHAR,
    gender VARCHAR,
    otherGender VARCHAR,
    employed BOOLEAN NOT NULL,
    levelOfEducation VARCHAR NOT NULL,

    -- How often have you felt that you were unable to control the important things in your life
    unableToControlImportantThings INT NOT NULL,
    -- How often have you felt confident about your ability to handle your personal problems
    oftenFeltConfidentHandlePersonalProblems INT NOT NULL,
    -- How often have you felt that things were going your way
    feltThingsGoYourWay INT NOT NULL,
    -- How often have you felt difficulties were piling up so high that you could not overcome them?
    feltDifficultiesPilingUp INT NOT NULL,

    -- I tend to bounce back quickly after hard times 
    bounceBackQuickly INT NOT NULL,
    -- I have a hard time making it through stressfull events
    hardTimeStressfullEvents INT NOT NULL,
    -- It does not take me long to recover from a stressfull event
    longRecoveryTime INT NOT NULL,
    -- It is hard for me to snap back when something bad happens
    hardToSnapBack INT NOT NULL,
    -- I usually come through difficult times with little trouble
    comeThroughDifficultTimes INT NOT NULL,
    -- I tend to take a long time to get over a set-backs in life
    longTimeGetOverSetBacks INT NOT NULL
);

CREATE TABLE user_keychains (
    id SERIAL PRIMARY KEY,
    userId SERIAL NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    apiKey VARCHAR NOT NULL,
    deviceMacs TEXT[],
    UNIQUE(userId)
);

CREATE TABLE keychains (
    id SERIAL PRIMARY KEY,
    userId SERIAL NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    time TIMESTAMPTZ NOT NULL,
    devMac VARCHAR NOT NULl,
    voc DOUBLE PRECISION,
    pm1 DOUBLE PRECISION,
    pm25 DOUBLE PRECISION,
    pm10 DOUBLE PRECISION,
    t DOUBLE PRECISION,
    h DOUBLE PRECISION,
    p DOUBLE PRECISION,
    lat DOUBLE PRECISION,
    lon DOUBLE PRECISION,

    UNIQUE(userId, time)
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

CREATE TABLE user_uhoo_business (
    id SERIAL PRIMARY KEY,
    userId SERIAL NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    clientSecret VARCHAR NOT NULL,
    deviceIds TEXT[],
    UNIQUE(userId)
);

CREATE TABLE user_uhoo_homes (
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

CREATE TABLE uhoo_business (
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

CREATE TABLE uhoo_homes (
    id SERIAL PRIMARY KEY,
    userId SERIAL NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    deviceid VARCHAR NOT NULL,
    virusIndex INT NOT NULL,
    temperature DOUBLE PRECISION NOT NULL,
    humidity DOUBLE PRECISION NOT NULL,
    pm25 INT NOT NULL,
    tvoc INT NOT NULL,
    co2 INT NOT NULL,
    co INT NOT NULL,
    airPressure DOUBLE PRECISION NOT NULL,
    ozone INT NOT NULL,
    no2 INT NOT NULL,
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

CREATE TABLE fitbit_heartrates (
    id SERIAL PRIMARY KEY,
    userId SERIAL NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    timestamp TIMESTAMPTZ NOT NULL,
    heartrate INT NOT NULL,

    UNIQUE(userId, timestamp)
);

CREATE TABLE fitbit_accelerometers (
    id SERIAL PRIMARY KEY,
    userId SERIAL NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    timestamp TIMESTAMPTZ NOT NULL,
    x DOUBLE PRECISION NOT NULL,
    y DOUBLE PRECISION NOT NULL,
    z DOUBLE PRECISION NOT NULL,

    UNIQUE(userId, timestamp)
);

CREATE TABLE fitbit_barometers (
    id SERIAL PRIMARY KEY,
    userId SERIAL NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    timestamp TIMESTAMPTZ NOT NULL,
    pressure INT NOT NULL,

    UNIQUE(userId, timestamp)
);

CREATE TABLE fitbit_gyroscopes (
    id SERIAL PRIMARY KEY,
    userId SERIAL NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    timestamp TIMESTAMPTZ NOT NULL,
    x DOUBLE PRECISION NOT NULL,
    y DOUBLE PRECISION NOT NULL,
    z DOUBLE PRECISION NOT NULL,

    UNIQUE(userId, timestamp)
);

CREATE TABLE fitbit_orientations (
    id SERIAL PRIMARY KEY,
    userId SERIAL NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    timestamp TIMESTAMPTZ NOT NULL,
    x DOUBLE PRECISION NOT NULL,
    y DOUBLE PRECISION NOT NULL,
    z DOUBLE PRECISION NOT NULL,
    scalar DOUBLE PRECISION NOT NULL,

    UNIQUE(userId, timestamp)
);

CREATE TABLE gis_locations (
    id SERIAL PRIMARY KEY,
    userId SERIAL NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    createdAt TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    timestamp INT NOT NULL,
    checked BOOLEAN NOT NULL DEFAULT FALSE,

    latitude DOUBLE PRECISION NOT NULL,
    longitude DOUBLE PRECISION NOT NULL,

    UNIQUE(userId, timestamp)
);

CREATE TABLE daily_surveys (
    id SERIAL PRIMARY KEY,
    userId SERIAL NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    createdAt TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- How often have you felt that you were unable to control the important things in your life? 
    unableToControlImportantThings INT NOT NULL,
    -- How often have you felt confident about your ability to handle your personal problems? 
    oftenFeltConfidentHandlePersonalProblems INT NOT NULL,
    -- How often have you felt that things were going your way? 
    feelThingsAreGoingMyWay INT NOT NULL,
    -- How often have you felt difficulties were piling up so high that you could not overcome them? 
    feelDifficultiesPilingCannotOvercome INT NOT NULL,

    -- To what extent are you currently experiencing stress with regard to the following topics
    stressYourHealth INT NOT NULL,
    stressYourFinances INT NOT NULL,
    stressFamilySocialRelationships INT NOT NULL,
    stressYourWord INT NOT NULL
);

CREATE TABLE hourly_surveys (
    id SERIAL PRIMARY KEY,
    userId SERIAL NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    createdAt TIMESTAMPTZ NOT NULL DEFAULT NOW(),
 
    currentStress INT NOT NULL,
    timestamp INT NOT NULL,
    location VARCHAR NOT NULL
);

CREATE TABLE vehicle_measurements (
    id SERIAL PRIMARY KEY,
    userId SERIAL NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    createdAt TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    date VARCHAR,
    time VARCHAR,
    timestamp INT,
    timestamp_iso8601 TIMESTAMPTZ,
    speed DOUBLE PRECISION,
    steeringAngle DOUBLE PRECISION,
    distance DOUBLE PRECISION,
    velocity DOUBLE PRECISION,
    accelerationPressure DOUBLE PRECISION,
    brakePressure DOUBLE PRECISION,
    lane DOUBLE PRECISION,
    scenarioNumber DOUBLE PRECISION
);