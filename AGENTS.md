# AGENTS.md

Contract for any agent (human or AI) working in this repository.
**Read it fully before touching code. If a change would violate this document,
stop and ask rather than guess.**

Detailed, task-specific guidance lives in [skills](#9-skills). Load the relevant
one when the task calls for it.

---

## 0. About this template

This repository is a **skeleton**: it ships the architecture, the rules and the
tooling — **not a domain model**.

- The layer crates are intentionally empty. Each `lib.rs` documents what belongs
  in it; `crates/application/src/ports.rs` is the placeholder for ports.
- Every mention of `Payment`, `PaymentId`, `Money`, `place_payment`,
  `DraftPayment`, etc. in this document and in the skills is an **illustrative
  example**. Those types are **deliberately not present in the codebase**. Do not
  assume they exist, do not re-create them under those names, and do not treat
  them as the application's domain.
- When you add a feature, translate the *pattern* (newtypes, typestate, ports,
  workflows, the dependency rule) onto **your own** domain vocabulary.

---

## 1. What we are optimizing for

Two qualities, weighed equally. Every change must preserve or improve both.

- **Correctness** — the software does what the domain says and *cannot* be made
  to do otherwise. In this kind of application a bug is not a glitch, it is a
  wrong decision.
- **Maintainability** — the software can be changed easily and safely for years,
  by people who did not write it.

We get both from the architecture (DDD + Onion/Hexagonal) **plus** Rust's type
system.

> When speed and safety appear to conflict here, safety wins. Prefer the
> explicit, boring, compiler-checked solution.

---

## 2. Golden rules (non-negotiable)

1. **Dependencies point inward only.** `domain ← application ← infrastructure ← cli`.
   Nothing inward may `use` something outward.
2. **The domain is pure.** No I/O, no `async`, no clocks, no randomness, no
   network/db, no framework types. Only types, invariants and pure functions.
3. **Make illegal states unrepresentable.** Model with newtypes, enums and the
   typestate pattern instead of `bool` flags, `Option` soup and primitives.
4. **Parse, don't validate.** Validate once at the boundary and return a type
   that carries the proof. Never re-check invariants deeper in the domain.
5. **Errors are data.** Fallible operations return `Result`. Library code must
   never `panic!`, `unwrap`, `expect` or index out of bounds on bad input.
6. **Domain workflows are pure functions.** `(State, Command) -> Result<(State, Events), DomainError>`.
7. **Application workflows own I/O**, depend only on ports, and are tested with
   fakes. They contain no business rules.
8. **The compiler is the first reviewer.** `make check` must pass before work is
   considered done: `fmt`, `clippy` (`all` + `pedantic`, `-D warnings`), `test`.
9. **No business logic in adapters.** A CLI/HTTP/DB adapter parses, wires and
   delegates. Nothing else.
10. **Document the "why".** Public items carry `///` docs. Comments explain
    intent and invariants, not mechanics.

---

## 3. Architecture

```
            ┌─────────────────────────────────────────────┐
 driving    │  cli  /  http  /  worker      (adapters)     │
 adapters   └───────────────┬─────────────────────────────┘
                            │ calls
            ┌───────────────▼─────────────────────────────┐
            │  application                                 │
            │  · application workflows, owns I/O           │
            │  · ports (traits) implemented outside        │
            └───────────────┬─────────────────────────────┘
                            │ calls
            ┌───────────────▼─────────────────────────────┐
            │  domain                                      │
            │  · types, invariants, business rules         │
            │  · pure workflows: (State, Cmd) -> (State,Ev)│
            └─────────────────────────────────────────────┘
                            ▲
        driven adapters ────┘  implement ports (db, http client, bus)
```

- **Domain** — the business, expressed as types and pure functions. Deterministic
  and trivially testable. This is the most valuable and the most stable layer.
- **Application** — orchestrates the domain and owns all I/O. Declares the
  **ports** the outside world must implement.
- **Infrastructure** — **driven adapters** that implement the ports (repository,
  unit of work, event bus, external services).
- **CLI / HTTP / worker** — **driving adapters** that call the application. The
  binary is the composition root: the only place allowed to know every layer.
- **Dependency direction is enforced by the crate graph at compile time.**

Every business operation is split into a **domain workflow**
`(State, Command) -> Result<(State, Events), Error>` and an **application
workflow** that does `load → domain → persist state + events (one tx)`. The full
recipe is in the [`add-workflow` skill](#9-skills).

---

## 4. Repository map

```
Cargo.toml                     workspace: members, shared deps, profiles
rust-toolchain.toml            pinned stable toolchain + rustfmt/clippy
Makefile                       `make check` runs fmt-check + clippy + test
flake.nix / .envrc             optional Nix dev shell (same toolchain)
.agents/skills/                task-specific agent skills (see §9)
plans/                         issue plans, NNN-slug.md (see §9 gh-issues)

crates/
  domain/                      app-domain         (lib: `domain`)           — empty
    src/lib.rs                 what belongs here + how to start
  application/                 app-application    (lib: `application`)      — empty
    src/lib.rs                 layer overview
    src/ports.rs               where port traits go (documented placeholder)
  infrastructure/              app-infrastructure (lib: `infrastructure`)  — empty
    src/lib.rs                 where adapters go
  cli/                         app-cli            (bin: `app`)              — empty
    src/main.rs                composition root
```

The crates are **empty on purpose**. As you add workflows the tree grows,
typically like this (names are yours to choose):

```
crates/domain/src/value_object.rs          newtype + invariants + unit tests
crates/domain/src/entity.rs                typestate transitions
crates/domain/src/event.rs                 domain events
crates/domain/src/error.rs                 domain errors
crates/domain/src/workflow/<name>.rs        pure domain workflow
crates/application/src/ports.rs            trait definitions
crates/application/src/workflow/<name>.rs   application workflow
crates/application/src/error.rs            per-workflow error enums
crates/infrastructure/src/<adapter>.rs     port implementations
crates/cli/src/main.rs                     composition root
```

There is no `Payment` module and there should not be one unless *your* domain is
payments.

---

## 5. Layer rules — what belongs where

| Concern | domain | application | infrastructure | cli |
|---|---|---|---|---|
| Business rules / invariants | ✅ | ❌ | ❌ | ❌ |
| Value objects, entities, typestates | ✅ | ❌ | ❌ | ❌ |
| Domain events | ✅ | ✅ (pass through) | ✅ (publish) | ❌ |
| Pure functions | ✅ | ❌ | ❌ | ❌ |
| Port (trait) definitions | ❌ | ✅ | ❌ | ❌ |
| Orchestration / I/O / `async` | ❌ | ✅ | ✅ | ✅ |
| Concrete DB / HTTP / bus code | ❌ | ❌ | ✅ | ❌ |
| Argument parsing / wiring | ❌ | ❌ | ❌ | ✅ |
| `unwrap` / `expect` | ❌ | ❌ | ❌¹ | ✅² |
| `panic!` | ❌ | ❌ | ❌ | ✅² |

¹ Only on a poisoned `Mutex` (a programmer error), and prefer not to.
² Only at the very top of `main`, and prefer returning `anyhow::Result` instead.

Allowed crate dependencies (already declared in the manifests):

| crate | may depend on |
|---|---|
| `domain` | (nothing in the workspace) |
| `application` | `domain` |
| `infrastructure` | `domain`, `application` |
| `cli` | all of the above |

---

## 6. Commands

```bash
make check          # fmt-check + clippy (-D warnings) + tests  ← run before done
make fmt            # cargo fmt --all
make clippy         # cargo clippy --workspace --all-targets -- -D warnings
make test           # cargo test --workspace
make build          # release build
make run            # cargo run -p app-cli

cargo check --workspace --all-targets   # fast feedback while iterating
cargo test -p <crate>                   # one crate
cargo doc --workspace --no-deps --open  # docs
```

Lint policy lives in `[workspace.lints]` in the root `Cargo.toml`; every crate
opts in with `[lints] workspace = true`. The policy enables the default clippy
set (`all`) **plus** `pedantic`, and denies broken intra-doc links. Because the
levels are `warn` in the manifest and the Makefile passes `-D warnings`, a plain
`cargo clippy` reports them while `make check` makes them fatal. A handful of
pedantic lints that fight this codebase's explicit, documented style are opted
out centrally (see the comments in `Cargo.toml`) — do not `#[allow]` them at the
call site instead.

Environment: `rust-toolchain.toml` pins stable for rustup; `flake.nix` +
`.envrc` provide the same via Nix. Use whichever is available.

---

## 7. Anti-patterns — do not do these

- ❌ Calling the database, HTTP, the clock or a random generator from the domain.
- ❌ Returning `bool` where an enum or `Result` is meant.
- ❌ `String`/`u64` parameters that mean different domain concepts.
- ❌ Public struct fields that let callers bypass invariants.
- ❌ `unwrap()`, `expect()`, `panic!`, `unreachable!()` on domain data.
- ❌ A `_ =>` match arm that hides a new enum variant.
- ❌ Business rules inside an adapter, a `main`, or a SQL query.
- ❌ A port trait defined in `infrastructure` and consumed inward.
- ❌ Publishing an event **after** committing state in a separate transaction.
- ❌ Adding `unsafe` (forbidden via `#![forbid(unsafe_code)]`).
- ❌ Silencing clippy with `#[allow(...)]` instead of fixing the code (the
documented `[workspace.lints]` opt-outs are the place to relax a lint).
- ❌ Copying the example names (`Payment`, `Money`, ...) into the real domain.

---

## 8. Definition of Done

A change is done only when all of the following hold:

- [ ] `make check` passes (format, clippy with `-D warnings`, all tests).
- [ ] New behaviour is covered by tests at the lowest appropriate layer.
- [ ] The dependency rule still holds; no outward dependency added inward.
- [ ] Types make illegal states unrepresentable (no new primitive obsession).
- [ ] Errors are values; no new panics in library code.
- [ ] Public API documented and re-exported from the crate root.
- [ ] `AGENTS.md` / `README.md` updated if a rule, structure or command changed.

---

## 9. Skills

Skills hold the detailed, task-specific guidance so this file stays short. Pi
loads a skill when its description matches the task, or you can force it with
`/skill:<name>`. Each lives in `.agents/skills/<name>/SKILL.md`.

| Skill | Load it when |
|---|---|
| `model-domain-types` | Adding or changing value objects, entities, commands, domain events or domain errors. |
| `add-workflow` | Implementing a new business operation/feature end to end. |
| `add-adapter` | Integrating a technology, or persisting/loading state through a port. |
| `write-tests` | Adding tests or deciding where a test belongs. |
| `rust-conventions` | Unsure about naming, errors, derives, async boundaries, docs or dependencies. |
| `gh-issues` | Listing, picking or planning a **GitHub** issue via the `gh` CLI. |
| `pusher` | Staging, committing, branching and pushing changes (never to `main`). |
| `checker` | Running the CI checks locally before pushing. |

---

## 10. Working with an AI agent

- **Let the compiler lead.** Change a type, then follow the compiler errors —
  that is fearless refactoring. Do not patch call sites by hand first.
- **Small steps, `cargo check` often.** Keep the workspace compiling.
- **Trust `clippy`.** It understands types, ownership and data flow. Treat its
  output as review comments, not noise.
- **Prefer the boring, explicit solution.** Explicit signatures are how agents
  and humans both understand the code.
- **Do not invent architecture on the fly.** If the existing pattern does not fit
  the task, propose a change to this document in the same PR.
- **Do not resurrect the examples.** `Payment` / `Money` illustrate the patterns;
  your code must use your own domain names.

> The compiler is a free, fast, deterministic reviewer inside the agent loop.
> Keep it green.
