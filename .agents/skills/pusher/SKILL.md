---
name: pusher
description: Stages, commits, and pushes changes to git. Runs cargo fmt before staging to ensure code is properly formatted. Creates a branch if on main and creates a PR if none exists.
---

# Pusher Skill

This skill stages, commits, and pushes changes to git.

## Essential Rule

**NEVER commit or push directly to the main branch.** Always create a feature branch first.

## Description

Format the source code with cargo fmt, then stage changes with git add, create a commit with git commit, and push to the remote with git push. Ensures changes are never pushed directly to main branch - creates a branch if on main, and creates a PR if none exists for the branch.

## When to use

Use this skill when you want to commit and push changes to the repository.


## Implementation

When this skill is invoked, perform the following steps:

### Step 1: Check current branch
Run `git branch --show-current` to determine the current branch.

### Step 2: Handle main branch (REQUIRED)
If on main branch:
- Changes to main brnach are **NOT allowed**
- Create a feature brach but do not ask the user for a new branch name
- Generate a branch name automatically based on the changes:
  - Check git status or diff to understand what changed
  - Use format: `<type>/<short-description>` where type is: feature/, fix/, refactor/, docs/, test/
  - Example: if adding tests, use `test/add-phase5-integration-tests`
- Exception: plan-only changes. If the changes contain only a new plan file `plans/NNN-<slug>.md` (no other code changes), the branch is about the **feature/fix described in the plan**, not about adding the plan file. Derive `<slug>` from the plan filename and `<type>` from the plan content/issue nature (`feature/`, `fix/`, `refactor/`, etc.) — e.g. `plans/013-patch-product-api.md` → `feature/patch-product-api`. Never use `docs/add-plan-...` or `docs/013-...`.
- Create and checkout the new branch with `git checkout -b <branch-name>`
- Set upstream with `git push -u origin <branch-name>`

### Step 3: Format the source code
Run `cargo fmt -- --check` to check if the code is formatted correctly.

If the code is not formatted correctly, run `cargo fmt` to format it automatically.

### Step 4: Stage the changes
Run `git add -A` to stage all changes.

### Step 5: Check the status
Run `git status` to see what will be committed.

### Step 6: Create a commit
Run `git commit` with an auto-generated commit message:
- Derive the message from the changes: summarize what files were changed and what was done
- Use conventional commit format: `<type>: <description>` where type is: feat, fix, refactor, docs, test, chore
- Exception: plan-only changes. If the commit contains only a new plan file `plans/NNN-<slug>.md`, the message describes the **feature/fix in the plan** (parsed from the plan header `# PLAN — <Title> (#<N>)`), not the plan file itself — e.g. `feat: patch product API (#37)`, never `docs: add plan 013 ...`.
- Examples:
  - "test: add phase5 integration tests for admin auth and blog multilingual"
  - "refactor: centralize validation code in admin_auth middleware"
  - "feat: add lang Tera function for view context"

### Step 7: Push to the remote
Run `git push` to push the commit to the remote repository.

### Step 8: Create a PR if none exists
Check if a PR already exists for the branch:
- Run `gh pr list --head <branch-name>` to check for existing PR
- If no PR exists, create one automatically:
  - Generate a title from the branch name (convert to title case, e.g., `test/add-phase5-integration-tests` → "Add phase5 integration tests")
  - Exception: plan-only branches (only a new `plans/NNN-<slug>.md`). The PR is about the **feature/fix in the plan**, not about adding the plan: title = the feature/fix title from the plan header (e.g. `Patch product API`), body summarizes the feature/fix, links the issue (`For #<N>` / `Closes #<N>`), and mentions the plan file path.
  - Generate a body summarizing the changes: list the files changed and summarize the commit message
  - Use `gh pr create --title "<title>" --body "<body>"`

## Output

- Print the result of each step
- If any step fails, print the error and stop
- Print the commit SHA after a successful push
- Print the PR URL after creating a PR
