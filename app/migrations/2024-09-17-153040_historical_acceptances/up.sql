-- Your SQL goes here
CREATE TABLE historical_acceptances (
    id SERIAL PRIMARY KEY,
    flash_card_id INTEGER REFERENCES flash_card(id) NOT NULL,
    is_right BOOLEAN NOT NULL,
    test_date TIMESTAMP NOT NULL
);