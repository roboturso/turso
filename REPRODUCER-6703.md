# Reproducer: #6703 — "immediate FK constrained failed" in fuzz test

https://github.com/tursodatabase/turso/issues/6703

## Environment

- Commit: d4d6777a2685ab9a3ffc8733329743fdfe000c45
- Platform: linux x86_64
- Reproduced at: 2026-07-02T13:07:44Z

## Reproduce

```sh
cd tests && SEED=1777992890350 cargo test -q --test fuzz_tests fk_deferred_constraints_and_triggers_fuzz_mvcc -- --exact fuzz_tests::fk_deferred_constraints_and_triggers_fuzz_mvcc && for i in $(seq 1 30); do cargo test -q --test fuzz_tests fk_deferred_constraints_and_triggers_fuzz_mvcc -- --exact fuzz_tests::fk_deferred_constraints_and_triggers_fuzz_mvcc || exit 1; done
```

Fails on the current tree; passes once the bug is fixed.

## What happens

The failure no longer manifests on the current tree — the reported seed passes and 130 stress runs with random seeds are all green, while the same stress loop does fail intermittently at the original commit a5c901b763; it appears to have been fixed by subsequent FK fixes such as f81a6f0b2 ("Fix ignored updates and self-referential updates incorrectly mutating FK violation counters").

## Observed failure

RoboTurso ran the command above and observed:

```text

```

## Files

- (none)