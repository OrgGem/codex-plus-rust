# Rust migration plan: remote-control-p0 (TypeScript/JavaScript)

1 package, 14 source files (1644 lines), 0 test files, 0 UI files, grouped into 14 units.

| Ready | Caution | Blocked | Keep in source language |
| ---: | ---: | ---: | ---: |
| 14 | 0 | 0 | 0 |

## Start here

Best first ports: safe to port (readiness) and worth porting (loops and string work, tests, size).

| # | Unit | Lines | Priority | Readiness | Tests | Bridge | Main risks |
| ---: | --- | ---: | ---: | ---: | ---: | --- | --- |
| 1 | `sidecar.mjs` | 80 | 59 | 74 | 0 | napi-rs | String offsets and slicing (3); Async and promises (14); Insertion-ordered maps and object keys (1) |
| 2 | `ipc-versions.mjs` | 43 | 58 | 96 | 0 | napi-rs | Exceptions (4); JSON round-trips (3) |
| 3 | `follower.mjs` | 121 | 57 | 87 | 0 | napi-rs | Async and promises (3); Class inheritance (1); Clock and random numbers (2) |
| 4 | `ipc-client.mjs` | 137 | 55 | 85 | 0 | napi-rs | Async and promises (4); Insertion-ordered maps and object keys (1); Class inheritance (1) |

### `sidecar.mjs`

80 lines in 1 file, wave 0, priority 59, readiness 74, bridge **napi-rs**.

- Leaf: imports no other unit.
- No tests found: write characterization tests against the original code first.
- String offsets and slicing x3: `sidecar.mjs:22`, `sidecar.mjs:23`, `sidecar.mjs:24`
- Async and promises x14: `sidecar.mjs:9`, `sidecar.mjs:11`, `sidecar.mjs:42`
- Insertion-ordered maps and object keys x1: `sidecar.mjs:18`
- Exceptions x4: `sidecar.mjs:27`, `sidecar.mjs:63`, `sidecar.mjs:77`
- JSON round-trips x4: `sidecar.mjs:28`, `sidecar.mjs:37`, `sidecar.mjs:45`
- Clock and random numbers x2: `sidecar.mjs:58`, `sidecar.mjs:59`

### `ipc-versions.mjs`

43 lines in 1 file, wave 0, priority 58, readiness 96, bridge **napi-rs**.

- Leaf: imports no other unit.
- No tests found: write characterization tests against the original code first.
- Exceptions x4: `ipc-versions.mjs:12`, `ipc-versions.mjs:22`, `ipc-versions.mjs:24`
- JSON round-trips x3: `ipc-versions.mjs:19`, `ipc-versions.mjs:36`, `ipc-versions.mjs:45`

### `follower.mjs`

121 lines in 1 file, wave 0, priority 57, readiness 87, bridge **napi-rs**.

- Leaf: imports no other unit.
- No tests found: write characterization tests against the original code first.
- Async and promises x3: `follower.mjs:63`, `follower.mjs:64`, `follower.mjs:68`
- Class inheritance x1: `follower.mjs:33`
- Clock and random numbers x2: `follower.mjs:93`, `follower.mjs:103`
- Exceptions x1: `follower.mjs:65`
- JSON round-trips x1: `follower.mjs:65`

### `ipc-client.mjs`

137 lines in 1 file, wave 0, priority 55, readiness 85, bridge **napi-rs**.

- Leaf: imports no other unit.
- No tests found: write characterization tests against the original code first.
- Async and promises x4: `ipc-client.mjs:74`, `ipc-client.mjs:79`, `ipc-client.mjs:80`
- Insertion-ordered maps and object keys x1: `ipc-client.mjs:58`
- Class inheritance x1: `ipc-client.mjs:51`
- JSON round-trips x3: `ipc-client.mjs:63`, `ipc-client.mjs:81`, `ipc-client.mjs:93`

## Order by package

Packages come after the workspace packages they use. Inside a package, port by wave: a unit's imports are in earlier waves.

### 1. remote-control-p0 (`.`, node, 0 test files)

| Wave | Unit | Status | Lines | Readiness | Bridge | Main risks |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 0 | `ipc-versions.mjs` | ready | 43 | 96 | napi-rs | Exceptions (4); JSON round-trips (3) |
| 0 | `follower.mjs` | ready | 121 | 87 | napi-rs | Async and promises (3); Class inheritance (1) |
| 0 | `ipc-client.mjs` | ready | 137 | 85 | napi-rs | Async and promises (4); Insertion-ordered maps and object keys (1) |
| 0 | `run-p4.mjs` | ready | 98 | 80 | napi-rs | String offsets and slicing (1); Async and promises (25) |
| 0 | `poc-headless-dump.mjs` | ready | 53 | 77 | napi-rs | String offsets and slicing (3); Async and promises (7) |
| 0 | `poc-headless-turn.mjs` | ready | 82 | 75 | napi-rs | String offsets and slicing (4); Async and promises (7) |
| 0 | `run-p3.mjs` | ready | 153 | 74 | napi-rs | String offsets and slicing (1); Async and promises (46) |
| 0 | `run-p5.mjs` | ready | 171 | 74 | napi-rs | String offsets and slicing (1); Async and promises (40) |
| 0 | `sidecar.mjs` | ready | 80 | 74 | napi-rs | String offsets and slicing (3); Async and promises (14) |
| 0 | `mock-responses.mjs` | ready | 100 | 72 | napi-rs | String offsets and slicing (3); Regex features the `regex` crate lacks (2) |
| 0 | `poc-headless-resume.mjs` | ready | 84 | 72 | napi-rs | String offsets and slicing (5); Async and promises (19) |
| 0 | `run-p1.mjs` | ready | 185 | 72 | napi-rs | String offsets and slicing (1); Async and promises (56) |
| 0 | `run-p2.mjs` | ready | 182 | 70 | napi-rs | String offsets and slicing (2); Async and promises (47) |
| 1 | `run-p0.mjs` | ready | 155 | 74 | napi-rs | String offsets and slicing (2); Async and promises (46) |

## Risk guide

- **String offsets and slicing** (High): JS indexes strings in UTF-16 code units, Rust in UTF-8 bytes. Offsets that cross the boundary must be converted, or non-ASCII text gets wrong ranges silently.
- **Regex features the `regex` crate lacks** (High): Lookaround, backreferences and the `y` flag are not in the `regex` crate. Use `fancy-regex` for those patterns, or rewrite them, and test them on non-ASCII text: `\w`, `\b` and case-insensitive matching differ between engines even where a pattern compiles. JS `/i` without `u` never matches a non-ASCII letter to an ASCII one, Rust `(?i)` does (`ſ` matches `s`, the Kelvin sign `K` matches `k`): write `(?i-u)` or compare with `eq_ignore_ascii_case`.
- **Insertion-ordered maps and object keys** (Medium): JS Map, Set and object keys keep insertion order. `HashMap` does not: use `indexmap` wherever order reaches output. With serde_json's `preserve_order`, `Map::remove` swaps the last key into the hole: use `shift_remove` for JS `delete`. Sort with `js::sort` when a comparator can return NaN: the order then depends on V8's algorithm.
- **Exceptions** (Low): Thrown errors become `Result`. Errors crossing the bridge must map back to the same JS error shape.
- **Async and promises** (Medium): Promises become futures. Bridge with napi-rs `async fn` or `wasm-bindgen-futures`, or keep the async shell in JS and port the sync core.
- **Class inheritance** (Medium): Class hierarchies become traits plus composition; `instanceof` checks become enums.
- **JSON round-trips** (Low): JSON.stringify key order and number formatting must match `serde_json` output if the result is user-visible.
- **Clock and random numbers** (Low): Results that depend on the time or on random numbers differ on every run, so a plain recording cannot pin them down. Record with `record --pin-clock` (TS/JS with the napi bridge, Python): `Date.now()`, `new Date()` and `Math.random` (`time.time`, `datetime.now` and `random`) then return fixed values during record, diff and gate, and the port reads the same ones from `src/port/pinned.rs` (`now_ms()`, `random()`, a documented mulberry32). Or pass the time and the seed in as parameters.

## Whole app

Shape: **server**.

| Entry point | Kind | Found from |
| --- | --- | --- |
| `mock-responses.mjs` | server | `createServer(` at mock-responses.mjs:47 |

### Every dependency

None.

### Pieces, in port order

- wave 0: `follower.mjs` (121 lines)
- wave 0: `ipc-client.mjs` (137 lines)
- wave 0: `ipc-versions.mjs` (43 lines)
- wave 0: `mock-responses.mjs` (100 lines)
- wave 0: `poc-headless-dump.mjs` (53 lines)
- wave 0: `poc-headless-resume.mjs` (84 lines)
- wave 0: `poc-headless-turn.mjs` (82 lines)
- wave 0: `run-p1.mjs` (185 lines)
- wave 0: `run-p2.mjs` (182 lines)
- wave 0: `run-p3.mjs` (153 lines)
- wave 0: `run-p4.mjs` (98 lines)
- wave 0: `run-p5.mjs` (171 lines)
- wave 0: `sidecar.mjs` (80 lines)
- wave 1: `run-p0.mjs` (155 lines)

### Decisions for the user

**D1** (shape): The app is a server, entered through mock-responses.mjs. Port every entry point, in that shape?

> Yes: an axum front door (`app front-door`) takes the server over first

