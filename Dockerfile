## Adapted from https://dev.to/karanpratapsingh/seeding-postgres-with-docker-19n7
FROM postgres:14-alpine as db
WORKDIR /app
COPY ./db_init.sh /docker-entrypoint-initdb.d
COPY ./schema.sql ./scripts/db/dump.sql

# Adapted from https://github.com/mr-pascal/medium-rust-dockerize/blob/master/Dockerfile
FROM rustlang/rust:nightly-slim as builder

WORKDIR /usr/src

RUN USER=root cargo new error-microservice

COPY Cargo.toml Cargo.lock /usr/src/error-microservice/

WORKDIR /usr/src/error-microservice

RUN apt-get update && apt-get install -y openssl build-essential pkg-config libssl-dev

# This is a dummy build to get the dependencies cached.
RUN cargo build --release

COPY src /usr/src/error-microservice/src/

## Touch main.rs to prevent cached release build
RUN touch /usr/src/error-microservice/src/main.rs

# This is the actual application build.
# RUN cargo build --release

EXPOSE 8080

# CMD [ "cargo run" ]

# CMD ["/usr/src/error-microservice/target/release/error-microservice"]
