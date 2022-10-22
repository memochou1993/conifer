// @generated automatically by Diesel CLI.

diesel::table! {
    records (id) {
        id -> Varchar,
        url -> Text,
        token -> Nullable<Varchar>,
        expired_at -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
