use diesel::prelude::*;
use rocket::serde::{Serialize, Deserialize};
use crate::schema::{learning_topic};
use crate::schema::{flash_card};
use crate::schema::{historical_acceptances};
use chrono::NaiveDateTime;



#[derive(Queryable, Selectable, Serialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crate::schema::learning_topic)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct LearningTopic {
    pub id: i32,
    pub subject: String,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = learning_topic)]
pub struct NewLearningTopic<'a> {
    pub subject: &'a str,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewLearningTopicRequest {
    pub subject: String,
}


#[derive(Queryable, Selectable, Serialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crate::schema::flash_card)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct FlashCard {
    pub id: i32,
    pub question: String,
    pub answer: String,
    pub learning_topic_id: i32,
    pub current_practice_day: NaiveDateTime,
    pub next_practice_day: NaiveDateTime,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = flash_card)]
pub struct NewFlashCard {
    pub question: String,
    pub answer: String,
    pub learning_topic_id: i32,
    pub current_practice_day: NaiveDateTime,
    pub next_practice_day: NaiveDateTime,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewFlashCardRequest {
    pub question: String,
    pub answer: String,
    pub learning_topic: String,
}


#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::historical_acceptances)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct HistoricalAcceptances {
    pub id: i32,
    pub flash_card_id: i32,
    pub answer_rate: i32,
    pub given_answer: String,
    pub test_date: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = historical_acceptances)]
pub struct NewHistoricalAcceptances {
    pub flash_card_id: i32,
    pub answer_rate: i32,
    pub given_answer: String,
    pub test_date: NaiveDateTime,
}
