use self::models::*;
use diesel::prelude::*;
use flash_card::*;


fn main() {
    use self::schema::flash_card::dsl::*;
    use self::schema::practice_schedule::dsl::*;
    use diesel::dsl::sql;

    let connection = &mut establish_connection();
    let limit = 30;
    
    let results = flash_card
        .inner_join(practice_schedule)
        .order_by(sql::<diesel::sql_types::Text>("RANDOM()"))
        .limit(limit)
        .select(FlashCard::as_select())
        .load::<FlashCard>(connection)
        .expect("Error loading inner join");

    for card in results {
        println!("ID: {}", card.id);
        println!("Question: {}", card.question);
        println!("Answer: {}", card.answer);
        println!("Practice Schedule ID: {}", card.practice_schedule_id);
        println!("--------------------");
    }
}
