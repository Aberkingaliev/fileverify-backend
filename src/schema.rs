// @generated automatically by Diesel CLI.

diesel::table! {
    advance_options (id) {
        id -> Uuid,
        validation_rule_id -> Uuid,
        is_email_validate -> Bool,
    }
}

diesel::table! {
    extension_for_rules (id) {
        id -> Uuid,
        validation_rule_id -> Uuid,
        advance_option_id -> Nullable<Uuid>,
        extension_id -> Int4,
    }
}

diesel::table! {
    extension_list (id) {
        id -> Int4,
        extension -> Varchar,
    }
}

diesel::table! {
    keywords_for_options (id) {
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
    }
}

diesel::joinable!(advance_options -> validation_rules (validation_rule_id));
diesel::joinable!(extension_for_rules -> extension_list (extension_id));
diesel::joinable!(extension_for_rules -> validation_rules (validation_rule_id));
diesel::joinable!(keywords_for_options -> advance_options (advance_option_id));
diesel::joinable!(tokens -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    advance_options,
    extension_for_rules,
    extension_list,
    keywords_for_options,
    tokens,
    users,
    validation_rules,
);
