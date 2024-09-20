-- Your SQL goes here
CREATE TABLE historical_acceptances (
    id SERIAL PRIMARY KEY,
    flash_card_id INTEGER REFERENCES flash_card(id) NOT NULL,
    answer_rate INTEGER NOT NULL,
    given_answer TEXT NOT NULL,
    test_date TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
)