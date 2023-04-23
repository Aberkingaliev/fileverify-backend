// @generated automatically by Diesel CLI.

diesel::table! {
    advance_options (id) {
        id -> Uuid,
        validation_rule_id -> Uuid,
        extension_id -> Int4,
        is_email_validate -> Bool,
    }
}

diesel::table! {
    extension_list (id) {
        id -> Int4,
        extension -> Varchar,
    }
}

diesel::table! {
    keywords (id) {
        id -> Uuid,
        advance_option_id -> Uuid,
        keyword -> Varchar,
    }
}

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

diesel::table! {
    validation_rules (id) {
        id -> Uuid,
        title -> Varchar,
        min_size -> Int8,
        max_size -> Int8,
        allowed_extension_id -> Int4,
    }
}

diesel::joinable!(advance_options -> extension_list (extension_id));
diesel::joinable!(advance_options -> validation_rules (validation_rule_id));
diesel::joinable!(keywords -> advance_options (advance_option_id));
diesel::joinable!(tokens -> users (user_id));
diesel::joinable!(validation_rules -> extension_list (allowed_extension_id));

diesel::allow_tables_to_appear_in_same_query!(
    advance_options,
    extension_list,
    keywords,
    tokens,
    users,
    validation_rules,
);
