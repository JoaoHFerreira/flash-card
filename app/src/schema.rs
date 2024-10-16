// @generated automatically by Diesel CLI.

diesel::table! {
    learning_topic (id) {
        id -> Int4,
        subject -> Text,
        created_at -> Nullable<Timestamp>,
    }
}
