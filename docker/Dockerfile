FROM rust:latest

WORKDIR /app

# Remove the build step from Dockerfile since the volume will be mounted at runtime
CMD ["cargo", "run", "--release"]
