FROM rust:slim-buster as builder

WORKDIR /app

# Armv7クロスコンパイル用ツールをインストール
RUN apt-get update && apt-get install -y gcc-aarch64-linux-gnu
RUN rustup target add aarch64-unknown-linux-gnu

# 設定ファイルをコピー
COPY Cargo.toml Cargo.toml
COPY ./.cargo/config.toml ./.cargo/config.toml

# ライブラリインストール後のキャッシュを作っておく
RUN mkdir -p ./src \
&& echo 'fn main(){}' >> ./src/main.rs \
&& cargo build --release

# ソースコードをコピー
COPY ./src ./src
