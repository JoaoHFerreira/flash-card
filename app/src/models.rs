use diesel::prelude::*;
use crate::schema::{posts};
use crate::schema::{learning_topic};


#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::learning_topic)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct LearningTopic {
    pub id: i32,
    pub subject: String,
}

#[derive(Insertable)]
#[diesel(table_name = learning_topic)]
pub struct NewLearningTopic<'a> {
    pub subject: &'a str,
}