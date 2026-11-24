# Rust migration plan: codex-wechat (Python)

1 package, 1 source file (716 lines), 0 test files, 0 UI files, grouped into 1 unit.

| Ready | Caution | Blocked | Keep in source language |
| ---: | ---: | ---: | ---: |
| 0 | 1 | 0 | 0 |

## Start here

Best first ports: safe to port (readiness) and worth porting (loops and string work, tests, size).

| # | Unit | Lines | Priority | Readiness | Tests | Bridge | Main risks |
| ---: | --- | ---: | ---: | ---: | ---: | --- | --- |
| 1 | `codex_wechat.py` | 716 | 31 | 37 | 0 | PyO3 | String offsets and slicing (10); Hash iteration order (4); Dynamic typing (4) |

### `codex_wechat.py`

716 lines in 1 file, wave 0, priority 31, readiness 37, bridge **PyO3**.

- Leaf: imports no other unit.
- No tests found: write characterization tests against the original code first.
- String offsets and slicing x10: `codex_wechat.py:272`, `codex_wechat.py:274`, `codex_wechat.py:397`
- Hash iteration order x4: `codex_wechat.py:126`, `codex_wechat.py:616`, `codex_wechat.py:618`
- Dynamic typing x4: `codex_wechat.py:110`, `codex_wechat.py:118`, `codex_wechat.py:655`
- Integer width and overflow x2: `codex_wechat.py:36`, `codex_wechat.py:36`
- Concurrency x2: `codex_wechat.py:422`, `codex_wechat.py:423`
- Division and modulo x1: `codex_wechat.py:570`
- Exceptions x30: `codex_wechat.py:99`, `codex_wechat.py:111`, `codex_wechat.py:119`
- Clock and random numbers x9: `codex_wechat.py:162`, `codex_wechat.py:444`, `codex_wechat.py:445`
- Number semantics x7: `codex_wechat.py:36`, `codex_wechat.py:36`, `codex_wechat.py:55`

## Order by package

Packages come after the workspace packages they use. Inside a package, port by wave: a unit's imports are in earlier waves.

### 1. codex-wechat (`.`, portable, 0 test files)

| Wave | Unit | Status | Lines | Readiness | Bridge | Main risks |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 0 | `codex_wechat.py` | caution | 716 | 37 | PyO3 | String offsets and slicing (10); Hash iteration order (4) |

## Risk guide

- **String offsets and slicing** (High): JS indexes strings in UTF-16 code units, Rust in UTF-8 bytes. Offsets that cross the boundary must be converted, or non-ASCII text gets wrong ranges silently.
- **Dynamic typing** (Medium): Values whose shape is checked at runtime need an explicit Rust enum or `serde_json::Value`.
- **Exceptions** (Low): Thrown errors become `Result`. Errors crossing the bridge must map back to the same JS error shape.
- **Number semantics** (Low): JS numbers are f64; bitwise ops truncate to i32; parseInt is lenient. Pick Rust integer types deliberately. Printing differs: `${n}` gives `1e+21`, `1e-7` and `0` for -0 where Rust's `{}` does not, so use `js::number_to_string`. `Math.round` rounds halves up (-2.5 gives -2), Rust's `round` away from zero: use `js::math_round`. `toFixed` rounds exact halves up ((2.5).toFixed(0) is "3"), Rust's `{:.0}` to even.
- **Integer width and overflow** (Medium): Python ints never overflow; C, Go and Java ints wrap or are undefined on overflow. Pick i64, i128 or a bignum crate deliberately, and use wrapping_* where the source wraps.
- **Division and modulo** (Medium): Python `//` and `%` floor toward negative infinity; Rust `/` and `%` truncate: use `div_euclid`/`rem_euclid` or explicit floor math. C division by zero is undefined.
- **Hash iteration order** (High): Set and map iteration order comes from the source language's hash function. A port cannot reproduce it: sort before output, or make the order part of the contract.
- **Concurrency** (Medium): Threads, goroutines and locks need a design in Rust (channels, Send/Sync), not a line-by-line translation; keep concurrency in the caller where possible.
- **Clock and random numbers** (Low): Results that depend on the time or on random numbers differ on every run, so a plain recording cannot pin them down. Record with `record --pin-clock` (TS/JS with the napi bridge, Python): `Date.now()`, `new Date()` and `Math.random` (`time.time`, `datetime.now` and `random`) then return fixed values during record, diff and gate, and the port reads the same ones from `src/port/pinned.rs` (`now_ms()`, `random()`, a documented mulberry32). Or pass the time and the seed in as parameters.

## Whole app

Shape: **cli**.

| Entry point | Kind | Found from |
| --- | --- | --- |
| `codex_wechat.py` | cli | `if __name__ == "__main__"` at codex_wechat.py:838 |

### Every dependency

None.

### Pieces, in port order

- wave 0: `codex_wechat.py` (716 lines)

### Decisions for the user

**D1** (shape): The app is a cli, entered through codex_wechat.py. Port every entry point, in that shape?

> Yes: a clap front door (`app front-door`) takes every command over first

