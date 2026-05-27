# ── Build stage ──────────────────────────────────────────
FROM rust:1.85-bookworm AS builder

RUN rustup target add wasm32-unknown-unknown
RUN cargo install dioxus-cli --version 0.7.1

WORKDIR /app

# Cache cargo dependencies
COPY Cargo.toml Cargo.lock* ./
RUN mkdir -p src && echo "fn main() {}" > src/main.rs
RUN cargo fetch --features fullstack 2>/dev/null || true

# Copy source and build
COPY . .
RUN dx bundle --web --fullstack --release

# ── Runtime stage ────────────────────────────────────────
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    libsqlite3-0 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/dx/todo_list/release/web/ ./
RUN mkdir -p /app/data

ENV DB_PATH=/app/data/todo_list.db
EXPOSE 8080

CMD ["./todo_list"]
