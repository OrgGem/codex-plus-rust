# Rust migration plan: claude-localization (Python)

1 package, 2 source files (639 lines), 0 test files, 0 UI files, grouped into 2 units.

| Ready | Caution | Blocked | Keep in source language |
| ---: | ---: | ---: | ---: |
| 1 | 1 | 0 | 0 |

## Start here

Best first ports: safe to port (readiness) and worth porting (loops and string work, tests, size).

| # | Unit | Lines | Priority | Readiness | Tests | Bridge | Main risks |
| ---: | --- | ---: | ---: | ---: | ---: | --- | --- |
| 1 | `sign-copy.py` | 152 | 62 | 75 | 0 | PyO3 | String offsets and slicing (3); Hash iteration order (2); Dynamic typing (5) |
| 2 | `build-copy.py` | 487 | 50 | 40 | 0 | PyO3 | String offsets and slicing (19); Regexes built at runtime (2); Dynamic typing (25) |

### `sign-copy.py`

152 lines in 1 file, wave 0, priority 62, readiness 75, bridge **PyO3**.

- Leaf: imports no other unit.
- No tests found: write characterization tests against the original code first.
- String offsets and slicing x3: `sign-copy.py:99`, `sign-copy.py:115`, `sign-copy.py:175`
- Hash iteration order x2: `sign-copy.py:28`, `sign-copy.py:33`
- Dynamic typing x5: `sign-copy.py:39`, `sign-copy.py:43`, `sign-copy.py:53`
- Exceptions x3: `sign-copy.py:44`, `sign-copy.py:60`, `sign-copy.py:180`

### `build-copy.py`

487 lines in 1 file, wave 0, priority 50, readiness 40, bridge **PyO3**.

- Leaf: imports no other unit.
- No tests found: write characterization tests against the original code first.
- String offsets and slicing x19: `build-copy.py:37`, `build-copy.py:57`, `build-copy.py:63`
- Regexes built at runtime x2: `build-copy.py:261`, `build-copy.py:261`
- Dynamic typing x25: `build-copy.py:21`, `build-copy.py:34`, `build-copy.py:41`
- Division and modulo x5: `build-copy.py:189`, `build-copy.py:284`, `build-copy.py:290`
- Generators x4: `build-copy.py:226`, `build-copy.py:226`, `build-copy.py:228`
- Exceptions x30: `build-copy.py:25`, `build-copy.py:37`, `build-copy.py:68`

## Order by package

Packages come after the workspace packages they use. Inside a package, port by wave: a unit's imports are in earlier waves.

### 1. claude-localization (`.`, portable, 0 test files)

| Wave | Unit | Status | Lines | Readiness | Bridge | Main risks |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 0 | `sign-copy.py` | ready | 152 | 75 | PyO3 | String offsets and slicing (3); Hash iteration order (2) |
| 0 | `build-copy.py` | caution | 487 | 40 | PyO3 | String offsets and slicing (19); Regexes built at runtime (2) |

## Risk guide

- **String offsets and slicing** (High): JS indexes strings in UTF-16 code units, Rust in UTF-8 bytes. Offsets that cross the boundary must be converted, or non-ASCII text gets wrong ranges silently.
- **Regexes built at runtime** (High): Patterns come from data or config in JS syntax. Rust regex syntax differs, so user-visible behavior can change. Pin it with tests on real patterns.
- **Dynamic typing** (Medium): Values whose shape is checked at runtime need an explicit Rust enum or `serde_json::Value`.
- **Exceptions** (Low): Thrown errors become `Result`. Errors crossing the bridge must map back to the same JS error shape.
- **Generators** (Medium): Lazy generators become iterators, or return a collected Vec across the bridge.
- **Division and modulo** (Medium): Python `//` and `%` floor toward negative infinity; Rust `/` and `%` truncate: use `div_euclid`/`rem_euclid` or explicit floor math. C division by zero is undefined.
- **Hash iteration order** (High): Set and map iteration order comes from the source language's hash function. A port cannot reproduce it: sort before output, or make the order part of the contract.

## Whole app

Shape: **cli**.

| Entry point | Kind | Found from |
| --- | --- | --- |
| `build-copy.py` | cli | `if __name__ == "__main__"` at build-copy.py:552 |
| `sign-copy.py` | cli | `if __name__ == "__main__"` at sign-copy.py:183 |

### Every dependency

None.

### Pieces, in port order

- wave 0: `build-copy.py` (487 lines)
- wave 0: `sign-copy.py` (152 lines)

### Decisions for the user

**D1** (shape): The app is a cli, entered through build-copy.py, sign-copy.py. Port every entry point, in that shape?

> Yes: a clap front door (`app front-door`) takes every command over first

