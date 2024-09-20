use self::models::*;
use diesel::prelude::*;
use flash_card::*;
use std::io;
use chrono::{Utc, Duration};

fn main() {
    let connection = &mut establish_connection();

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
        let test_date = Utc::now().naive_utc();

        let historical_register = create_historical_acceptances(
            connection,
            card.id,
            rating,
            user_answer.trim().to_string(),
            test_date,
        );

        let current_practice_day = card.next_practice_day;
        let next_practice_day = current_practice_day + Duration::days(1);
        
        let _ = update_flash_card_by_id(
            connection,
            card.id,
            current_practice_day,
            next_practice_day,
        );

        println!("\nCreated historic with id {}", historical_register.id);
        println!("\nCreated historic with id {}", card.current_practice_day);

    }
}

fn get_daily_flash_cards() -> Vec<FlashCard> {
    use self::schema::flash_card::dsl::*;
    use diesel::dsl::sql;
    use diesel::dsl::now;

    let connection = &mut establish_connection();
    let limit = 1;

    flash_card
        .filter(current_practice_day.le(now))
        .order_by(sql::<diesel::sql_types::Text>("RANDOM()"))
        .limit(limit)
        .select(FlashCard::as_select())
        .load(connection)
        .expect("Error loading inner join")
}