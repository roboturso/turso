# Reproducer: #3729 — Checkpoint database on upon `.exit`

https://github.com/tursodatabase/turso/issues/3729

## Environment

- Commit: de92295193ed43eef5364368c4c82d840428c274
- Platform: linux x86_64
- Reproduced at: 2026-07-12T16:52:49Z

## Reproduce

```sh
cargo build --bin tursodb && uv sync --all-extras --dev --package turso_test -q && uv run --project limbo_test test-exit-checkpoint
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed: the `.exit` command calls std::process::exit without closing the connection (unlike `.quit`), so the WAL is never checkpointed into the main database file and copying/moving just the .db file silently loses all data.

## Observed failure

RoboTurso ran the command above and observed:

```text
Installed 4 packages in 7ms
[2;36m[19:52:49][0m[2;36m [0m[1;31mERROR[0m expected [1;36m3[0m rows in t31 from the      ]8;id=15754510;file:///home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-3729/testing/cli_tests/test_exit_checkpoint.py\[2mtest_exit_checkpoint.py[0m]8;;\[2m:[0m]8;id=15754511;file:///home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-3729/testing/cli_tests/test_exit_checkpoint.py#55\[2m55[0m]8;;\
[2;36m           [0mcopied database file, got output: [32m'  × [0m    [2m                          [0m
[2;36m           [0m[32mParse error: no such table: t31\n\n'[0m       [2m                          [0m
[2;36m          [0m[2;36m [0m[1;31mERROR[0m `.exit` did not checkpoint the WAL   ]8;id=15754517;file:///home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-3729/testing/cli_tests/test_exit_checkpoint.py\[2mtest_exit_checkpoint.py[0m]8;;\[2m:[0m]8;id=15754518;file:///home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-3729/testing/cli_tests/test_exit_checkpoint.py#56\[2m56[0m]8;;\
[2;36m           [0minto the main database file [1m([0missue #[1;36m3729[0m[1m)[0m  [2m                          [0m
```

## Files

- `testing/cli_tests/test_exit_checkpoint.py`
- `testing/pyproject.toml`