// @generated automatically by Diesel CLI.

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

diesel::joinable!(user_airthings -> users (userid));
diesel::joinable!(user_gray_wolfs -> users (userid));
diesel::joinable!(user_uhoo_auras -> users (userid));

diesel::allow_tables_to_appear_in_same_query!(
    user_airthings,
    user_gray_wolfs,
    user_uhoo_auras,
    users,
);
