FROM rust:1.68.2 AS base
RUN cargo install cargo-chef
WORKDIR app

FROM base AS planner
COPY . .
RUN cargo base prepare --recipe-path recipe.json

FROM base AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo base cook --release --recipe-path recipe.json

ENV SQLX_OFFLINE true
RUN cargo build --release
RUN ls -lsah target/release

FROM debian:bullseye-slim AS runtime
WORKDIR app

ENV TZ=Etc/UTC
ENV RUST_LOG=info,sqlx=warn

COPY --from=builder /app/target/release/privacy-redirect /app
COPY frontend /app/
RUN ls -lsah /app

ENTRYPOINT ["/app/privacy-redirect"]
