// @generated automatically by Diesel CLI.

diesel::table! {
    flash_card (id) {
        id -> Int4,
        question -> Text,
        answer -> Text,
        learning_topic_id -> Int4,
        current_practice_day -> Timestamp,
        next_practice_day -> Timestamp,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    historical_acceptances (id) {
        id -> Int4,
        flash_card_id -> Int4,
        answer_rate -> Int4,
        given_answer -> Text,
        test_date -> Timestamp,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    learning_topic (id) {
        id -> Int4,
        subject -> Text,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(flash_card -> learning_topic (learning_topic_id));

diesel::allow_tables_to_appear_in_same_query!(
    flash_card,
    historical_acceptances,
    learning_topic,
);
