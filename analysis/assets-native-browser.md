# Rust migration plan: native-browser (TypeScript/JavaScript)

1 package, 3 source files (705 lines), 0 test files, 0 UI files, grouped into 3 units.

| Ready | Caution | Blocked | Keep in source language |
| ---: | ---: | ---: | ---: |
| 2 | 1 | 0 | 0 |

## Start here

Best first ports: safe to port (readiness) and worth porting (loops and string work, tests, size).

| # | Unit | Lines | Priority | Readiness | Tests | Bridge | Main risks |
| ---: | --- | ---: | ---: | ---: | ---: | --- | --- |
| 1 | `inspect-service.mjs` | 271 | 49 | 74 | 0 | napi-rs | String offsets and slicing (3); Insertion-ordered maps and object keys (4); JSON round-trips (3) |

### `inspect-service.mjs`

271 lines in 1 file, wave 0, priority 49, readiness 74, bridge **napi-rs**.

- 1 npm dependencies not in the crate table: check them by hand.
- Leaf: imports no other unit.
- No tests found: write characterization tests against the original code first.
- String offsets and slicing x3: `inspect-service.mjs:48`, `inspect-service.mjs:252`, `inspect-service.mjs:253`
- Insertion-ordered maps and object keys x4: `inspect-service.mjs:6`, `inspect-service.mjs:21`, `inspect-service.mjs:175`
- JSON round-trips x3: `inspect-service.mjs:33`, `inspect-service.mjs:272`, `inspect-service.mjs:274`
- Exceptions x2: `inspect-service.mjs:12`, `inspect-service.mjs:271`

## Order by package

Packages come after the workspace packages they use. Inside a package, port by wave: a unit's imports are in earlier waves.

### 1. native-browser (`.`, node, 0 test files)

| Wave | Unit | Status | Lines | Readiness | Bridge | Main risks |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 0 | `require-identification.mjs` | ready | 40 | 92 | napi-rs | Async and promises (4); Exceptions (2) |
| 0 | `inspect-service.mjs` | ready | 271 | 74 | napi-rs | String offsets and slicing (3); Insertion-ordered maps and object keys (4) |
| 0 | `inspect-service.cjs` | caution | 394 | 0 | napi-rs | String offsets and slicing (254); Regex features the `regex` crate lacks (10) |

## npm dependencies

| Package | Units | Rust | Fit | Note |
| --- | ---: | --- | --- | --- |
| `@babel/traverse` | 1 | ? | unknown |  |
| `supports-color` | 1 | ? | unknown |  |
| `@babel/parser` | 1 | oxc | good | oxc-parser is already a binding of the Rust crate. |

## Risk guide

- **String offsets and slicing** (High): JS indexes strings in UTF-16 code units, Rust in UTF-8 bytes. Offsets that cross the boundary must be converted, or non-ASCII text gets wrong ranges silently.
- **Regex features the `regex` crate lacks** (High): Lookaround, backreferences and the `y` flag are not in the `regex` crate. Use `fancy-regex` for those patterns, or rewrite them, and test them on non-ASCII text: `\w`, `\b` and case-insensitive matching differ between engines even where a pattern compiles. JS `/i` without `u` never matches a non-ASCII letter to an ASCII one, Rust `(?i)` does (`ſ` matches `s`, the Kelvin sign `K` matches `k`): write `(?i-u)` or compare with `eq_ignore_ascii_case`.
- **Regexes built at runtime** (High): Patterns come from data or config in JS syntax. Rust regex syntax differs, so user-visible behavior can change. Pin it with tests on real patterns.
- **Dynamic typing** (Medium): Values whose shape is checked at runtime need an explicit Rust enum or `serde_json::Value`.
- **Insertion-ordered maps and object keys** (Medium): JS Map, Set and object keys keep insertion order. `HashMap` does not: use `indexmap` wherever order reaches output. With serde_json's `preserve_order`, `Map::remove` swaps the last key into the hole: use `shift_remove` for JS `delete`. Sort with `js::sort` when a comparator can return NaN: the order then depends on V8's algorithm.
- **Exceptions** (Low): Thrown errors become `Result`. Errors crossing the bridge must map back to the same JS error shape.
- **Async and promises** (Medium): Promises become futures. Bridge with napi-rs `async fn` or `wasm-bindgen-futures`, or keep the async shell in JS and port the sync core.
- **Generators** (Medium): Lazy generators become iterators, or return a collected Vec across the bridge.
- **Class inheritance** (Medium): Class hierarchies become traits plus composition; `instanceof` checks become enums.
- **Reflection and prototypes** (Medium): Proxy, Reflect, Symbol and prototype tricks need a redesign, not a translation.
- **Number semantics** (Low): JS numbers are f64; bitwise ops truncate to i32; parseInt is lenient. Pick Rust integer types deliberately. Printing differs: `${n}` gives `1e+21`, `1e-7` and `0` for -0 where Rust's `{}` does not, so use `js::number_to_string`. `Math.round` rounds halves up (-2.5 gives -2), Rust's `round` away from zero: use `js::math_round`. `toFixed` rounds exact halves up ((2.5).toFixed(0) is "3"), Rust's `{:.0}` to even.
- **JSON round-trips** (Low): JSON.stringify key order and number formatting must match `serde_json` output if the result is user-visible.
- **Clock and random numbers** (Low): Results that depend on the time or on random numbers differ on every run, so a plain recording cannot pin them down. Record with `record --pin-clock` (TS/JS with the napi bridge, Python): `Date.now()`, `new Date()` and `Math.random` (`time.time`, `datetime.now` and `random`) then return fixed values during record, diff and gate, and the port reads the same ones from `src/port/pinned.rs` (`now_ms()`, `random()`, a documented mulberry32). Or pass the time and the seed in as parameters.

## Whole app

Shape: **library**.

| Entry point | Kind | Found from |
| --- | --- | --- |
| native-browser: `.` | library | no main and no server: the package's public API |

### Every dependency

| Dependency | Declared | Used by | Rust | Fit | Note |
| --- | --- | --- | --- | --- | --- |
| `@babel/traverse` | no | 1 | ? | unknown |  |
| `supports-color` | no | 1 | ? | unknown |  |
| `@babel/parser` | no | 1 | oxc | good | oxc-parser is already a binding of the Rust crate. |

### Pieces, in port order

- wave 0: `inspect-service.cjs` (394 lines)
- wave 0: `inspect-service.mjs` (271 lines)
- wave 0: `require-identification.mjs` (40 lines)

### Decisions for the user

**D1** (shape): The app is a library, entered through native-browser (.). Port every entry point, in that shape?

> Yes: the Rust crate's public API becomes the library's front door

**D2** (dependency @babel/traverse): Which crate replaces @babel/traverse (used by 1 unit)?

> Research it against primary sources into rust/research/@babel-traverse.md before porting what uses it

**D3** (dependency supports-color): Which crate replaces supports-color (used by 1 unit)?

> Research it against primary sources into rust/research/supports-color.md before porting what uses it

