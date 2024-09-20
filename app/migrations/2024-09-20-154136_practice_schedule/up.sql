-- Your SQL goes here
CREATE TABLE practice_schedule (
    id SERIAL PRIMARY KEY, 
    current_practice_day TIMESTAMP NOT NULL,
    next_practice_day TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
)
