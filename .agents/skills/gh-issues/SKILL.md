---
name: gh-issues
description: Use ONLY for GitHub (gh CLI) issues in this repository — e.g. "list open GitHub issues", "gh issue list", "pick a GitHub issue", "turn a GitHub issue into a plan". Lists open GitHub issues via the `gh` CLI, asks which one to work on, and when one is selected (only on main) creates a plan file in plans/ and pushes it via the pusher skill. Do NOT use this skill for general problem solving, troubleshooting, debugging, code review findings, or any issue that is not a tracked GitHub issue.
---

# GitHub Issues Skill

This skill manages **GitHub issues tracked in this repository** (via the `gh` CLI): list open GitHub issues, let the user pick one, then draft a plan file for it and push the plan. It is not for generic "issues" such as bugs you find while debugging, review comments, or configuration problems unless those are tracked as GitHub issues.

## Essential Rules

- **Never do work on a GitHub issue unprompted.** Always list open GitHub issues first and wait for the user to pick one.
- **Plan files are only created when on `main`.** If not on `main`, list issues but stop before creating any file.
- **Never commit or push directly to `main`.** Pushing is delegated to the `pusher` skill (which creates a feature branch + PR when on `main`).
- Plans live in `plans/` at the repo root, named `NNN-short-slug.md` (e.g. `013-patch-product-api.md`).

## When to use

Use this skill ONLY when the user explicitly refers to **GitHub issues** in this repo — e.g. they want to see open GitHub issues (`gh issue list`), pick a tracked GitHub issue to work on, or turn a GitHub issue into an implementation plan.

Do NOT use this skill for:
- general bugs, errors, or problems the user is troubleshooting
- code review findings or lint/CI failures
- any "issue" that is not a GitHub issue in this repository

## Implementation

When this skill is invoked, perform the following steps:

### Step 1: Check current branch
Run `git branch --show-current` to determine the current branch. Record whether it is `main` (`is_main=true/false`).
Do NOT switch branches yourself. This flag gates Step 5.

### Step 2: List open GitHub issues
Run:
```bash
gh issue list --state open --limit 30
```
If there are no open GitHub issues, report that and stop.

### Step 3: Present GitHub issues and ask which to work on
Print each open issue as:
```
#<number> — <title> (<labels or "no labels">, updated <date>)
```
Then fetch a one-line summary for context if needed (do NOT dump full bodies for all issues yet).

**Stop here and ask the user which issue to work on. Wait for their reply. Do not proceed to Step 4 without an explicit selection.**

### Step 4: Load the selected issue
Once the user picks a GitHub issue number `<N>`:
1. Re-check the branch: `git branch --show-current`.
2. If NOT on `main`, stop: report "Not on `main` (on `<branch>`), so no plan file will be created per this skill. Switch to `main` and re-invoke." Do not create any file.
3. If on `main`, fetch full details:
```bash
gh issue view <N> --json number,title,body,labels,assignees,milestone,comments
```

### Step 5: Create the plan file in plans/
Only runs when `is_main=true` and a GitHub issue was explicitly selected.

1. Determine the next plan number: `ls plans/ | sort` and take max + 1, zero-padded to 3 digits (e.g. max `012-...` → next `013-...`).
2. Derive the slug from the issue title: lowercase, non-alphanumeric → `-`, trim dashes (e.g. "Patch product API" → `patch-product-api`).
3. Path: `plans/<NNN>-<slug>.md` (e.g. `plans/013-patch-product-api.md`).
4. Write the plan with this structure:
   - `# PLAN — <Issue title> (#<N>)`
   - `## Goal` — what the issue asks for, plus explicit out-of-scope items. Quote/link the issue (`gh issue view <N>` URL).
   - `## Background` — relevant current behaviour, file pointers.
   - `## Approach` — design respecting `AGENTS.md` (DDD layer table: domain ← application ← infrastructure ← cli; newtype/typestate patterns; thin cli adapters; port traits in application, adapters in infrastructure).
   - `## Inventory of touch points` — concrete files to change / add.
   - `## Steps` — ordered checkboxes, implementation order.
   - `## Testing strategy` — per `AGENTS.md` and the `write-tests` skill (domain unit tests, application workflow tests with in-memory fakes, infrastructure integration tests against the real backing technology; run `make check` before finishing).
   - `## Rollback` — how to safely revert.
   - `## Checklist summary`
5. Do NOT implement the issue — only write the plan file.

### Step 6: Push the plan via the pusher skill
The branch, commit, and PR are about the **feature/fix defined in the plan**, not about the plan file itself. Do NOT use `docs/...` branch names or `docs: add plan ...` messages.

Follow the `pusher` skill (`.agents/skills/pusher/SKILL.md`) with these overrides:
- Branch name: `<type>/<slug>` derived from the issue, where `<slug>` is the slug from Step 5.2 and `<type>` reflects the issue nature (`feature/`, `fix/`, `refactor/`, etc. — e.g. `feature/patch-product-api`). Create and checkout this branch explicitly instead of letting `pusher` infer a `docs/...` name from the diff.
- `make check` (plan files are markdown — no formatting needed, but run the check so the tree is clean).
- Stage, commit with a conventional message describing the feature/fix, e.g. `feat: patch product API (#37)` — not `docs: add plan 013 ...`. The plan file is included in the commit but the message describes the issue's feature/fix.
- Push and open a PR (per `pusher`) with title = the issue title (e.g. `Patch product API`) and body summarizing the feature/fix, linking the issue (`Closes #<N>` only if implementation is included, otherwise `For #<N>`) and mentioning the plan file path. Always end the PR description with the line `This pull request resolves #<N>` (replacing `<N>` with the issue number) so the PR is linked to the issue and the issue is closed automatically when the PR is merged. Report the branch, commit SHA, and PR URL.

## Output

- Print the current branch and the open GitHub issue list.
- Explicitly ask which GitHub issue to work on and wait.
- After selection: print the plan file path, or the reason for stopping (not on `main`).
- After pushing: print the commit SHA, branch name, and PR URL (per `pusher` skill).
- If any step fails, print the error and stop.
