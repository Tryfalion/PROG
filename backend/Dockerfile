FROM rust:1.72-bullseye AS builder
WORKDIR /usr/src/backend

# Install system deps required to build Postgres-backed Rust crates
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    libpq-dev \
    ca-certificates \
 && rm -rf /var/lib/apt/lists/*

# Copy crate files (build context is ./backend so paths are relative here)
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migrations ./migrations

# Build release to cache dependencies (tests will be run at container runtime)
RUN cargo build --release || true

WORKDIR /usr/src/backend

# Default command: run the specific integration test (uses DATABASE_URL in compose)
CMD ["bash", "-lc", "cargo test --test postgres_container_test -- --nocapture"]
