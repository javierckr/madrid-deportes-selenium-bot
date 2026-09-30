FROM rust:1 AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:trixie-slim

WORKDIR /app

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/madrid-deportes-selenium-bot /app/madrid-deportes-selenium-bot

CMD ["/app/madrid-deportes-selenium-bot"]
