FROM rust:1.59 as builder
WORKDIR /app
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/dynamic_dns_rust /usr/local/bin/dynamic_dns_rust
CMD dynamic_dns_rust
