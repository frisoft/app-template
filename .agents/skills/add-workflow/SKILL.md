---
name: add-workflow
description: Add a new application workflow end to end in this Rust DDD template — pure domain workflow, application workflow, ports, tests and adapter wiring. Use when implementing a new business operation or feature.
---

# Add an application workflow end to end

Rules live in `AGENTS.md`. Follow this order every time. Replace `<...>` with your
domain's names. The snippets are **illustrative examples**.

## The two workflows

Every business operation is split into a **domain workflow** and an
**application workflow**.

**Domain workflow (pure decision)** — `crates/domain/src/workflow/<name>.rs`

```rust,ignore
pub fn place_payment(
    state: DraftPayment,
    command: PlacePayment,
) -> Result<(PlacedPayment, Vec<DomainEvent>), DomainError>
```

No I/O, no `async`, deterministic. Returns new state + events.

**Application workflow (orchestration)** — `crates/application/src/workflow/<name>.rs`

```rust,ignore
pub async fn place_payment(
    command: PlacePayment,
    repository: &impl PaymentRepository,
    unit_of_work: &impl PaymentUnitOfWork,
) -> Result<PlacedPayment, PlacePaymentError> {
    let state = repository.load_draft(&command.id).await?;                   // I/O via port
    let (placed, events) = domain::workflow::place_payment(state, command)?; // pure decision
    unit_of_work.commit(&placed, &events).await?;                            // state + events, one tx
    Ok(placed)
}
```

The application workflow always: **load state → run the domain workflow →
persist state + publish events in one transaction** (unit of work / outbox).
Never publish an event for state that was not persisted.

## Steps

1. **Model the types** in `domain` (use the `model-domain-types` skill): the
   state(s) the operation starts from and transitions to, any new value objects,
   and the command.
2. **Write the pure domain workflow** in `crates/domain/src/workflow/`:
   `(State, Command) -> Result<(State, Vec<DomainEvent>), DomainError>`. Add new
   event and error variants.
3. **Test the domain workflow** with plain unit tests, including failure cases.
4. **Declare the ports** the application workflow needs in
   `crates/application/src/ports.rs` (load, save, publish, external calls). Add
   methods to existing ports when they belong to the same concept.
5. **Write the application workflow** in `crates/application/src/workflow/`,
   orchestration only: load → domain → commit. Add its error enum in
   `crates/application/src/error.rs`.
6. **Test the application workflow** with in-memory fakes; assert state and
   events are committed atomically and that failures commit nothing
   (see the `write-tests` skill).
7. **Implement/extend adapters** in `crates/infrastructure` for any new port
   (see the `add-adapter` skill).
8. **Expose it** through a driving adapter in `crates/cli` (or `http`): parse,
   wire, delegate. No logic.
9. **Run `make check`.** Fix every warning; clippy warnings are errors.

## Checklist

- [ ] No new dependency from an inner layer to an outer one.
- [ ] Domain workflow has no `async`, I/O or framework types.
- [ ] New domain concepts are newtypes/enums, not primitives or `bool`.
- [ ] Fallible operations return `Result`; no `unwrap` outside tests/`main`.
- [ ] Domain workflow is pure and unit-tested (happy + failure paths).
- [ ] Application workflow tested with fakes; commit is atomic.
- [ ] Public items documented; crate re-exports updated.
- [ ] `make check` is green.
