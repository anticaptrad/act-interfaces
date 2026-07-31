# act-interfaces agent instructions

## Blacklisted operations and dependencies

- Do not run `git reset`, `git filter-repo`, or `git clean`.
- Do not run `rm` except when explicitly deleting known temporary or scratch files.
- `dotenv` is blacklisted. Do not install or use it.

## Repository role and invariants

- This repository owns versioned, language-neutral contracts. Runtime implementations belong elsewhere.
- Do not commit credentials, OAuth tokens, API keys, deployment secrets, raw upstream bodies, or generated authentication material.
- Preserve private-by-default publishing, explicit idempotency for mutations, redacted lifecycle events, and the canonical `@anticaptrad` channel boundary.
- Never silently remove required fields, action values, error codes, or event phases from an existing version. Breaking changes require a new version directory.
- Keep validation deterministic and dependency-light. Generated clients are outputs for `act-clients`, not hand-edited canonical sources here.

## Instruction discovery

Resolve `$PWD`, walk upward through every parent directory to the filesystem root, read every readable lowercase `agents.md` on that ancestor chain, and apply them root-to-leaf. Do not search siblings. Deduplicate resolved paths/inodes, avoid symlink cycles, and report unreadable files.

## Git and remote synchronization

Before editing, inspect status, branch, remotes, and the remote default branch. Fetch and prune before branching and again before pushing. Avoid rebase in favor of merge.

- Do not force-push or rewrite shared history.
- Do not bypass review or required CI.

## Semantic conflict resolution

Resolve conflicts by combining both sides' intent. Do not mechanically choose ours, theirs, current, or incoming. Preserve compatible schema evolution, action coverage, idempotency requirements, privacy controls, error envelopes, lifecycle-event semantics, tests, and documentation.

After resolving, reread every affected file, run `python3 tests/validate_schemas.py`, and search the full worktree for conflict markers:

```sh
grep -RInE '^(<<<<<<<|=======|>>>>>>>)' --exclude-dir=.git .
```
