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
use csv;



#[derive(Debug)] // Added for printing the struct
struct Record {
    question: String,
    answer: String,
    subject: String,
}

#[post("/batch_csv_import", data = "<csv_data>")]
async fn batch_csv_import(_content_type: &ContentType, csv_data: Data<'_>) {
    let bytes = csv_data.open(10.megabytes()).into_bytes().await
        .expect("Failed to read request data");

    let csv_string = String::from_utf8_lossy(&bytes.value);

    println!("Raw CSV data: {}", csv_string); // Print the raw data

    let mut records = Vec::new();

    let lines = csv_string.split("\n");
    for line in lines {
        let fields: Vec<&str> = line.split(",").collect();
        if fields.len() == 3 {
            records.push(Record {
                question: fields[0].to_string(),
                answer: fields[1].to_string(),
                subject: fields[2].to_string(),
            });
        } else {
            println!("Invalid record: {}", line);
        }
    }

    // let records: Vec<Record> = csv_string
    // .split('\n')
    // .map(|line| line.split(',').collect::<Vec<&str>>())
    // .filter(|fields| fields.len() == 3)
    // .map(|fields| Record {
    //     question: fields[0].to_string(),
    //     answer: fields[1].to_string(),
    //     subject: fields[2].to_string(),
    // })
    // .collect();

    // Print the records
    for record in &records {
        println!("{:?}", record);
    }
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