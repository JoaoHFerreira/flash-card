#[macro_use] extern crate rocket;
use rocket::serde::{json::Json};
use rocket::http::Status;
use chrono::{Utc, Duration};
use self::models::*;
use flash_card::*;
use diesel::prelude::*;



#[post("/learning_topic", data = "<new_subject>")]
fn add_learning_topic(new_subject: Json<NewLearningTopicRequest>) -> Result<Json<LearningTopic>, Status> {
    let mut conn = &mut establish_connection();

    let learning_topic = create_learning_topic(&mut conn, &new_subject.subject);
    Ok(Json(learning_topic))
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
            ]
        )
        .configure(rocket::Config {
            port: 8000,
            address: [0, 0, 0, 0].into(),
            ..Default::default()
        })
}