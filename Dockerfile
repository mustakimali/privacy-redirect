FROM rust:1.73 as chef
RUN cargo install cargo-chef
RUN apt-get update -y && apt-get install protobuf-compiler -y
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

RUN cargo build --release
COPY tracking-params tracking-params
COPY web web
COPY browser-ext browser-ext
COPY Cargo* ./

ENV SQLX_OFFLINE true
RUN mkdir /app/static
RUN cargo test --release
RUN cargo build --release
RUN ls -lsah target/release

FROM debian:bullseye-slim AS runtime
WORKDIR app

ENV TZ=Etc/UTC
ENV RUST_LOG=info

RUN apt-get update -y && apt-get install ca-certificates -y

COPY --from=builder /app/target/release/privacy-redirect /app
COPY frontend /app/frontend
COPY browser-ext/script.js /app/frontend/script.js

ENTRYPOINT ["/app/privacy-redirect"]
