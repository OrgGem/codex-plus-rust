# Rust migration plan: rice-maid-pet (TypeScript/JavaScript)

1 package, 3 source files (383 lines), 1 test file, 0 UI files, grouped into 3 units.

| Ready | Caution | Blocked | Keep in source language |
| ---: | ---: | ---: | ---: |
| 3 | 0 | 0 | 0 |

## Start here

No unit exports anything a bridge can carry yet: the units below expose constants, types or nothing a port could call.

## Order by package

Packages come after the workspace packages they use. Inside a package, port by wave: a unit's imports are in earlier waves.

### 1. rice-maid-pet (`.`, node, 1 test file)

| Wave | Unit | Status | Lines | Readiness | Bridge | Main risks |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 0 | `backend/` | ready | 18 | 100 | napi-rs | Clock and random numbers (1) |
| 0 | `hooks/` | ready | 8 | 100 | napi-rs | Async and promises (2); Exceptions (1) |
| 0 | `renderer.js` | ready | 357 | 89 | napi-rs | Async and promises (2); Insertion-ordered maps and object keys (1) |

## Risk guide

- **Insertion-ordered maps and object keys** (Medium): JS Map, Set and object keys keep insertion order. `HashMap` does not: use `indexmap` wherever order reaches output. With serde_json's `preserve_order`, `Map::remove` swaps the last key into the hole: use `shift_remove` for JS `delete`. Sort with `js::sort` when a comparator can return NaN: the order then depends on V8's algorithm.
- **Exceptions** (Low): Thrown errors become `Result`. Errors crossing the bridge must map back to the same JS error shape.
- **Async and promises** (Medium): Promises become futures. Bridge with napi-rs `async fn` or `wasm-bindgen-futures`, or keep the async shell in JS and port the sync core.
- **Clock and random numbers** (Low): Results that depend on the time or on random numbers differ on every run, so a plain recording cannot pin them down. Record with `record --pin-clock` (TS/JS with the napi bridge, Python): `Date.now()`, `new Date()` and `Math.random` (`time.time`, `datetime.now` and `random`) then return fixed values during record, diff and gate, and the port reads the same ones from `src/port/pinned.rs` (`now_ms()`, `random()`, a documented mulberry32). Or pass the time and the seed in as parameters.

## Whole app

Shape: **server**.

| Entry point | Kind | Found from |
| --- | --- | --- |
| `renderer.js` | server | `.listen(` at renderer.js:261 |

### Every dependency

None.

### Pieces, in port order

- wave 0: `backend/` (18 lines)
- wave 0: `hooks/` (8 lines)
- wave 0: `renderer.js` (357 lines)

### Decisions for the user

**D1** (shape): The app is a server, entered through renderer.js. Port every entry point, in that shape?

> Yes: an axum front door (`app front-door`) takes the server over first

