#[macro_use] extern crate rocket;
use rocket::serde::json::Json;
use rocket::http::Status;
use chrono::{Utc, Duration};
use self::models::*;
use flash_card::*;
use diesel::prelude::*;
use serde::Deserialize;
use rocket::http::ContentType;
use rocket::post;
use rocket::data::{Data, ToByteUnit};
use serde::Serialize;
use std::collections::HashMap;
use std::process;



#[post("/batch_csv_import", data = "<csv_data>")]
async fn batch_csv_import(_content_type: &ContentType, csv_data: Data<'_>) {
    use crate::schema::learning_topic;

    let bytes = csv_data.open(10.megabytes()).into_bytes().await
        .expect("Failed to read request data");

    let csv_string = String::from_utf8_lossy(&bytes.value);

    let connection = &mut establish_connection();
    let all_topics: Vec<LearningTopic> = learning_topic::table
        .load::<LearningTopic>(connection)
        .expect("Error loading learning topics");

    let topic_map: HashMap<String, i32> = all_topics
        .into_iter()
        .map(|topic| (topic.subject, topic.id))
        .collect();

    
    let current_practice_day = Utc::now().naive_utc();
    let next_practice_day = current_practice_day + Duration::days(1);


    let mut new_flash_cards: Vec<NewFlashCard> = Vec::new();
    let lines = csv_string.split("\n");
    for line in lines {
        let fields: Vec<&str> = line.split(",").collect();
        if fields.len() == 3 {

            let subject = fields[2].trim();

            let learning_topic_id = match topic_map.get(subject) {
                Some(&id) => id,
                None => {
                    eprintln!("Error: Unknown topic '{}' Terminating program.", subject);
                    process::exit(1);
                }
            };
        
            let new_flash_card = NewFlashCard {
                question: fields[0].to_string(),
                answer: fields[1].to_string(),
                learning_topic_id,
                current_practice_day,
                next_practice_day,
            };

            new_flash_cards.push(new_flash_card);


        } else {
            println!("Invalid record: {}", line);
        }
    }

    batch_flash_card(connection, new_flash_cards);
    println!("Batch was inserted")

}

#[post("/learning_topic", data = "<new_subject>")]
fn add_learning_topic(new_subject: Json<NewLearningTopicRequest>) -> Result<Json<LearningTopic>, Status> {
    let mut conn = &mut establish_connection();

    let learning_topic = create_learning_topic(&mut conn, &new_subject.subject);
    Ok(Json(learning_topic))
}

#[get("/learning_topic")]
fn get_learning_topics() -> Result<Json<Vec<LearningTopic>>, Status> {
    use crate::schema::learning_topic;
    use crate::models::LearningTopic;

    let conn = &mut establish_connection();

    let learn_topics = learning_topic::table
    .load::<LearningTopic>(conn)
    .expect("Error loading learning topics");

    
    Ok(Json(learn_topics))
}

#[post("/flash_card", data = "<new_flash_card>")]
fn add_flash_card(new_flash_card: Json<NewFlashCardRequest>) -> Result<Json<FlashCard>, Status> {
    use crate::schema::learning_topic;
    use crate::models::LearningTopic;

    let conn = &mut establish_connection();

    let learn_topic = learning_topic::table
    .filter(learning_topic::subject.eq(new_flash_card.learning_topic.clone()))
    .load::<LearningTopic>(conn)
    .expect("Error loading learning topics");

    let current_practice_day = Utc::now().naive_utc();
    let next_practice_day = current_practice_day + Duration::days(1);

    let flash_card = create_flash_card(
        conn,
        new_flash_card.question.clone(),
        new_flash_card.answer.clone(),
        learn_topic[0].id,
        current_practice_day,
        next_practice_day,
    );
    
    Ok(Json(flash_card))
}


#[get("/flash_card")]
fn get_flash_cards() -> Result<Json<Vec<FlashCard>>, Status> {
    use crate::models::FlashCard;
    use crate::schema::flash_card;

    let conn = &mut establish_connection();

    let flash_cards = flash_card::table
    .load::<FlashCard>(conn)
    .expect("Error loading learning topics");

    Ok(Json(flash_cards))
}

#[get("/")]
fn hello() -> &'static str {
    "Hello, World!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            hello, 
            add_learning_topic,
            add_flash_card,
            get_flash_cards,
            get_learning_topics,
            batch_csv_import,
            ]
        )
        .configure(rocket::Config {
            port: 8000,
            address: [0, 0, 0, 0].into(),
            ..Default::default()
        })
}