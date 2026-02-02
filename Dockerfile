FROM rust:1.70 as builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml ./
COPY main-app ./main-app
COPY crates ./crates

# Build application
RUN cargo build --release -p main-app

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/main-app /app/main-app

EXPOSE 8080

CMD ["/app/main-app"]
