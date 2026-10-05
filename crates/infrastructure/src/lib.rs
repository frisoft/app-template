//! Infrastructure layer — adapters.
//!
//! Adapters implement the ports declared by the application layer. This is where
//! concrete technologies live: databases, HTTP clients, message brokers, file
//! systems. Adapters depend inward on `domain` and `application`; nothing inward
//! depends on them.
//!
//! # Where to start
//!
//! - Add one module per adapter, e.g. `pub mod postgres_order_repository;`.
//! - `impl` the application's port traits for your adapter type.
//! - Map the technology's error type into the port's error type.
//! - Test each adapter against the real backing technology (or a faithful
//!   in-memory stand-in).
//!
//! Swapping one technology for another must touch only this crate and the
//! composition root — never `domain` or `application`.
//!
//! See `AGENTS.md` for the full architecture contract, rules and recipes.
//! The `Payment` / `Money` snippets in that document are **illustrative
//! examples only** — they are intentionally not part of this template.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

// TODO: add your adapters here.
