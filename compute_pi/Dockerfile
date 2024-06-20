FROM rust:slim-bookworm as builder

WORKDIR /app

COPY Cargo.toml Cargo.toml

RUN mkdir -p ./src \
&& echo 'fn main(){}' >> ./src/main.rs \
&& cargo build --release

COPY ./src ./src
