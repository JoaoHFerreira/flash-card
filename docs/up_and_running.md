# Up And Running

This document provides an overview of the available Makefile commands for managing the Docker environment and interacting with the API endpoints. Each command is executed using `make <command_name>`.

## Commands

### Docker Commands

1. **up**
   - Builds and starts all Docker services defined in `docker/docker-compose.yml`.
   - **Usage:**
     ```sh
     make up
     ```
   
2. **psql**
   - Opens a PostgreSQL command line interface within the running `db` container.
   - **Usage:**
     ```sh
     make psql
     ```

### API Commands

1. **api-hello**
   - Sends a GET request to the root API endpoint.
   - **Usage:**
     ```sh
     make api-hello
     ```
   
2. **api-get-flash-cards**
   - Retrieves all flashcards by sending a GET request to the `/flash_card` endpoint.
   - **Usage:**
     ```sh
     make api-get-flash-cards
     ```

3. **api-add-flash-card**
   - Adds a new flashcard using a POST request to the `/flash_card` endpoint.
   - **Usage:**
     ```sh
     make api-add-flash-card
     ```
   - Example request payload:
     ```json
     {
       "question": "What is Rust?",
       "answer": "Rust is a systems programming language.",
       "learning_topic": "Programming"
     }
     ```

4. **api-batch-flash-card**
   - Uploads a CSV file of flashcards using a POST request to the `/batch_csv_import` endpoint.
   - **Usage:**
     ```sh
     make api-batch-flash-card
     ```
   - The CSV file is expected to be located at `./app/csv_files/test.csv`.

5. **api-get-learning-topic**
   - Retrieves all learning topics by sending a GET request to the `/learning_topic` endpoint.
   - **Usage:**
     ```sh
     make api-get-learning-topic
     ```

6. **api-add-learning-topic**
   - Adds a new learning topic using a POST request to the `/learning_topic` endpoint.
   - **Usage:**
     ```sh
     make api-add-learning-topic
     ```
   - Example request payload:
     ```json
     {
       "subject": "API"
     }
     ```

### Testing Commands

1. **api-test**
   - Tests multiple API endpoints by sequentially calling:
     - `api-hello`
     - `api-get-flash-cards`
     - `api-add-flash-card`
     - `api-batch-flash-card`
     - `api-get-learning-topic`
     - `api-add-learning-topic`
   - The script prints a summary of which endpoints succeeded or failed.
   - **Usage:**
     ```sh
     make api-test
     ```
