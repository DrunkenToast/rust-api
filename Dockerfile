FROM lukemathwalker/cargo-chef:latest-rust-1.56.0 AS chef
WORKDIR rust-api

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /rust-api/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build rust-apilication
COPY . .
RUN cargo build --release --bin rust-api

# We do not need the Rust toolchain to run the binary!
FROM debian:buster-slim AS runtime
RUN apt-get update && apt-get install -y sqlite3
WORKDIR rust-api
COPY --from=builder /rust-api/release/rust-api /usr/local/bin
EXPOSE 3000
ENTRYPOINT ["/usr/local/bin/rust-api"]

