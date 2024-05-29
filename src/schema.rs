// @generated automatically by Diesel CLI.

diesel::table! {
    modules (hash) {
        hash -> Text,
        binary -> Binary,
        title -> Text,
        description -> Nullable<Text>,
    }
}
