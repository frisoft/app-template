# IMPROVEMENTS — agent-friendliness & consistency backlog

Findings from a review of `AGENTS.md`, `.agents/skills/`, the manifests, Makefile,
flake, toolchain config and crate stubs. `make check` is currently green; the
items below are drift, dead references and workflow gaps, not failing builds.

Check a box when the fix lands. Keep the *Location*, *Problem* and *Fix* lines —
they are the acceptance criteria.

---

## High — broken / dangling references

- [x] **H1 — `checker` skill references a nonexistent CI workflow.** ✅ Done
  - Location: `.agents/skills/checker/SKILL.md:23`; also `AGENTS.md` §6 ("Everything CI runs") and §9 (`checker` row).
  - Problem: It tells the agent to read `.github/workflows/ci.yaml`, but there is **no `.github/` directory** at all. The skill cannot run as written. `AGENTS.md` implies a CI exists.
  - Fix: Make `make check` the single source of truth. Rewrite `checker` to run `make check` (drop the workflow-read step), and update the `AGENTS.md` §6/§9 wording. Alternatively, actually add `.github/workflows/ci.yaml` — but then keep the two in sync.
  - **Resolved:** Added `.github/workflows/ci.yaml` with three parallel jobs (`Format` → `make fmt-check`, `Clippy` → `make clippy`, `Test` → `make test`). CI is now the file `checker` reads, and both share the `make` targets, so the two cannot drift.

- [x] **H2 — README links outside the repository.** ✅ Done
  - Location: `README.md:6` (`../rust-presentation/slides.md`).
  - Problem: The path is outside the repo, so the link is dead for any agent or fresh clone.
  - Fix: Vendor the slides into the repo (e.g. `docs/`), replace with a stable external URL, or remove the sentence.
  - **Resolved:** Removed the external `slides.md` reference; the README now simply introduces the design ideas inline.

---

## Medium — agent-workflow inconsistencies

- [x] **M2 — Formatting command drift.** ✅ Done
  - Location: `.agents/skills/pusher/SKILL.md:43` vs `Makefile` / `AGENTS.md` §6.
  - Problem: `pusher` uses `cargo fmt -- --check`; the repo standard is `cargo fmt --all -- --check`.
  - Fix: Standardise on `cargo fmt --all -- --check` in `pusher`.
  - **Resolved:** `pusher` Step 3 now calls the `make fmt-check` / `make fmt` targets instead of spelling out a `cargo` invocation, so the skill and the Makefile cannot drift again.

- [ ] **M3 — Stale Redis/Docker copy-paste in `checker`.**
  - Location: `.agents/skills/checker/SKILL.md:28`.
  - Problem: "Do NOT set any environment variables (e.g., REDIS_URL, DATABASE_URL). Do NOT use docker" — this template has no Redis/Postgres/Docker. Leftover from another project; misleading about required setup.
  - Fix: Remove the Redis/DB/Docker specifics (or replace with whatever this repo actually needs, currently: nothing).

- [ ] **M4 — Toolchain not actually pinned; MSRV untested.**
  - Location: `rust-toolchain.toml` (`channel = "stable"`), `Cargo.toml` (`rust-version = "1.75"`), `flake.nix` comment, `AGENTS.md` §6.
  - Problem: `stable` floats (local `rustc` is 1.99.0). The flake comment claims a "pinned toolchain", but `fromRustupToolchainFile` with `stable` is not version-pinned. The declared MSRV 1.75 is enforced nowhere.
  - Fix: Pin a concrete version in `rust-toolchain.toml` (e.g. `channel = "1.75.0"`, or bump MSRV and pin to it), update the flake comment, and add an MSRV check if CI is added.

---

## Low — nits and redundancy

- [ ] **L1 — Lint attributes declared twice, not uniformly.**
  - Location: `Cargo.toml` `[workspace.lints.rust]` + `crates/{domain,application,infrastructure}/src/lib.rs` (`#![forbid(unsafe_code)]`, `#![warn(missing_docs)]`); `crates/cli/src/main.rs` has neither.
  - Problem: Redundant (workspace lints already cover it) and inconsistent across crates.
  - Fix: Pick one mechanism and apply it uniformly to every crate.

- [ ] **L2 — Typos in `pusher`.**
  - Location: `.agents/skills/pusher/SKILL.md` Step 2.
  - Problem: "main brnach", "feature brach".
  - Fix: Correct to "branch".

- [ ] **L3 — Aspirational adapters may confuse.**
  - Location: `AGENTS.md` §3/§4 (`http` / `worker`) vs `README.md` layout (only `cli`).
  - Problem: The extra adapters do not exist; an agent may hunt for them.
  - Fix: Add a short "(not included; add under `crates/`)" note, or align the two documents.

- [ ] **L4 — `write-tests` uses `#[tokio::test]` while `tokio` ships commented out.**
  - Location: `.agents/skills/write-tests/SKILL.md` vs `crates/application/Cargo.toml` / `crates/infrastructure/Cargo.toml`.
  - Problem: An agent can miss the "uncomment dev-dependency" step.
  - Fix: Note the dependency-uncomment step inline in the skill's test example.

---

## Verification

- [ ] Re-run `make check` after the changes above.
- [ ] Confirm every skill's file/path references resolve inside the repo.
- [ ] Confirm no document refers to CI unless `.github/workflows/` exists (see H1).
