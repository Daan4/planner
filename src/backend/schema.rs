// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Text,
        content -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}
