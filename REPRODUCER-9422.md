# Reproducer: #9422 — sqlite/conformance: csv01.test fails all 467 tests because the csv module is never registered

https://github.com/tursodatabase/turso/issues/9422

## Environment

- Commit: a3f3748ed074d245ff3e32661b0d40d19d669deb
- Platform: linux x86_64
- Reproduced at: 2026-09-30T13:40:49Z

## Reproduce

```sh
make -C bindings/tcl build >/dev/null 2>&1 && tclsh bindings/tcl/test_csv_module.tcl
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed: `load_static_extension db csv` does not register the csv module, so every `CREATE VIRTUAL TABLE ... USING csv(...)` fails with 'no such module: csv' and all 467 csv01 tests fail.

## Observed failure

RoboTurso ran the command above and observed:

```text
FAIL  csv module works on connection 0
      want: abcd efgh
      got:  ERROR: Parse error: no such module: csv
FAIL  csv module works on connection 1
      want: abcd efgh
      got:  ERROR: Parse error: no such module: csv
FAIL  csv module works on connection 2
      want: abcd efgh
      got:  ERROR: Parse error: no such module: csv
FAIL  bare header parameter means header=true
      want: a b
      got:  ERROR: Parse error: no such module: csv
PASS  unknown extension names are still accepted

1 passed, 4 failed
```

## Files

- `bindings/tcl/test_csv_module.tcl`