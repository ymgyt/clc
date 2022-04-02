# https://github.com/LukeMathWalker/cargo-chef#without-the-pre-built-image
FROM rust:1.59.0-slim-bullseye AS chef
RUN cargo install cargo-chef
WORKDIR clc

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /clc/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json --features cache
COPY . .
RUN cargo build --release --bin clcgql --features cache

FROM debian:bullseye-slim AS runner
WORKDIR clc
COPY --from=builder /clc/target/release/clcgql /usr/local/bin/clcgql

# To connect a repository to container image
LABEL org.opencontainers.image.source https://github.com/ymgyt/clc

ENV CLCGQL_LOG=info
ENV CLCGQL_LOG_COLOR=false

ENTRYPOINT ["/usr/local/bin/clcgql"]
