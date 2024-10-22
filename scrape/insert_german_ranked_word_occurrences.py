import psycopg2

class InsertGermanRankedWordOccurrencesCommand:
    def execute(self):
        with psycopg2.connect(
            dbname="flash_card_db",
            user="user",
            password="pass",
            host="db",
            port="5432"
        ) as conn:
            with conn.cursor() as cur:
                with open("sql/insert_into_german_ranked_word_occurrences.sql", 'r') as f:
                    query = f.read()

                cur.execute(query)
                conn.commit()
