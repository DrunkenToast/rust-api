FROM rust as builder
WORKDIR /usr/src/rust-api
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y sqlite3 curl
COPY --from=builder /usr/local/cargo/bin/rust-api /usr/local/bin/rust-api
ENV SERIAL_PORT /dev/ttyACM0
EXPOSE 3000
CMD ["rust-api"]

