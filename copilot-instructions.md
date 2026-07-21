# Copilot Instructions – LedgerGate

## Project Overview

LedgerGate is a professional accounting and reconciliation system designed for solo self-employed individuals and small businesses (1–10 people). The system automatically imports and ingests invoices and payments (from banks and payment providers), allocating payments to invoices even via partial allocations. System status is derived entirely from facts (N2). 

It specifically identifies and handles special cases such as currency mismatches, overpayments, ambiguous matches, and prepayments. Real-time updates are pushed via WebSockets to a React frontend that visualizes the inbox, invoice lists, live streams, and metrics.

As a university project in "Fortgeschrittene Programmierung," its primary goal is to demonstrate the unique advantages of Rust (real-time concurrency, stable latency, strict type modeling for financial data) while providing comprehensible business value.

---

## Architecture

The project follows a strict Domain-Driven Design (DDD) approach with clear layer separation across a 4-phase architecture:

1. **Problem Space (DDD):** Pure business logic describing glossaries, rules, special cases, and matching priorities.
2. **Rust Types:** Implementation of the domain logic with pure, testable Rust types (e.g., `Money`, `Currency`, `InvoiceStatus`) with no infrastructure dependencies.
3. **Store/Persistence Layer (N2):** PostgreSQL database persisting raw facts. Derived statuses are strictly calculated via SQL Views. Support for multiple partial allocations per payment/invoice.
4. **API/Realtime (Axum + WebSockets):** REST APIs for ingest and webhook events, Tokio scheduler for bank pull simulations, and WebSocket hubs for real-time frontend event streaming and metrics (e.g., p50/p95/p99 latency).

---

## Tech Stack

| Layer          | Technology                                                                 |
|----------------|----------------------------------------------------------------------------|
| Backend        | Rust, Axum, Tokio, rust_decimal (no floats), Serde, tracing                |
| Database       | PostgreSQL, sqlx (async, macros), sqlx-cli for migrations                  |
| Frontend       | React (Vite), native WebSocket Client, fetching (e.g., React Query)        |
| Tests/Testing  | Testcontainers (Postgres), Axum oneshot, cargo-nextest, JUnit XML Output   |

---

## Spec-Driven Development Workflow 🚨 (CRITICAL)

**As an AI assistant working on this project, you must STRICTLY enforce Spec-Driven Development (SDD).**

Under no circumstances are you allowed to jump directly into writing feature implementation code. You must adhere to the following sequence for every new feature or modification:

1. **Write the Spec First:** Based on the user's request, draft a clear text/logical specification ("Spec"). Identify constraints, data flows, matching priorities, and handle all edge cases explicitly before proceeding.
2. **Derive Tests:** Based exclusively on the specification, define and write the corresponding Unit Tests and Integration Tests (utilizing Testcontainers or Axum oneshot where appropriate).
3. **Approval:** Present the spec and testing obligations to the user for explicit approval.
4. **Implement:** ONLY when the spec and tests are defined and approved, proceed with implementing the actual feature code to satisfy the tests.

---

## Human Readability

**This project is part of a university course ("Fortgeschrittene Programmierung"). Human readability is the top priority — always favor clear, teachable code over clever or obscure optimizations.**

- Write code as if the reader is learning Rust and advanced programming concepts for the first time.
- Avoid advanced or obscure language features (complex trait wizardry, macro-heavy patterns) unless they are genuinely the simplest way to express the logic.
- When a simpler approach exists — even if marginally less efficient — choose the simpler approach.
- Every non-trivial block of logic must include a plain-language comment explaining *what* it does and *why*, not just *how*.
- Prefer explicit variable names (e.g., `allocation_amount` over `alloc_amt`).
- Do not apply micro-optimizations. If a performance optimization is necessary, it must be accompanied by a detailed comment.

---

## Coding Standards

### Rust (Backend)
- Use safe Rust. Any `unsafe` block requires an explicit justification comment.
- Ensure strict type modeling for financial data (use `rust_decimal`, NEVER floats).
- Rely on PostgreSQL ENUMs (`currency_code`, `credit_debit_indicator`, etc.) and constraints (e.g., `NUMERIC(18,6)`).
- Status derivation must happen in SQL Views (N2) or pure domain logic functions based on N2 facts.
- Business rules (matching algorithms, partial allocations) must be purely testable unit functions.

### React (Frontend)
- Adhere to functional React components and hooks.
- Decouple state management, WebSocket event handling, and UI rendering.

### Modularity
- Module boundaries should reflect logical separation of concerns (`problemraum/`, `store/`, `services/`, `api/`, `ws/`, `ingest/`).

---

## Project & Repository Conventions
- Migrations must be handled linearly in `backend/migrations/` via `sqlx-cli`.
- Tests must be organized into `db/`, `integration_*.rs`, and `api_smoke_*.rs`.

### Commit Discipline

- Every completed change must be committed immediately using the **[Conventional Commits](https://www.conventionalcommits.org/)** format: `<type>(<optional scope>): <short description>`.
- Commits must be atomic — one logical change per commit.
- **Do not push** (`git push`) unless the user explicitly requests it.

### AI Prompt Logging

- This is a mandatory step. Every prompt submitted to an LLM must be logged in the `ai/` directory at the repository root.
- Log entries must record at minimum the **Model** used and the **Prompt** (full text as sent).

---

## Testing Obligations

- **Coverage Goal:** At least 50% coverage, focused heavily on the Problem Space logic and DB/Services layers.
- **Unit Tests:** For all pure functions (like reconciliation, matching rules, struct behavior). Must be fast and stable.
- **Integration Tests:** Use Testcontainers with Postgres. Reset the DB state per test (`TRUNCATE RESTART IDENTITY CASCADE`).
- **Smoke Tests:** Use Axum Router `oneshot` for in-process API testing without real ports.
- **Rules:** Whenever code logic is added or altered, the corresponding test suite must be updated according to the Spec-Driven Development rule. If an agent defers test creation, it MUST explicitly inform the developer.
