use self::models::*;
use diesel::prelude::*;
use flash_card::*;


fn main(){
    use self::schema::flash_card::dsl::*;

    let connection = &mut establish_connection();

    let results = flash_card
        .limit(30)
        .select(FlashCard::as_select())
        .load(connection)
        .expect("Error loading flash cards");

    for card in results{
        println!("{}", card.question);
    }

}