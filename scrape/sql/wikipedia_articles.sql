    CREATE TABLE wikipedia_articles (
        id TEXT PRIMARY KEY,
        content TEXT NOT NULL,
        country VARCHAR(10)
    );


ALTER TABLE wikipedia_articles
ADD COLUMN country_archer_10_at_most VARCHAR(10);