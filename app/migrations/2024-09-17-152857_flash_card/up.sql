-- Your SQL goes here
CREATE TABLE flash_card (
    id SERIAL PRIMARY KEY, 
    question TEXT NOT NULL,
    answer TEXT NOT NULL,
    learning_topic_id INTEGER REFERENCES learning_topic(id) NOT NULL,
    practice_schedule_id INTEGER REFERENCES practice_schedule(id) NOT NULL
)