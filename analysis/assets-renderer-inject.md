# Rust migration plan: renderer-inject (TypeScript/JavaScript)

1 package, 56 source files (38943 lines), 0 test files, 0 UI files, grouped into 56 units.

| Ready | Caution | Blocked | Keep in source language |
| ---: | ---: | ---: | ---: |
| 37 | 16 | 3 | 0 |

> 4 files had parse errors; their units are marked caution.

## Start here

No unit is a safe first port yet. See the blocked units and dependencies below.

## Order by package

Packages come after the workspace packages they use. Inside a package, port by wave: a unit's imports are in earlier waves.

### 1. renderer-inject (`.`, portable, 0 test files)

| Wave | Unit | Status | Lines | Readiness | Bridge | Main risks |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 0 | `99-tail.js` | caution | 2 | 100 | wasm-bindgen | - |
| 0 | `98-avatar-overlay-layout.js` | ready | 45 | 99 | wasm-bindgen | - |
| 0 | `99-startup.js` | ready | 45 | 99 | wasm-bindgen | - |
| 0 | `32-thread-goal.js` | ready | 17 | 98 | wasm-bindgen | Clock and random numbers (2) |
| 0 | `96-paste-fix.js` | ready | 38 | 98 | wasm-bindgen | Exceptions (1) |
| 0 | `82-boot-shell-recovery.js` | ready | 80 | 95 | wasm-bindgen | Clock and random numbers (5); Exceptions (2) |
| 0 | `98-scan-schedule.js` | ready | 171 | 95 | wasm-bindgen | Exceptions (1) |
| 0 | `31-config-read-deadline.js` | ready | 66 | 93 | wasm-bindgen | Insertion-ordered maps and object keys (1); Async and promises (1) |
| 0 | `81-capacity-auto-retry.js` | ready | 334 | 88 | wasm-bindgen | Clock and random numbers (4); Exceptions (2) |
| 0 | `00-prelude.js` | caution | 538 | 87 | wasm-bindgen | - |
| 0 | `01-registry.js` | ready | 181 | 84 | wasm-bindgen | Insertion-ordered maps and object keys (17); Exceptions (8) |
| 0 | `32-fork-readiness-fallback.js` | ready | 128 | 84 | wasm-bindgen | Reflection and prototypes (4); Insertion-ordered maps and object keys (2) |
| 0 | `50-navigation.js` | ready | 180 | 83 | wasm-bindgen | Regex features the `regex` crate lacks (2); Insertion-ordered maps and object keys (4) |
| 0 | `33-offline-auto-review.js` | ready | 152 | 81 | wasm-bindgen | String offsets and slicing (2); Insertion-ordered maps and object keys (3) |
| 0 | `92-extension-views.js` | ready | 157 | 80 | wasm-bindgen | Regexes built at runtime (1); Insertion-ordered maps and object keys (4) |
| 0 | `96-prompt-args.js` | ready | 209 | 75 | wasm-bindgen | String offsets and slicing (8); Insertion-ordered maps and object keys (2) |
| 0 | `34-statsig-overrides.js` | ready | 322 | 74 | wasm-bindgen | String offsets and slicing (1); Insertion-ordered maps and object keys (8) |
| 0 | `45-remote-control-page.js` | ready | 304 | 74 | wasm-bindgen | String offsets and slicing (1); Async and promises (13) |
| 0 | `91-extension-api.js` | ready | 473 | 74 | wasm-bindgen | Async and promises (4); Insertion-ordered maps and object keys (3) |
| 0 | `85-weixin-desktop-sync.js` | ready | 111 | 73 | wasm-bindgen | String offsets and slicing (1); Async and promises (17) |
| 0 | `95-conversation-view.js` | ready | 634 | 72 | wasm-bindgen | Insertion-ordered maps and object keys (4); Reflection and prototypes (2) |
| 0 | `61-plugin-market-ui.js` | ready | 287 | 71 | wasm-bindgen | String offsets and slicing (5); Async and promises (6) |
| 0 | `96-builtin-extensions.js` | ready | 418 | 71 | wasm-bindgen | String offsets and slicing (1); Async and promises (11) |
| 0 | `96-shell-command.js` | ready | 253 | 69 | wasm-bindgen | String offsets and slicing (2); Async and promises (9) |
| 0 | `80-session-share.js` | ready | 625 | 68 | wasm-bindgen | Async and promises (17); Insertion-ordered maps and object keys (5) |
| 0 | `96-context-tuning.js` | ready | 451 | 67 | wasm-bindgen | String offsets and slicing (1); Async and promises (16) |
| 0 | `44-provider-gateway.js` | ready | 504 | 66 | wasm-bindgen | String offsets and slicing (2); Async and promises (49) |
| 0 | `45-weixin-connect-page.js` | ready | 572 | 66 | wasm-bindgen | String offsets and slicing (1); Async and promises (21) |
| 0 | `46-service-settings.js` | ready | 596 | 66 | wasm-bindgen | String offsets and slicing (1); Async and promises (15) |
| 0 | `92-extension-extras.js` | ready | 380 | 66 | wasm-bindgen | String offsets and slicing (5); Async and promises (4) |
| 0 | `86-turn-rewind-menu.js` | ready | 305 | 65 | wasm-bindgen | String offsets and slicing (1); Regex features the `regex` crate lacks (1) |
| 0 | `92-extension-host.js` | ready | 561 | 65 | wasm-bindgen | String offsets and slicing (4); Regex features the `regex` crate lacks (1) |
| 0 | `92-extension-slots.js` | ready | 698 | 63 | wasm-bindgen | String offsets and slicing (2); Regex features the `regex` crate lacks (1) |
| 0 | `85-history-turn-recovery.js` | ready | 289 | 62 | wasm-bindgen | String offsets and slicing (3); Regex features the `regex` crate lacks (1) |
| 0 | `85-turn-rewind-controller.js` | ready | 258 | 62 | wasm-bindgen | String offsets and slicing (5); Async and promises (32) |
| 0 | `85-side-chat-save.js` | ready | 329 | 57 | wasm-bindgen | String offsets and slicing (2); Regex features the `regex` crate lacks (1) |
| 0 | `47-task-board-page.js` | ready | 831 | 53 | wasm-bindgen | String offsets and slicing (4); Async and promises (15) |
| 0 | `93-dictation.js` | ready | 578 | 53 | wasm-bindgen | String offsets and slicing (4); Regex features the `regex` crate lacks (1) |
| 0 | `96-feature-lab.js` | ready | 944 | 52 | wasm-bindgen | String offsets and slicing (2); Async and promises (20) |
| 0 | `62-plugin-market-adapter.js` | caution | 457 | 48 | wasm-bindgen | String offsets and slicing (3); Regex features the `regex` crate lacks (1) |
| 0 | `43-provider-page.js` | caution | 965 | 44 | wasm-bindgen | String offsets and slicing (6); Async and promises (28) |
| 0 | `41-extension-sources.js` | caution | 862 | 42 | wasm-bindgen | String offsets and slicing (9); Regex features the `regex` crate lacks (1) |
| 0 | `90-action-groups.js` | caution | 1030 | 42 | wasm-bindgen | Regex features the `regex` crate lacks (2); String offsets and slicing (1) |
| 0 | `20-menu.js` | blocked | 552 | 41 | wasm-bindgen | Regexes built at runtime (3); String offsets and slicing (2) |
| 0 | `41-extension-detail.js` | caution | 533 | 41 | wasm-bindgen | Regex features the `regex` crate lacks (12); String offsets and slicing (7) |
| 0 | `91-extension-rpc.js` | caution | 881 | 27 | wasm-bindgen | String offsets and slicing (30); Async and promises (73) |
| 0 | `96-hooks-editor.js` | caution | 1163 | 26 | wasm-bindgen | String offsets and slicing (4); Regex features the `regex` crate lacks (1) |
| 0 | `30-service-tier.js` | caution | 1623 | 15 | wasm-bindgen | String offsets and slicing (2); Async and promises (111) |
| 0 | `96-dsh-client.js` | blocked | 1047 | 13 | wasm-bindgen | String offsets and slicing (8); Dynamic code (eval, new Function, computed import) (1) |
| 0 | `91-extension-runtime.js` | blocked | 1442 | 6 | wasm-bindgen | String offsets and slicing (4); Dynamic code (eval, new Function, computed import) (2) |
| 0 | `70-model-catalog.js` | caution | 1643 | 3 | wasm-bindgen | String offsets and slicing (9); Regex features the `regex` crate lacks (1) |
| 0 | `10-style.js` | caution | 3510 | 0 | wasm-bindgen | String offsets and slicing (11); Insertion-ordered maps and object keys (6) |
| 0 | `40-backend-settings.js` | caution | 2798 | 0 | wasm-bindgen | String offsets and slicing (6); Regex features the `regex` crate lacks (5) |
| 0 | `42-provider-editing.generated.js` | caution | 3209 | 0 | wasm-bindgen | String offsets and slicing (57); Regex features the `regex` crate lacks (5) |
| 0 | `60-plugin-marketplace.js` | caution | 1919 | 0 | wasm-bindgen | String offsets and slicing (6); Regex features the `regex` crate lacks (3) |
| 0 | `71-model-picker-groups.js` | caution | 2243 | 0 | wasm-bindgen | String offsets and slicing (17); Regex features the `regex` crate lacks (7) |

## Blocked

- `20-menu.js`: Uses dynamic code (eval, new Function or computed imports).
- `91-extension-runtime.js`: Uses dynamic code (eval, new Function or computed imports).
- `96-dsh-client.js`: Uses dynamic code (eval, new Function or computed imports).

## Risk guide

- **String offsets and slicing** (High): JS indexes strings in UTF-16 code units, Rust in UTF-8 bytes. Offsets that cross the boundary must be converted, or non-ASCII text gets wrong ranges silently.
- **Regex features the `regex` crate lacks** (High): Lookaround, backreferences and the `y` flag are not in the `regex` crate. Use `fancy-regex` for those patterns, or rewrite them, and test them on non-ASCII text: `\w`, `\b` and case-insensitive matching differ between engines even where a pattern compiles. JS `/i` without `u` never matches a non-ASCII letter to an ASCII one, Rust `(?i)` does (`ſ` matches `s`, the Kelvin sign `K` matches `k`): write `(?i-u)` or compare with `eq_ignore_ascii_case`.
- **Regexes built at runtime** (High): Patterns come from data or config in JS syntax. Rust regex syntax differs, so user-visible behavior can change. Pin it with tests on real patterns.
- **Dynamic code (eval, new Function, computed import)** (High): eval, new Function and computed imports have no Rust equivalent. Keep this part in JS or replace it with an explicit registry.
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

Shape: **server**.

| Entry point | Kind | Found from |
| --- | --- | --- |
| `96-builtin-extensions.js` | server | `.listen(` at 96-builtin-extensions.js:172 |
| `96-paste-fix.js` | server | `.listen(` at 96-paste-fix.js:40 |
| `96-prompt-args.js` | server | `.listen(` at 96-prompt-args.js:216 |
| `96-shell-command.js` | server | `.listen(` at 96-shell-command.js:257 |

### Every dependency

None.

### Pieces, in port order

- wave 0: `00-prelude.js` (538 lines)
- wave 0: `01-registry.js` (181 lines)
- wave 0: `10-style.js` (3510 lines)
- wave 0: `20-menu.js` (552 lines)
- wave 0: `30-service-tier.js` (1623 lines)
- wave 0: `31-config-read-deadline.js` (66 lines)
- wave 0: `32-fork-readiness-fallback.js` (128 lines)
- wave 0: `32-thread-goal.js` (17 lines)
- wave 0: `33-offline-auto-review.js` (152 lines)
- wave 0: `34-statsig-overrides.js` (322 lines)
- wave 0: `40-backend-settings.js` (2798 lines)
- wave 0: `41-extension-detail.js` (533 lines)
- wave 0: `41-extension-sources.js` (862 lines)
- wave 0: `42-provider-editing.generated.js` (3209 lines)
- wave 0: `43-provider-page.js` (965 lines)
- wave 0: `44-provider-gateway.js` (504 lines)
- wave 0: `45-remote-control-page.js` (304 lines)
- wave 0: `45-weixin-connect-page.js` (572 lines)
- wave 0: `46-service-settings.js` (596 lines)
- wave 0: `47-task-board-page.js` (831 lines)
- wave 0: `50-navigation.js` (180 lines)
- wave 0: `60-plugin-marketplace.js` (1919 lines)
- wave 0: `61-plugin-market-ui.js` (287 lines)
- wave 0: `62-plugin-market-adapter.js` (457 lines)
- wave 0: `70-model-catalog.js` (1643 lines)
- wave 0: `71-model-picker-groups.js` (2243 lines)
- wave 0: `80-session-share.js` (625 lines)
- wave 0: `81-capacity-auto-retry.js` (334 lines)
- wave 0: `82-boot-shell-recovery.js` (80 lines)
- wave 0: `85-history-turn-recovery.js` (289 lines)
- wave 0: `85-side-chat-save.js` (329 lines)
- wave 0: `85-turn-rewind-controller.js` (258 lines)
- wave 0: `85-weixin-desktop-sync.js` (111 lines)
- wave 0: `86-turn-rewind-menu.js` (305 lines)
- wave 0: `90-action-groups.js` (1030 lines)
- wave 0: `91-extension-api.js` (473 lines)
- wave 0: `91-extension-rpc.js` (881 lines)
- wave 0: `91-extension-runtime.js` (1442 lines)
- wave 0: `92-extension-extras.js` (380 lines)
- wave 0: `92-extension-host.js` (561 lines)
- wave 0: `92-extension-slots.js` (698 lines)
- wave 0: `92-extension-views.js` (157 lines)
- wave 0: `93-dictation.js` (578 lines)
- wave 0: `95-conversation-view.js` (634 lines)
- wave 0: `96-builtin-extensions.js` (418 lines)
- wave 0: `96-context-tuning.js` (451 lines)
- wave 0: `96-dsh-client.js` (1047 lines)
- wave 0: `96-feature-lab.js` (944 lines)
- wave 0: `96-hooks-editor.js` (1163 lines)
- wave 0: `96-paste-fix.js` (38 lines)
- wave 0: `96-prompt-args.js` (209 lines)
- wave 0: `96-shell-command.js` (253 lines)
- wave 0: `98-avatar-overlay-layout.js` (45 lines)
- wave 0: `98-scan-schedule.js` (171 lines)
- wave 0: `99-startup.js` (45 lines)
- wave 0: `99-tail.js` (2 lines)

### Decisions for the user

**D1** (shape): The app is a server, entered through 96-builtin-extensions.js, 96-paste-fix.js, 96-prompt-args.js, 96-shell-command.js. Port every entry point, in that shape?

> Yes: an axum front door (`app front-door`) takes the server over first

**D2** (blocked 20-menu.js): 20-menu.js is blocked: Uses dynamic code (eval, new Function or computed imports). Rework that part in Rust, or keep the unit outside?

> Research the blocker first; then port it with the piece that uses it

**D3** (blocked 91-extension-runtime.js): 91-extension-runtime.js is blocked: Uses dynamic code (eval, new Function or computed imports). Rework that part in Rust, or keep the unit outside?

> Research the blocker first; then port it with the piece that uses it

**D4** (blocked 96-dsh-client.js): 96-dsh-client.js is blocked: Uses dynamic code (eval, new Function or computed imports). Rework that part in Rust, or keep the unit outside?

> Research the blocker first; then port it with the piece that uses it

