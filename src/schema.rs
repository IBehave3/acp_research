// @generated automatically by Diesel CLI.

diesel::table! {
    airthings (id) {
        id -> Int4,
        deviceid -> Varchar,
        userid -> Int4,
        battery -> Nullable<Float8>,
        co2 -> Nullable<Float8>,
        humidity -> Nullable<Float8>,
        pm1 -> Nullable<Float8>,
        pm25 -> Nullable<Float8>,
        pressure -> Nullable<Float8>,
        radonshorttermavg -> Nullable<Float8>,
        temp -> Nullable<Float8>,
        time -> Nullable<Int4>,
        voc -> Nullable<Float8>,
        relaydevicetype -> Nullable<Varchar>,
    }
}

diesel::table! {
    daily_surveys (id) {
        id -> Int4,
        userid -> Int4,
        createdat -> Timestamptz,
        unabletocontrolimportantthings -> Int4,
        oftenfeltconfidenthandlepersonalproblems -> Int4,
        feelthingsaregoingmyway -> Int4,
        feeldifficultiespilingcannotovercome -> Int4,
        stressyourhealth -> Int4,
        stressyourfinances -> Int4,
        stressfamilysocialrelationships -> Int4,
        stressyourword -> Int4,
    }
}

diesel::table! {
    fitbit_accelerometers (id) {
        id -> Int4,
        userid -> Int4,
        timestamp -> Timestamptz,
        x -> Float8,
        y -> Float8,
        z -> Float8,
    }
}

diesel::table! {
    fitbit_barometers (id) {
        id -> Int4,
        userid -> Int4,
        timestamp -> Timestamptz,
        pressure -> Int4,
    }
}

diesel::table! {
    fitbit_gyroscopes (id) {
        id -> Int4,
        userid -> Int4,
        timestamp -> Timestamptz,
        x -> Float8,
        y -> Float8,
        z -> Float8,
    }
}

diesel::table! {
    fitbit_heartrates (id) {
        id -> Int4,
        userid -> Int4,
        timestamp -> Timestamptz,
        heartrate -> Int4,
    }
}

diesel::table! {
    fitbit_orientations (id) {
        id -> Int4,
        userid -> Int4,
        timestamp -> Timestamptz,
        x -> Float8,
        y -> Float8,
        z -> Float8,
        scalar -> Float8,
    }
}

diesel::table! {
    gis_locations (id) {
        id -> Int4,
        userid -> Int4,
        createdat -> Timestamptz,
        timestamp -> Int4,
        checked -> Bool,
        latitude -> Float8,
        longitude -> Float8,
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
    hourly_surveys (id) {
        id -> Int4,
        userid -> Int4,
        createdat -> Timestamptz,
        currentstress -> Int4,
        timestamp -> Int4,
        location -> Varchar,
    }
}

diesel::table! {
    keychains (id) {
        id -> Int4,
        userid -> Int4,
        time -> Timestamptz,
        devmac -> Varchar,
        voc -> Nullable<Float8>,
        pm1 -> Nullable<Float8>,
        pm25 -> Nullable<Float8>,
        pm10 -> Nullable<Float8>,
        t -> Nullable<Float8>,
        h -> Nullable<Float8>,
        p -> Nullable<Float8>,
        lat -> Nullable<Float8>,
        lon -> Nullable<Float8>,
    }
}

diesel::table! {
    uhoo_business (id) {
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
    uhoo_homes (id) {
        id -> Int4,
        userid -> Int4,
        deviceid -> Varchar,
        virusindex -> Int4,
        temperature -> Float8,
        humidity -> Float8,
        pm25 -> Int4,
        tvoc -> Int4,
        co2 -> Int4,
        co -> Int4,
        airpressure -> Float8,
        ozone -> Int4,
        no2 -> Int4,
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
    user_fitbit_two (id) {
        id -> Int4,
        userid -> Int4,
        verificationcode -> Varchar,
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
    user_keychains (id) {
        id -> Int4,
        userid -> Int4,
        apikey -> Varchar,
        devicemacs -> Nullable<Array<Nullable<Text>>>,
    }
}

diesel::table! {
    user_locations (id) {
        id -> Int4,
        userid -> Int4,
        timestamp -> Int4,
        location -> Varchar,
    }
}

diesel::table! {
    user_uhoo_business (id) {
        id -> Int4,
        userid -> Int4,
        clientsecret -> Varchar,
        deviceids -> Nullable<Array<Nullable<Text>>>,
    }
}

diesel::table! {
    user_uhoo_homes (id) {
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
        age -> Int4,
        race -> Nullable<Array<Nullable<Text>>>,
        otherrace -> Nullable<Varchar>,
        gender -> Nullable<Varchar>,
        othergender -> Nullable<Varchar>,
        employed -> Bool,
        levelofeducation -> Varchar,
        unabletocontrolimportantthings -> Int4,
        oftenfeltconfidenthandlepersonalproblems -> Int4,
        feltthingsgoyourway -> Int4,
        feltdifficultiespilingup -> Int4,
        bouncebackquickly -> Int4,
        hardtimestressfullevents -> Int4,
        longrecoverytime -> Int4,
        hardtosnapback -> Int4,
        comethroughdifficulttimes -> Int4,
        longtimegetoversetbacks -> Int4,
    }
}

diesel::table! {
    vehicle_measurements (id) {
        id -> Int4,
        userid -> Int4,
        createdat -> Timestamptz,
        date -> Nullable<Varchar>,
        time -> Nullable<Varchar>,
        timestamp -> Nullable<Int4>,
        timestamp_iso8601 -> Nullable<Timestamptz>,
        speed -> Nullable<Float8>,
        steeringangle -> Nullable<Float8>,
        distance -> Nullable<Float8>,
        velocity -> Nullable<Float8>,
        accelerationpressure -> Nullable<Float8>,
        brakepressure -> Nullable<Float8>,
        lane -> Nullable<Float8>,
        scenarionumber -> Nullable<Float8>,
    }
}

diesel::joinable!(airthings -> users (userid));
diesel::joinable!(daily_surveys -> users (userid));
diesel::joinable!(fitbit_accelerometers -> users (userid));
diesel::joinable!(fitbit_barometers -> users (userid));
diesel::joinable!(fitbit_gyroscopes -> users (userid));
diesel::joinable!(fitbit_heartrates -> users (userid));
diesel::joinable!(fitbit_orientations -> users (userid));
diesel::joinable!(gis_locations -> users (userid));
diesel::joinable!(gray_wolf_sensors -> gray_wolfs (graywolfsid));
diesel::joinable!(gray_wolfs -> users (userid));
diesel::joinable!(hourly_surveys -> users (userid));
diesel::joinable!(keychains -> users (userid));
diesel::joinable!(uhoo_business -> users (userid));
diesel::joinable!(uhoo_homes -> users (userid));
diesel::joinable!(user_airthings -> users (userid));
diesel::joinable!(user_fitbit_two -> users (userid));
diesel::joinable!(user_gray_wolfs -> users (userid));
diesel::joinable!(user_keychains -> users (userid));
diesel::joinable!(user_locations -> users (userid));
diesel::joinable!(user_uhoo_business -> users (userid));
diesel::joinable!(user_uhoo_homes -> users (userid));
diesel::joinable!(vehicle_measurements -> users (userid));

diesel::allow_tables_to_appear_in_same_query!(
    airthings,
    daily_surveys,
    fitbit_accelerometers,
    fitbit_barometers,
    fitbit_gyroscopes,
    fitbit_heartrates,
    fitbit_orientations,
    gis_locations,
    gray_wolf_sensors,
    gray_wolfs,
    hourly_surveys,
    keychains,
    uhoo_business,
    uhoo_homes,
    user_airthings,
    user_fitbit_two,
    user_gray_wolfs,
    user_keychains,
    user_locations,
    user_uhoo_business,
    user_uhoo_homes,
    users,
    vehicle_measurements,
);
