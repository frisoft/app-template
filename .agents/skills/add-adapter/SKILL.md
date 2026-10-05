---
name: add-adapter
description: Add or swap an infrastructure adapter implementing an application port — database, HTTP client, event bus or external service. Use when integrating a technology or persisting/loading state.
---

# Add or swap an adapter

Rules live in `AGENTS.md`. Snippets are **illustrative examples**.

## Principle

Ports are **owned by the inside** (`application`); adapters implement them
**outside** (`infrastructure` or a driving-adapter crate). This inverts the
dependency direction and keeps the application independent of any technology.

## Steps

1. Define the port (trait) in `crates/application/src/ports.rs` **first**, if it
   does not exist. Keep it minimal: only the operations an application workflow
   calls.
2. Implement the trait in `crates/infrastructure/src/<adapter>.rs` using the
   concrete technology.
3. Map the technology's error type into the port's error type — do not leak
   `sqlx::Error`, `reqwest::Error`, etc. inward.
4. Wire it in the composition root (`crates/cli/src/main.rs`).
5. Integration-test the adapter against the real technology (or a faithful
   in-memory stand-in) — see the `write-tests` skill.

## Port shape

```rust,ignore
use async_trait::async_trait;

#[async_trait]
pub trait OrderRepository: Send + Sync {
    async fn load(&self, id: &OrderId) -> Result<Order, RepositoryError>;
}

#[async_trait]
pub trait UnitOfWork: Send + Sync {
    async fn commit(&self, state: &Order, events: &[DomainEvent])
        -> Result<(), RepositoryError>;
}
```

- Trait: `Send + Sync`.
- Errors: one port error type per boundary; convert the technology error into it.
- Commit state **and** events together (unit of work / outbox).

## Checklist

- [ ] Port defined in `application`, not in the adapter.
- [ ] Adapter contains no business rules — only translation and I/O.
- [ ] Technology errors mapped, never leaked inward.
- [ ] Wired in the composition root only.
- [ ] Integration test exists.

Swapping one database for another, or a CLI for HTTP, must touch only the adapter
and the composition root. If it touches `domain` or `application`, the boundary
is wrong.
