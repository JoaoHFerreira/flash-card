WITH ranked_words AS (
    SELECT
        TRIM(LOWER(word)) AS cleaned_word,
        COUNT(*) AS total_occurrences
    FROM (
        SELECT unnest(regexp_split_to_array(content, '\s+')) AS word
        FROM wikipedia_articles
    ) AS words
    WHERE TRIM(word) <> ''
    AND word ~ '^[a-zA-Z]+$'
    GROUP BY cleaned_word
)
SELECT
    ROW_NUMBER() OVER (ORDER BY total_occurrences DESC) AS rank,
    cleaned_word,
    total_occurrences
FROM ranked_words
ORDER BY total_occurrences DESC;
