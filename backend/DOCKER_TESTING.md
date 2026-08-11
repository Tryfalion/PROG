# Backend Docker Testing Guide

This file explains the quickest way to run the backend integration test suite with Docker Compose.

## What this setup does

- Starts a real PostgreSQL container.
- Runs the backend integration test `postgres_container_test`.
- Applies the SQL migration from `backend/migrations/20260623000001_core_n2_tables_and_views.sql`.
- Verifies that the invoice status resolves to `Paid` after an allocation is inserted.

## Recommended way to run

From the repository root:

```bash
chmod +x backend/run-tests-in-docker.sh
./backend/run-tests-in-docker.sh
```

## Local run without Compose

If you want to run the test directly from the backend crate, use:

```bash
cd backend
cargo test --test postgres_container_test -- --nocapture
```

This works with the `DATABASE_URL` environment variable if it is set. Otherwise the test falls back to `testcontainers` and starts PostgreSQL itself.

## How to set the workspace root in a terminal

If your terminal starts in the wrong folder, go to the repository root first:

```bash
cd /path/to/PROG
pwd
ls
```

Then run the commands above from there.

## Helpful log commands

```bash
docker compose logs tester
docker compose logs postgres
```

## Notes

- First run takes longer because Rust dependencies are compiled.
- Docker must be running before you start the compose test.
- If compose cannot reach the database, check that the `postgres` service is healthy and that the `DATABASE_URL` points to `postgres` on port `5432`.
