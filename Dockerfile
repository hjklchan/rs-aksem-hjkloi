ARG BASE_IMAGE=rust:1.78-slim-buster

FROM ${BASE_IMAGE} AS planner

WORKDIR /app

RUN cargo install cargo-chef --version 0.1.67
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM ${BASE_IMAGE} AS cacher

WORKDIR /app
RUN cargo install cargo-chef --version 0.1.67
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM $BASE_IMAGE as builder
WORKDIR /app

# Copy over the cached dependencies
COPY --from=cacher /app/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME

RUN rustup target add x86_64-unknown-linux-musl
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM scratch
COPY --from=builder /usr/local/cargo/bin/rs-aksem-hjkoi .
CMD ["./rs-aksem-hjkoi"]