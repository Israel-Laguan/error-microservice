# Adapted from https://github.com/mr-pascal/medium-rust-dockerize/blob/master/Dockerfile
FROM rustlang/rust:nightly-slim as builder

WORKDIR /usr/src

RUN USER=root cargo new error-microservice

COPY Cargo.toml Cargo.lock /usr/src/error-microservice/

WORKDIR /usr/src/error-microservice

RUN apt-get update && apt-get install -y openssl build-essential pkg-config libssl-dev

## Install target platform (Cross-Compilation) --> Needed for Alpine
RUN rustup target add x86_64-unknown-linux-gnu

# This is a dummy build to get the dependencies cached.
RUN cargo build --target x86_64-unknown-linux-gnu --release

COPY src /usr/src/error-microservice/src/

## Touch main.rs to prevent cached release build
RUN touch /usr/src/error-microservice/src/main.rs

# This is the actual application build.
RUN cargo build --target x86_64-unknown-linux-gnu --release

################
##### Runtime
FROM alpine:3.16.0 AS runtime
RUN apk --no-cache add ca-certificates

# Copy application binary from builder image
COPY --from=builder /usr/src/error-microservice/target/x86_64-unknown-linux-gnu/release/error-microservice /usr/local/bin

EXPOSE 8080

CMD ["/usr/local/bin/error-microservice"]
