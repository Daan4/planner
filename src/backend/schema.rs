// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Integer,
        content -> Text,
        created_at -> Timestamp,
        modified_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}
