FROM rust:1.72-bullseye AS builder
WORKDIR /usr/src/backend

# Install system deps required to build Postgres-backed Rust crates
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    libpq-dev \
    ca-certificates \
 && rm -rf /var/lib/apt/lists/*

# Copy only Cargo files first for cached build of dependencies
COPY Cargo.toml Cargo.lock ./
COPY backend/Cargo.toml ./
COPY backend/src ./src
COPY backend/migrations ./migrations

# Build tests artifacts (do not run tests at build time; runtime executes them)
RUN cargo build --tests --release

WORKDIR /usr/src/backend

# Default command runs the specific integration test against $DATABASE_URL
CMD ["bash", "-lc", "cargo test --test postgres_container_test -- --nocapture"]
