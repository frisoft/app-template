# Rust DDD / Hexagonal Application Template

A workspace skeleton for building complex applications in Rust where the domain
rules live in the type system and I/O is pushed to the edges.

The design follows these ideas:

- **Domain-Driven Design** — the domain speaks the language of the business.
- **Onion / Hexagonal architecture** — dependencies point inward only.
- **Type-Driven Development (TyDD)** — make illegal states unrepresentable.
- **Functional core, imperative shell** — pure domain decisions, orchestrated I/O.
- **AI-friendly** — explicit types, clear boundaries, compiler-checked refactors.

This template ships the **structure and the rules**, not a domain model. The
`Payment` / `Money` snippets you will meet in the docs are illustrative examples
only; they are intentionally absent from the code.

## Layout

```
crates/
  domain/          pure types, invariants, domain workflows (no I/O)   — empty
  application/     application workflows + ports (owns all I/O)      — empty
  infrastructure/  adapters implementing the ports                     — empty
  cli/             driving adapter / composition root (`app`)          — empty
```

Each crate's `lib.rs` (and `application/src/ports.rs`) documents exactly what
belongs there and how to start.

See [`AGENTS.md`](AGENTS.md) for the full architecture contract. **Read it
before contributing.** Detailed, task-specific playbooks live in
[`.agents/skills/`](.agents/skills) and are loaded on demand by Pi (e.g.
`/skill:add-workflow`).

## Quick start

```bash
make check                 # fmt + clippy (-D warnings) + tests
cargo test --workspace
cargo run -p app-cli       # the composition root
```

If you use Nix, `direnv allow` (or `nix develop`) provides the toolchain. A
`rust-toolchain.toml` is provided for rustup users.
