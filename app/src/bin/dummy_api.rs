#[macro_use] extern crate rocket;
use rocket::serde::{json::Json, Deserialize};
use rocket::http::Status;
use self::models::*;
use flash_card::*;


#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct NewLearningTopicRequest {
    subject: String,
}

#[post("/learning_topic", data = "<new_topic>")]
fn add_learning_topic(new_topic: Json<NewLearningTopicRequest>) -> Result<Json<LearningTopic>, Status> {
    let mut conn = &mut establish_connection();

    let learning_topic = create_learning_topic(&mut conn, &new_topic.subject);
    Ok(Json(learning_topic))
}


#[get("/")]
fn hello() -> &'static str {
    "Hello, World!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello, add_learning_topic])
        .configure(rocket::Config {
            port: 8000,
            address: [0, 0, 0, 0].into(),
            ..Default::default()
        })
}