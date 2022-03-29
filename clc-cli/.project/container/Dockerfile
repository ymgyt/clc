# https://github.com/LukeMathWalker/cargo-chef#without-the-pre-built-image
FROM rust:1.59.0-slim-bullseye AS chef
RUN cargo install cargo-chef
WORKDIR clc

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /clc/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin clc

FROM debian:bullseye-slim AS runner
WORKDIR clc
COPY --from=builder /clc/target/release/clc /usr/local/bin/clc
ENTRYPOINT ["/usr/local/bin/clc"]
