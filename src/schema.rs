// @generated automatically by Diesel CLI.

diesel::table! {
    airthings (id) {
        id -> Int4,
        deviceid -> Varchar,
        userid -> Int4,
        battery -> Float8,
        co2 -> Float8,
        humidity -> Float8,
        pm1 -> Float8,
        pm25 -> Float8,
        pressure -> Float8,
        radonshorttermavg -> Float8,
        temp -> Float8,
        time -> Int4,
        voc -> Float8,
        relaydevicetype -> Varchar,
    }
}

diesel::table! {
    gray_wolf_sensors (id) {
        id -> Int4,
        graywolfsid -> Int4,
        sensor -> Varchar,
        unit -> Varchar,
        value -> Float8,
        sensorid -> Int4,
        status -> Varchar,
    }
}

diesel::table! {
    gray_wolfs (id) {
        id -> Int4,
        userid -> Int4,
        deviceid -> Varchar,
        version -> Float8,
        generator -> Varchar,
        api -> Varchar,
        error -> Varchar,
        battery -> Varchar,
        status -> Varchar,
        serialnumber -> Varchar,
        timestamp -> Timestamptz,
    }
}

diesel::table! {
    uhoo_auras (id) {
        id -> Int4,
        userid -> Int4,
        deviceid -> Varchar,
        virusindex -> Int4,
        temperature -> Float8,
        humidity -> Float8,
        pm25 -> Int4,
        tvoc -> Int4,
        co2 -> Int4,
        airpressure -> Float8,
        ozone -> Int4,
        no2 -> Int4,
        pm1 -> Int4,
        pm4 -> Int4,
        pm10 -> Int4,
        ch2o -> Int4,
        light -> Int4,
        sound -> Int4,
        h2s -> Int4,
        no -> Int4,
        so2 -> Int4,
        nh3 -> Int4,
        oxygen -> Int4,
        timestamp -> Int4,
        temperatureunit -> Varchar,
        tempunit -> Varchar,
        humidityunit -> Varchar,
        pm25unit -> Varchar,
        dustunit -> Varchar,
        tvocunit -> Varchar,
        vocunit -> Varchar,
        co2unit -> Varchar,
        counit -> Varchar,
        airpressureunit -> Varchar,
        pressureunit -> Varchar,
        ozoneunit -> Varchar,
        no2unit -> Varchar,
        pm1unit -> Varchar,
        pm4unit -> Varchar,
        pm10unit -> Varchar,
        ch2ounit -> Varchar,
        lightunit -> Varchar,
        h2sunit -> Varchar,
        nounit -> Varchar,
        so2unit -> Varchar,
        nh3unit -> Varchar,
        oxygenunit -> Varchar,
    }
}

diesel::table! {
    user_airthings (id) {
        id -> Int4,
        userid -> Int4,
        clientsecret -> Varchar,
        clientid -> Varchar,
        groupid -> Varchar,
        deviceids -> Nullable<Array<Nullable<Text>>>,
    }
}

diesel::table! {
    user_gray_wolfs (id) {
        id -> Int4,
        userid -> Int4,
        apikey -> Varchar,
        deviceids -> Nullable<Array<Nullable<Text>>>,
    }
}

diesel::table! {
    user_uhoo_auras (id) {
        id -> Int4,
        userid -> Int4,
        clientsecret -> Varchar,
        deviceids -> Nullable<Array<Nullable<Text>>>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        createdat -> Timestamptz,
        username -> Varchar,
        passwordhash -> Varchar,
        salt -> Varchar,
    }
}

diesel::joinable!(airthings -> users (userid));
diesel::joinable!(gray_wolf_sensors -> gray_wolfs (graywolfsid));
diesel::joinable!(gray_wolfs -> users (userid));
diesel::joinable!(uhoo_auras -> users (userid));
diesel::joinable!(user_airthings -> users (userid));
diesel::joinable!(user_gray_wolfs -> users (userid));
diesel::joinable!(user_uhoo_auras -> users (userid));

diesel::allow_tables_to_appear_in_same_query!(
    airthings,
    gray_wolf_sensors,
    gray_wolfs,
    uhoo_auras,
    user_airthings,
    user_gray_wolfs,
    user_uhoo_auras,
    users,
);
