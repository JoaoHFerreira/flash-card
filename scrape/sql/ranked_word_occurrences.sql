CREATE TABLE ranked_word_occurrences (
    rank SERIAL PRIMARY KEY,
    cleaned_word TEXT NOT NULL,
    total_occurrences INT NOT NULL
);
