# Testing the backend (Docker and local)

Prerequisites
- Docker and Docker Compose installed and running (required for compose runs and for `testcontainers` fallback).
- Rust toolchain (nightly not required) if you want to run tests locally without compose.

Quick run (recommended, reproducible):

```bash
# from repo root
./backend/run-tests-in-docker.sh
```

This builds the `tester` image, starts Postgres, runs the integration test `postgres_container_test`, and exits with the test result.

Run locally (direct `cargo test`, requires Docker for testcontainers fallback):

```bash
cd backend
# uses DATABASE_URL if set (compose), otherwise starts a transient Postgres via testcontainers
cargo test --test postgres_container_test -- --nocapture
```

Collect logs (if using Docker Compose):

```bash
docker compose logs tester
docker compose logs postgres
```

Troubleshooting
- If tests fail to connect, ensure Docker is running and port mapping is allowed.
- First run will compile dependencies; subsequent runs are faster.

Notes
- The integration test executes the migration SQL from `backend/migrations/20260623000001_core_n2_tables_and_views.sql` and uses the store helpers to insert and assert invoice status.

How to run:
cd to PROG/backend
./run-tests-in-docker.sh