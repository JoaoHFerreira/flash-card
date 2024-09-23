-- Your SQL goes here
CREATE TABLE learning_topic (
    id SERIAL PRIMARY KEY, 
    subject TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
)