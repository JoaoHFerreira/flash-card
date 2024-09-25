#[macro_use] extern crate rocket;
use rocket::serde::{json::Json};
use rocket::http::Status;
use chrono::{Utc, Duration};
use self::models::*;
use flash_card::*;
use diesel::prelude::*;
use std::collections::HashMap;
use csv::Reader;
use std::io::Cursor;
use rocket::Data;
use rocket::data::ByteUnit;
use rocket::tokio::io::AsyncReadExt;





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

#[post("/batch_csv_import", data = "<file>")]
async fn batch_csv_import(file: Data<'_>) -> Result<Json<Vec<FlashCard>>, Status> {
    use crate::models::{NewFlashCard, LearningTopic};
    use crate::schema::learning_topic;

    let conn = &mut establish_connection();

    // Load learning topics...
    let all_topics: Vec<LearningTopic> = learning_topic::table
        .load::<LearningTopic>(conn)
        .expect("Error loading learning topics");

    let topic_map: HashMap<String, i32> = all_topics
        .into_iter()
        .map(|topic| (topic.subject, topic.id))
        .collect();

    let current_practice_day = Utc::now().naive_utc();
    let next_practice_day = current_practice_day + Duration::days(1);

    let mut buffer = Vec::new();
    
    // Await the read_to_end operation
    file.open(ByteUnit::default()).read_to_end(&mut buffer)
        .await.map_err(|_| Status::InternalServerError)?;

    let mut rdr = Reader::from_reader(Cursor::new(buffer));
    let mut new_flash_cards: Vec<NewFlashCard> = Vec::new();

    for result in rdr.records() {
        let record = result.map_err(|_| Status::BadRequest)?;
        if record.len() < 3 {
            return Err(Status::BadRequest);
        }

        let question = &record[0];
        let answer = &record[1];
        let subject = &record[2];

        let learning_topic_id = *topic_map.get(subject).ok_or(Status::BadRequest)?;

        let new_flash_card = NewFlashCard {
            question: question.to_string(),
            answer: answer.to_string(),
            learning_topic_id,
            current_practice_day,
            next_practice_day,
        };
        new_flash_cards.push(new_flash_card);
    }


    let inserted_cards = batch_flash_card(conn, new_flash_cards);

    Ok(Json(inserted_cards))
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