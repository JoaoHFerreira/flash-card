use self::models::*;
use diesel::prelude::*;
use flash_card::*;
use std::io;

fn main() {
    let results = get_daily_flash_cards();
    for card in results {
        println!("ID: {}", card.id);
        println!("Question: {}", card.question);
        
        let mut user_answer = String::new();
        println!("Enter your answer:");
        io::stdin().read_line(&mut user_answer).expect("Failed to read line");
        
        println!("Your answer: {}", user_answer.trim());
        println!("Correct answer: {}", card.answer);
        
        let mut user_rating = String::new();
        println!("Classify your answer from 1 to 5 (1 = terrible, 5 = amazing):");
        io::stdin().read_line(&mut user_rating).expect("Failed to read line");
        
        let rating: i32 = user_rating.trim_end().parse::<i32>().unwrap();
        
        println!("Your rating: {}", rating);
        println!("Practice Schedule ID: {}", card.practice_schedule_id);
        println!("--------------------");
    }
}

fn get_daily_flash_cards() -> Vec<FlashCard> {
    use self::schema::flash_card::dsl::*;
    use self::schema::practice_schedule::dsl::*;
    use diesel::dsl::sql;
    use diesel::dsl::now;

    let connection = &mut establish_connection();
    let limit = 30;

    flash_card
        .inner_join(practice_schedule)
        .filter(current_practice_day.le(now))
        .order_by(sql::<diesel::sql_types::Text>("RANDOM()"))
        .limit(limit)
        .select(FlashCard::as_select())
        .load(connection)
        .expect("Error loading inner join")
}