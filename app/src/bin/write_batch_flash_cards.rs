use std::collections::HashMap;
use std::process;
use diesel::prelude::*;
use csv::Reader;
use chrono::{Utc, Duration};
use crate::models::NewFlashCard;
use crate::models::LearningTopic;
use crate::schema::learning_topic;
use flash_card::*;

fn main() {
    let connection = &mut establish_connection();
    let file_path = "/app/csv_files/test.csv";

    // Fetch all learning topics
    let all_topics: Vec<LearningTopic> = learning_topic::table
        .load::<LearningTopic>(connection)
        .expect("Error loading learning topics");

    // Create a HashMap for quick lookup
    let topic_map: HashMap<String, i32> = all_topics
        .into_iter()
        .map(|topic| (topic.subject, topic.id))
        .collect();

    
    let current_practice_day = Utc::now().naive_utc();
    let next_practice_day = current_practice_day + Duration::days(1);
    
    let mut reader = Reader::from_path(file_path).expect("Could not open file");
    let mut new_flash_cards: Vec<NewFlashCard> = Vec::new();

    for (index, record) in reader.records().enumerate() {
        let record = record.expect("Invalid record");
        
        let question = &record[0];
        let answer = &record[1];
        let subject = &record[2];

        // Get learning topic id, or break if not exists
        let learning_topic_id = match topic_map.get(subject) {
            Some(&id) => id,
            None => {
                eprintln!("Error: Unknown topic '{}' found in record {}. Terminating program.", subject, index + 1);
                process::exit(1);
            }
        };

        // Prepare new flash card
        let new_flash_card = NewFlashCard {
            question: question.to_string(),
            answer: answer.to_string(),
            learning_topic_id,
            current_practice_day,
            next_practice_day,
        };

        new_flash_cards.push(new_flash_card);
    }

    // Perform batch insert
    batch_flash_card(connection, new_flash_cards);
    println!("Batch was inserted")
    
}
