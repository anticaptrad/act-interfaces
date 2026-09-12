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

## Repository-local Git worktrees

- Create or use a Git worktree only when the human operator explicitly authorizes it for the current task. Concurrency or a dirty checkout is not permission by itself.
- Put every authorized worktree at `<repository-root>/tmp/worktrees/<name>`; from the repository root, use `./tmp/worktrees/<name>`. Never place worktrees beside repositories or organization directories.
- Keep `tmp`, `temp`, `tmp/worktrees`, and `temp/worktrees` ignored in the repository-root `.gitignore`. Do not commit files from those directories.
- Relocate or remove a worktree only when the operator explicitly requests it. Before removal, preserve and publish intended changes, verify its commit is represented on the target branch, and confirm there are no tracked, untracked, ignored-sensitive, or in-use files that must survive. Remove it with `git worktree remove <path>` without `--force`; never delete a worktree directory with `rm`.
