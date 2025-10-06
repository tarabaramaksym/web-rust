FROM rust:1.84 as builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

COPY . .
RUN touch src/main.rs
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/rust-by-example /app/rust-by-example
COPY --from=builder /app/src /app/src
COPY --from=builder /app/pub /app/pub

RUN mkdir -p /app/pub/generated

ENV RUST_LOG=info

EXPOSE 8080

CMD ["/app/rust-by-example"]

