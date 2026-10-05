---
name: model-domain-types
description: Model Rust domain types with Type-Driven Development — newtypes, typestate, enums, Option/Result, immutability. Use when adding or changing value objects, entities, commands, domain events, or domain errors in the domain crate.
---

# Model domain types (Type-Driven Development)

Always-on rules live in `AGENTS.md` (§2 Golden rules, §5 Layer rules). This skill
is the detailed playbook for the `domain` crate.

The code snippets below are **illustrative examples**; copy the *pattern*, not
the names.

## Reach for these tools first

**Newtype / parse-don't-validate.** Never let a raw `u64`/`String` mean a domain
concept. Wrap it and validate in the constructor; afterwards the type *is* the
proof.

```rust,ignore
pub struct PaymentId(u64);                 // field private
impl PaymentId {
    pub fn new(raw: u64) -> Result<Self, PaymentIdError> { /* validate once */ }
    pub fn as_u64(self) -> u64 { self.0 }  // explicit escape hatch
}
```

Distinct newtypes (`OrderId`, `CustomerId`, `AccountId`, ...) cannot be swapped by
accident, because they are different types. Validation happens **once**, at the
boundary.

**Typestate for lifecycles.** Encode each state as its own type; transitions
consume `self` and return the next type. Illegal transitions fail to compile.

```rust,ignore
DraftPayment --place()--> PlacedPayment --pay()--> PaidPayment
```

Validation belongs in the transition that can fail, not in the constructor if the
value is legitimately incomplete at that point.

**Enums for "one of".** A value is one variant *or* another. Match exhaustively;
never add a `_ =>` arm that silently swallows a new variant.

```rust,ignore
enum Fee { FlatPennies(u64), Percentage(f64) }
```

Adding a variant must break every workflow that has to change — that is the
feature.

**`Option` for absence, `Result` for failure.** Both are in the signature, so
callers cannot forget them. No sentinel values, no `-1`, no empty strings, no
hidden exceptions.

**Value objects are immutable.** Return new values instead of mutating. Prefer
`&self` for reads and consuming `self` for transitions.

## Domain workflow shape

A domain workflow is a pure function:

```
(State, Command) -> Result<(State, Vec<DomainEvent>), DomainError>
```

- No I/O, no `async`, no clock, no randomness, no framework types.
- Deterministic: same `(state, command)` always yields the same result.
- Returns the new state **and** the facts (events) that describe what happened.
- Signature stays in the domain's vocabulary; callers must handle the `Result`.

## Checklist

- [ ] Each domain primitive is a newtype with a private field and a fallible
      constructor.
- [ ] Invalid combinations are impossible by construction (typestate/enums), not
      checked at runtime with `bool` flags.
- [ ] Every fallible operation returns `Result`; no `unwrap`/`expect`/`panic!`.
- [ ] No `async`, I/O, clock or randomness anywhere in the module.
- [ ] Public types have `///` docs and are re-exported from `crates/domain/src/lib.rs`.
- [ ] Unit tests cover each branch, including failure paths (see the
      `write-tests` skill).

## Adding these to the crate

1. Create `crates/domain/src/<concept>.rs`.
2. Declare it in `crates/domain/src/lib.rs` and re-export the public surface.
3. Put a pure domain workflow in `crates/domain/src/workflow/<name>.rs`.
4. Continue with the `add-workflow` skill to wire it through the application.
