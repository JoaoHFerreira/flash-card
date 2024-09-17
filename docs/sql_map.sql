-- Table: flash_card
CREATE TABLE flash_card (
    id SERIAL PRIMARY KEY, 
    question TEXT NOT NULL,
    answer TEXT NOT NULL,
    learning_topic_id INTEGER REFERENCES learning_topic(id) NOT NULL,
    practice_schedule_id INTEGER REFERENCES practice_schedule(id) NOT NULL
);
CREATE INDEX idx_flash_card_learning_topic_id ON flash_card(learning_topic_id);
CREATE INDEX idx_flash_card_practice_schedule_id ON flash_card(practice_schedule_id);

-- Table: learning_topic
CREATE TABLE learning_topic (
    id SERIAL PRIMARY KEY, 
    subject TEXT NOT NULL
);

-- Table: practice_schedule
CREATE TABLE practice_schedule (
    id SERIAL PRIMARY KEY, 
    current_practice_day TIMESTAMP NOT NULL,
    next_practice_day TIMESTAMP NOT NULL
);

-- Table: historical_acceptances
CREATE TABLE historical_acceptances (
    id SERIAL PRIMARY KEY,
    flash_card_id INTEGER REFERENCES flash_card(id) NOT NULL,
    is_right BOOLEAN NOT NULL,
    test_date TIMESTAMP NOT NULL
);
CREATE INDEX idx_historical_acceptances_flash_card_id ON historical_acceptances(flash_card_id);