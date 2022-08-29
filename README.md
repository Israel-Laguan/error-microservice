# error-microservice

First, install rustup

```sh
# Be sure to have installed build-essential, i.e. sudo apt install build-essential
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustc --version
```

then change to nightly

```sh
rustup toolchain install nightly
rustup default nightly
```

```sh
# Build local
cargo build

# Run local
cargo run
```

```sh

# Build container
docker build -t error-microservice .

# Run container
docker run --rm -p 8080:8080 --name server error-microservice
```

## Objectives

There will be errors for each app
