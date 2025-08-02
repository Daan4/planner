// @generated automatically by Diesel CLI.

diesel::table! {
    backlogs (id) {
        id -> Text,
        name -> Text,
    }
}

diesel::table! {
    roles (id) {
        id -> Text,
        name -> Text,
    }
}

diesel::table! {
    tasks (id) {
        id -> Text,
        title -> Text,
        important -> Bool,
        urgent -> Bool,
        content -> Nullable<Text>,
        completed -> Bool,
        role_id -> Nullable<Text>,
        backlog_id -> Nullable<Text>,
        scheduled_date -> Nullable<Date>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(tasks -> backlogs (backlog_id));
diesel::joinable!(tasks -> roles (role_id));

diesel::allow_tables_to_appear_in_same_query!(
    backlogs,
    roles,
    tasks,
);
