diesel::table! {
    tokens (id) {
        id -> Uuid,
        user_id -> Uuid,
        refresh_token -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        name -> Varchar,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        is_activated -> Bool,
        activation_link -> Varchar,
    }
}

diesel::joinable!(tokens -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(tokens, users);
