FROM rust:latest

WORKDIR /app

RUN cargo install diesel_cli
RUN cargo install diesel_cli --no-default-features --features postgres

CMD ["cargo", "run", "--bin", "dummy_api"]