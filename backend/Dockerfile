FROM rust:1.75-bullseye AS builder
WORKDIR /usr/src/backend

# Install system deps required to build Postgres-backed Rust crates
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    libpq-dev \
    ca-certificates \
 && rm -rf /var/lib/apt/lists/*

# Build context is ./backend (see docker-compose.yml), so all paths below are relative to it.
COPY Cargo.toml ./
COPY src ./src
COPY migrations ./migrations
COPY tests ./tests

# Build tests artifacts (do not run tests at build time; runtime executes them)
RUN cargo build --tests --release

# Default command runs the full test suite (all [[test]] targets registered in Cargo.toml).
# --test-threads=1: tests share ONE Postgres instance (DATABASE_URL) here, so they must run
# sequentially to avoid table TRUNCATEs from one test wiping data used by a concurrent test.
CMD ["bash", "-lc", "cargo test --release -- --nocapture --test-threads=1"]
