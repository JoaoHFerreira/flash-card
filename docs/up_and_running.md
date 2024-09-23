# Key Docker and Rust Commands

# Table of Contents
1. [Key Docker and Rust Commands](#key-docker-and-rust-commands)
   - [Docker Commands](#docker-commands)
     1. [Build and start containers](#build-and-start-containers)
     2. [Run Rust code](#run-rust-code)
     3. [Access bash in container for debugging](#access-bash-in-container-for-debugging)
     4. [Stop containers](#stop-containers)
     5. [View container logs](#view-container-logs)
   - [Database Operations (Diesel ORM)](#database-operations-diesel-orm)
     1. [Set up Diesel ORM](#set-up-diesel-orm)
     2. [Generate new migration](#generate-new-migration)
     3. [Run migrations](#run-migrations)
     4. [Undo last migration](#undo-last-migration)
     5. [Connect to database](#connect-to-database)
2. [Creating a New Action](#creating-a-new-action)



## Docker Commands

1. Build and start containers:
   ```
   docker-compose up --build
   ```

2. Run Rust code:
   ```
   docker-compose run --rm -it flash-card cargo run
   ```

3. Access bash in container for debugging:
   ```
   docker-compose exec flash-card bash
   docker-compose run --rm -it flash-card bash
   ```

4. Stop containers:
   ```
   docker-compose down
   ```

5. View container logs:
   ```
   docker-compose logs
   ```

## Database Operations (Diesel ORM)

1. Set up Diesel ORM:
   ```
   diesel migration setup
   ```

2. Generate new migration:
   ```
   diesel migration generate create_posts
   ```

3. Run migrations:
   ```
   diesel migration run
   ```

4. Undo last migration:
   ```
   diesel migration redo
   ```

5. Connect to database:
   ```
   docker-compose exec db psql -U user -d flash_card_db
   ```

6. Running Entire App
   ```
   cargo run --bin flash_card
   ```


## 2. [Creating a New Action](#creating-a-new-action)
The flow is:
1. Create the models based on schema and test;
2. After everything ok in previous step, add the funcionality into lib.rs and test;
3. After everything ok in previous step, add the action and test;
4. After everything ok in previous step, then we are ready to create the PR

