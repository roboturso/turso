# Reproducer: #7794 — Concurrent execution of a reused prepared statement returns the same row for every call (js bindings)

https://github.com/tursodatabase/turso/issues/7794

## Environment

- Commit: dac993f26632263b5c192dd1bff39d84ed007729
- Platform: linux x86_64
- Reproduced at: 2026-07-12T10:01:39Z

## Reproduce

```sh
cd bindings/javascript && yarn install && yarn workspace @tursodatabase/database-common build && cd packages/native && npx napi build --platform --esm --manifest-path ../../Cargo.toml --output-dir . && npx vitest --run issue-7794-concurrent-statement.test.ts
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed: bindParams runs before execLock.acquire() in the promise Statement methods, so concurrent get/all calls on a reused prepared statement overwrite each other's bindings and all resolve with the last-bound row.

## Observed failure

RoboTurso ran the command above and observed:

```text
➤ YN0000: · Yarn 4.9.2
➤ YN0000: ┌ Resolution step
➤ YN0000: └ Completed
➤ YN0000: ┌ Post-resolution validation
➤ YN0086: │ Some peer dependencies are incorrectly met by dependencies; run yarn explain peer-requirements for details.
➤ YN0000: └ Completed
➤ YN0000: ┌ Fetch step
➤ YN0000: └ Completed in 0s 216ms
➤ YN0000: ┌ Link step
➤ YN0000: └ Completed
➤ YN0000: · Done with warnings in 0s 407ms

> @tursodatabase/database-common@0.7.0-pre.20 tsc-build
> npm exec tsc


 RUN  v3.2.7 /home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7794/bindings/javascript/packages/native

 ❯ issue-7794-concurrent-statement.test.ts (1 test | 1 failed) 119ms
   × issue-7794: concurrent reuse of a prepared statement returns each caller its own row 118ms
     → expected [ Array(10) ] to deeply equal [ 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 ]

⎯⎯⎯⎯⎯⎯⎯ Failed Tests 1 ⎯⎯⎯⎯⎯⎯⎯

 FAIL  issue-7794-concurrent-statement.test.ts > issue-7794: concurrent reuse of a prepared statement returns each caller its own row
AssertionError: expected [ Array(10) ] to deeply equal [ 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 ]

[32m- Expected[39m
[31m+ Received[39m

[2m  [[22m
[32m-   1,[39m
[32m-   2,[39m
[32m-   3,[39m
[32m-   4,[39m
[32m-   5,[39m
[32m-   6,[39m
[32m-   7,[39m
[32m-   8,[39m
[32m-   9,[39m
[31m+   10,[39m
[31m+   10,[39m
[31m+   10,[39m
[31m+   10,[39m
[31m+   10,[39m
[31m+   10,[39m
[31m+   10,[39m
[31m+   10,[39m
[31m+   10,[39m
[2m    10,[22m
[2m  ][22m

 ❯ issue-7794-concurrent-statement.test.ts:28:27
     26|     // Concurrent execution of the same statement object via get().
     27|     const concurrentGet = (await Promise.all(ids.map((id) => stmt.get(…
     28|     expect(concurrentGet).toEqual(ids);
       |                           ^
     29| 
     30|     // Same via all().

⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯[1/1]⎯


 Test Files  1 failed (1)
      Tests  1 failed (1)
   Start at  13:01:39
   Duration  492ms (transform 68ms, setup 0ms, collect 147ms, tests 119ms, environment 0ms, prepare 82ms)
```

## Files

- `bindings/javascript/packages/native/issue-7794-concurrent-statement.test.ts`