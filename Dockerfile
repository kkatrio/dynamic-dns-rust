FROM rust:1.77 as builder
WORKDIR /app
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt update && apt install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/dynamic_dns_rust /usr/local/bin/dynamic_dns_rust
CMD /usr/local/bin/dynamic_dns_rust
