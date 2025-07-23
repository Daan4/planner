// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Integer,
        content -> Text,
        date_created -> Timestamp,
        date_modified -> Nullable<Timestamp>,
        date_deleted -> Nullable<Timestamp>,
    }
}
