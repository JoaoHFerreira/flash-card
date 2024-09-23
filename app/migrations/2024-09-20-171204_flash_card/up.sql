-- Your SQL goes here
CREATE TABLE flash_card (
    id SERIAL PRIMARY KEY, 
    question TEXT NOT NULL,
    answer TEXT NOT NULL,
    learning_topic_id INTEGER REFERENCES learning_topic(id) NOT NULL,
    current_practice_day TIMESTAMP NOT NULL,
    next_practice_day TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
)