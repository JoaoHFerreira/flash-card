# Up and Running with Docker and Rust

## Navigating to the Docker Directory

Before running any commands, make sure you navigate to the Docker directory. Use the following command:

```bash
cd docker
```

## Creating the Docker Image

Once you're in the Docker directory, you can create the Docker image based on the `Dockerfile` and also run the Docker containers using `docker-compose`:

```bash
docker-compose up --build
```

## Running Rust Code

The following command is used for running and debugging the Rust code. Think of the `docker-compose run --rm -it flash-card` part as being the root terminal, and the remaining part as the actual Rust command:

```bash
docker-compose run --rm -it flash-card cargo run
```

## Prepare ORM
```
diesel migration setup
```

## Generate Folders
```
diesel migration generate create_posts
```
After run `generate` command fill the created sql files with respective tables, in up and down.sql

## Create tables
```
diesel migration run
```

## Remove last change
```
diesel migration redo
```

## Connect to database
```
docker-compose exec db psql -U user -d flash_card_db
```

```
cargo run --bin name_of_the_bin_folder
```