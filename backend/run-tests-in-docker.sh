#!/usr/bin/env bash
set -euo pipefail

# Build containers and run the tester service. Exits with the tester exit code.
docker compose up --build --abort-on-container-exit --exit-code-from tester

# Cleanup (optional):
# docker compose down -v
