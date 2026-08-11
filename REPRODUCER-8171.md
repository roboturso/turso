# Reproducer: #8171 — wasm/OPFS: statement can hang forever when core asks for a re-poll but no I/O is pending

https://github.com/tursodatabase/turso/issues/8171

## Environment

- Commit: 79163249538197d01dec5ea7f65519454ed792e2
- Platform: linux x86_64
- Reproduced at: 2026-08-11T07:07:12Z

## Reproduce

```sh
cd bindings/javascript && yarn install && rustup target add wasm32-wasip1-threads && export TMPDIR="$HOME/tmp-build" && mkdir -p "$TMPDIR" && { [ -d "$TMPDIR/wasi-sdk-25.0-x86_64-linux" ] || (cd "$TMPDIR" && wget -q https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-25/wasi-sdk-25.0-x86_64-linux.tar.gz && tar -xf wasi-sdk-25.0-x86_64-linux.tar.gz); } && export WASI_SDK_PATH="$TMPDIR/wasi-sdk-25.0-x86_64-linux" && export TARGET_CXXFLAGS="--target=wasm32-wasi-threads --sysroot=$WASI_SDK_PATH/share/wasi-sysroot -pthread -mllvm -wasm-enable-sjlj -lsetjmp" && export TARGET_CFLAGS="$TARGET_CXXFLAGS" && yarn workspace @tursodatabase/database-common build && yarn workspace @tursodatabase/database-wasm-common build && yarn workspace @tursodatabase/database-wasm build && cd packages/wasm && yarn exec playwright install chromium && CI=1 yarn exec vitest --browser=chromium --run busy-timeout-hang.test.ts
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed: on wasm/OPFS a busy-handler retry makes stepSync() return STEP_IO with no I/O completion pending, so the driver parks on the IONotifier forever — a write with busy_timeout=200 on a locked database never settles (reproduced in headless Chromium against real OPFS).

## Observed failure

RoboTurso ran the command above and observed:

```text
➤ YN0000: · Yarn 4.9.2
➤ YN0000: ┌ Resolution step
➤ YN0000: └ Completed
➤ YN0000: ┌ Fetch step
➤ YN0000: └ Completed in 0s 216ms
➤ YN0000: ┌ Link step
➤ YN0000: └ Completed
➤ YN0000: · Done in 0s 425ms

> @tursodatabase/database-common@0.8.0-pre.3 tsc-build
> npm exec tsc


> @tursodatabase/database-wasm-common@0.8.0-pre.3 tsc-build
> npm exec tsc


> @tursodatabase/database-wasm@0.8.0-pre.3 napi-build
> napi build --features browser --profile release-official --platform --target wasm32-wasip1-threads --no-js --manifest-path ../../Cargo.toml --output-dir . && rm index.d.ts turso.wasi* wasi* browser.js


> @tursodatabase/database-wasm@0.8.0-pre.3 tsc-build
> npm exec tsc && cp turso.wasm32-wasi.wasm ./dist/turso.wasm32-wasi.wasm && WASM_FILE=turso.wasm32-wasi.wasm JS_FILE=./dist/wasm-inline.js node ../../scripts/inline-wasm-base64.js && npm run bundle


> @tursodatabase/database-wasm@0.8.0-pre.3 bundle
> vite build

vite v7.3.6 building client environment for production...
transforming...
✓ 18 modules transformed.
rendering chunks...
computing gzip size...
bundle/main.es.js  15,665.07 kB │ gzip: 5,344.27 kB
✓ built in 2.08s
BEWARE: your OS is not officially supported by Playwright; downloading fallback build for ubuntu24.04-x64.
BEWARE: your OS is not officially supported by Playwright; downloading fallback build for ubuntu24.04-x64.
BEWARE: your OS is not officially supported by Playwright; downloading fallback build for ubuntu24.04-x64.

[1m[46m RUN [49m[22m [36mv3.2.7 [39m[90m/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-8171/bindings/javascript/packages/wasm[39m

[90mstdout[2m | busy-timeout-hang.test.ts[2m > [22m[2mwrite on a locked database with busy_timeout errors instead of hanging
[22m[39mINSERT outcome: still pending after 10s
 [31m❯[39m [30m[43m chromium [49m[39m busy-timeout-hang.test.ts [2m([22m[2m1 test[22m[2m | [22m[31m1 failed[39m[2m)[22m[33m 11069[2mms[22m[39m
[31m   [31m×[31m write on a locked database with busy_timeout errors instead of hanging[39m[33m 11069[2mms[22m[39m
[31m     → expected 'still pending after 10s' not to be 'still pending after 10s' // Object.is equality[39m

[31m⎯⎯⎯⎯⎯⎯⎯[39m[1m[41m Failed Tests 1 [49m[22m[31m⎯⎯⎯⎯⎯⎯⎯[39m

[41m[1m FAIL [22m[49m [30m[43m chromium [49m[39m busy-timeout-hang.test.ts[2m > [22mwrite on a locked database with busy_timeout errors instead of hanging
[31m[1mAssertionError[22m: expected 'still pending after 10s' not to be 'still pending after 10s' // Object.is equality[39m

Failure screenshot:
  - [2m__screenshots__/busy-timeout-hang.test.ts/write-on-a-locked-database-with-busy-timeout-errors-instead-of-hanging-1.png[22m

[36m [2m❯[22m busy-timeout-hang.test.ts:[2m42:24[22m[39m
    [90m 40| [39m    ])[33m;[39m
    [90m 41| [39m    console[33m.[39m[34mlog[39m([32m`INSERT outcome: [39m[36m${[39moutcome[36m}[39m[32m`[39m)[33m;[39m
    [90m 42| [39m    [34mexpect[39m(outcome)[33m.[39mnot[33m.[39m[34mtoBe[39m([33mSTILL_PENDING[39m)[33m;[39m
    [90m   | [39m                       [31m^[39m
    [90m 43| [39m
    [90m 44| [39m    // The writer still holds the lock, so the insert cannot have succ…

[31m[2m⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯[1/1]⎯[22m[39m


[2m Test Files [22m [1m[31m1 failed[39m[22m[90m (1)[39m
[2m      Tests [22m [1m[31m1 failed[39m[22m[90m (1)[39m
[2m   Start at [22m 10:06:59
[2m   Duration [22m 12.96s[2m (transform 0ms, setup 0ms, collect 122ms, tests 11.07s, environment 0ms, prepare 787ms)[22m
```

## Files

- `bindings/javascript/packages/wasm/busy-timeout-hang.test.ts`