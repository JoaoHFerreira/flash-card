# Database Modeling
## Tables
### flash_card

* id: UUID (Primary Key, Btree Index)
* question: Text
* answer: Text
* learning_topic_id: UUID (Foreign Key referencing learning_topic.id, Btree Index)
* practice_schedule_id: UUID (Foreign Key referencing practice_schedule.id, Btree Index)

### learning_topic

* id: UUID (Primary Key, Btree Index)
* subject: Text

### practice_schedule

* id: UUID (Primary Key, Btree Index)
* current_practice_day: Timestamp
* next_practice_day: Timestamp

### historical_acceptances

* id: UUID (Primary Key, Btree Index)
* flash_card_id: UUID (Foreign Key referencing flash_card.id, Btree Index)
* is_right: Boolean
* test_date: Timestamp
