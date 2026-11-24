# Rust migration plan: claude-localization (TypeScript/JavaScript)

1 package, 5 source files (1607 lines), 0 test files, 0 UI files, grouped into 5 units.

| Ready | Caution | Blocked | Keep in source language |
| ---: | ---: | ---: | ---: |
| 5 | 0 | 0 | 0 |

## Start here

Best first ports: safe to port (readiness) and worth porting (loops and string work, tests, size).

| # | Unit | Lines | Priority | Readiness | Tests | Bridge | Main risks |
| ---: | --- | ---: | ---: | ---: | ---: | --- | --- |
| 1 | `bash-timeout.js` | 106 | 53 | 78 | 0 | napi-rs | String offsets and slicing (2); Regexes built at runtime (2); Async and promises (1) |

### `bash-timeout.js`

106 lines in 1 file, wave 0, priority 53, readiness 78, bridge **napi-rs**.

- Leaf: imports no other unit.
- No tests found: write characterization tests against the original code first.
- String offsets and slicing x2: `bash-timeout.js:30`, `bash-timeout.js:35`
- Regexes built at runtime x2: `bash-timeout.js:29`, `bash-timeout.js:34`
- Async and promises x1: `bash-timeout.js:61`
- Exceptions x8: `bash-timeout.js:26`, `bash-timeout.js:31`, `bash-timeout.js:36`
- JSON round-trips x1: `bash-timeout.js:96`

## Order by package

Packages come after the workspace packages they use. Inside a package, port by wave: a unit's imports are in earlier waves.

### 1. claude-localization (`.`, node, 0 test files)

| Wave | Unit | Status | Lines | Readiness | Bridge | Main risks |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 0 | `bash-timeout.js` | ready | 106 | 78 | napi-rs | String offsets and slicing (2); Regexes built at runtime (2) |
| 0 | `page-shim.js` | ready | 158 | 73 | napi-rs | String offsets and slicing (3); Reflection and prototypes (3) |
| 0 | `inject-cn.js` | ready | 298 | 66 | napi-rs | String offsets and slicing (3); Async and promises (32) |
| 0 | `page-dictation.js` | ready | 592 | 62 | napi-rs | String offsets and slicing (5); Async and promises (14) |
| 1 | `hook.js` | ready | 453 | 68 | napi-rs | String offsets and slicing (1); Async and promises (7) |

## npm dependencies

| Package | Units | Rust | Fit | Note |
| --- | ---: | --- | --- | --- |
| `electron` | 1 | ? | unknown |  |

## Risk guide

- **String offsets and slicing** (High): JS indexes strings in UTF-16 code units, Rust in UTF-8 bytes. Offsets that cross the boundary must be converted, or non-ASCII text gets wrong ranges silently.
- **Regexes built at runtime** (High): Patterns come from data or config in JS syntax. Rust regex syntax differs, so user-visible behavior can change. Pin it with tests on real patterns.
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
| `inject-cn.js` | server | `createServer(` at inject-cn.js:73 |

### Every dependency

| Dependency | Declared | Used by | Rust | Fit | Note |
| --- | --- | --- | --- | --- | --- |
| `electron` | no | 1 | ? | unknown |  |

### Pieces, in port order

- wave 0: `bash-timeout.js` (106 lines)
- wave 0: `inject-cn.js` (298 lines)
- wave 0: `page-dictation.js` (592 lines)
- wave 0: `page-shim.js` (158 lines)
- wave 1: `hook.js` (453 lines)

### Decisions for the user

**D1** (shape): The app is a server, entered through inject-cn.js. Port every entry point, in that shape?

> Yes: an axum front door (`app front-door`) takes the server over first

**D2** (dependency electron): Which crate replaces electron (used by 1 unit)?

> Research it against primary sources into rust/research/electron.md before porting what uses it

