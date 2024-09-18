pub mod models;
pub mod schema;

use diesel::prelude::*;
use chrono::NaiveDateTime;
use dotenvy::dotenv;
use std::env;
use self::models::{NewPost, Post};
use self::models::{LearningTopic, NewLearningTopic};
use self::models::{PracticeSchedule, NewPracticeSchedule};


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


pub fn create_post(conn: &mut PgConnection, title: &str, body: &str) -> Post {
    use crate::schema::posts;

    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
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