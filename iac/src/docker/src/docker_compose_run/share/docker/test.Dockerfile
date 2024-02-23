FROM rust:1.75

WORKDIR /usr/src/workspace/

# download and build lib
COPY ./Cargo.toml /usr/src/workspace/Cargo.toml
RUN mkdir /usr/src/workspace/src \
    && echo "fn main() {}" > /usr/src/workspace/src/main.rs
RUN cargo build --release

# build app
COPY ./src/* /usr/src/workspace/src/
RUN cargo test --no-run --release
