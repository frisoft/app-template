//! Application layer — application workflows and their ports.
//!
//! This layer orchestrates the pure domain and owns **all** I/O. It knows
//! nothing about HTTP, SQL, files or the runtime: it depends only on the traits
//! (ports) it defines, which the infrastructure layer implements.
//!
//! Dependencies point inward: `application` depends on `domain`, never the
//! other way around.
//!
//! # Where to start
//!
//! - Add ports (the traits the outside world must implement) in [`ports`].
//! - Add one module per application workflow, e.g. `pub mod place_order;`, each
//!   exposing a single `async` function that follows the pipeline
//!   **load state → pure domain workflow → persist state + publish events**.
//! - Add one error enum per application workflow (`thiserror`) so callers can
//!   tell a domain rejection from an infrastructure failure.
//!
//! See `AGENTS.md` for the full architecture contract, rules and recipes.
//! The `Payment` / `Money` snippets in that document are **illustrative
//! examples only** — they are intentionally not part of this template.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod ports;

// TODO: add one module per application workflow here, e.g. `pub mod place_order;`.
