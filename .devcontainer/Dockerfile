FROM rust:1.76.0-bookworm

RUN apt-get update && apt-get install -y sqlite3

ENV CARGO_BUILD_TARGET_DIR=/tmp/target
RUN rustup component add rustfmt clippy
RUN cargo install cargo-update
RUN cargo install cargo-shuttle
RUN cargo install sqlx-cli --no-default-features --features sqlite