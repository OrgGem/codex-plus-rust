# Rust migration plan: scripts (Python)

1 package, 1 source file (79 lines), 0 test files, 0 UI files, grouped into 1 unit.

| Ready | Caution | Blocked | Keep in source language |
| ---: | ---: | ---: | ---: |
| 1 | 0 | 0 | 0 |

## Start here

Best first ports: safe to port (readiness) and worth porting (loops and string work, tests, size).

| # | Unit | Lines | Priority | Readiness | Tests | Bridge | Main risks |
| ---: | --- | ---: | ---: | ---: | ---: | --- | --- |
| 1 | `sync_official_models.py` | 79 | 63 | 86 | 0 | PyO3 | Hash iteration order (2); Dynamic typing (3); Exceptions (2) |

### `sync_official_models.py`

79 lines in 1 file, wave 0, priority 63, readiness 86, bridge **PyO3**.

- Leaf: imports no other unit.
- No tests found: write characterization tests against the original code first.
- Hash iteration order x2: `sync_official_models.py:35`, `sync_official_models.py:63`
- Dynamic typing x3: `sync_official_models.py:42`, `sync_official_models.py:44`, `sync_official_models.py:51`
- Exceptions x2: `sync_official_models.py:43`, `sync_official_models.py:52`

## Order by package

Packages come after the workspace packages they use. Inside a package, port by wave: a unit's imports are in earlier waves.

### 1. scripts (`.`, portable, 0 test files)

| Wave | Unit | Status | Lines | Readiness | Bridge | Main risks |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 0 | `sync_official_models.py` | ready | 79 | 86 | PyO3 | Hash iteration order (2); Dynamic typing (3) |

## Risk guide

- **Dynamic typing** (Medium): Values whose shape is checked at runtime need an explicit Rust enum or `serde_json::Value`.
- **Exceptions** (Low): Thrown errors become `Result`. Errors crossing the bridge must map back to the same JS error shape.
- **Hash iteration order** (High): Set and map iteration order comes from the source language's hash function. A port cannot reproduce it: sort before output, or make the order part of the contract.

## Whole app

Shape: **cli**.

| Entry point | Kind | Found from |
| --- | --- | --- |
| `sync_official_models.py` | cli | `if __name__ == "__main__"` at sync_official_models.py:99 |

### Every dependency

None.

### Pieces, in port order

- wave 0: `sync_official_models.py` (79 lines)

### Decisions for the user

**D1** (shape): The app is a cli, entered through sync_official_models.py. Port every entry point, in that shape?

> Yes: a clap front door (`app front-door`) takes every command over first

