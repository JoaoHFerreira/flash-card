use std::io::stdin;
use crate::schema::learning_topic;
use crate::models::LearningTopic;
use flash_card::*;
use chrono::{Utc, Duration};
use diesel::prelude::*;

fn main() {
    let connection = &mut establish_connection();
    let mut question = String::new();
    let mut answer = String::new();
    let mut subject = String::new();

    println!("Type the question?");
    stdin().read_line(&mut question).unwrap();
    let question = question.trim_end();

    println!("Type the answer!");
    stdin().read_line(&mut answer).unwrap();
    let answer = answer.trim_end();

    println!("Type the subject!");
    stdin().read_line(&mut subject).unwrap();
    let subject = subject.trim_end();

    let learn_topic = learning_topic::table
        .filter(learning_topic::subject.eq(subject))
        .load::<LearningTopic>(connection)
        .expect("Error loading learning topics");

    let current_practice_day = Utc::now().naive_utc();
    let next_practice_day = current_practice_day + Duration::days(1);
    let practice_schedule = create_practice_schedule(
        connection,
        current_practice_day,
        next_practice_day,
    );

    let flash_card = create_flash_card(
        connection,
        question.to_string(),
        answer.to_string(),
        learn_topic[0].id,
        practice_schedule.id,
    );

    println!("\nCreated flash_card  with id {}", flash_card.id);
}