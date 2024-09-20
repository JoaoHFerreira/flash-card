pub mod models;
pub mod schema;

use diesel::prelude::*;
use chrono::NaiveDateTime;
use dotenvy::dotenv;
use std::env;
use self::models::{LearningTopic, NewLearningTopic};
use self::models::{PracticeSchedule, NewPracticeSchedule};
use self::models::{FlashCard, NewFlashCard};
use self::models::{HistoricalAcceptances, NewHistoricalAcceptances};


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


pub fn create_learning_topic(conn: &mut PgConnection, subject: &str) -> LearningTopic {
    use crate::schema::learning_topic;

    let new_learning_topic = NewLearningTopic { subject };

    diesel::insert_into(learning_topic::table)
        .values(&new_learning_topic)
        .returning(LearningTopic::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn create_practice_schedule(
    conn: &mut PgConnection,
    current_practice_day: NaiveDateTime,
    next_practice_day: NaiveDateTime,
) -> PracticeSchedule {
    use crate::schema::practice_schedule;

    let new_practice_schedule = NewPracticeSchedule { current_practice_day, next_practice_day };

    diesel::insert_into(practice_schedule::table)
        .values(&new_practice_schedule)
        .returning(PracticeSchedule::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn create_flash_card(
    conn: &mut PgConnection,
    question: String,
    answer: String,
    learning_topic_id: i32,
    practice_schedule_id: i32,
) -> FlashCard {
    use crate::schema::flash_card;

    let new_flash_card =NewFlashCard { 
        question,
        answer,
        learning_topic_id,
        practice_schedule_id 
    };

    diesel::insert_into(flash_card::table)
        .values(&new_flash_card)
        .returning(FlashCard::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn batch_flash_card(
    conn: &mut PgConnection,
    new_flash_cards: Vec<NewFlashCard>
) -> Vec<FlashCard> {
    use crate::schema::flash_card;

    diesel::insert_into(flash_card::table)
        .values(&new_flash_cards)
        .returning(FlashCard::as_returning())
        .get_results(conn)
        .expect("Error saving new flash cards")
}

pub fn create_historical_acceptances(
    conn: &mut PgConnection,
    flash_card_id: i32,
    answer_rate: i32,
    given_answer: String,
    test_date: NaiveDateTime,
) -> HistoricalAcceptances {
    use crate::schema::historical_acceptances;
    let new_historical_acceptance = NewHistoricalAcceptances {
        flash_card_id,
        answer_rate,
        given_answer,
        test_date,
    };
    diesel::insert_into(historical_acceptances::table)
        .values(&new_historical_acceptance)
        .returning(HistoricalAcceptances::as_returning())
        .get_result(conn)
        .expect("Error saving new historical acceptance")
}