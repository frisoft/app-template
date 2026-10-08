//! Domain layer — the heart of the application.
//!
//! This crate is a **blank slate**: it ships with the layer wired up and
//! documented, but no domain model. Add your own types here.
//!
//! It must contain **only**:
//!
//! - value objects (newtypes over primitives, validated in the constructor),
//! - entities and their typestate (one type per state, transitions consume `self`),
//! - domain events (past-tense facts),
//! - domain errors (as data), and
//! - **pure** domain workflows: `(State, Command) -> Result<(State, Events), Error>`.
//!
//! It must **never** depend on I/O, async runtimes, frameworks, clocks,
//! randomness, or any other crate in this workspace. Keeping the domain pure is
//! what makes it trivially testable and the rest of the system swappable.
//!
//! # Where to start
//!
//! Declare your modules below, for example:
//!
//! ```ignore
//! pub mod order_id;
//! pub mod order;
//! pub mod event;
//! pub mod error;
//! pub mod workflow;
//! ```
//!
//! Then re-export the public surface from this module so callers depend on
//! `domain::Type` rather than `domain::order::Type`.
//!
//! See `AGENTS.md` for the full architecture contract, rules and recipes.
//! The `Payment` / `Money` snippets in that document are **illustrative
//! examples only** — they are intentionally not part of this template.

// TODO: add your domain modules here.
