// @generated automatically by Diesel CLI.

diesel::table! {
    flash_card (id) {
        id -> Int4,
        question -> Text,
        answer -> Text,
        learning_topic_id -> Int4,
        practice_schedule_id -> Int4,
    }
}

diesel::table! {
    historical_acceptances (id) {
        id -> Int4,
        flash_card_id -> Int4,
        is_right -> Bool,
        test_date -> Timestamp,
    }
}

diesel::table! {
    learning_topic (id) {
        id -> Int4,
        subject -> Text,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::table! {
    practice_schedule (id) {
        id -> Int4,
        current_practice_day -> Timestamp,
        next_practice_day -> Timestamp,
    }
}

diesel::joinable!(flash_card -> learning_topic (learning_topic_id));
diesel::joinable!(flash_card -> practice_schedule (practice_schedule_id));
diesel::joinable!(historical_acceptances -> flash_card (flash_card_id));

diesel::allow_tables_to_appear_in_same_query!(
    flash_card,
    historical_acceptances,
    learning_topic,
    posts,
    practice_schedule,
);
