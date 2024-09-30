# Database Modeling

## Tables

### flash_card

* id: SERIAL (Primary Key)
* question: TEXT (Not Null)
* answer: TEXT (Not Null)
* learning_topic_id: INTEGER (Foreign Key referencing `learning_topic.id`, Not Null)
* current_practice_day: TIMESTAMP (Not Null)
* next_practice_day: TIMESTAMP (Not Null)
* created_at: TIMESTAMP (Defaults to `CURRENT_TIMESTAMP`)

### learning_topic

* id: SERIAL (Primary Key)
* subject: TEXT (Not Null)
* created_at: TIMESTAMP (Defaults to `CURRENT_TIMESTAMP`)

### historical_acceptances

* id: SERIAL (Primary Key)
* flash_card_id: INTEGER (Foreign Key referencing `flash_card.id`, Not Null)
* answer_rate: INTEGER (Not Null)
* given_answer: TEXT (Not Null)
* test_date: TIMESTAMP (Not Null)
* created_at: TIMESTAMP (Defaults to `CURRENT_TIMESTAMP`)
