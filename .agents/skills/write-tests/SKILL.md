---
name: write-tests
description: Write and place tests in this Rust DDD template — pure domain unit tests, application workflow tests with fakes, infrastructure integration tests, property tests. Use when adding tests or deciding where a test belongs.
---

# Write tests

Rules live in `AGENTS.md`. Tests describe behaviour, not implementation.

## Layered strategy — cheapest first

1. **Domain unit tests** (in-module `#[cfg(test)]`) — the majority. Pure, fast,
   no fakes. Assert invariants and every branch of every workflow.
2. **Domain integration tests** (`crates/domain/tests/`) — still pure; drive a
   full domain workflow from state + command to state + events.
3. **Application workflow tests** (`crates/application/tests/`) — inject
   in-memory fakes implementing the ports. Assert the domain ran and that state
   **and** events were committed. Assert failures commit nothing.
4. **Infrastructure integration tests** (`crates/infrastructure/tests/`) — test
   each adapter against its real backing technology (or a faithful in-memory
   stand-in).

## Rules

- Never mock the domain. It is pure — call it directly.
- A test that needs a real database belongs in `infrastructure`, not in
  `application`.
- Cover the failure path, not just the happy path.
- Prefer property tests for invariants that must hold for *all* inputs.
- Add the test next to the code it exercises; keep test names behavioural
  (`draft_cannot_be_placed_with_zero_amount`), not mechanical.

## Application workflow test shape

> `#[tokio::test]` needs the `tokio` **dev-dependency**. It ships commented out;
> uncomment it under `[dev-dependencies]` in the crate you are testing
> (`crates/application/Cargo.toml` / `crates/infrastructure/Cargo.toml`):
> `# tokio = { workspace = true, features = ["macros", "rt"] }`.

```rust,ignore
#[tokio::test]
async fn places_a_draft_and_commits_state_with_events() {
    let store = FakeStore::default();          // implements the ports
    let id = PaymentId::new(1).unwrap();
    *store.draft.lock().unwrap() =
        Some(DraftPayment::new(id.clone(), Money::from_pennies(500)));

    let placed = place_payment(PlacePayment { id: id.clone() }, &store, &store)
        .await
        .unwrap();

    assert_eq!(placed.id(), &id);
    assert_eq!(store.committed.lock().unwrap().len(), 1);
    assert_eq!(store.published.lock().unwrap().len(), 1);
}
```

## Checklist

- [ ] Happy path and failure path both covered.
- [ ] Fakes live in the test, implementing the real ports.
- [ ] No real I/O outside `infrastructure` tests.
- [ ] `cargo test -p <crate>` is green.
