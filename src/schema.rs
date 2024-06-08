// @generated automatically by Diesel CLI.

diesel::table! {
    modules (hash) {
        hash -> Text,
        binary -> Binary,
        title -> Text,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    workflows (id) {
        id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        pipeline -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(modules, workflows,);
