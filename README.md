# error-microservice

## Cargo config

See the comments in `.cargo/config.yml` and install the required tools for your platform

## Pre-commit hooks

You will need to install `lefthook` tool and generate the Git hooks:

- Follow the [installation instructions](https://github.com/evilmartians/lefthook/blob/master/docs/full_guide.md#installation)
- `lefthook install`


Then, you should be able to verify the installation:

```
Lefthook run pre-commit
```

the output should look like this:

```
✔️  fmt
✔️  test
✔️  build
✔️  clippy
```

## Installation

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
