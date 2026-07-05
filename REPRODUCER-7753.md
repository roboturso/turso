# Reproducer: #7753 — JSON parser rejects JSON5 whitespace and line continuations that SQLite accepts

https://github.com/tursodatabase/turso/issues/7753

## Environment

- Commit: d4d6777a2685ab9a3ffc8733329743fdfe000c45
- Platform: linux x86_64
- Reproduced at: 2026-07-05T08:43:50Z

## Reproduce

```sh
cd testing/sqltests && cargo run -q --bin test-runner -- run tests/json/json5-whitespace.sqltest --backend rust
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed: all nine JSON5 whitespace/line-continuation inputs raise 'malformed JSON' in tursodb while sqlite3 accepts them, matching the reported gaps in core/json/jsonb.rs.

## Observed failure

RoboTurso ran the command above and observed:

```text
[1mtests/json/json5-whitespace.sqltest[0m
  [[31mFAIL[0m] json5-line-continuation-crlf             [2m(4.02ms)[0m
  [[31mFAIL[0m] json5-line-continuation-cr               [2m(4.31ms)[0m
  [[31mFAIL[0m] json5-whitespace-vertical-tab            [2m(5.36ms)[0m
  [[31mFAIL[0m] json5-whitespace-line-separator          [2m(5.46ms)[0m
  [[31mFAIL[0m] json5-whitespace-zs-en-quad              [2m(5.42ms)[0m
  [[31mFAIL[0m] json5-whitespace-nbsp                    [2m(5.49ms)[0m
  [[31mFAIL[0m] json5-whitespace-form-feed               [2m(4.82ms)[0m
  [[31mFAIL[0m] json5-whitespace-bom                     [2m(5.78ms)[0m
  [[31mFAIL[0m] json5-whitespace-paragraph-separator     [2m(7.13ms)[0m

[1m[31mFailures:[39m[0m

[31m── json5-line-continuation-crlf (tests/json/json5-whitespace.sqltest) - :memory:[39m
   expected success but got error: Parse error: malformed JSON

[31m── json5-line-continuation-cr (tests/json/json5-whitespace.sqltest) - :memory:[39m
   expected success but got error: Parse error: malformed JSON

[31m── json5-whitespace-vertical-tab (tests/json/json5-whitespace.sqltest) - :memory:[39m
   expected success but got error: Parse error: malformed JSON

[31m── json5-whitespace-line-separator (tests/json/json5-whitespace.sqltest) - :memory:[39m
   expected success but got error: Parse error: malformed JSON

[31m── json5-whitespace-zs-en-quad (tests/json/json5-whitespace.sqltest) - :memory:[39m
   expected success but got error: Parse error: malformed JSON

[31m── json5-whitespace-nbsp (tests/json/json5-whitespace.sqltest) - :memory:[39m
   expected success but got error: Parse error: malformed JSON

[31m── json5-whitespace-form-feed (tests/json/json5-whitespace.sqltest) - :memory:[39m
   expected success but got error: Parse error: malformed JSON

[31m── json5-whitespace-bom (tests/json/json5-whitespace.sqltest) - :memory:[39m
   expected success but got error: Parse error: malformed JSON

[31m── json5-whitespace-paragraph-separator (tests/json/json5-whitespace.sqltest) - :memory:[39m
   expected success but got error: Parse error: malformed JSON

[1mSummary:[0m
  [31m9 failed[39m
  [2mTotal time: 7.34ms[0m

[1m[31mSome tests failed.[39m[0m
```

## Files

- `testing/sqltests/tests/json/json5-whitespace.sqltest`