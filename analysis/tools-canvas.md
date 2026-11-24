# Rust migration plan: conversation-canvas (TypeScript/JavaScript)

1 package, 18 source files (2461 lines), 11 test files, 0 UI files, grouped into 18 units.

| Ready | Caution | Blocked | Keep in source language |
| ---: | ---: | ---: | ---: |
| 16 | 0 | 2 | 0 |

## Start here

Best first ports: safe to port (readiness) and worth porting (loops and string work, tests, size).

| # | Unit | Lines | Priority | Readiness | Tests | Bridge | Main risks |
| ---: | --- | ---: | ---: | ---: | ---: | --- | --- |
| 1 | `tree-canvas.mjs` | 36 | 95 | 100 | 1 | wasm-bindgen | Insertion-ordered maps and object keys (4) |
| 2 | `organize.mjs` | 45 | 92 | 93 | 1 | wasm-bindgen | String offsets and slicing (1); Insertion-ordered maps and object keys (6); Exceptions (12) |
| 3 | `model.mjs` | 57 | 91 | 88 | 4 | wasm-bindgen | String offsets and slicing (2); Insertion-ordered maps and object keys (8); Exceptions (2) |
| 4 | `native-history.mjs` | 68 | 90 | 83 | 1 | wasm-bindgen | Regex features the `regex` crate lacks (2); String offsets and slicing (1); Async and promises (8) |
| 5 | `long-organizer.mjs` | 147 | 84 | 68 | 2 | wasm-bindgen | String offsets and slicing (10); Insertion-ordered maps and object keys (20); Async and promises (16) |
| 6 | `external-api.mjs` | 133 | 76 | 76 | 1 | wasm-bindgen | String offsets and slicing (4); Regex features the `regex` crate lacks (1); Async and promises (10) |

### `tree-canvas.mjs`

36 lines in 1 file, wave 1, priority 95, readiness 100, bridge **wasm-bindgen**.

- 1 test file imports it directly.
- Portable code: wasm-bindgen keeps it running in browsers and Node. It needs the wasm32-unknown-unknown target and wasm-bindgen; `bridge --kind napi` runs with what Node already has.
- Insertion-ordered maps and object keys x4: `tree-canvas.mjs:6`, `tree-canvas.mjs:7`, `tree-canvas.mjs:7`

### `organize.mjs`

45 lines in 1 file, wave 0, priority 92, readiness 93, bridge **wasm-bindgen**.

- Leaf: imports no other unit.
- 1 test file imports it directly.
- Portable code: wasm-bindgen keeps it running in browsers and Node. It needs the wasm32-unknown-unknown target and wasm-bindgen; `bridge --kind napi` runs with what Node already has.
- String offsets and slicing x1: `organize.mjs:41`
- Insertion-ordered maps and object keys x6: `organize.mjs:14`, `organize.mjs:14`, `organize.mjs:20`
- Exceptions x12: `organize.mjs:3`, `organize.mjs:13`, `organize.mjs:16`
- JSON round-trips x3: `organize.mjs:3`, `organize.mjs:9`, `organize.mjs:43`

### `model.mjs`

57 lines in 1 file, wave 0, priority 91, readiness 88, bridge **wasm-bindgen**.

- Leaf: imports no other unit.
- 4 test files import it directly.
- Portable code: wasm-bindgen keeps it running in browsers and Node. It needs the wasm32-unknown-unknown target and wasm-bindgen; `bridge --kind napi` runs with what Node already has.
- String offsets and slicing x2: `model.mjs:17`, `model.mjs:53`
- Insertion-ordered maps and object keys x8: `model.mjs:21`, `model.mjs:23`, `model.mjs:31`
- Exceptions x2: `model.mjs:7`, `model.mjs:12`
- JSON round-trips x1: `model.mjs:7`

### `native-history.mjs`

68 lines in 1 file, wave 1, priority 90, readiness 83, bridge **wasm-bindgen**.

- 1 test file imports it directly.
- Portable code: wasm-bindgen keeps it running in browsers and Node. It needs the wasm32-unknown-unknown target and wasm-bindgen; `bridge --kind napi` runs with what Node already has.
- Regex features the `regex` crate lacks x2: `native-history.mjs:4`, `native-history.mjs:41`
- String offsets and slicing x1: `native-history.mjs:33`
- Async and promises x8: `native-history.mjs:9`, `native-history.mjs:16`, `native-history.mjs:28`
- Insertion-ordered maps and object keys x5: `native-history.mjs:9`, `native-history.mjs:10`, `native-history.mjs:10`
- Exceptions x9: `native-history.mjs:14`, `native-history.mjs:17`, `native-history.mjs:19`

### `long-organizer.mjs`

147 lines in 1 file, wave 1, priority 84, readiness 68, bridge **wasm-bindgen**.

- 2 test files import it directly.
- Portable code: wasm-bindgen keeps it running in browsers and Node. It needs the wasm32-unknown-unknown target and wasm-bindgen; `bridge --kind napi` runs with what Node already has.
- String offsets and slicing x10: `long-organizer.mjs:8`, `long-organizer.mjs:22`, `long-organizer.mjs:23`
- Insertion-ordered maps and object keys x20: `long-organizer.mjs:40`, `long-organizer.mjs:41`, `long-organizer.mjs:43`
- Async and promises x16: `long-organizer.mjs:4`, `long-organizer.mjs:8`, `long-organizer.mjs:12`
- Exceptions x12: `long-organizer.mjs:78`, `long-organizer.mjs:79`, `long-organizer.mjs:84`
- JSON round-trips x8: `long-organizer.mjs:33`, `long-organizer.mjs:60`, `long-organizer.mjs:70`
- Clock and random numbers x2: `long-organizer.mjs:129`, `long-organizer.mjs:142`

## Order by package

Packages come after the workspace packages they use. Inside a package, port by wave: a unit's imports are in earlier waves.

### 1. codex-plus-conversation-canvas (`.`, portable, 11 test files)

| Wave | Unit | Status | Lines | Readiness | Bridge | Main risks |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 0 | `build-userscript.mjs` | ready | 14 | 100 | napi-rs | - |
| 0 | `dev/` | ready | 132 | 100 | napi-rs | - |
| 0 | `view-pages.mjs` | ready | 15 | 100 | wasm-bindgen | String offsets and slicing (3) |
| 0 | `src/native-api.js` | ready | 13 | 98 | wasm-bindgen | Async and promises (3); Exceptions (4) |
| 0 | `src/tree-canvas-view.js` | ready | 58 | 98 | wasm-bindgen | Insertion-ordered maps and object keys (3); JSON round-trips (1) |
| 0 | `native-runtime.mjs` | blocked | 42 | 94 | wasm-bindgen | Dynamic code (eval, new Function, computed import) (1); Insertion-ordered maps and object keys (3) |
| 0 | `organize.mjs` | ready | 45 | 93 | wasm-bindgen | String offsets and slicing (1); Insertion-ordered maps and object keys (6) |
| 0 | `src/canvas-store.js` | ready | 26 | 93 | wasm-bindgen | Async and promises (10); Insertion-ordered maps and object keys (1) |
| 0 | `model.mjs` | ready | 57 | 88 | wasm-bindgen | String offsets and slicing (2); Insertion-ordered maps and object keys (8) |
| 0 | `src/native-sidechat.js` | ready | 158 | 81 | wasm-bindgen | Async and promises (23); Insertion-ordered maps and object keys (6) |
| 0 | `external-api.mjs` | ready | 133 | 76 | wasm-bindgen | String offsets and slicing (4); Regex features the `regex` crate lacks (1) |
| 0 | `src/canvas-panel.js` | ready | 324 | 61 | wasm-bindgen | String offsets and slicing (5); Async and promises (31) |
| 0 | `public/` | blocked | 1166 | 1 | wasm-bindgen | String offsets and slicing (30); Regex features the `regex` crate lacks (3) |
| 1 | `tree-canvas.mjs` | ready | 36 | 100 | wasm-bindgen | Insertion-ordered maps and object keys (4) |
| 1 | `tree-markup.mjs` | ready | 9 | 100 | wasm-bindgen | String offsets and slicing (2); Insertion-ordered maps and object keys (1) |
| 1 | `bridge-data.mjs` | ready | 18 | 99 | wasm-bindgen | String offsets and slicing (2); Insertion-ordered maps and object keys (1) |
| 1 | `native-history.mjs` | ready | 68 | 83 | wasm-bindgen | Regex features the `regex` crate lacks (2); String offsets and slicing (1) |
| 1 | `long-organizer.mjs` | ready | 147 | 68 | wasm-bindgen | String offsets and slicing (10); Insertion-ordered maps and object keys (20) |

## Blocked

- `native-runtime.mjs`: Uses dynamic code (eval, new Function or computed imports).
- `public/`: Uses dynamic code (eval, new Function or computed imports).

## Risk guide

- **String offsets and slicing** (High): JS indexes strings in UTF-16 code units, Rust in UTF-8 bytes. Offsets that cross the boundary must be converted, or non-ASCII text gets wrong ranges silently.
- **Regex features the `regex` crate lacks** (High): Lookaround, backreferences and the `y` flag are not in the `regex` crate. Use `fancy-regex` for those patterns, or rewrite them, and test them on non-ASCII text: `\w`, `\b` and case-insensitive matching differ between engines even where a pattern compiles. JS `/i` without `u` never matches a non-ASCII letter to an ASCII one, Rust `(?i)` does (`ſ` matches `s`, the Kelvin sign `K` matches `k`): write `(?i-u)` or compare with `eq_ignore_ascii_case`.
- **Dynamic code (eval, new Function, computed import)** (High): eval, new Function and computed imports have no Rust equivalent. Keep this part in JS or replace it with an explicit registry.
- **Insertion-ordered maps and object keys** (Medium): JS Map, Set and object keys keep insertion order. `HashMap` does not: use `indexmap` wherever order reaches output. With serde_json's `preserve_order`, `Map::remove` swaps the last key into the hole: use `shift_remove` for JS `delete`. Sort with `js::sort` when a comparator can return NaN: the order then depends on V8's algorithm.
- **Exceptions** (Low): Thrown errors become `Result`. Errors crossing the bridge must map back to the same JS error shape.
- **Async and promises** (Medium): Promises become futures. Bridge with napi-rs `async fn` or `wasm-bindgen-futures`, or keep the async shell in JS and port the sync core.
- **Reflection and prototypes** (Medium): Proxy, Reflect, Symbol and prototype tricks need a redesign, not a translation.
- **Number semantics** (Low): JS numbers are f64; bitwise ops truncate to i32; parseInt is lenient. Pick Rust integer types deliberately. Printing differs: `${n}` gives `1e+21`, `1e-7` and `0` for -0 where Rust's `{}` does not, so use `js::number_to_string`. `Math.round` rounds halves up (-2.5 gives -2), Rust's `round` away from zero: use `js::math_round`. `toFixed` rounds exact halves up ((2.5).toFixed(0) is "3"), Rust's `{:.0}` to even.
- **JSON round-trips** (Low): JSON.stringify key order and number formatting must match `serde_json` output if the result is user-visible.
- **Clock and random numbers** (Low): Results that depend on the time or on random numbers differ on every run, so a plain recording cannot pin them down. Record with `record --pin-clock` (TS/JS with the napi bridge, Python): `Date.now()`, `new Date()` and `Math.random` (`time.time`, `datetime.now` and `random`) then return fixed values during record, diff and gate, and the port reads the same ones from `src/port/pinned.rs` (`now_ms()`, `random()`, a documented mulberry32). Or pass the time and the seed in as parameters.

## Whole app

Shape: **server**.

| Entry point | Kind | Found from |
| --- | --- | --- |
| `dev/embedding-harness.mjs` | server | `createServer(` at dev/embedding-harness.mjs:6 |

### Every dependency

None.

### Pieces, in port order

- wave 0: `build-userscript.mjs` (14 lines)
- wave 0: `dev/` (132 lines)
- wave 0: `external-api.mjs` (133 lines)
- wave 0: `model.mjs` (57 lines)
- wave 0: `native-runtime.mjs` (42 lines)
- wave 0: `organize.mjs` (45 lines)
- wave 0: `public/` (1166 lines)
- wave 0: `src/canvas-panel.js` (324 lines)
- wave 0: `src/canvas-store.js` (26 lines)
- wave 0: `src/native-api.js` (13 lines)
- wave 0: `src/native-sidechat.js` (158 lines)
- wave 0: `src/tree-canvas-view.js` (58 lines)
- wave 0: `view-pages.mjs` (15 lines)
- wave 1: `bridge-data.mjs` (18 lines)
- wave 1: `long-organizer.mjs` (147 lines)
- wave 1: `native-history.mjs` (68 lines)
- wave 1: `tree-canvas.mjs` (36 lines)
- wave 1: `tree-markup.mjs` (9 lines)

### Decisions for the user

**D1** (shape): The app is a server, entered through dev/embedding-harness.mjs. Port every entry point, in that shape?

> Yes: an axum front door (`app front-door`) takes the server over first

**D2** (blocked native-runtime.mjs): native-runtime.mjs is blocked: Uses dynamic code (eval, new Function or computed imports). Rework that part in Rust, or keep the unit outside?

> Research the blocker first; then port it with the piece that uses it

**D3** (blocked public/): public/ is blocked: Uses dynamic code (eval, new Function or computed imports). Rework that part in Rust, or keep the unit outside?

> Research the blocker first; then port it with the piece that uses it

