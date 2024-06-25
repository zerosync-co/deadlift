// @generated automatically by Diesel CLI.

diesel::table! {
    modules (hash) {
        hash -> Text,
        binary -> Binary,
        title -> Text,
        description -> Nullable<Text>,
        subject -> Text,
    }
}

diesel::table! {
    workflow_modules (id) {
        id -> Integer,
        workflow_id -> Integer,
        module_hash -> Text,
        parent_workflow_module_id -> Nullable<Integer>,
    }
}

diesel::table! {
    workflows (id) {
        id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
    }
}

diesel::joinable!(workflow_modules -> modules (module_hash));
diesel::joinable!(workflow_modules -> workflows (workflow_id));

diesel::allow_tables_to_appear_in_same_query!(modules, workflow_modules, workflows,);
