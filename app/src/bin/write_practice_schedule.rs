use flash_card::*;
use chrono::{Utc, Duration};

fn main() {
    let connection = &mut establish_connection();

    let current_practice_day = Utc::now().naive_utc();
    let next_practice_day = current_practice_day + Duration::days(1);

    let practice_schedule = create_practice_schedule(
        connection,
        current_practice_day,
        next_practice_day,
    );
    println!("\nCreated practice schedule with id {}", practice_schedule.id);
}
