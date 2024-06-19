# Building Stage
FROM rust:1.78-slim-buster AS builder

RUN cargo new appbuilder --bin
WORKDIR /appbuilder
COPY Cargo.toml /appbuilder/
RUN cargo build --release