use flash_card::*;
use std::io::{stdin};

fn main() {
    let connection = &mut establish_connection();

    let mut subject = String::new();

    println!("What would you like your subject to be?");
    stdin().read_line(&mut subject).unwrap();
    let subject = subject.trim_end().to_lowercase();

    let learning_topic = create_learning_topic(connection, &subject);
    println!("\nSaved draft {subject} with id {}", learning_topic.id);
}


#[cfg(windows)]
const EOF: &str = "CTRL+Z";