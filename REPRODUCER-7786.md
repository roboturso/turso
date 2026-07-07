# Reproducer: #7786 — JSON functions corrupt multi-byte UTF-8

https://github.com/tursodatabase/turso/issues/7786

## Environment

- Commit: c5d78b048156bfb9e79782e17db019e700564f54
- Platform: linux x86_64
- Reproduced at: 2026-07-07T18:32:24Z

## Reproduce

```sh
make -C testing/sqltests run-rust ARGS='--filter "*issue-7786*" --snapshot-filter __never__'
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed: json_object/json_array/json_group_array mangle multi-byte UTF-8 when the string also contains a control character, because the TEXT5 serialization path in core/json/jsonb.rs decodes bytes as latin1 via `string.push(ch as char)`.

## Observed failure

RoboTurso ran the command above and observed:

```text
make: Entering directory '/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7786/testing/sqltests'
cargo build 
   Compiling turso_ext v0.7.0-pre.18 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7786/extensions/core)
   Compiling turso_core v0.7.0-pre.18 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7786/core)
   Compiling turso_sync_engine v0.7.0-pre.18 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7786/sync/engine)
   Compiling turso_sdk_kit v0.7.0-pre.18 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7786/sdk-kit)
   Compiling turso_sync_sdk_kit v0.7.0-pre.18 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7786/sync/sdk-kit)
   Compiling turso v0.7.0-pre.18 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7786/bindings/rust)
   Compiling test-runner v0.1.0 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7786/testing/sqltests)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 27.61s
cargo run --bin test-runner  run tests --backend rust  --filter "*issue-7786*" --snapshot-filter __never__
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
     Running `/home/penberg/src/tursodatabase/roboturso-turso/target/debug/test-runner run tests --backend rust --filter '*issue-7786*' --snapshot-filter __never__`
Generating integrity-check fixtures...
Generating default databases...
[1mtests/json/json_multibyte_utf8_escape.sqltest[0m
  [[31mFAIL[0m] json-object-multibyte-utf8-with-carriage-return-issue-7786 [2m(4.62ms)[0m
  [[31mFAIL[0m] json-array-multibyte-utf8-with-tab-issue-7786 [2m(3.84ms)[0m
  [[31mFAIL[0m] json-object-multibyte-utf8-with-newline-issue-7786 [2m(3.49ms)[0m
  [[31mFAIL[0m] json-object-three-byte-utf8-with-newline-issue-7786 [2m(2.69ms)[0m
  [[31mFAIL[0m] json-group-array-multibyte-utf8-with-newline-issue-7786 [2m(5.59ms)[0m

[1m[31mFailures:[39m[0m

[31m── json-object-multibyte-utf8-with-carriage-return-issue-7786 (tests/json/json_multibyte_utf8_escape.sqltest) - :memory:[39m
   --- expected
   +++ actual
   -{"k":"é\r"}
   +{"k":"Ã©\r"}

[31m── json-array-multibyte-utf8-with-tab-issue-7786 (tests/json/json_multibyte_utf8_escape.sqltest) - :memory:[39m
   --- expected
   +++ actual
   -["ä\t"]
   +["Ã¤\t"]

[31m── json-object-multibyte-utf8-with-newline-issue-7786 (tests/json/json_multibyte_utf8_escape.sqltest) - :memory:[39m
   --- expected
   +++ actual
   -{"k":"ä\n"}
   +{"k":"Ã¤\n"}

[31m── json-object-three-byte-utf8-with-newline-issue-7786 (tests/json/json_multibyte_utf8_escape.sqltest) - :memory:[39m
   --- expected
   +++ actual
   -{"k":"€\n"}
   +{"k":"â¬\n"}

[31m── json-group-array-multibyte-utf8-with-newline-issue-7786 (tests/json/json_multibyte_utf8_escape.sqltest) - :memory:[39m
   --- expected
   +++ actual
   -["ä\n"]
   +["Ã¤\n"]

[1mSummary:[0m
  [31m5 failed[39m
  [2mTotal time: 16.65ms[0m

[1m[31mSome tests failed.[39m[0m
make: *** [Makefile:69: run-rust] Error 1
make: Leaving directory '/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7786/testing/sqltests'
```

## Files

- `testing/sqltests/tests/json/json_multibyte_utf8_escape.sqltest`