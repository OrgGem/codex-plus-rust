# Rust migration plan: scripts (TypeScript/JavaScript)

1 package, 28 source files (5357 lines), 3 test files, 0 UI files, grouped into 28 units.

| Ready | Caution | Blocked | Keep in source language |
| ---: | ---: | ---: | ---: |
| 26 | 0 | 2 | 0 |

## Start here

Best first ports: safe to port (readiness) and worth porting (loops and string work, tests, size).

| # | Unit | Lines | Priority | Readiness | Tests | Bridge | Main risks |
| ---: | --- | ---: | ---: | ---: | ---: | --- | --- |
| 1 | `prepare-sherpa-cache.mjs` | 59 | 76 | 100 | 1 | napi-rs | Exceptions (4) |
| 2 | `release-meta.mjs` | 132 | 64 | 81 | 0 | napi-rs | Regexes built at runtime (3); String offsets and slicing (2); Insertion-ordered maps and object keys (1) |
| 3 | `image-overlay-fixture.cjs` | 139 | 51 | 65 | 0 | napi-rs | Async and promises (32); Insertion-ordered maps and object keys (4); Class inheritance (2) |

### `prepare-sherpa-cache.mjs`

59 lines in 1 file, wave 0, priority 76, readiness 100, bridge **napi-rs**.

- Leaf: imports no other unit.
- 1 test file imports it directly.
- Exceptions x4: `prepare-sherpa-cache.mjs:22`, `prepare-sherpa-cache.mjs:31`, `prepare-sherpa-cache.mjs:48`

### `release-meta.mjs`

132 lines in 1 file, wave 0, priority 64, readiness 81, bridge **napi-rs**.

- Leaf: imports no other unit.
- Regexes built at runtime x3: `release-meta.mjs:55`, `release-meta.mjs:56`, `release-meta.mjs:91`
- String offsets and slicing x2: `release-meta.mjs:96`, `release-meta.mjs:143`
- Insertion-ordered maps and object keys x1: `release-meta.mjs:30`
- JSON round-trips x3: `release-meta.mjs:43`, `release-meta.mjs:49`, `release-meta.mjs:60`

### `image-overlay-fixture.cjs`

139 lines in 1 file, wave 0, priority 51, readiness 65, bridge **napi-rs**.

- 1 npm dependencies not in the crate table: check them by hand.
- Leaf: imports no other unit.
- Async and promises x32: `image-overlay-fixture.cjs:31`, `image-overlay-fixture.cjs:31`, `image-overlay-fixture.cjs:34`
- Insertion-ordered maps and object keys x4: `image-overlay-fixture.cjs:74`, `image-overlay-fixture.cjs:74`, `image-overlay-fixture.cjs:95`
- Class inheritance x2: `image-overlay-fixture.cjs:108`, `image-overlay-fixture.cjs:114`
- Reflection and prototypes x1: `image-overlay-fixture.cjs:86`
- Exceptions x2: `image-overlay-fixture.cjs:22`, `image-overlay-fixture.cjs:127`
- Clock and random numbers x2: `image-overlay-fixture.cjs:62`, `image-overlay-fixture.cjs:64`

## Order by package

Packages come after the workspace packages they use. Inside a package, port by wave: a unit's imports are in earlier waves.

### 1. scripts (`.`, node, 3 test files)

| Wave | Unit | Status | Lines | Readiness | Bridge | Main risks |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 0 | `prepare-sherpa-cache.mjs` | ready | 59 | 100 | napi-rs | Exceptions (4) |
| 0 | `provider-editing-i18n-shim.mjs` | ready | 10 | 100 | napi-rs | - |
| 0 | `assemble-whale-runtime.mjs` | ready | 81 | 93 | napi-rs | String offsets and slicing (2); Exceptions (4) |
| 0 | `assemble-provider-editing.mjs` | ready | 77 | 89 | napi-rs | String offsets and slicing (1); Async and promises (3) |
| 0 | `test-issue-renderer-regressions.cjs` | ready | 76 | 86 | napi-rs | Regexes built at runtime (1); Async and promises (25) |
| 0 | `check-native-fast.cjs` | blocked | 92 | 85 | napi-rs | Dynamic code (eval, new Function, computed import) (2); String offsets and slicing (1) |
| 0 | `probe-extensions-cdp.mjs` | ready | 212 | 85 | napi-rs | Async and promises (19); Insertion-ordered maps and object keys (1) |
| 0 | `probe-extension-menu-cdp.mjs` | ready | 226 | 84 | napi-rs | Async and promises (22); Insertion-ordered maps and object keys (1) |
| 0 | `split-renderer-inject.mjs` | ready | 81 | 84 | napi-rs | String offsets and slicing (4); Async and promises (5) |
| 0 | `assemble-native-browser-inspector.mjs` | ready | 47 | 83 | napi-rs | String offsets and slicing (1); Async and promises (7) |
| 0 | `assemble-renderer-inject.mjs` | blocked | 117 | 83 | napi-rs | Dynamic code (eval, new Function, computed import) (1); Async and promises (8) |
| 0 | `test-background-resume.cjs` | ready | 167 | 83 | napi-rs | String offsets and slicing (6); Insertion-ordered maps and object keys (3) |
| 0 | `relaunch-local.mjs` | ready | 221 | 82 | napi-rs | String offsets and slicing (1); Regex features the `regex` crate lacks (1) |
| 0 | `test-session-action-layout.cjs` | ready | 168 | 82 | napi-rs | Regexes built at runtime (1); Async and promises (21) |
| 0 | `probe-extension-api-cdp.mjs` | ready | 273 | 81 | napi-rs | Async and promises (23); Insertion-ordered maps and object keys (2) |
| 0 | `prune-local-builds.mjs` | ready | 199 | 81 | napi-rs | String offsets and slicing (5); Insertion-ordered maps and object keys (2) |
| 0 | `release-meta.mjs` | ready | 132 | 81 | napi-rs | Regexes built at runtime (3); String offsets and slicing (2) |
| 0 | `build-plugin-market-index.mjs` | ready | 76 | 79 | napi-rs | String offsets and slicing (6); Insertion-ordered maps and object keys (5) |
| 0 | `test-theme-shell-performance.cjs` | ready | 177 | 79 | napi-rs | Regexes built at runtime (1); Async and promises (68) |
| 0 | `image-overlay-fixture.cjs` | ready | 139 | 65 | napi-rs | Async and promises (32); Insertion-ordered maps and object keys (4) |
| 0 | `test-skills-repository-browser.mjs` | ready | 401 | 57 | napi-rs | String offsets and slicing (1); Regex features the `regex` crate lacks (1) |
| 1 | `release.mjs` | ready | 128 | 91 | napi-rs | String offsets and slicing (2); Insertion-ordered maps and object keys (1) |
| 1 | `test-image-viewer-surfaces.cjs` | ready | 96 | 88 | napi-rs | Async and promises (68); Number semantics (1) |
| 1 | `test-image-native-surfaces.cjs` | ready | 276 | 81 | napi-rs | Async and promises (152); Number semantics (1) |
| 1 | `test-image-interaction-surfaces.cjs` | ready | 298 | 80 | napi-rs | Async and promises (223); Number semantics (1) |
| 1 | `test-image-composite.cjs` | ready | 251 | 74 | napi-rs | String offsets and slicing (1); Async and promises (102) |
| 1 | `test-image-native-media.cjs` | ready | 444 | 68 | napi-rs | String offsets and slicing (2); Async and promises (152) |
| 1 | `test-image-overlay.cjs` | ready | 833 | 60 | napi-rs | Async and promises (480); Reflection and prototypes (1) |

## npm dependencies

| Package | Units | Rust | Fit | Note |
| --- | ---: | --- | --- | --- |
| `esbuild` | 2 | ? | unknown |  |
| `pngjs` | 1 | ? | unknown |  |
| `playwright` | 4 | chromiumoxide | partial | drives Chrome over CDP; the page API differs |
| `sharp` | 1 | image | good |  |

## Blocked

- `assemble-renderer-inject.mjs`: Uses dynamic code (eval, new Function or computed imports).
- `check-native-fast.cjs`: Uses dynamic code (eval, new Function or computed imports).

## Risk guide

- **String offsets and slicing** (High): JS indexes strings in UTF-16 code units, Rust in UTF-8 bytes. Offsets that cross the boundary must be converted, or non-ASCII text gets wrong ranges silently.
- **Regex features the `regex` crate lacks** (High): Lookaround, backreferences and the `y` flag are not in the `regex` crate. Use `fancy-regex` for those patterns, or rewrite them, and test them on non-ASCII text: `\w`, `\b` and case-insensitive matching differ between engines even where a pattern compiles. JS `/i` without `u` never matches a non-ASCII letter to an ASCII one, Rust `(?i)` does (`ſ` matches `s`, the Kelvin sign `K` matches `k`): write `(?i-u)` or compare with `eq_ignore_ascii_case`.
- **Regexes built at runtime** (High): Patterns come from data or config in JS syntax. Rust regex syntax differs, so user-visible behavior can change. Pin it with tests on real patterns.
- **Dynamic code (eval, new Function, computed import)** (High): eval, new Function and computed imports have no Rust equivalent. Keep this part in JS or replace it with an explicit registry.
- **Insertion-ordered maps and object keys** (Medium): JS Map, Set and object keys keep insertion order. `HashMap` does not: use `indexmap` wherever order reaches output. With serde_json's `preserve_order`, `Map::remove` swaps the last key into the hole: use `shift_remove` for JS `delete`. Sort with `js::sort` when a comparator can return NaN: the order then depends on V8's algorithm.
- **Exceptions** (Low): Thrown errors become `Result`. Errors crossing the bridge must map back to the same JS error shape.
- **Async and promises** (Medium): Promises become futures. Bridge with napi-rs `async fn` or `wasm-bindgen-futures`, or keep the async shell in JS and port the sync core.
- **Class inheritance** (Medium): Class hierarchies become traits plus composition; `instanceof` checks become enums.
- **Reflection and prototypes** (Medium): Proxy, Reflect, Symbol and prototype tricks need a redesign, not a translation.
- **Number semantics** (Low): JS numbers are f64; bitwise ops truncate to i32; parseInt is lenient. Pick Rust integer types deliberately. Printing differs: `${n}` gives `1e+21`, `1e-7` and `0` for -0 where Rust's `{}` does not, so use `js::number_to_string`. `Math.round` rounds halves up (-2.5 gives -2), Rust's `round` away from zero: use `js::math_round`. `toFixed` rounds exact halves up ((2.5).toFixed(0) is "3"), Rust's `{:.0}` to even.
- **JSON round-trips** (Low): JSON.stringify key order and number formatting must match `serde_json` output if the result is user-visible.
- **Clock and random numbers** (Low): Results that depend on the time or on random numbers differ on every run, so a plain recording cannot pin them down. Record with `record --pin-clock` (TS/JS with the napi bridge, Python): `Date.now()`, `new Date()` and `Math.random` (`time.time`, `datetime.now` and `random`) then return fixed values during record, diff and gate, and the port reads the same ones from `src/port/pinned.rs` (`now_ms()`, `random()`, a documented mulberry32). Or pass the time and the seed in as parameters.

## Whole app

Shape: **server**.

| Entry point | Kind | Found from |
| --- | --- | --- |
| `assemble-whale-runtime.mjs` | server | `express()` at assemble-whale-runtime.mjs:74 |
| `check-native-fast.cjs` | cli | `#!/usr/bin/env node` at check-native-fast.cjs:1 |
| `prune-local-builds.mjs` | cli | `#!/usr/bin/env node` at prune-local-builds.mjs:1 |
| `relaunch-local.mjs` | cli | `#!/usr/bin/env node` at relaunch-local.mjs:1 |
| `release-meta.mjs` | cli | `#!/usr/bin/env node` at release-meta.mjs:1 |
| `release.mjs` | cli | `#!/usr/bin/env node` at release.mjs:1 |
| `test-skills-repository-browser.mjs` | server | `createServer(` at test-skills-repository-browser.mjs:23 |

### Every dependency

| Dependency | Declared | Used by | Rust | Fit | Note |
| --- | --- | --- | --- | --- | --- |
| `esbuild` | no | 2 | ? | unknown |  |
| `pngjs` | no | 1 | ? | unknown |  |
| `playwright` | no | 4 | chromiumoxide | partial | drives Chrome over CDP; the page API differs |
| `sharp` | no | 1 | image | good |  |

### Pieces, in port order

- wave 0: `assemble-native-browser-inspector.mjs` (47 lines)
- wave 0: `assemble-provider-editing.mjs` (77 lines)
- wave 0: `assemble-renderer-inject.mjs` (117 lines)
- wave 0: `assemble-whale-runtime.mjs` (81 lines)
- wave 0: `build-plugin-market-index.mjs` (76 lines)
- wave 0: `check-native-fast.cjs` (92 lines)
- wave 0: `image-overlay-fixture.cjs` (139 lines)
- wave 0: `prepare-sherpa-cache.mjs` (59 lines)
- wave 0: `probe-extension-api-cdp.mjs` (273 lines)
- wave 0: `probe-extension-menu-cdp.mjs` (226 lines)
- wave 0: `probe-extensions-cdp.mjs` (212 lines)
- wave 0: `provider-editing-i18n-shim.mjs` (10 lines)
- wave 0: `prune-local-builds.mjs` (199 lines)
- wave 0: `relaunch-local.mjs` (221 lines)
- wave 0: `release-meta.mjs` (132 lines)
- wave 0: `split-renderer-inject.mjs` (81 lines)
- wave 0: `test-background-resume.cjs` (167 lines)
- wave 0: `test-issue-renderer-regressions.cjs` (76 lines)
- wave 0: `test-session-action-layout.cjs` (168 lines)
- wave 0: `test-skills-repository-browser.mjs` (401 lines)
- wave 0: `test-theme-shell-performance.cjs` (177 lines)
- wave 1: `release.mjs` (128 lines)
- wave 1: `test-image-composite.cjs` (251 lines)
- wave 1: `test-image-interaction-surfaces.cjs` (298 lines)
- wave 1: `test-image-native-media.cjs` (444 lines)
- wave 1: `test-image-native-surfaces.cjs` (276 lines)
- wave 1: `test-image-overlay.cjs` (833 lines)
- wave 1: `test-image-viewer-surfaces.cjs` (96 lines)

### Decisions for the user

**D1** (shape): The app is a server, entered through assemble-whale-runtime.mjs, check-native-fast.cjs, prune-local-builds.mjs, relaunch-local.mjs, release-meta.mjs, release.mjs, test-skills-repository-browser.mjs. Port every entry point, in that shape?

> Yes: an axum front door (`app front-door`) takes the server over first

**D2** (blocked assemble-renderer-inject.mjs): assemble-renderer-inject.mjs is blocked: Uses dynamic code (eval, new Function or computed imports). Rework that part in Rust, or keep the unit outside?

> Research the blocker first; then port it with the piece that uses it

**D3** (blocked check-native-fast.cjs): check-native-fast.cjs is blocked: Uses dynamic code (eval, new Function or computed imports). Rework that part in Rust, or keep the unit outside?

> Research the blocker first; then port it with the piece that uses it

**D4** (dependency esbuild): Which crate replaces esbuild (used by 2 units)?

> Research it against primary sources into rust/research/esbuild.md before porting what uses it

**D5** (dependency pngjs): Which crate replaces pngjs (used by 1 unit)?

> Research it against primary sources into rust/research/pngjs.md before porting what uses it

**D6** (dependency playwright): playwright maps to chromiumoxide only in part (used by 4 units; drives Chrome over CDP; the page API differs). Use it?

> Use chromiumoxide, and record cases for the behavior it does not cover

