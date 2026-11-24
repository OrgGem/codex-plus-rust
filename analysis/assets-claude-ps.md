# Rust migration plan: claude-localization (PowerShell)

1 package, 2 source files (148 lines), 0 test files, 0 UI files, grouped into 2 units.

| Ready | Caution | Blocked | Keep in source language |
| ---: | ---: | ---: | ---: |
| 2 | 0 | 0 | 0 |

## Start here

Best first ports: safe to port (readiness) and worth porting (loops and string work, tests, size).

| # | Unit | Lines | Priority | Readiness | Tests | Bridge | Main risks |
| ---: | --- | ---: | ---: | ---: | ---: | --- | --- |
| 1 | `build-copy.ps1` | 80 | 59 | 69 | 0 | PowerShell P/Invoke (C ABI) | Callback-shaped APIs (10); Case-insensitive comparison (7); Null references (4) |
| 2 | `inject-cn.ps1` | 68 | 49 | 63 | 0 | PowerShell P/Invoke (C ABI) | Hash iteration order (3); Implicit output and the pipeline (3); Dynamic typing (2) |

### `build-copy.ps1`

80 lines in 1 file, wave 0, priority 59, readiness 69, bridge **PowerShell P/Invoke (C ABI)**.

- Leaf: imports no other unit.
- No tests found: write characterization tests against the original code first.
- Callback-shaped APIs x10: `build-copy.ps1:52`, `build-copy.ps1:52`, `build-copy.ps1:60`
- Case-insensitive comparison x7: `build-copy.ps1:57`, `build-copy.ps1:60`, `build-copy.ps1:62`
- Null references x4: `build-copy.ps1:11`, `build-copy.ps1:12`, `build-copy.ps1:35`
- Implicit output and the pipeline x3: `build-copy.ps1:75`, `build-copy.ps1:86`, `build-copy.ps1:87`
- Insertion-ordered maps and object keys x1: `build-copy.ps1:60`
- Implicit type conversion x1: `build-copy.ps1:62`
- Exceptions x11: `build-copy.ps1:13`, `build-copy.ps1:18`, `build-copy.ps1:36`

### `inject-cn.ps1`

68 lines in 1 file, wave 0, priority 49, readiness 63, bridge **PowerShell P/Invoke (C ABI)**.

- Leaf: imports no other unit.
- No tests found: write characterization tests against the original code first.
- Hash iteration order x3: `inject-cn.ps1:29`, `inject-cn.ps1:29`, `inject-cn.ps1:42`
- Implicit output and the pipeline x3: `inject-cn.ps1:26`, `inject-cn.ps1:63`, `inject-cn.ps1:70`
- Dynamic typing x2: `inject-cn.ps1:27`, `inject-cn.ps1:28`
- Callback-shaped APIs x2: `inject-cn.ps1:13`, `inject-cn.ps1:13`
- Null references x2: `inject-cn.ps1:11`, `inject-cn.ps1:55`
- Case-insensitive comparison x2: `inject-cn.ps1:12`, `inject-cn.ps1:13`
- Concurrency x1: `inject-cn.ps1:9`
- Implicit type conversion x1: `inject-cn.ps1:24`
- Exceptions x7: `inject-cn.ps1:6`, `inject-cn.ps1:13`, `inject-cn.ps1:20`
- JSON round-trips x6: `inject-cn.ps1:29`, `inject-cn.ps1:34`, `inject-cn.ps1:39`
- Clock and random numbers x3: `inject-cn.ps1:7`, `inject-cn.ps1:10`, `inject-cn.ps1:12`

## Order by package

Packages come after the workspace packages they use. Inside a package, port by wave: a unit's imports are in earlier waves.

### 1. claude-localization (`.`, portable, 0 test files)

| Wave | Unit | Status | Lines | Readiness | Bridge | Main risks |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 0 | `build-copy.ps1` | ready | 80 | 69 | PowerShell P/Invoke (C ABI) | Callback-shaped APIs (10); Case-insensitive comparison (7) |
| 0 | `inject-cn.ps1` | ready | 68 | 63 | PowerShell P/Invoke (C ABI) | Hash iteration order (3); Implicit output and the pipeline (3) |

## Risk guide

- **Dynamic typing** (Medium): Values whose shape is checked at runtime need an explicit Rust enum or `serde_json::Value`.
- **Insertion-ordered maps and object keys** (Medium): JS Map, Set and object keys keep insertion order. `HashMap` does not: use `indexmap` wherever order reaches output. With serde_json's `preserve_order`, `Map::remove` swaps the last key into the hole: use `shift_remove` for JS `delete`. Sort with `js::sort` when a comparator can return NaN: the order then depends on V8's algorithm.
- **Exceptions** (Low): Thrown errors become `Result`. Errors crossing the bridge must map back to the same JS error shape.
- **Callback-shaped APIs** (Medium): Function-typed parameters and methods cross the bridge on every call. Keep boundaries coarse: pass data in, get data out.
- **JSON round-trips** (Low): JSON.stringify key order and number formatting must match `serde_json` output if the result is user-visible.
- **Hash iteration order** (High): Set and map iteration order comes from the source language's hash function. A port cannot reproduce it: sort before output, or make the order part of the contract.
- **Concurrency** (Medium): Threads, goroutines and locks need a design in Rust (channels, Send/Sync), not a line-by-line translation; keep concurrency in the caller where possible.
- **Null references** (Medium): Any reference may be null in the source: decide per parameter whether it is `Option` and what null means, instead of mapping every value to `Option`.
- **Clock and random numbers** (Low): Results that depend on the time or on random numbers differ on every run, so a plain recording cannot pin them down. Record with `record --pin-clock` (TS/JS with the napi bridge, Python): `Date.now()`, `new Date()` and `Math.random` (`time.time`, `datetime.now` and `random`) then return fixed values during record, diff and gate, and the port reads the same ones from `src/port/pinned.rs` (`now_ms()`, `random()`, a documented mulberry32). Or pass the time and the seed in as parameters.
- **Implicit output and the pipeline** (Medium): Every value a PowerShell function does not assign or discard is output, not only what `return` names, and arrays unroll on the way out: no output reads as `$null`, one item as that item, `,@(x)` keeps the array. `$r = f ...` is what callers see, so the bridge records and compares that value; the port returns it whole (a `Vec` for several items). `Write-Host` text and non-terminating `Write-Error` records go to other streams: recorded as `output`, which a port cannot produce, so such cases fail until the function stops writing them.
- **Case-insensitive comparison** (Medium): `-eq`, `-ne`, `-like`, `-match`, `-replace`, `-split`, `-contains`, `-in`, `switch` and `@{}` keys ignore case unless written `-ceq`, `-cmatch`, `switch -CaseSensitive`. Rust's `==`, `contains` and `HashMap` do not: compare with `eq_ignore_ascii_case` or `to_lowercase()` on both sides, write regexes with `(?i)`, and key maps by a folded string. Test with non-ASCII letters: .NET folds with invariant culture rules.
- **Implicit type conversion** (Medium): PowerShell converts the right operand to the left one's type (`'10' -eq 10` is true, `10 -eq '10.0'` is true, `'a' + 1` is `a1`), `/` on two integers gives a double (`5 / 2` is 2.5), an `[int]` that overflows becomes a `[double]`, and `[int]` casts round halves to even (`[int]2.5` is 2). `$null -eq $x` and `$x -eq $null` differ when `$x` is an array (the second filters it). Write each conversion out in Rust and record cases at the edges.

## Whole app

Shape: **cli**.

| Entry point | Kind | Found from |
| --- | --- | --- |
| `build-copy.ps1` | cli | `$args` at build-copy.ps1:69 |

### Every dependency

None.

### Pieces, in port order

- wave 0: `build-copy.ps1` (80 lines)
- wave 0: `inject-cn.ps1` (68 lines)

### Decisions for the user

**D1** (shape): The app is a cli, entered through build-copy.ps1. Port every entry point, in that shape?

> Yes: a clap front door (`app front-door`) takes every command over first

