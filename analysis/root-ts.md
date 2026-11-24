# Rust migration plan: codex-plus (TypeScript/JavaScript)

3 packages, 322 source files (212877 lines), 277 test files, 74 UI files, grouped into 217 units.

| Ready | Caution | Blocked | Keep in source language |
| ---: | ---: | ---: | ---: |
| 112 | 20 | 10 | 75 |

> 7 files had parse errors; their units are marked caution.

## Start here

Best first ports: safe to port (readiness) and worth porting (loops and string work, tests, size).

| # | Unit | Lines | Priority | Readiness | Tests | Bridge | Main risks |
| ---: | --- | ---: | ---: | ---: | ---: | --- | --- |
| 1 | `tools/conversation-canvas/tree-canvas.mjs` | 36 | 95 | 100 | 1 | wasm-bindgen | Insertion-ordered maps and object keys (4) |
| 2 | `tools/conversation-canvas/organize.mjs` | 45 | 92 | 93 | 1 | wasm-bindgen | String offsets and slicing (1); Insertion-ordered maps and object keys (6); Exceptions (12) |
| 3 | `tools/conversation-canvas/model.mjs` | 57 | 91 | 88 | 4 | wasm-bindgen | String offsets and slicing (2); Insertion-ordered maps and object keys (8); Exceptions (2) |
| 4 | `tools/conversation-canvas/native-history.mjs` | 68 | 90 | 83 | 1 | wasm-bindgen | Regex features the `regex` crate lacks (2); String offsets and slicing (1); Async and promises (8) |
| 5 | `apps/codex-plus-manager/src/code-highlight.ts` | 253 | 87 | 85 | 1 | napi-rs | String offsets and slicing (21) |
| 6 | `apps/codex-plus-manager/src/management-model.ts` | 41 | 86 | 92 | 2 | napi-rs | String offsets and slicing (3); Insertion-ordered maps and object keys (1); Clock and random numbers (1) |
| 7 | `tools/conversation-canvas/long-organizer.mjs` | 147 | 84 | 68 | 2 | wasm-bindgen | String offsets and slicing (10); Insertion-ordered maps and object keys (20); Async and promises (16) |
| 8 | `apps/codex-plus-manager/src/goals-config.ts` | 88 | 82 | 100 | 1 | napi-rs | String offsets and slicing (1) |
| 9 | `apps/codex-plus-manager/src/provider-name.ts` | 60 | 82 | 92 | 2 | napi-rs | String offsets and slicing (3); Regexes built at runtime (1) |
| 10 | `apps/codex-plus-manager/src/provider-api-keys.ts` | 77 | 80 | 94 | 3 | napi-rs | String offsets and slicing (2); Insertion-ordered maps and object keys (2); Clock and random numbers (1) |

### `tools/conversation-canvas/tree-canvas.mjs`

36 lines in 1 file, wave 1, priority 95, readiness 100, bridge **wasm-bindgen**.

- 1 test file imports it directly.
- Portable code: wasm-bindgen keeps it running in browsers and Node. It needs the wasm32-unknown-unknown target and wasm-bindgen; `bridge --kind napi` runs with what Node already has.
- Insertion-ordered maps and object keys x4: `tools/conversation-canvas/tree-canvas.mjs:6`, `tools/conversation-canvas/tree-canvas.mjs:7`, `tools/conversation-canvas/tree-canvas.mjs:7`

### `tools/conversation-canvas/organize.mjs`

45 lines in 1 file, wave 0, priority 92, readiness 93, bridge **wasm-bindgen**.

- Leaf: imports no other unit.
- 1 test file imports it directly.
- Portable code: wasm-bindgen keeps it running in browsers and Node. It needs the wasm32-unknown-unknown target and wasm-bindgen; `bridge --kind napi` runs with what Node already has.
- String offsets and slicing x1: `tools/conversation-canvas/organize.mjs:41`
- Insertion-ordered maps and object keys x6: `tools/conversation-canvas/organize.mjs:14`, `tools/conversation-canvas/organize.mjs:14`, `tools/conversation-canvas/organize.mjs:20`
- Exceptions x12: `tools/conversation-canvas/organize.mjs:3`, `tools/conversation-canvas/organize.mjs:13`, `tools/conversation-canvas/organize.mjs:16`
- JSON round-trips x3: `tools/conversation-canvas/organize.mjs:3`, `tools/conversation-canvas/organize.mjs:9`, `tools/conversation-canvas/organize.mjs:43`

### `tools/conversation-canvas/model.mjs`

57 lines in 1 file, wave 0, priority 91, readiness 88, bridge **wasm-bindgen**.

- Leaf: imports no other unit.
- 4 test files import it directly.
- Portable code: wasm-bindgen keeps it running in browsers and Node. It needs the wasm32-unknown-unknown target and wasm-bindgen; `bridge --kind napi` runs with what Node already has.
- String offsets and slicing x2: `tools/conversation-canvas/model.mjs:17`, `tools/conversation-canvas/model.mjs:53`
- Insertion-ordered maps and object keys x8: `tools/conversation-canvas/model.mjs:21`, `tools/conversation-canvas/model.mjs:23`, `tools/conversation-canvas/model.mjs:31`
- Exceptions x2: `tools/conversation-canvas/model.mjs:7`, `tools/conversation-canvas/model.mjs:12`
- JSON round-trips x1: `tools/conversation-canvas/model.mjs:7`

### `tools/conversation-canvas/native-history.mjs`

68 lines in 1 file, wave 1, priority 90, readiness 83, bridge **wasm-bindgen**.

- 1 test file imports it directly.
- Portable code: wasm-bindgen keeps it running in browsers and Node. It needs the wasm32-unknown-unknown target and wasm-bindgen; `bridge --kind napi` runs with what Node already has.
- Regex features the `regex` crate lacks x2: `tools/conversation-canvas/native-history.mjs:4`, `tools/conversation-canvas/native-history.mjs:41`
- String offsets and slicing x1: `tools/conversation-canvas/native-history.mjs:33`
- Async and promises x8: `tools/conversation-canvas/native-history.mjs:9`, `tools/conversation-canvas/native-history.mjs:16`, `tools/conversation-canvas/native-history.mjs:28`
- Insertion-ordered maps and object keys x5: `tools/conversation-canvas/native-history.mjs:9`, `tools/conversation-canvas/native-history.mjs:10`, `tools/conversation-canvas/native-history.mjs:10`
- Exceptions x9: `tools/conversation-canvas/native-history.mjs:14`, `tools/conversation-canvas/native-history.mjs:17`, `tools/conversation-canvas/native-history.mjs:19`

### `apps/codex-plus-manager/src/code-highlight.ts`

253 lines in 1 file, wave 0, priority 87, readiness 85, bridge **napi-rs**.

- Leaf: imports no other unit.
- 1 test file imports it directly.
- String offsets and slicing x21: `apps/codex-plus-manager/src/code-highlight.ts:82`, `apps/codex-plus-manager/src/code-highlight.ts:90`, `apps/codex-plus-manager/src/code-highlight.ts:92`

## Order by package

Packages come after the workspace packages they use. Inside a package, port by wave: a unit's imports are in earlier waves.

### 1. codex-plus (`.`, node, 30 test files)

| Wave | Unit | Status | Lines | Readiness | Bridge | Main risks |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 0 | `assets/cu/` | ready | 32 | 100 | napi-rs | - |
| 0 | `crates/codex-plus-core/src/skills_github_proxy_fixtures/` | ready | 114 | 100 | napi-rs | Exceptions (1); JSON round-trips (1) |
| 0 | `tools/rice-maid-pet/backend/` | ready | 18 | 100 | napi-rs | Clock and random numbers (1) |
| 0 | `tools/rice-maid-pet/hooks/` | ready | 8 | 100 | napi-rs | Async and promises (2); Exceptions (1) |
| 0 | `tools/claude-extension-demo/` | ready | 21 | 99 | napi-rs | String offsets and slicing (1) |
| 0 | `assets/inject/upstream/glass-vision/` | ready | 111 | 91 | napi-rs | String offsets and slicing (3); Number semantics (4) |
| 0 | `assets/inject/upstream/snow-skin/` | ready | 126 | 89 | napi-rs | String offsets and slicing (3); Insertion-ordered maps and object keys (1) |
| 0 | `tools/rice-maid-pet/` | ready | 357 | 89 | napi-rs | Async and promises (2); Insertion-ordered maps and object keys (1) |
| 0 | `assets/extension-creator/` | ready | 167 | 82 | napi-rs | String offsets and slicing (7); Insertion-ordered maps and object keys (1) |
| 0 | `assets/extensions/typing-effects/` | ready | 413 | 79 | napi-rs | String offsets and slicing (2); Reflection and prototypes (1) |
| 0 | `site/js/` | ready | 266 | 78 | napi-rs | String offsets and slicing (5); Async and promises (3) |
| 0 | `site/scripts/` | ready | 88 | 78 | napi-rs | String offsets and slicing (8); Async and promises (6) |
| 0 | `assets/claude-extensions/` | ready | 165 | 77 | napi-rs | String offsets and slicing (2); Async and promises (32) |
| 0 | `assets/extensions/proxy-request-status/` | ready | 314 | 76 | napi-rs | String offsets and slicing (2); Async and promises (5) |
| 0 | `assets/inject/upstream/cidala-tiger/windows/` | ready | 436 | 71 | napi-rs | String offsets and slicing (4); Regex features the `regex` crate lacks (1) |
| 0 | `tools/` | ready | 299 | 69 | napi-rs | String offsets and slicing (11); Regexes built at runtime (1) |
| 0 | `services/share-site/` | ready | 346 | 65 | napi-rs | String offsets and slicing (5); Async and promises (34) |
| 0 | `assets/inject/floating-panel/outline/` | ready | 645 | 60 | napi-rs | Regex features the `regex` crate lacks (5); String offsets and slicing (3) |
| 0 | `assets/inject/floating-panel/stepwise/` | ready | 536 | 58 | napi-rs | String offsets and slicing (12); Regex features the `regex` crate lacks (4) |
| 0 | `assets/inject/upstream/cidala-tiger/macos/` | ready | 879 | 52 | napi-rs | String offsets and slicing (5); Insertion-ordered maps and object keys (14) |
| 0 | `docs/` | caution | 850 | 43 | napi-rs | String offsets and slicing (3); Regex features the `regex` crate lacks (3) |
| 0 | `assets/inject/upstream/dream-skin/macos/` | caution | 1132 | 38 | napi-rs | String offsets and slicing (11); Insertion-ordered maps and object keys (22) |
| 0 | `assets/inject/upstream/dream-skin/windows/` | caution | 1152 | 38 | napi-rs | String offsets and slicing (11); Insertion-ordered maps and object keys (22) |
| 0 | `assets/extensions/image-overlay/` | caution | 1674 | 37 | napi-rs | String offsets and slicing (1); Insertion-ordered maps and object keys (34) |
| 0 | `assets/extensions/usage-statusline/` | caution | 1220 | 36 | napi-rs | String offsets and slicing (7); Insertion-ordered maps and object keys (16) |
| 0 | `assets/claude-localization/` | caution | 1607 | 12 | napi-rs | String offsets and slicing (14); Regexes built at runtime (2) |
| 0 | `assets/inject/floating-panel/runtime/` | caution | 1752 | 9 | napi-rs | Regex features the `regex` crate lacks (16); String offsets and slicing (8) |
| 0 | `assets/extensions/agent-pet/` | caution | 1772 | 4 | napi-rs | String offsets and slicing (12); Async and promises (77) |
| 0 | `assets/dsh-host/shims/` | caution | 2148 | 0 | napi-rs | String offsets and slicing (13); Regex features the `regex` crate lacks (2) |
| 0 | `assets/inject/` | blocked | 41928 | 0 | napi-rs | String offsets and slicing (308); Regex features the `regex` crate lacks (57) |
| 0 | `assets/inject/floating-panel/core/` | blocked | 5200 | 0 | napi-rs | String offsets and slicing (5); Regex features the `regex` crate lacks (1) |
| 0 | `assets/inject/renderer-inject/` | blocked | 38943 | 0 | napi-rs | String offsets and slicing (240); Regex features the `regex` crate lacks (47) |
| 0 | `assets/inject/upstream/whale-widget/` | caution | 34519 | 0 | napi-rs | String offsets and slicing (150); Regex features the `regex` crate lacks (4) |
| 0 | `assets/native-browser/` | caution | 705 | 0 | napi-rs | String offsets and slicing (257); Regex features the `regex` crate lacks (10) |
| 0 | `scripts/` | blocked | 5357 | 0 | napi-rs | String offsets and slicing (35); Regexes built at runtime (6) |
| 0 | `site/manager-preview/assets/` | caution | 797 | 0 | napi-rs | String offsets and slicing (224); Regex features the `regex` crate lacks (22) |
| 0 | `tools/remote-control-p0/` | caution | 1644 | 0 | napi-rs | String offsets and slicing (26); Regex features the `regex` crate lacks (2) |
| 1 | `assets/dsh-host/` | blocked | 6902 | 0 | napi-rs | String offsets and slicing (74); Dynamic code (eval, new Function, computed import) (4) |

### 2. codex-plus-manager (`apps/codex-plus-manager`, node, 236 test files)

| Wave | Unit | Status | Lines | Readiness | Bridge | Main risks |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 0 | `apps/codex-plus-manager/src/agent-collaboration-queue.ts` | ready | 48 | 100 | napi-rs | - |
| 0 | `apps/codex-plus-manager/src/aggregate-members.ts` | ready | 106 | 100 | napi-rs | Insertion-ordered maps and object keys (1); Exceptions (1) |
| 0 | `apps/codex-plus-manager/src/aggregate-routes.ts` | ready | 70 | 100 | napi-rs | Dynamic typing (2); Number semantics (1) |
| 0 | `apps/codex-plus-manager/src/auto-compact.ts` | ready | 40 | 100 | napi-rs | - |
| 0 | `apps/codex-plus-manager/src/dictation-settings.ts` | ready | 47 | 100 | napi-rs | Number semantics (1) |
| 0 | `apps/codex-plus-manager/src/enhancement-navigation.ts` | ready | 14 | 100 | napi-rs | - |
| 0 | `apps/codex-plus-manager/src/extension-config-schema.ts` | ready | 71 | 100 | napi-rs | Insertion-ordered maps and object keys (1) |
| 0 | `apps/codex-plus-manager/src/extension-manager-pages.ts` | ready | 119 | 100 | napi-rs | Callback-shaped APIs (1); Exceptions (1) |
| 0 | `apps/codex-plus-manager/src/goals-config.ts` | ready | 88 | 100 | napi-rs | String offsets and slicing (1) |
| 0 | `apps/codex-plus-manager/src/history-recovery-report.ts` | ready | 15 | 100 | napi-rs | - |
| 0 | `apps/codex-plus-manager/src/launch-status.ts` | ready | 28 | 100 | napi-rs | Insertion-ordered maps and object keys (2) |
| 0 | `apps/codex-plus-manager/src/mac-titlebar.ts` | ready | 21 | 100 | napi-rs | Dynamic typing (1); Exceptions (1) |
| 0 | `apps/codex-plus-manager/src/mcp-presets.ts` | caution | 82 | 100 | napi-rs | Callback-shaped APIs (1); JSON round-trips (2) |
| 0 | `apps/codex-plus-manager/src/model-groups.ts` | ready | 70 | 100 | napi-rs | Regexes built at runtime (1); Insertion-ordered maps and object keys (2) |
| 0 | `apps/codex-plus-manager/src/multi-launch-model.ts` | ready | 85 | 100 | napi-rs | - |
| 0 | `apps/codex-plus-manager/src/native-auth-model.ts` | ready | 28 | 100 | napi-rs | - |
| 0 | `apps/codex-plus-manager/src/notice-model.ts` | ready | 17 | 100 | napi-rs | Insertion-ordered maps and object keys (3) |
| 0 | `apps/codex-plus-manager/src/onboarding-model.ts` | ready | 119 | 100 | napi-rs | Insertion-ordered maps and object keys (1) |
| 0 | `apps/codex-plus-manager/src/overview-model.ts` | ready | 53 | 100 | napi-rs | Insertion-ordered maps and object keys (1); Number semantics (1) |
| 0 | `apps/codex-plus-manager/src/page-groups.ts` | ready | 26 | 100 | napi-rs | Callback-shaped APIs (2) |
| 0 | `apps/codex-plus-manager/src/plugin-market-model.ts` | ready | 121 | 100 | napi-rs | String offsets and slicing (1) |
| 0 | `apps/codex-plus-manager/src/provider-brand-icons.ts` | ready | 64 | 100 | napi-rs | - |
| 0 | `apps/codex-plus-manager/src/provider-brands.ts` | ready | 69 | 100 | napi-rs | Insertion-ordered maps and object keys (1) |
| 0 | `apps/codex-plus-manager/src/provider-sync-flow.ts` | ready | 63 | 100 | napi-rs | - |
| 0 | `apps/codex-plus-manager/src/provider-sync-model.ts` | ready | 68 | 100 | napi-rs | Callback-shaped APIs (1) |
| 0 | `apps/codex-plus-manager/src/provider-sync-target.ts` | ready | 21 | 100 | napi-rs | - |
| 0 | `apps/codex-plus-manager/src/provider-test.ts` | ready | 76 | 100 | napi-rs | Number semantics (2) |
| 0 | `apps/codex-plus-manager/src/relay-live-files.ts` | ready | 22 | 100 | napi-rs | - |
| 0 | `apps/codex-plus-manager/src/relay-session.ts` | ready | 17 | 100 | napi-rs | - |
| 0 | `apps/codex-plus-manager/src/settings-rebase.ts` | caution | 67 | 100 | napi-rs | Insertion-ordered maps and object keys (3); JSON round-trips (2) |
| 0 | `apps/codex-plus-manager/src/subscription-policy.ts` | ready | 45 | 100 | napi-rs | - |
| 0 | `apps/codex-plus-manager/src/theme-mode.ts` | ready | 56 | 100 | napi-rs | Callback-shaped APIs (2); Exceptions (4) |
| 0 | `apps/codex-plus-manager/src/vlm-test-translation.ts` | caution | 37 | 100 | napi-rs | Callback-shaped APIs (1); Number semantics (1) |
| 0 | `apps/codex-plus-manager/src/whale-settings.ts` | ready | 45 | 100 | napi-rs | - |
| 0 | `apps/codex-plus-manager/src/agent-cache-model.ts` | ready | 60 | 99 | napi-rs | String offsets and slicing (1); Insertion-ordered maps and object keys (3) |
| 0 | `apps/codex-plus-manager/src/command-palette-model.ts` | keep | 30 | 99 | keep | Regex features the `regex` crate lacks (1); Callback-shaped APIs (1) |
| 0 | `apps/codex-plus-manager/src/flow-provider-colors.ts` | ready | 31 | 99 | napi-rs | Insertion-ordered maps and object keys (2); Exceptions (2) |
| 0 | `apps/codex-plus-manager/src/presets.ts` | ready | 273 | 99 | napi-rs | - |
| 0 | `apps/codex-plus-manager/src/settings-events.ts` | ready | 8 | 99 | napi-rs | - |
| 0 | `apps/codex-plus-manager/src/escape-layer.ts` | ready | 52 | 98 | napi-rs | String offsets and slicing (1); Callback-shaped APIs (5) |
| 0 | `apps/codex-plus-manager/src/relay-headers.ts` | ready | 102 | 98 | napi-rs | Insertion-ordered maps and object keys (1) |
| 0 | `apps/codex-plus-manager/src/model-routes.ts` | ready | 167 | 97 | napi-rs | Insertion-ordered maps and object keys (3); Exceptions (1) |
| 0 | `apps/codex-plus-manager/src/SkillListViewport.tsx` | keep | 39 | 95 | keep | String offsets and slicing (1); Callback-shaped APIs (1) |
| 0 | `apps/codex-plus-manager/src/config-doctor-model.ts` | ready | 235 | 95 | napi-rs | String offsets and slicing (2); Number semantics (2) |
| 0 | `apps/codex-plus-manager/src/github-repository.ts` | ready | 73 | 95 | napi-rs | String offsets and slicing (2); Insertion-ordered maps and object keys (2) |
| 0 | `apps/codex-plus-manager/src/popover-layer.ts` | ready | 180 | 95 | napi-rs | Callback-shaped APIs (2); Insertion-ordered maps and object keys (1) |
| 0 | `apps/codex-plus-manager/src/script-market-model.ts` | ready | 103 | 95 | napi-rs | String offsets and slicing (3); Number semantics (2) |
| 0 | `apps/codex-plus-manager/src/agent-mcp-model.ts` | ready | 66 | 94 | napi-rs | String offsets and slicing (1); Regex features the `regex` crate lacks (1) |
| 0 | `apps/codex-plus-manager/src/lib/` | ready | 5 | 94 | napi-rs | - |
| 0 | `apps/codex-plus-manager/src/enhancement-catalog.ts` | ready | 161 | 93 | napi-rs | Regex features the `regex` crate lacks (2); Insertion-ordered maps and object keys (1) |
| 0 | `apps/codex-plus-manager/src/TopbarActions.tsx` | keep | 14 | 92 | keep | - |
| 0 | `apps/codex-plus-manager/src/management-model.ts` | ready | 41 | 92 | napi-rs | String offsets and slicing (3); Insertion-ordered maps and object keys (1) |
| 0 | `apps/codex-plus-manager/src/provider-name.ts` | ready | 60 | 92 | napi-rs | String offsets and slicing (3); Regexes built at runtime (1) |
| 0 | `apps/codex-plus-manager/src/update-notification.ts` | ready | 109 | 91 | napi-rs | String offsets and slicing (2); Callback-shaped APIs (2) |
| 0 | `apps/codex-plus-manager/src/legacy-webkit.ts` | ready | 92 | 89 | napi-rs | String offsets and slicing (1); Reflection and prototypes (2) |
| 0 | `apps/codex-plus-manager/src/relay-billing.ts` | ready | 223 | 89 | napi-rs | String offsets and slicing (1); Insertion-ordered maps and object keys (1) |
| 0 | `apps/codex-plus-manager/src/use-modal-focus.ts` | keep | 61 | 89 | keep | String offsets and slicing (2); Insertion-ordered maps and object keys (1) |
| 0 | `apps/codex-plus-manager/src/dream-skin.ts` | ready | 333 | 88 | napi-rs | Regex features the `regex` crate lacks (1); Async and promises (2) |
| 0 | `apps/codex-plus-manager/src/extension-manager-bridge.ts` | ready | 159 | 86 | napi-rs | String offsets and slicing (1); Callback-shaped APIs (10) |
| 0 | `apps/codex-plus-manager/src/agent-focus-model.ts` | ready | 73 | 85 | napi-rs | Async and promises (2); Callback-shaped APIs (2) |
| 0 | `apps/codex-plus-manager/src/code-highlight.ts` | ready | 253 | 85 | napi-rs | String offsets and slicing (21) |
| 0 | `apps/codex-plus-manager/src/context-config-text.ts` | ready | 248 | 82 | napi-rs | String offsets and slicing (5); Regex features the `regex` crate lacks (1) |
| 0 | `apps/codex-plus-manager/src/log-view-model.ts` | ready | 185 | 80 | napi-rs | Regex features the `regex` crate lacks (5); String offsets and slicing (3) |
| 0 | `apps/codex-plus-manager/src/skill-agent-batch.ts` | ready | 142 | 79 | napi-rs | String offsets and slicing (2); Callback-shaped APIs (13) |
| 0 | `apps/codex-plus-manager/src/ReleaseNotes.ts` | keep | 140 | 78 | keep | String offsets and slicing (14); Regex features the `regex` crate lacks (1) |
| 0 | `apps/codex-plus-manager/src/skill-groups.ts` | ready | 277 | 75 | napi-rs | String offsets and slicing (7); Insertion-ordered maps and object keys (14) |
| 0 | `apps/codex-plus-manager/src/skills-batch-react.manual.ts` | blocked | 420 | 51 | napi-rs | Dynamic code (eval, new Function, computed import) (1); Async and promises (239) |
| 0 | `apps/codex-plus-manager/src/runtime-context-capnweb-failure.manual.cjs` | blocked | 434 | 35 | napi-rs | String offsets and slicing (10); Dynamic code (eval, new Function, computed import) (6) |
| 0 | `apps/codex-plus-manager/src/model-picker-views-runtime.manual.ts` | blocked | 915 | 33 | napi-rs | Regexes built at runtime (1); Dynamic code (eval, new Function, computed import) (1) |
| 0 | `apps/codex-plus-manager/src/i18n-en.ts` | caution | 4760 | 0 | napi-rs | - |
| 1 | `apps/codex-plus-manager/src/extension-host-model.ts` | ready | 27 | 100 | napi-rs | - |
| 1 | `apps/codex-plus-manager/src/skill-repository.ts` | ready | 18 | 100 | napi-rs | String offsets and slicing (1); Exceptions (1) |
| 1 | `apps/codex-plus-manager/src/use-escape-layer.ts` | keep | 19 | 86 | keep | Callback-shaped APIs (1) |
| 1 | `apps/codex-plus-manager/src/i18n.ts` | ready | 66 | 80 | napi-rs | Callback-shaped APIs (3); Insertion-ordered maps and object keys (1) |
| 1 | `apps/codex-plus-manager/src/components/ui/` | keep | 138 | 78 | keep | - |
| 1 | `apps/codex-plus-manager/src/agent-focus-store.ts` | keep | 108 | 73 | keep | Async and promises (2); Dynamic typing (1) |
| 1 | `apps/codex-plus-manager/src/model-metadata.ts` | caution | 929 | 44 | napi-rs | String offsets and slicing (4); Regex features the `regex` crate lacks (3) |
| 2 | `apps/codex-plus-manager/src/use-popover-layer.ts` | keep | 153 | 95 | keep | Callback-shaped APIs (1) |
| 2 | `apps/codex-plus-manager/src/CodexLaunchButton.tsx` | keep | 48 | 94 | keep | Callback-shaped APIs (2) |
| 2 | `apps/codex-plus-manager/src/CommandPalette.tsx` | keep | 80 | 94 | keep | Callback-shaped APIs (1) |
| 2 | `apps/codex-plus-manager/src/CuPipWindow.tsx` | keep | 51 | 93 | keep | - |
| 2 | `apps/codex-plus-manager/src/model-reasoning.ts` | ready | 149 | 93 | napi-rs | String offsets and slicing (2); Insertion-ordered maps and object keys (3) |
| 2 | `apps/codex-plus-manager/src/model-windows.ts` | ready | 258 | 93 | napi-rs | Insertion-ordered maps and object keys (4); JSON round-trips (8) |
| 2 | `apps/codex-plus-manager/src/SkillBackupUsage.tsx` | keep | 59 | 91 | keep | Clock and random numbers (2); Exceptions (1) |
| 2 | `apps/codex-plus-manager/src/ExtensionManagerPanel.tsx` | keep | 102 | 90 | keep | Callback-shaped APIs (3); Async and promises (1) |
| 2 | `apps/codex-plus-manager/src/RelayHeaderRows.tsx` | keep | 69 | 89 | keep | String offsets and slicing (1); Callback-shaped APIs (1) |
| 2 | `apps/codex-plus-manager/src/components/` | keep | 160 | 89 | keep | String offsets and slicing (1); Callback-shaped APIs (2) |
| 2 | `apps/codex-plus-manager/src/SkillFileTimes.tsx` | keep | 36 | 88 | keep | String offsets and slicing (1) |
| 2 | `apps/codex-plus-manager/src/skill-source-scan.ts` | ready | 205 | 88 | napi-rs | Async and promises (4); Insertion-ordered maps and object keys (3) |
| 2 | `apps/codex-plus-manager/src/ExtensionPublishDialog.tsx` | keep | 183 | 80 | keep | Async and promises (5); Callback-shaped APIs (2) |
| 2 | `apps/codex-plus-manager/src/cx-layout.tsx` | keep | 187 | 75 | keep | Callback-shaped APIs (4) |
| 2 | `apps/codex-plus-manager/src/app-dialog.tsx` | keep | 170 | 74 | keep | Async and promises (3); Callback-shaped APIs (3) |
| 2 | `apps/codex-plus-manager/src/gateway-model.ts` | caution | 1679 | 5 | napi-rs | String offsets and slicing (31); Regex features the `regex` crate lacks (3) |
| 3 | `apps/codex-plus-manager/src/agent-dashboard-model.ts` | ready | 59 | 100 | napi-rs | String offsets and slicing (1) |
| 3 | `apps/codex-plus-manager/src/gateway-agent-safety.ts` | ready | 106 | 100 | napi-rs | - |
| 3 | `apps/codex-plus-manager/src/gateway-auto-compact-model.ts` | ready | 25 | 100 | napi-rs | Insertion-ordered maps and object keys (2) |
| 3 | `apps/codex-plus-manager/src/provider-form-model.ts` | ready | 88 | 100 | napi-rs | Insertion-ordered maps and object keys (2); Number semantics (1) |
| 3 | `apps/codex-plus-manager/src/provider-pool-layout.ts` | ready | 19 | 100 | napi-rs | Insertion-ordered maps and object keys (1) |
| 3 | `apps/codex-plus-manager/src/provider-meta-sync-model.ts` | ready | 148 | 97 | napi-rs | Regex features the `regex` crate lacks (1); Insertion-ordered maps and object keys (1) |
| 3 | `apps/codex-plus-manager/src/UsageTabs.tsx` | keep | 39 | 96 | keep | Callback-shaped APIs (3) |
| 3 | `apps/codex-plus-manager/src/tray-panel-model.ts` | ready | 205 | 96 | napi-rs | String offsets and slicing (1); Exceptions (4) |
| 3 | `apps/codex-plus-manager/src/provider-balances.ts` | ready | 64 | 95 | napi-rs | Callback-shaped APIs (4); Insertion-ordered maps and object keys (1) |
| 3 | `apps/codex-plus-manager/src/OnboardingTour.tsx` | keep | 120 | 93 | keep | Callback-shaped APIs (1) |
| 3 | `apps/codex-plus-manager/src/onboarding-setup-model.ts` | ready | 191 | 87 | napi-rs | String offsets and slicing (2); Dynamic typing (2) |
| 3 | `apps/codex-plus-manager/src/request-flow-model.ts` | ready | 134 | 86 | napi-rs | String offsets and slicing (3); Insertion-ordered maps and object keys (5) |
| 3 | `apps/codex-plus-manager/src/gateway-store.ts` | keep | 60 | 81 | keep | Callback-shaped APIs (4); Insertion-ordered maps and object keys (1) |
| 3 | `apps/codex-plus-manager/src/AppSelect.tsx` | keep | 120 | 80 | keep | Callback-shaped APIs (1) |
| 3 | `apps/codex-plus-manager/src/gateway-ui.tsx` | keep | 276 | 56 | keep | Async and promises (5); Callback-shaped APIs (5) |
| 4 | `apps/codex-plus-manager/src/hub-overview-model.ts` | ready | 160 | 98 | napi-rs | Insertion-ordered maps and object keys (1) |
| 4 | `apps/codex-plus-manager/src/SubscriptionPolicyFields.tsx` | keep | 56 | 96 | keep | Insertion-ordered maps and object keys (2); Callback-shaped APIs (1) |
| 4 | `apps/codex-plus-manager/src/RequestFlow.tsx` | keep | 96 | 90 | keep | Async and promises (2); Exceptions (1) |
| 4 | `apps/codex-plus-manager/src/ProviderKeyList.tsx` | keep | 107 | 88 | keep | Async and promises (6); Callback-shaped APIs (1) |
| 4 | `apps/codex-plus-manager/src/GatewayModelMatrix.tsx` | keep | 146 | 87 | keep | String offsets and slicing (1); Callback-shaped APIs (1) |
| 4 | `apps/codex-plus-manager/src/ProviderModelsDialog.tsx` | keep | 146 | 84 | keep | Async and promises (2); Callback-shaped APIs (2) |
| 4 | `apps/codex-plus-manager/src/ProviderReasoningDialog.tsx` | keep | 162 | 84 | keep | Dynamic typing (2); Callback-shaped APIs (2) |
| 4 | `apps/codex-plus-manager/src/ProviderTestDialog.tsx` | keep | 212 | 79 | keep | Callback-shaped APIs (4); Async and promises (2) |
| 4 | `apps/codex-plus-manager/src/extension-config-form.tsx` | keep | 211 | 79 | keep | Callback-shaped APIs (5); Insertion-ordered maps and object keys (2) |
| 4 | `apps/codex-plus-manager/src/PluginMarketScreen.tsx` | keep | 319 | 78 | keep | Async and promises (7); Exceptions (3) |
| 4 | `apps/codex-plus-manager/src/AgentDisconnectDialog.tsx` | keep | 141 | 77 | keep | Async and promises (5); Callback-shaped APIs (3) |
| 4 | `apps/codex-plus-manager/src/ConfigDoctorPanel.tsx` | keep | 360 | 77 | keep | Async and promises (8); Exceptions (4) |
| 4 | `apps/codex-plus-manager/src/ComputerUsePreview.tsx` | keep | 244 | 75 | keep | Async and promises (9); Callback-shaped APIs (2) |
| 4 | `apps/codex-plus-manager/src/NativeAuthControls.tsx` | keep | 137 | 75 | keep | String offsets and slicing (1); Async and promises (8) |
| 4 | `apps/codex-plus-manager/src/ProviderMetaSyncDialog.tsx` | keep | 184 | 73 | keep | String offsets and slicing (1); Insertion-ordered maps and object keys (4) |
| 4 | `apps/codex-plus-manager/src/RelayBillingLine.tsx` | keep | 334 | 72 | keep | Callback-shaped APIs (7); Insertion-ordered maps and object keys (2) |
| 4 | `apps/codex-plus-manager/src/ProviderSyncDialog.tsx` | keep | 181 | 71 | keep | String offsets and slicing (1); Insertion-ordered maps and object keys (7) |
| 4 | `apps/codex-plus-manager/src/ProviderExtrasDialog.tsx` | keep | 372 | 66 | keep | Callback-shaped APIs (7); Async and promises (4) |
| 4 | `apps/codex-plus-manager/src/GatewaySessions.tsx` | keep | 220 | 65 | keep | String offsets and slicing (2); Async and promises (18) |
| 5 | `apps/codex-plus-manager/src/ProviderWindowsDialog.tsx` | keep | 192 | 79 | keep | Insertion-ordered maps and object keys (8); Async and promises (2) |
| 5 | `apps/codex-plus-manager/src/GatewayAgents.tsx` | keep | 1340 | 26 | keep | String offsets and slicing (3); Callback-shaped APIs (21) |
| 6 | `apps/codex-plus-manager/src/ProviderFormDialog.tsx` | keep | 408 | 63 | keep | String offsets and slicing (1); Insertion-ordered maps and object keys (6) |
| 7 | `apps/codex-plus-manager/src/GatewayProviders.tsx` | keep | 1451 | 4 | keep | String offsets and slicing (5); Async and promises (50) |
| 8 | `apps/codex-plus-manager/src/HubOverview.tsx` | keep | 474 | 55 | keep | String offsets and slicing (1); Callback-shaped APIs (4) |
| 9 | `apps/codex-plus-manager/src/SessionAgentTabs.tsx` | keep | 28 | 100 | keep | Callback-shaped APIs (1) |
| 9 | `apps/codex-plus-manager/src/AgentFocusDialog.tsx` | keep | 146 | 79 | keep | String offsets and slicing (1); Async and promises (2) |
| 9 | `apps/codex-plus-manager/src/AgentDashboard.tsx` | keep | 214 | 74 | keep | String offsets and slicing (1); Callback-shaped APIs (4) |
| 9 | `apps/codex-plus-manager/src/OnboardingSetup.tsx` | keep | 318 | 71 | keep | Async and promises (10); Callback-shaped APIs (6) |
| 9 | `apps/codex-plus-manager/src/ManagementPages.tsx` | keep | 326 | 47 | keep | Regex features the `regex` crate lacks (1); Async and promises (37) |
| 9 | `apps/codex-plus-manager/src/TrayPanel.tsx` | keep | 937 | 37 | keep | String offsets and slicing (3); Async and promises (25) |
| 10 | `apps/codex-plus-manager/src/ExtensionCard.tsx` | keep | 83 | 86 | keep | String offsets and slicing (1); Callback-shaped APIs (1) |
| 10 | `apps/codex-plus-manager/src/SkillInPlaceUpdateDrawer.tsx` | keep | 64 | 86 | keep | Async and promises (2); Callback-shaped APIs (2) |
| 10 | `apps/codex-plus-manager/src/SkillDuplicatesDrawer.tsx` | keep | 60 | 85 | keep | Async and promises (3); Callback-shaped APIs (2) |
| 10 | `apps/codex-plus-manager/src/agent-cache.tsx` | keep | 201 | 76 | keep | Async and promises (5); Insertion-ordered maps and object keys (2) |
| 10 | `apps/codex-plus-manager/src/ProjectSessionPage.tsx` | keep | 155 | 74 | keep | Insertion-ordered maps and object keys (12); Async and promises (8) |
| 10 | `apps/codex-plus-manager/src/GatewayUsage.tsx` | keep | 396 | 73 | keep | Async and promises (9); Insertion-ordered maps and object keys (1) |
| 10 | `apps/codex-plus-manager/src/SkillSourceMatchDrawer.tsx` | keep | 176 | 73 | keep | Insertion-ordered maps and object keys (6); Async and promises (4) |
| 10 | `apps/codex-plus-manager/src/SkillVersionPicker.tsx` | keep | 55 | 73 | keep | Async and promises (13); Insertion-ordered maps and object keys (3) |
| 10 | `apps/codex-plus-manager/src/AgentCollaborationPage.tsx` | keep | 545 | 68 | keep | Async and promises (8); Insertion-ordered maps and object keys (1) |
| 10 | `apps/codex-plus-manager/src/RemoteControlPage.tsx` | keep | 327 | 65 | keep | String offsets and slicing (1); Async and promises (11) |
| 10 | `apps/codex-plus-manager/src/DshPluginsPanel.tsx` | keep | 492 | 63 | keep | Async and promises (20); Callback-shaped APIs (2) |
| 10 | `apps/codex-plus-manager/src/ComputerUsePage.tsx` | keep | 928 | 44 | keep | String offsets and slicing (3); Async and promises (34) |
| 11 | `apps/codex-plus-manager/src/ExtensionCuration.tsx` | keep | 179 | 72 | keep | String offsets and slicing (3); Callback-shaped APIs (3) |
| 11 | `apps/codex-plus-manager/src/CommunityExtensionsPanel.tsx` | keep | 393 | 55 | keep | String offsets and slicing (5); Async and promises (10) |
| 11 | `apps/codex-plus-manager/src/GatewayPage.tsx` | keep | 978 | 49 | keep | String offsets and slicing (1); Async and promises (15) |
| 11 | `apps/codex-plus-manager/src/ManagementExtra.tsx` | keep | 1087 | 4 | keep | String offsets and slicing (1); Async and promises (92) |
| 12 | `apps/codex-plus-manager/src/MyPublishedPanel.tsx` | keep | 206 | 75 | keep | String offsets and slicing (1); Async and promises (5) |
| 12 | `apps/codex-plus-manager/src/CuratedPackages.tsx` | keep | 211 | 69 | keep | String offsets and slicing (5); Async and promises (5) |
| 13 | `apps/codex-plus-manager/src/provider-api-keys.ts` | ready | 77 | 94 | napi-rs | String offsets and slicing (2); Insertion-ordered maps and object keys (2) |
| 13 | `apps/codex-plus-manager/src/relay-profile-editing.ts` | caution | 1416 | 10 | napi-rs | String offsets and slicing (22); Regex features the `regex` crate lacks (2) |
| 13 | `apps/codex-plus-manager/src/App.tsx` | keep | 18015 | 0 | keep | String offsets and slicing (28); Regex features the `regex` crate lacks (2) |
| 14 | `apps/codex-plus-manager/src/main.tsx` | keep | 66 | 92 | keep | Class inheritance (1) |
| 14 | `apps/codex-plus-manager/web-preview/` | keep | 1051 | 40 | keep | String offsets and slicing (5); Async and promises (18) |

### 3. codex-plus-conversation-canvas (`tools/conversation-canvas`, portable, 11 test files)

| Wave | Unit | Status | Lines | Readiness | Bridge | Main risks |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 0 | `tools/conversation-canvas/build-userscript.mjs` | ready | 14 | 100 | napi-rs | - |
| 0 | `tools/conversation-canvas/dev/` | ready | 132 | 100 | napi-rs | - |
| 0 | `tools/conversation-canvas/view-pages.mjs` | ready | 15 | 100 | wasm-bindgen | String offsets and slicing (3) |
| 0 | `tools/conversation-canvas/src/native-api.js` | ready | 13 | 98 | wasm-bindgen | Async and promises (3); Exceptions (4) |
| 0 | `tools/conversation-canvas/src/tree-canvas-view.js` | ready | 58 | 98 | wasm-bindgen | Insertion-ordered maps and object keys (3); JSON round-trips (1) |
| 0 | `tools/conversation-canvas/native-runtime.mjs` | blocked | 42 | 94 | wasm-bindgen | Dynamic code (eval, new Function, computed import) (1); Insertion-ordered maps and object keys (3) |
| 0 | `tools/conversation-canvas/organize.mjs` | ready | 45 | 93 | wasm-bindgen | String offsets and slicing (1); Insertion-ordered maps and object keys (6) |
| 0 | `tools/conversation-canvas/src/canvas-store.js` | ready | 26 | 93 | wasm-bindgen | Async and promises (10); Insertion-ordered maps and object keys (1) |
| 0 | `tools/conversation-canvas/model.mjs` | ready | 57 | 88 | wasm-bindgen | String offsets and slicing (2); Insertion-ordered maps and object keys (8) |
| 0 | `tools/conversation-canvas/src/native-sidechat.js` | ready | 158 | 81 | wasm-bindgen | Async and promises (23); Insertion-ordered maps and object keys (6) |
| 0 | `tools/conversation-canvas/external-api.mjs` | ready | 133 | 76 | wasm-bindgen | String offsets and slicing (4); Regex features the `regex` crate lacks (1) |
| 0 | `tools/conversation-canvas/src/canvas-panel.js` | ready | 324 | 61 | wasm-bindgen | String offsets and slicing (5); Async and promises (31) |
| 0 | `tools/conversation-canvas/public/` | blocked | 1166 | 1 | wasm-bindgen | String offsets and slicing (30); Regex features the `regex` crate lacks (3) |
| 1 | `tools/conversation-canvas/tree-canvas.mjs` | ready | 36 | 100 | wasm-bindgen | Insertion-ordered maps and object keys (4) |
| 1 | `tools/conversation-canvas/tree-markup.mjs` | ready | 9 | 100 | wasm-bindgen | String offsets and slicing (2); Insertion-ordered maps and object keys (1) |
| 1 | `tools/conversation-canvas/bridge-data.mjs` | ready | 18 | 99 | wasm-bindgen | String offsets and slicing (2); Insertion-ordered maps and object keys (1) |
| 1 | `tools/conversation-canvas/native-history.mjs` | ready | 68 | 83 | wasm-bindgen | Regex features the `regex` crate lacks (2); String offsets and slicing (1) |
| 1 | `tools/conversation-canvas/long-organizer.mjs` | ready | 147 | 68 | wasm-bindgen | String offsets and slicing (10); Insertion-ordered maps and object keys (20) |

## npm dependencies

| Package | Units | Rust | Fit | Note |
| --- | ---: | --- | --- | --- |
| `esbuild` | 2 | ? | unknown |  |
| `@babel/traverse` | 1 | ? | unknown |  |
| `clsx` | 1 | ? | unknown |  |
| `electron` | 1 | ? | unknown |  |
| `pngjs` | 1 | ? | unknown |  |
| `supports-color` | 1 | ? | unknown |  |
| `tailwind-merge` | 1 | ? | unknown |  |
| `playwright` | 1 | chromiumoxide | partial | drives Chrome over CDP; the page API differs |
| `typescript` | 1 | oxc | partial | Parsing is covered. Type checking has no Rust equivalent. |
| `@babel/parser` | 1 | oxc | good | oxc-parser is already a binding of the Rust crate. |
| `sharp` | 1 | image | good |  |

## Blocked

- `apps/codex-plus-manager/src/model-picker-views-runtime.manual.ts`: Uses dynamic code (eval, new Function or computed imports).
- `apps/codex-plus-manager/src/runtime-context-capnweb-failure.manual.cjs`: Uses dynamic code (eval, new Function or computed imports).
- `apps/codex-plus-manager/src/skills-batch-react.manual.ts`: Uses dynamic code (eval, new Function or computed imports).
- `assets/dsh-host/`: Uses dynamic code (eval, new Function or computed imports).
- `assets/inject/`: Uses dynamic code (eval, new Function or computed imports).
- `assets/inject/floating-panel/core/`: Uses dynamic code (eval, new Function or computed imports).
- `assets/inject/renderer-inject/`: Uses dynamic code (eval, new Function or computed imports).
- `scripts/`: Uses dynamic code (eval, new Function or computed imports).
- `tools/conversation-canvas/native-runtime.mjs`: Uses dynamic code (eval, new Function or computed imports).
- `tools/conversation-canvas/public/`: Uses dynamic code (eval, new Function or computed imports).

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
- **Callback-shaped APIs** (Medium): Function-typed parameters and methods cross the bridge on every call. Keep boundaries coarse: pass data in, get data out.
- **Number semantics** (Low): JS numbers are f64; bitwise ops truncate to i32; parseInt is lenient. Pick Rust integer types deliberately. Printing differs: `${n}` gives `1e+21`, `1e-7` and `0` for -0 where Rust's `{}` does not, so use `js::number_to_string`. `Math.round` rounds halves up (-2.5 gives -2), Rust's `round` away from zero: use `js::math_round`. `toFixed` rounds exact halves up ((2.5).toFixed(0) is "3"), Rust's `{:.0}` to even.
- **JSON round-trips** (Low): JSON.stringify key order and number formatting must match `serde_json` output if the result is user-visible.
- **Clock and random numbers** (Low): Results that depend on the time or on random numbers differ on every run, so a plain recording cannot pin them down. Record with `record --pin-clock` (TS/JS with the napi bridge, Python): `Date.now()`, `new Date()` and `Math.random` (`time.time`, `datetime.now` and `random`) then return fixed values during record, diff and gate, and the port reads the same ones from `src/port/pinned.rs` (`now_ms()`, `random()`, a documented mulberry32). Or pass the time and the seed in as parameters.

## Whole app

Shape: **server**.

| Entry point | Kind | Found from |
| --- | --- | --- |
| `apps/codex-plus-manager/src/model-picker-views-runtime.manual.ts` | server | `createServer(` at apps/codex-plus-manager/src/model-picker-views-runtime.manual.ts:177 |
| `assets/claude-localization/inject-cn.js` | server | `createServer(` at assets/claude-localization/inject-cn.js:73 |
| `assets/dsh-host/bridge.mjs` | server | `.listen(` at assets/dsh-host/bridge.mjs:830 |
| `assets/extensions/agent-pet/main.js` | server | `.listen(` at assets/extensions/agent-pet/main.js:19 |
| `assets/extensions/image-overlay/main.js` | server | `.listen(` at assets/extensions/image-overlay/main.js:33 |
| `assets/inject/renderer-inject.js` | server | `.listen(` at assets/inject/renderer-inject.js:38188 |
| `assets/inject/renderer-inject/96-builtin-extensions.js` | server | `.listen(` at assets/inject/renderer-inject/96-builtin-extensions.js:172 |
| `assets/inject/renderer-inject/96-paste-fix.js` | server | `.listen(` at assets/inject/renderer-inject/96-paste-fix.js:40 |
| `assets/inject/renderer-inject/96-prompt-args.js` | server | `.listen(` at assets/inject/renderer-inject/96-prompt-args.js:216 |
| `assets/inject/renderer-inject/96-shell-command.js` | server | `.listen(` at assets/inject/renderer-inject/96-shell-command.js:257 |
| `assets/inject/upstream/whale-widget/full-widget-codex.js` | server | `express()` at assets/inject/upstream/whale-widget/full-widget-codex.js:6442 |
| `assets/inject/upstream/whale-widget/full-widget.js` | server | `express()` at assets/inject/upstream/whale-widget/full-widget.js:6432 |
| `crates/codex-plus-core/src/skills_github_proxy_fixtures/tls_redirect_server.js` | server | `createServer(` at crates/codex-plus-core/src/skills_github_proxy_fixtures/tls_redirect_server.js:45 |
| `scripts/assemble-whale-runtime.mjs` | server | `express()` at scripts/assemble-whale-runtime.mjs:74 |
| `scripts/check-native-fast.cjs` | cli | `#!/usr/bin/env node` at scripts/check-native-fast.cjs:1 |
| `scripts/prune-local-builds.mjs` | cli | `#!/usr/bin/env node` at scripts/prune-local-builds.mjs:1 |
| `scripts/relaunch-local.mjs` | cli | `#!/usr/bin/env node` at scripts/relaunch-local.mjs:1 |
| `scripts/release-meta.mjs` | cli | `#!/usr/bin/env node` at scripts/release-meta.mjs:1 |
| `scripts/release.mjs` | cli | `#!/usr/bin/env node` at scripts/release.mjs:1 |
| `scripts/test-skills-repository-browser.mjs` | server | `createServer(` at scripts/test-skills-repository-browser.mjs:23 |
| `site/scripts/build-data.mjs` | cli | `#!/usr/bin/env node` at site/scripts/build-data.mjs:1 |
| `tools/conversation-canvas/dev/embedding-harness.mjs` | server | `createServer(` at tools/conversation-canvas/dev/embedding-harness.mjs:6 |
| `tools/remote-control-p0/mock-responses.mjs` | server | `createServer(` at tools/remote-control-p0/mock-responses.mjs:47 |
| `tools/rice-maid-pet/renderer.js` | server | `.listen(` at tools/rice-maid-pet/renderer.js:261 |

### Every dependency

| Dependency | Declared | Used by | Rust | Fit | Note |
| --- | --- | --- | --- | --- | --- |
| `vite` | yes | 0 | - | none | Dev server or UI framework: keep in JS. |
| `lucide-react` | yes | 55 | ? | unknown |  |
| `@tauri-apps/api` | yes | 47 | ? | unknown |  |
| `@tauri-apps/plugin-dialog` | yes | 6 | ? | unknown |  |
| `@dnd-kit/core` | yes | 3 | ? | unknown |  |
| `@dnd-kit/sortable` | yes | 3 | ? | unknown |  |
| `@dnd-kit/utilities` | yes | 3 | ? | unknown |  |
| `@fontsource/jetbrains-mono` | yes | 2 | ? | unknown |  |
| `esbuild` | no | 2 | ? | unknown |  |
| `react-dom` | yes | 2 | ? | unknown |  |
| `@babel/traverse` | no | 1 | ? | unknown |  |
| `@radix-ui/react-slot` | yes | 1 | ? | unknown |  |
| `class-variance-authority` | yes | 1 | ? | unknown |  |
| `clsx` | yes | 1 | ? | unknown |  |
| `electron` | no | 1 | ? | unknown |  |
| `pngjs` | no | 1 | ? | unknown |  |
| `supports-color` | no | 1 | ? | unknown |  |
| `tailwind-merge` | yes | 1 | ? | unknown |  |
| `@fontsource/inter` | yes | 0 | ? | unknown |  |
| `@tauri-apps/cli` | yes | 0 | ? | unknown |  |
| `react` | yes | 0 | ? | unknown |  |
| `playwright` | no | 1 | chromiumoxide | partial | drives Chrome over CDP; the page API differs |
| `typescript` | yes | 1 | oxc | partial | Parsing is covered. Type checking has no Rust equivalent. |
| `@babel/parser` | no | 1 | oxc | good | oxc-parser is already a binding of the Rust crate. |
| `sharp` | no | 1 | image | good |  |

### Pieces, in port order

- wave 0: `apps/codex-plus-manager/src/agent-cache-model.ts` (60 lines)
- wave 0: `apps/codex-plus-manager/src/agent-collaboration-queue.ts` (48 lines)
- wave 0: `apps/codex-plus-manager/src/agent-focus-model.ts` (73 lines)
- wave 0: `apps/codex-plus-manager/src/agent-mcp-model.ts` (66 lines)
- wave 0: `apps/codex-plus-manager/src/aggregate-members.ts` (106 lines)
- wave 0: `apps/codex-plus-manager/src/aggregate-routes.ts` (70 lines)
- wave 0: `apps/codex-plus-manager/src/auto-compact.ts` (40 lines)
- wave 0: `apps/codex-plus-manager/src/code-highlight.ts` (253 lines)
- wave 0: `apps/codex-plus-manager/src/config-doctor-model.ts` (235 lines)
- wave 0: `apps/codex-plus-manager/src/context-config-text.ts` (248 lines)
- wave 0: `apps/codex-plus-manager/src/dictation-settings.ts` (47 lines)
- wave 0: `apps/codex-plus-manager/src/dream-skin.ts` (333 lines)
- wave 0: `apps/codex-plus-manager/src/enhancement-catalog.ts` (161 lines)
- wave 0: `apps/codex-plus-manager/src/enhancement-navigation.ts` (14 lines)
- wave 0: `apps/codex-plus-manager/src/escape-layer.ts` (52 lines)
- wave 0: `apps/codex-plus-manager/src/extension-config-schema.ts` (71 lines)
- wave 0: `apps/codex-plus-manager/src/extension-manager-bridge.ts` (159 lines)
- wave 0: `apps/codex-plus-manager/src/extension-manager-pages.ts` (119 lines)
- wave 0: `apps/codex-plus-manager/src/flow-provider-colors.ts` (31 lines)
- wave 0: `apps/codex-plus-manager/src/github-repository.ts` (73 lines)
- wave 0: `apps/codex-plus-manager/src/goals-config.ts` (88 lines)
- wave 0: `apps/codex-plus-manager/src/history-recovery-report.ts` (15 lines)
- wave 0: `apps/codex-plus-manager/src/i18n-en.ts` (4760 lines)
- wave 0: `apps/codex-plus-manager/src/launch-status.ts` (28 lines)
- wave 0: `apps/codex-plus-manager/src/legacy-webkit.ts` (92 lines)
- wave 0: `apps/codex-plus-manager/src/lib/` (5 lines)
- wave 0: `apps/codex-plus-manager/src/log-view-model.ts` (185 lines)
- wave 0: `apps/codex-plus-manager/src/mac-titlebar.ts` (21 lines)
- wave 0: `apps/codex-plus-manager/src/management-model.ts` (41 lines)
- wave 0: `apps/codex-plus-manager/src/mcp-presets.ts` (82 lines)
- wave 0: `apps/codex-plus-manager/src/model-groups.ts` (70 lines)
- wave 0: `apps/codex-plus-manager/src/model-picker-views-runtime.manual.ts` (915 lines)
- wave 0: `apps/codex-plus-manager/src/model-routes.ts` (167 lines)
- wave 0: `apps/codex-plus-manager/src/multi-launch-model.ts` (85 lines)
- wave 0: `apps/codex-plus-manager/src/native-auth-model.ts` (28 lines)
- wave 0: `apps/codex-plus-manager/src/notice-model.ts` (17 lines)
- wave 0: `apps/codex-plus-manager/src/onboarding-model.ts` (119 lines)
- wave 0: `apps/codex-plus-manager/src/overview-model.ts` (53 lines)
- wave 0: `apps/codex-plus-manager/src/page-groups.ts` (26 lines)
- wave 0: `apps/codex-plus-manager/src/plugin-market-model.ts` (121 lines)
- wave 0: `apps/codex-plus-manager/src/popover-layer.ts` (180 lines)
- wave 0: `apps/codex-plus-manager/src/presets.ts` (273 lines)
- wave 0: `apps/codex-plus-manager/src/provider-brand-icons.ts` (64 lines)
- wave 0: `apps/codex-plus-manager/src/provider-brands.ts` (69 lines)
- wave 0: `apps/codex-plus-manager/src/provider-name.ts` (60 lines)
- wave 0: `apps/codex-plus-manager/src/provider-sync-flow.ts` (63 lines)
- wave 0: `apps/codex-plus-manager/src/provider-sync-model.ts` (68 lines)
- wave 0: `apps/codex-plus-manager/src/provider-sync-target.ts` (21 lines)
- wave 0: `apps/codex-plus-manager/src/provider-test.ts` (76 lines)
- wave 0: `apps/codex-plus-manager/src/relay-billing.ts` (223 lines)
- wave 0: `apps/codex-plus-manager/src/relay-headers.ts` (102 lines)
- wave 0: `apps/codex-plus-manager/src/relay-live-files.ts` (22 lines)
- wave 0: `apps/codex-plus-manager/src/relay-session.ts` (17 lines)
- wave 0: `apps/codex-plus-manager/src/runtime-context-capnweb-failure.manual.cjs` (434 lines)
- wave 0: `apps/codex-plus-manager/src/script-market-model.ts` (103 lines)
- wave 0: `apps/codex-plus-manager/src/settings-events.ts` (8 lines)
- wave 0: `apps/codex-plus-manager/src/settings-rebase.ts` (67 lines)
- wave 0: `apps/codex-plus-manager/src/skill-agent-batch.ts` (142 lines)
- wave 0: `apps/codex-plus-manager/src/skill-groups.ts` (277 lines)
- wave 0: `apps/codex-plus-manager/src/skills-batch-react.manual.ts` (420 lines)
- wave 0: `apps/codex-plus-manager/src/subscription-policy.ts` (45 lines)
- wave 0: `apps/codex-plus-manager/src/theme-mode.ts` (56 lines)
- wave 0: `apps/codex-plus-manager/src/update-notification.ts` (109 lines)
- wave 0: `apps/codex-plus-manager/src/vlm-test-translation.ts` (37 lines)
- wave 0: `apps/codex-plus-manager/src/whale-settings.ts` (45 lines)
- wave 0: `assets/claude-extensions/` (165 lines)
- wave 0: `assets/claude-localization/` (1607 lines)
- wave 0: `assets/cu/` (32 lines)
- wave 0: `assets/dsh-host/shims/` (2148 lines)
- wave 0: `assets/extension-creator/` (167 lines)
- wave 0: `assets/extensions/agent-pet/` (1772 lines)
- wave 0: `assets/extensions/image-overlay/` (1674 lines)
- wave 0: `assets/extensions/proxy-request-status/` (314 lines)
- wave 0: `assets/extensions/typing-effects/` (413 lines)
- wave 0: `assets/extensions/usage-statusline/` (1220 lines)
- wave 0: `assets/inject/` (41928 lines)
- wave 0: `assets/inject/floating-panel/core/` (5200 lines)
- wave 0: `assets/inject/floating-panel/outline/` (645 lines)
- wave 0: `assets/inject/floating-panel/runtime/` (1752 lines)
- wave 0: `assets/inject/floating-panel/stepwise/` (536 lines)
- wave 0: `assets/inject/renderer-inject/` (38943 lines)
- wave 0: `assets/inject/upstream/cidala-tiger/macos/` (879 lines)
- wave 0: `assets/inject/upstream/cidala-tiger/windows/` (436 lines)
- wave 0: `assets/inject/upstream/dream-skin/macos/` (1132 lines)
- wave 0: `assets/inject/upstream/dream-skin/windows/` (1152 lines)
- wave 0: `assets/inject/upstream/glass-vision/` (111 lines)
- wave 0: `assets/inject/upstream/snow-skin/` (126 lines)
- wave 0: `assets/inject/upstream/whale-widget/` (34519 lines)
- wave 0: `assets/native-browser/` (705 lines)
- wave 0: `crates/codex-plus-core/src/skills_github_proxy_fixtures/` (114 lines)
- wave 0: `docs/` (850 lines)
- wave 0: `scripts/` (5357 lines)
- wave 0: `services/share-site/` (346 lines)
- wave 0: `site/js/` (266 lines)
- wave 0: `site/manager-preview/assets/` (797 lines)
- wave 0: `site/scripts/` (88 lines)
- wave 0: `tools/` (299 lines)
- wave 0: `tools/claude-extension-demo/` (21 lines)
- wave 0: `tools/conversation-canvas/build-userscript.mjs` (14 lines)
- wave 0: `tools/conversation-canvas/dev/` (132 lines)
- wave 0: `tools/conversation-canvas/external-api.mjs` (133 lines)
- wave 0: `tools/conversation-canvas/model.mjs` (57 lines)
- wave 0: `tools/conversation-canvas/native-runtime.mjs` (42 lines)
- wave 0: `tools/conversation-canvas/organize.mjs` (45 lines)
- wave 0: `tools/conversation-canvas/public/` (1166 lines)
- wave 0: `tools/conversation-canvas/src/canvas-panel.js` (324 lines)
- wave 0: `tools/conversation-canvas/src/canvas-store.js` (26 lines)
- wave 0: `tools/conversation-canvas/src/native-api.js` (13 lines)
- wave 0: `tools/conversation-canvas/src/native-sidechat.js` (158 lines)
- wave 0: `tools/conversation-canvas/src/tree-canvas-view.js` (58 lines)
- wave 0: `tools/conversation-canvas/view-pages.mjs` (15 lines)
- wave 0: `tools/remote-control-p0/` (1644 lines)
- wave 0: `tools/rice-maid-pet/` (357 lines)
- wave 0: `tools/rice-maid-pet/backend/` (18 lines)
- wave 0: `tools/rice-maid-pet/hooks/` (8 lines)
- wave 1: `apps/codex-plus-manager/src/extension-host-model.ts` (27 lines)
- wave 1: `apps/codex-plus-manager/src/i18n.ts` (66 lines)
- wave 1: `apps/codex-plus-manager/src/model-metadata.ts` (929 lines)
- wave 1: `apps/codex-plus-manager/src/skill-repository.ts` (18 lines)
- wave 1: `assets/dsh-host/` (6902 lines)
- wave 1: `tools/conversation-canvas/bridge-data.mjs` (18 lines)
- wave 1: `tools/conversation-canvas/long-organizer.mjs` (147 lines)
- wave 1: `tools/conversation-canvas/native-history.mjs` (68 lines)
- wave 1: `tools/conversation-canvas/tree-canvas.mjs` (36 lines)
- wave 1: `tools/conversation-canvas/tree-markup.mjs` (9 lines)
- wave 2: `apps/codex-plus-manager/src/gateway-model.ts` (1679 lines)
- wave 2: `apps/codex-plus-manager/src/model-reasoning.ts` (149 lines)
- wave 2: `apps/codex-plus-manager/src/model-windows.ts` (258 lines)
- wave 2: `apps/codex-plus-manager/src/skill-source-scan.ts` (205 lines)
- wave 3: `apps/codex-plus-manager/src/agent-dashboard-model.ts` (59 lines)
- wave 3: `apps/codex-plus-manager/src/gateway-agent-safety.ts` (106 lines)
- wave 3: `apps/codex-plus-manager/src/gateway-auto-compact-model.ts` (25 lines)
- wave 3: `apps/codex-plus-manager/src/onboarding-setup-model.ts` (191 lines)
- wave 3: `apps/codex-plus-manager/src/provider-balances.ts` (64 lines)
- wave 3: `apps/codex-plus-manager/src/provider-form-model.ts` (88 lines)
- wave 3: `apps/codex-plus-manager/src/provider-meta-sync-model.ts` (148 lines)
- wave 3: `apps/codex-plus-manager/src/provider-pool-layout.ts` (19 lines)
- wave 3: `apps/codex-plus-manager/src/request-flow-model.ts` (134 lines)
- wave 3: `apps/codex-plus-manager/src/tray-panel-model.ts` (205 lines)
- wave 4: `apps/codex-plus-manager/src/hub-overview-model.ts` (160 lines)
- wave 13: `apps/codex-plus-manager/src/provider-api-keys.ts` (77 lines)
- wave 13: `apps/codex-plus-manager/src/relay-profile-editing.ts` (1416 lines)

### Decisions for the user

**D1** (shape): The app is a server, entered through apps/codex-plus-manager/src/model-picker-views-runtime.manual.ts, assets/claude-localization/inject-cn.js, assets/dsh-host/bridge.mjs, assets/extensions/agent-pet/main.js, assets/extensions/image-overlay/main.js, assets/inject/renderer-inject.js, assets/inject/renderer-inject/96-builtin-extensions.js, assets/inject/renderer-inject/96-paste-fix.js, assets/inject/renderer-inject/96-prompt-args.js, assets/inject/renderer-inject/96-shell-command.js, assets/inject/upstream/whale-widget/full-widget-codex.js, assets/inject/upstream/whale-widget/full-widget.js, crates/codex-plus-core/src/skills_github_proxy_fixtures/tls_redirect_server.js, scripts/assemble-whale-runtime.mjs, scripts/check-native-fast.cjs, scripts/prune-local-builds.mjs, scripts/relaunch-local.mjs, scripts/release-meta.mjs, scripts/release.mjs, scripts/test-skills-repository-browser.mjs, site/scripts/build-data.mjs, tools/conversation-canvas/dev/embedding-harness.mjs, tools/remote-control-p0/mock-responses.mjs, tools/rice-maid-pet/renderer.js. Port every entry point, in that shape?

> Yes: an axum front door (`app front-door`) takes the server over first

**D2** (keep apps/codex-plus-manager/src/AgentCollaborationPage.tsx): apps/codex-plus-manager/src/AgentCollaborationPage.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D3** (keep apps/codex-plus-manager/src/AgentDashboard.tsx): apps/codex-plus-manager/src/AgentDashboard.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D4** (keep apps/codex-plus-manager/src/AgentDisconnectDialog.tsx): apps/codex-plus-manager/src/AgentDisconnectDialog.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D5** (keep apps/codex-plus-manager/src/AgentFocusDialog.tsx): apps/codex-plus-manager/src/AgentFocusDialog.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D6** (keep apps/codex-plus-manager/src/App.tsx): apps/codex-plus-manager/src/App.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D7** (keep apps/codex-plus-manager/src/AppSelect.tsx): apps/codex-plus-manager/src/AppSelect.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D8** (keep apps/codex-plus-manager/src/CodexLaunchButton.tsx): apps/codex-plus-manager/src/CodexLaunchButton.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D9** (keep apps/codex-plus-manager/src/CommandPalette.tsx): apps/codex-plus-manager/src/CommandPalette.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D10** (keep apps/codex-plus-manager/src/CommunityExtensionsPanel.tsx): apps/codex-plus-manager/src/CommunityExtensionsPanel.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D11** (keep apps/codex-plus-manager/src/ComputerUsePage.tsx): apps/codex-plus-manager/src/ComputerUsePage.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D12** (keep apps/codex-plus-manager/src/ComputerUsePreview.tsx): apps/codex-plus-manager/src/ComputerUsePreview.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D13** (keep apps/codex-plus-manager/src/ConfigDoctorPanel.tsx): apps/codex-plus-manager/src/ConfigDoctorPanel.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D14** (keep apps/codex-plus-manager/src/CuPipWindow.tsx): apps/codex-plus-manager/src/CuPipWindow.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D15** (keep apps/codex-plus-manager/src/CuratedPackages.tsx): apps/codex-plus-manager/src/CuratedPackages.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D16** (keep apps/codex-plus-manager/src/DshPluginsPanel.tsx): apps/codex-plus-manager/src/DshPluginsPanel.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D17** (keep apps/codex-plus-manager/src/ExtensionCard.tsx): apps/codex-plus-manager/src/ExtensionCard.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D18** (keep apps/codex-plus-manager/src/ExtensionCuration.tsx): apps/codex-plus-manager/src/ExtensionCuration.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D19** (keep apps/codex-plus-manager/src/ExtensionManagerPanel.tsx): apps/codex-plus-manager/src/ExtensionManagerPanel.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D20** (keep apps/codex-plus-manager/src/ExtensionPublishDialog.tsx): apps/codex-plus-manager/src/ExtensionPublishDialog.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D21** (keep apps/codex-plus-manager/src/GatewayAgents.tsx): apps/codex-plus-manager/src/GatewayAgents.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D22** (keep apps/codex-plus-manager/src/GatewayModelMatrix.tsx): apps/codex-plus-manager/src/GatewayModelMatrix.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D23** (keep apps/codex-plus-manager/src/GatewayPage.tsx): apps/codex-plus-manager/src/GatewayPage.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D24** (keep apps/codex-plus-manager/src/GatewayProviders.tsx): apps/codex-plus-manager/src/GatewayProviders.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D25** (keep apps/codex-plus-manager/src/GatewaySessions.tsx): apps/codex-plus-manager/src/GatewaySessions.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D26** (keep apps/codex-plus-manager/src/GatewayUsage.tsx): apps/codex-plus-manager/src/GatewayUsage.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D27** (keep apps/codex-plus-manager/src/HubOverview.tsx): apps/codex-plus-manager/src/HubOverview.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D28** (keep apps/codex-plus-manager/src/ManagementExtra.tsx): apps/codex-plus-manager/src/ManagementExtra.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D29** (keep apps/codex-plus-manager/src/ManagementPages.tsx): apps/codex-plus-manager/src/ManagementPages.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D30** (keep apps/codex-plus-manager/src/MyPublishedPanel.tsx): apps/codex-plus-manager/src/MyPublishedPanel.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D31** (keep apps/codex-plus-manager/src/NativeAuthControls.tsx): apps/codex-plus-manager/src/NativeAuthControls.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D32** (keep apps/codex-plus-manager/src/OnboardingSetup.tsx): apps/codex-plus-manager/src/OnboardingSetup.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D33** (keep apps/codex-plus-manager/src/OnboardingTour.tsx): apps/codex-plus-manager/src/OnboardingTour.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D34** (keep apps/codex-plus-manager/src/PluginMarketScreen.tsx): apps/codex-plus-manager/src/PluginMarketScreen.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D35** (keep apps/codex-plus-manager/src/ProjectSessionPage.tsx): apps/codex-plus-manager/src/ProjectSessionPage.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D36** (keep apps/codex-plus-manager/src/ProviderExtrasDialog.tsx): apps/codex-plus-manager/src/ProviderExtrasDialog.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D37** (keep apps/codex-plus-manager/src/ProviderFormDialog.tsx): apps/codex-plus-manager/src/ProviderFormDialog.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D38** (keep apps/codex-plus-manager/src/ProviderKeyList.tsx): apps/codex-plus-manager/src/ProviderKeyList.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D39** (keep apps/codex-plus-manager/src/ProviderMetaSyncDialog.tsx): apps/codex-plus-manager/src/ProviderMetaSyncDialog.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D40** (keep apps/codex-plus-manager/src/ProviderModelsDialog.tsx): apps/codex-plus-manager/src/ProviderModelsDialog.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D41** (keep apps/codex-plus-manager/src/ProviderReasoningDialog.tsx): apps/codex-plus-manager/src/ProviderReasoningDialog.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D42** (keep apps/codex-plus-manager/src/ProviderSyncDialog.tsx): apps/codex-plus-manager/src/ProviderSyncDialog.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D43** (keep apps/codex-plus-manager/src/ProviderTestDialog.tsx): apps/codex-plus-manager/src/ProviderTestDialog.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D44** (keep apps/codex-plus-manager/src/ProviderWindowsDialog.tsx): apps/codex-plus-manager/src/ProviderWindowsDialog.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D45** (keep apps/codex-plus-manager/src/RelayBillingLine.tsx): apps/codex-plus-manager/src/RelayBillingLine.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D46** (keep apps/codex-plus-manager/src/RelayHeaderRows.tsx): apps/codex-plus-manager/src/RelayHeaderRows.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D47** (keep apps/codex-plus-manager/src/ReleaseNotes.ts): apps/codex-plus-manager/src/ReleaseNotes.ts stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D48** (keep apps/codex-plus-manager/src/RemoteControlPage.tsx): apps/codex-plus-manager/src/RemoteControlPage.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D49** (keep apps/codex-plus-manager/src/RequestFlow.tsx): apps/codex-plus-manager/src/RequestFlow.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D50** (keep apps/codex-plus-manager/src/SessionAgentTabs.tsx): apps/codex-plus-manager/src/SessionAgentTabs.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D51** (keep apps/codex-plus-manager/src/SkillBackupUsage.tsx): apps/codex-plus-manager/src/SkillBackupUsage.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D52** (keep apps/codex-plus-manager/src/SkillDuplicatesDrawer.tsx): apps/codex-plus-manager/src/SkillDuplicatesDrawer.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D53** (keep apps/codex-plus-manager/src/SkillFileTimes.tsx): apps/codex-plus-manager/src/SkillFileTimes.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D54** (keep apps/codex-plus-manager/src/SkillInPlaceUpdateDrawer.tsx): apps/codex-plus-manager/src/SkillInPlaceUpdateDrawer.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D55** (keep apps/codex-plus-manager/src/SkillListViewport.tsx): apps/codex-plus-manager/src/SkillListViewport.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D56** (keep apps/codex-plus-manager/src/SkillSourceMatchDrawer.tsx): apps/codex-plus-manager/src/SkillSourceMatchDrawer.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D57** (keep apps/codex-plus-manager/src/SkillVersionPicker.tsx): apps/codex-plus-manager/src/SkillVersionPicker.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D58** (keep apps/codex-plus-manager/src/SubscriptionPolicyFields.tsx): apps/codex-plus-manager/src/SubscriptionPolicyFields.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D59** (keep apps/codex-plus-manager/src/TopbarActions.tsx): apps/codex-plus-manager/src/TopbarActions.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D60** (keep apps/codex-plus-manager/src/TrayPanel.tsx): apps/codex-plus-manager/src/TrayPanel.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D61** (keep apps/codex-plus-manager/src/UsageTabs.tsx): apps/codex-plus-manager/src/UsageTabs.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D62** (keep apps/codex-plus-manager/src/agent-cache.tsx): apps/codex-plus-manager/src/agent-cache.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D63** (keep apps/codex-plus-manager/src/agent-focus-store.ts): apps/codex-plus-manager/src/agent-focus-store.ts stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D64** (keep apps/codex-plus-manager/src/app-dialog.tsx): apps/codex-plus-manager/src/app-dialog.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D65** (keep apps/codex-plus-manager/src/command-palette-model.ts): apps/codex-plus-manager/src/command-palette-model.ts stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D66** (keep apps/codex-plus-manager/src/components/): apps/codex-plus-manager/src/components/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D67** (keep apps/codex-plus-manager/src/components/ui/): apps/codex-plus-manager/src/components/ui/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D68** (keep apps/codex-plus-manager/src/cx-layout.tsx): apps/codex-plus-manager/src/cx-layout.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D69** (keep apps/codex-plus-manager/src/extension-config-form.tsx): apps/codex-plus-manager/src/extension-config-form.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D70** (keep apps/codex-plus-manager/src/gateway-store.ts): apps/codex-plus-manager/src/gateway-store.ts stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D71** (keep apps/codex-plus-manager/src/gateway-ui.tsx): apps/codex-plus-manager/src/gateway-ui.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D72** (keep apps/codex-plus-manager/src/main.tsx): apps/codex-plus-manager/src/main.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D73** (keep apps/codex-plus-manager/src/use-escape-layer.ts): apps/codex-plus-manager/src/use-escape-layer.ts stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D74** (keep apps/codex-plus-manager/src/use-modal-focus.ts): apps/codex-plus-manager/src/use-modal-focus.ts stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D75** (keep apps/codex-plus-manager/src/use-popover-layer.ts): apps/codex-plus-manager/src/use-popover-layer.ts stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D76** (keep apps/codex-plus-manager/web-preview/): apps/codex-plus-manager/web-preview/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D77** (blocked apps/codex-plus-manager/src/model-picker-views-runtime.manual.ts): apps/codex-plus-manager/src/model-picker-views-runtime.manual.ts is blocked: Uses dynamic code (eval, new Function or computed imports). Rework that part in Rust, or keep the unit outside?

> Research the blocker first; then port it with the piece that uses it

**D78** (blocked apps/codex-plus-manager/src/runtime-context-capnweb-failure.manual.cjs): apps/codex-plus-manager/src/runtime-context-capnweb-failure.manual.cjs is blocked: Uses dynamic code (eval, new Function or computed imports). Rework that part in Rust, or keep the unit outside?

> Research the blocker first; then port it with the piece that uses it

**D79** (blocked apps/codex-plus-manager/src/skills-batch-react.manual.ts): apps/codex-plus-manager/src/skills-batch-react.manual.ts is blocked: Uses dynamic code (eval, new Function or computed imports). Rework that part in Rust, or keep the unit outside?

> Research the blocker first; then port it with the piece that uses it

**D80** (blocked assets/dsh-host/): assets/dsh-host/ is blocked: Uses dynamic code (eval, new Function or computed imports). Rework that part in Rust, or keep the unit outside?

> Research the blocker first; then port it with the piece that uses it

**D81** (blocked assets/inject/): assets/inject/ is blocked: Uses dynamic code (eval, new Function or computed imports). Rework that part in Rust, or keep the unit outside?

> Research the blocker first; then port it with the piece that uses it

**D82** (blocked assets/inject/floating-panel/core/): assets/inject/floating-panel/core/ is blocked: Uses dynamic code (eval, new Function or computed imports). Rework that part in Rust, or keep the unit outside?

> Research the blocker first; then port it with the piece that uses it

**D83** (blocked assets/inject/renderer-inject/): assets/inject/renderer-inject/ is blocked: Uses dynamic code (eval, new Function or computed imports). Rework that part in Rust, or keep the unit outside?

> Research the blocker first; then port it with the piece that uses it

**D84** (blocked scripts/): scripts/ is blocked: Uses dynamic code (eval, new Function or computed imports). Rework that part in Rust, or keep the unit outside?

> Research the blocker first; then port it with the piece that uses it

**D85** (blocked tools/conversation-canvas/native-runtime.mjs): tools/conversation-canvas/native-runtime.mjs is blocked: Uses dynamic code (eval, new Function or computed imports). Rework that part in Rust, or keep the unit outside?

> Research the blocker first; then port it with the piece that uses it

**D86** (blocked tools/conversation-canvas/public/): tools/conversation-canvas/public/ is blocked: Uses dynamic code (eval, new Function or computed imports). Rework that part in Rust, or keep the unit outside?

> Research the blocker first; then port it with the piece that uses it

**D87** (dependency vite): No crate does what vite does (declared, no unit imports it; Dev server or UI framework: keep in JS). Reimplement what the app uses, or drop the feature?

> Reimplement only the part the app uses, proved by its recorded cases

**D88** (dependency lucide-react): Which crate replaces lucide-react (used by 55 units)?

> Research it against primary sources into rust/research/lucide-react.md before porting what uses it

**D89** (dependency @tauri-apps/api): Which crate replaces @tauri-apps/api (used by 47 units)?

> Research it against primary sources into rust/research/@tauri-apps-api.md before porting what uses it

**D90** (dependency @tauri-apps/plugin-dialog): Which crate replaces @tauri-apps/plugin-dialog (used by 6 units)?

> Research it against primary sources into rust/research/@tauri-apps-plugin-dialog.md before porting what uses it

**D91** (dependency @dnd-kit/core): Which crate replaces @dnd-kit/core (used by 3 units)?

> Research it against primary sources into rust/research/@dnd-kit-core.md before porting what uses it

**D92** (dependency @dnd-kit/sortable): Which crate replaces @dnd-kit/sortable (used by 3 units)?

> Research it against primary sources into rust/research/@dnd-kit-sortable.md before porting what uses it

**D93** (dependency @dnd-kit/utilities): Which crate replaces @dnd-kit/utilities (used by 3 units)?

> Research it against primary sources into rust/research/@dnd-kit-utilities.md before porting what uses it

**D94** (dependency @fontsource/jetbrains-mono): Which crate replaces @fontsource/jetbrains-mono (used by 2 units)?

> Research it against primary sources into rust/research/@fontsource-jetbrains-mono.md before porting what uses it

**D95** (dependency esbuild): Which crate replaces esbuild (used by 2 units)?

> Research it against primary sources into rust/research/esbuild.md before porting what uses it

**D96** (dependency react-dom): Which crate replaces react-dom (used by 2 units)?

> Research it against primary sources into rust/research/react-dom.md before porting what uses it

**D97** (dependency @babel/traverse): Which crate replaces @babel/traverse (used by 1 unit)?

> Research it against primary sources into rust/research/@babel-traverse.md before porting what uses it

**D98** (dependency @radix-ui/react-slot): Which crate replaces @radix-ui/react-slot (used by 1 unit)?

> Research it against primary sources into rust/research/@radix-ui-react-slot.md before porting what uses it

**D99** (dependency class-variance-authority): Which crate replaces class-variance-authority (used by 1 unit)?

> Research it against primary sources into rust/research/class-variance-authority.md before porting what uses it

**D100** (dependency clsx): Which crate replaces clsx (used by 1 unit)?

> Research it against primary sources into rust/research/clsx.md before porting what uses it

**D101** (dependency electron): Which crate replaces electron (used by 1 unit)?

> Research it against primary sources into rust/research/electron.md before porting what uses it

**D102** (dependency pngjs): Which crate replaces pngjs (used by 1 unit)?

> Research it against primary sources into rust/research/pngjs.md before porting what uses it

**D103** (dependency supports-color): Which crate replaces supports-color (used by 1 unit)?

> Research it against primary sources into rust/research/supports-color.md before porting what uses it

**D104** (dependency tailwind-merge): Which crate replaces tailwind-merge (used by 1 unit)?

> Research it against primary sources into rust/research/tailwind-merge.md before porting what uses it

**D105** (dependency @fontsource/inter): Which crate replaces @fontsource/inter (declared, no unit imports it)?

> Research it against primary sources into rust/research/@fontsource-inter.md before porting what uses it

**D106** (dependency @tauri-apps/cli): Which crate replaces @tauri-apps/cli (declared, no unit imports it)?

> Research it against primary sources into rust/research/@tauri-apps-cli.md before porting what uses it

**D107** (dependency react): Which crate replaces react (declared, no unit imports it)?

> Research it against primary sources into rust/research/react.md before porting what uses it

**D108** (dependency playwright): playwright maps to chromiumoxide only in part (used by 1 unit; drives Chrome over CDP; the page API differs). Use it?

> Use chromiumoxide, and record cases for the behavior it does not cover

**D109** (dependency typescript): typescript maps to oxc only in part (used by 1 unit; Parsing is covered. Type checking has no Rust equivalent). Use it?

> Use oxc, and record cases for the behavior it does not cover

