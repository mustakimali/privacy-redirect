FROM lukemathwalker/cargo-chef:latest-rust-1.66 AS chef
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

FROM node:18 as frontend-builder
WORKDIR app
COPY frontend/package.json ./
COPY frontend/yarn.lock ./
RUN yarn install

COPY frontend ./
RUN yarn build

RUN ls -lsah .

FROM debian:bullseye-slim AS runtime
WORKDIR app

ENV TZ=Etc/UTC
ENV RUST_LOG=info

COPY --from=builder /app/target/release/privacy-redirect /app
COPY --from=frontend-builder /app/build /app/static
COPY browser-ext/script.js /app/static/script.js

ENTRYPOINT ["/app/privacy-redirect"]
