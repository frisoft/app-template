---
name: rust-conventions
description: Rust conventions for this template — naming, error handling with thiserror, derives, async boundaries, docs, formatting, and dependency policy. Use when writing or reviewing Rust code and unsure about style, naming, or dependencies.
---

# Rust conventions

Rules live in `AGENTS.md`. This skill is the detailed style reference.

## Naming

- Types: `UpperCamelCase`. Functions/methods/modules: `snake_case`. Constants:
  `SCREAMING_SNAKE_CASE`.
- Commands are imperative nouns of intent: `PlacePayment`, `RefundPayment`.
- Workflows are verbs: `place_payment`, `pay_payment`.
- Events are past tense: `PaymentPlaced`, `PaymentPaid`.
- Errors are named for the operation: `PlacePaymentError`, `RepositoryError`.
- Avoid generic names (`Manager`, `Service`, `Helper`, `Utils`); name the concept.

## Errors

- One error enum per operation in the layer that owns it.
- Use `thiserror` and convert with `#[from]` / `#[error(transparent)]`.
- Never lose context: `RepositoryError::Unavailable(String)` should say what
  failed.
- Never convert a domain error into a generic infrastructure error, or vice
  versa.

## Functions and types

- Small, focused, single responsibility. If a function needs a paragraph to
  name, split it.
- Annotate `#[must_use]` on pure constructors, getters and query methods.
- Derive the minimum: usually `Debug, Clone, PartialEq, Eq`. Add `Hash`/`Ord`
  only when needed.
- Prefer `&self` + `impl Trait`/generics for ports so callers can inject fakes.
- Keep the public surface explicit with re-exports in `lib.rs`.

## Async

- `async` exists only in `application`, `infrastructure` and `cli`. **Never in
  `domain`.**
- Errors and cancellation propagate with `?`; do not spawn detached tasks that
  hide failures.

## Docs and comments

- `//!` on every crate and module; `///` on every public item
  (`#![warn(missing_docs)]`).
- Explain *why* and *what invariant holds*, not *what the line does*.
- Keep the presentation vocabulary: state, command, event, workflow, port,
  adapter.

## Formatting

- `cargo fmt` with the repo `rustfmt.toml`. Do not hand-format.

## Dependencies

- Add workspace dependencies in the root `Cargo.toml`
  `[workspace.dependencies]` and inherit with `dep.workspace = true`. Do not pin
  versions in member crates. The workspace lists a starter menu (`thiserror`,
  `async-trait`, `tokio`, `clap`, `anyhow`); uncomment what a crate needs.
- Keep `domain` dependency-light (`thiserror` at most). Purity is the point.
- Prefer well-maintained crates with a clear ownership story. For anything new
  that touches money, time, crypto or concurrency, stop and justify it.
- Useful additions as the app grows: `uuid` (ids), `rust_decimal` (exact money),
  `serde` (boundary serialisation), `tracing` (observability), `tower`/`axum`
  (HTTP adapter), `sqlx`/`diesel` (DB adapter), `proptest` (property tests).
  Keep these **out of `domain`**.
