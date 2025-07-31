// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Text,
        title -> Text,
        important -> Bool,
        urgent -> Bool,
        role -> Nullable<Text>,
        content -> Nullable<Text>,
        completed -> Bool,
        scheduled_date -> Nullable<Date>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}
