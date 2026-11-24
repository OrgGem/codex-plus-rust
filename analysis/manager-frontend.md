# Rust migration plan: codex-plus-manager (TypeScript/JavaScript)

1 package, 96 source files (55773 lines), 236 test files, 74 UI files, grouped into 161 units.

| Ready | Caution | Blocked | Keep in source language |
| ---: | ---: | ---: | ---: |
| 76 | 7 | 3 | 75 |

## Start here

Best first ports: safe to port (readiness) and worth porting (loops and string work, tests, size).

| # | Unit | Lines | Priority | Readiness | Tests | Bridge | Main risks |
| ---: | --- | ---: | ---: | ---: | ---: | --- | --- |
| 1 | `src/code-highlight.ts` | 253 | 87 | 85 | 1 | napi-rs | String offsets and slicing (21) |
| 2 | `src/management-model.ts` | 41 | 86 | 92 | 2 | napi-rs | String offsets and slicing (3); Insertion-ordered maps and object keys (1); Clock and random numbers (1) |
| 3 | `src/goals-config.ts` | 88 | 82 | 100 | 1 | napi-rs | String offsets and slicing (1) |
| 4 | `src/provider-name.ts` | 60 | 82 | 92 | 2 | napi-rs | String offsets and slicing (3); Regexes built at runtime (1) |
| 5 | `src/provider-api-keys.ts` | 77 | 80 | 94 | 3 | napi-rs | String offsets and slicing (2); Insertion-ordered maps and object keys (2); Clock and random numbers (1) |
| 6 | `src/request-flow-model.ts` | 134 | 80 | 86 | 1 | napi-rs | String offsets and slicing (3); Insertion-ordered maps and object keys (5); JSON round-trips (1) |
| 7 | `src/overview-model.ts` | 53 | 79 | 100 | 1 | napi-rs | Insertion-ordered maps and object keys (1); Number semantics (1) |
| 8 | `src/provider-sync-model.ts` | 68 | 79 | 100 | 2 | napi-rs | Callback-shaped APIs (1) |
| 9 | `src/hub-overview-model.ts` | 160 | 79 | 98 | 2 | napi-rs | Insertion-ordered maps and object keys (1) |
| 10 | `src/agent-mcp-model.ts` | 66 | 79 | 94 | 1 | napi-rs | String offsets and slicing (1); Regex features the `regex` crate lacks (1); Insertion-ordered maps and object keys (2) |

### `src/code-highlight.ts`

253 lines in 1 file, wave 0, priority 87, readiness 85, bridge **napi-rs**.

- Leaf: imports no other unit.
- 1 test file imports it directly.
- String offsets and slicing x21: `src/code-highlight.ts:82`, `src/code-highlight.ts:90`, `src/code-highlight.ts:92`

### `src/management-model.ts`

41 lines in 1 file, wave 0, priority 86, readiness 92, bridge **napi-rs**.

- Leaf: imports no other unit.
- 2 test files import it directly.
- String offsets and slicing x3: `src/management-model.ts:19`, `src/management-model.ts:20`, `src/management-model.ts:22`
- Insertion-ordered maps and object keys x1: `src/management-model.ts:29`
- Clock and random numbers x1: `src/management-model.ts:24`

### `src/goals-config.ts`

88 lines in 1 file, wave 0, priority 82, readiness 100, bridge **napi-rs**.

- Leaf: imports no other unit.
- 1 test file imports it directly.
- String offsets and slicing x1: `src/goals-config.ts:18`

### `src/provider-name.ts`

60 lines in 1 file, wave 0, priority 82, readiness 92, bridge **napi-rs**.

- Leaf: imports no other unit.
- 2 test files import it directly.
- String offsets and slicing x3: `src/provider-name.ts:11`, `src/provider-name.ts:17`, `src/provider-name.ts:59`
- Regexes built at runtime x1: `src/provider-name.ts:9`

### `src/provider-api-keys.ts`

77 lines in 1 file, wave 13, priority 80, readiness 94, bridge **napi-rs**.

- Import cycle with src/relay-profile-editing.ts, src/App.tsx: port them together.
- 3 test files import it directly.
- String offsets and slicing x2: `src/provider-api-keys.ts:86`, `src/provider-api-keys.ts:86`
- Insertion-ordered maps and object keys x2: `src/provider-api-keys.ts:21`, `src/provider-api-keys.ts:71`
- Clock and random numbers x1: `src/provider-api-keys.ts:73`

## Order by package

Packages come after the workspace packages they use. Inside a package, port by wave: a unit's imports are in earlier waves.

### 1. codex-plus-manager (`.`, node, 236 test files)

| Wave | Unit | Status | Lines | Readiness | Bridge | Main risks |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 0 | `src/agent-collaboration-queue.ts` | ready | 48 | 100 | napi-rs | - |
| 0 | `src/aggregate-members.ts` | ready | 106 | 100 | napi-rs | Insertion-ordered maps and object keys (1); Exceptions (1) |
| 0 | `src/aggregate-routes.ts` | ready | 70 | 100 | napi-rs | Dynamic typing (2); Number semantics (1) |
| 0 | `src/auto-compact.ts` | ready | 40 | 100 | napi-rs | - |
| 0 | `src/dictation-settings.ts` | ready | 47 | 100 | napi-rs | Number semantics (1) |
| 0 | `src/enhancement-navigation.ts` | ready | 14 | 100 | napi-rs | - |
| 0 | `src/extension-config-schema.ts` | ready | 71 | 100 | napi-rs | Insertion-ordered maps and object keys (1) |
| 0 | `src/extension-manager-pages.ts` | ready | 119 | 100 | napi-rs | Callback-shaped APIs (1); Exceptions (1) |
| 0 | `src/goals-config.ts` | ready | 88 | 100 | napi-rs | String offsets and slicing (1) |
| 0 | `src/history-recovery-report.ts` | ready | 15 | 100 | napi-rs | - |
| 0 | `src/launch-status.ts` | ready | 28 | 100 | napi-rs | Insertion-ordered maps and object keys (2) |
| 0 | `src/mac-titlebar.ts` | ready | 21 | 100 | napi-rs | Dynamic typing (1); Exceptions (1) |
| 0 | `src/mcp-presets.ts` | caution | 82 | 100 | napi-rs | Callback-shaped APIs (1); JSON round-trips (2) |
| 0 | `src/model-groups.ts` | ready | 70 | 100 | napi-rs | Regexes built at runtime (1); Insertion-ordered maps and object keys (2) |
| 0 | `src/multi-launch-model.ts` | ready | 85 | 100 | napi-rs | - |
| 0 | `src/native-auth-model.ts` | ready | 28 | 100 | napi-rs | - |
| 0 | `src/notice-model.ts` | ready | 17 | 100 | napi-rs | Insertion-ordered maps and object keys (3) |
| 0 | `src/onboarding-model.ts` | ready | 119 | 100 | napi-rs | Insertion-ordered maps and object keys (1) |
| 0 | `src/overview-model.ts` | ready | 53 | 100 | napi-rs | Insertion-ordered maps and object keys (1); Number semantics (1) |
| 0 | `src/page-groups.ts` | ready | 26 | 100 | napi-rs | Callback-shaped APIs (2) |
| 0 | `src/plugin-market-model.ts` | ready | 121 | 100 | napi-rs | String offsets and slicing (1) |
| 0 | `src/provider-brand-icons.ts` | ready | 64 | 100 | napi-rs | - |
| 0 | `src/provider-brands.ts` | ready | 69 | 100 | napi-rs | Insertion-ordered maps and object keys (1) |
| 0 | `src/provider-sync-flow.ts` | ready | 63 | 100 | napi-rs | - |
| 0 | `src/provider-sync-model.ts` | ready | 68 | 100 | napi-rs | Callback-shaped APIs (1) |
| 0 | `src/provider-sync-target.ts` | ready | 21 | 100 | napi-rs | - |
| 0 | `src/provider-test.ts` | ready | 76 | 100 | napi-rs | Number semantics (2) |
| 0 | `src/relay-live-files.ts` | ready | 22 | 100 | napi-rs | - |
| 0 | `src/relay-session.ts` | ready | 17 | 100 | napi-rs | - |
| 0 | `src/settings-rebase.ts` | caution | 67 | 100 | napi-rs | Insertion-ordered maps and object keys (3); JSON round-trips (2) |
| 0 | `src/subscription-policy.ts` | ready | 45 | 100 | napi-rs | - |
| 0 | `src/theme-mode.ts` | ready | 56 | 100 | napi-rs | Callback-shaped APIs (2); Exceptions (4) |
| 0 | `src/vlm-test-translation.ts` | caution | 37 | 100 | napi-rs | Callback-shaped APIs (1); Number semantics (1) |
| 0 | `src/whale-settings.ts` | ready | 45 | 100 | napi-rs | - |
| 0 | `src/agent-cache-model.ts` | ready | 60 | 99 | napi-rs | String offsets and slicing (1); Insertion-ordered maps and object keys (3) |
| 0 | `src/command-palette-model.ts` | keep | 30 | 99 | keep | Regex features the `regex` crate lacks (1); Callback-shaped APIs (1) |
| 0 | `src/flow-provider-colors.ts` | ready | 31 | 99 | napi-rs | Insertion-ordered maps and object keys (2); Exceptions (2) |
| 0 | `src/presets.ts` | ready | 273 | 99 | napi-rs | - |
| 0 | `src/settings-events.ts` | ready | 8 | 99 | napi-rs | - |
| 0 | `src/escape-layer.ts` | ready | 52 | 98 | napi-rs | String offsets and slicing (1); Callback-shaped APIs (5) |
| 0 | `src/relay-headers.ts` | ready | 102 | 98 | napi-rs | Insertion-ordered maps and object keys (1) |
| 0 | `src/model-routes.ts` | ready | 167 | 97 | napi-rs | Insertion-ordered maps and object keys (3); Exceptions (1) |
| 0 | `src/SkillListViewport.tsx` | keep | 39 | 95 | keep | String offsets and slicing (1); Callback-shaped APIs (1) |
| 0 | `src/config-doctor-model.ts` | ready | 235 | 95 | napi-rs | String offsets and slicing (2); Number semantics (2) |
| 0 | `src/github-repository.ts` | ready | 73 | 95 | napi-rs | String offsets and slicing (2); Insertion-ordered maps and object keys (2) |
| 0 | `src/popover-layer.ts` | ready | 180 | 95 | napi-rs | Callback-shaped APIs (2); Insertion-ordered maps and object keys (1) |
| 0 | `src/script-market-model.ts` | ready | 103 | 95 | napi-rs | String offsets and slicing (3); Number semantics (2) |
| 0 | `src/agent-mcp-model.ts` | ready | 66 | 94 | napi-rs | String offsets and slicing (1); Regex features the `regex` crate lacks (1) |
| 0 | `src/lib/` | ready | 5 | 94 | napi-rs | - |
| 0 | `src/enhancement-catalog.ts` | ready | 161 | 93 | napi-rs | Regex features the `regex` crate lacks (2); Insertion-ordered maps and object keys (1) |
| 0 | `src/TopbarActions.tsx` | keep | 14 | 92 | keep | - |
| 0 | `src/management-model.ts` | ready | 41 | 92 | napi-rs | String offsets and slicing (3); Insertion-ordered maps and object keys (1) |
| 0 | `src/provider-name.ts` | ready | 60 | 92 | napi-rs | String offsets and slicing (3); Regexes built at runtime (1) |
| 0 | `src/update-notification.ts` | ready | 109 | 91 | napi-rs | String offsets and slicing (2); Callback-shaped APIs (2) |
| 0 | `src/legacy-webkit.ts` | ready | 92 | 89 | napi-rs | String offsets and slicing (1); Reflection and prototypes (2) |
| 0 | `src/relay-billing.ts` | ready | 223 | 89 | napi-rs | String offsets and slicing (1); Insertion-ordered maps and object keys (1) |
| 0 | `src/use-modal-focus.ts` | keep | 61 | 89 | keep | String offsets and slicing (2); Insertion-ordered maps and object keys (1) |
| 0 | `src/dream-skin.ts` | ready | 333 | 88 | napi-rs | Regex features the `regex` crate lacks (1); Async and promises (2) |
| 0 | `src/extension-manager-bridge.ts` | ready | 159 | 86 | napi-rs | String offsets and slicing (1); Callback-shaped APIs (10) |
| 0 | `src/agent-focus-model.ts` | ready | 73 | 85 | napi-rs | Async and promises (2); Callback-shaped APIs (2) |
| 0 | `src/code-highlight.ts` | ready | 253 | 85 | napi-rs | String offsets and slicing (21) |
| 0 | `src/context-config-text.ts` | ready | 248 | 82 | napi-rs | String offsets and slicing (5); Regex features the `regex` crate lacks (1) |
| 0 | `src/log-view-model.ts` | ready | 185 | 80 | napi-rs | Regex features the `regex` crate lacks (5); String offsets and slicing (3) |
| 0 | `src/skill-agent-batch.ts` | ready | 142 | 79 | napi-rs | String offsets and slicing (2); Callback-shaped APIs (13) |
| 0 | `src/ReleaseNotes.ts` | keep | 140 | 78 | keep | String offsets and slicing (14); Regex features the `regex` crate lacks (1) |
| 0 | `src/skill-groups.ts` | ready | 277 | 75 | napi-rs | String offsets and slicing (7); Insertion-ordered maps and object keys (14) |
| 0 | `src/skills-batch-react.manual.ts` | blocked | 420 | 51 | napi-rs | Dynamic code (eval, new Function, computed import) (1); Async and promises (239) |
| 0 | `src/runtime-context-capnweb-failure.manual.cjs` | blocked | 434 | 35 | napi-rs | String offsets and slicing (10); Dynamic code (eval, new Function, computed import) (6) |
| 0 | `src/model-picker-views-runtime.manual.ts` | blocked | 915 | 33 | napi-rs | Regexes built at runtime (1); Dynamic code (eval, new Function, computed import) (1) |
| 0 | `src/i18n-en.ts` | caution | 4760 | 0 | napi-rs | - |
| 1 | `src/extension-host-model.ts` | ready | 27 | 100 | napi-rs | - |
| 1 | `src/skill-repository.ts` | ready | 18 | 100 | napi-rs | String offsets and slicing (1); Exceptions (1) |
| 1 | `src/use-escape-layer.ts` | keep | 19 | 86 | keep | Callback-shaped APIs (1) |
| 1 | `src/i18n.ts` | ready | 66 | 80 | napi-rs | Callback-shaped APIs (3); Insertion-ordered maps and object keys (1) |
| 1 | `src/components/ui/` | keep | 138 | 78 | keep | - |
| 1 | `src/agent-focus-store.ts` | keep | 108 | 73 | keep | Async and promises (2); Dynamic typing (1) |
| 1 | `src/model-metadata.ts` | caution | 929 | 44 | napi-rs | String offsets and slicing (4); Regex features the `regex` crate lacks (3) |
| 2 | `src/use-popover-layer.ts` | keep | 153 | 95 | keep | Callback-shaped APIs (1) |
| 2 | `src/CodexLaunchButton.tsx` | keep | 48 | 94 | keep | Callback-shaped APIs (2) |
| 2 | `src/CommandPalette.tsx` | keep | 80 | 94 | keep | Callback-shaped APIs (1) |
| 2 | `src/CuPipWindow.tsx` | keep | 51 | 93 | keep | - |
| 2 | `src/model-reasoning.ts` | ready | 149 | 93 | napi-rs | String offsets and slicing (2); Insertion-ordered maps and object keys (3) |
| 2 | `src/model-windows.ts` | ready | 258 | 93 | napi-rs | Insertion-ordered maps and object keys (4); JSON round-trips (8) |
| 2 | `src/SkillBackupUsage.tsx` | keep | 59 | 91 | keep | Clock and random numbers (2); Exceptions (1) |
| 2 | `src/ExtensionManagerPanel.tsx` | keep | 102 | 90 | keep | Callback-shaped APIs (3); Async and promises (1) |
| 2 | `src/RelayHeaderRows.tsx` | keep | 69 | 89 | keep | String offsets and slicing (1); Callback-shaped APIs (1) |
| 2 | `src/components/` | keep | 160 | 89 | keep | String offsets and slicing (1); Callback-shaped APIs (2) |
| 2 | `src/SkillFileTimes.tsx` | keep | 36 | 88 | keep | String offsets and slicing (1) |
| 2 | `src/skill-source-scan.ts` | ready | 205 | 88 | napi-rs | Async and promises (4); Insertion-ordered maps and object keys (3) |
| 2 | `src/ExtensionPublishDialog.tsx` | keep | 183 | 80 | keep | Async and promises (5); Callback-shaped APIs (2) |
| 2 | `src/cx-layout.tsx` | keep | 187 | 75 | keep | Callback-shaped APIs (4) |
| 2 | `src/app-dialog.tsx` | keep | 170 | 74 | keep | Async and promises (3); Callback-shaped APIs (3) |
| 2 | `src/gateway-model.ts` | caution | 1679 | 5 | napi-rs | String offsets and slicing (31); Regex features the `regex` crate lacks (3) |
| 3 | `src/agent-dashboard-model.ts` | ready | 59 | 100 | napi-rs | String offsets and slicing (1) |
| 3 | `src/gateway-agent-safety.ts` | ready | 106 | 100 | napi-rs | - |
| 3 | `src/gateway-auto-compact-model.ts` | ready | 25 | 100 | napi-rs | Insertion-ordered maps and object keys (2) |
| 3 | `src/provider-form-model.ts` | ready | 88 | 100 | napi-rs | Insertion-ordered maps and object keys (2); Number semantics (1) |
| 3 | `src/provider-pool-layout.ts` | ready | 19 | 100 | napi-rs | Insertion-ordered maps and object keys (1) |
| 3 | `src/provider-meta-sync-model.ts` | ready | 148 | 97 | napi-rs | Regex features the `regex` crate lacks (1); Insertion-ordered maps and object keys (1) |
| 3 | `src/UsageTabs.tsx` | keep | 39 | 96 | keep | Callback-shaped APIs (3) |
| 3 | `src/tray-panel-model.ts` | ready | 205 | 96 | napi-rs | String offsets and slicing (1); Exceptions (4) |
| 3 | `src/provider-balances.ts` | ready | 64 | 95 | napi-rs | Callback-shaped APIs (4); Insertion-ordered maps and object keys (1) |
| 3 | `src/OnboardingTour.tsx` | keep | 120 | 93 | keep | Callback-shaped APIs (1) |
| 3 | `src/onboarding-setup-model.ts` | ready | 191 | 87 | napi-rs | String offsets and slicing (2); Dynamic typing (2) |
| 3 | `src/request-flow-model.ts` | ready | 134 | 86 | napi-rs | String offsets and slicing (3); Insertion-ordered maps and object keys (5) |
| 3 | `src/gateway-store.ts` | keep | 60 | 81 | keep | Callback-shaped APIs (4); Insertion-ordered maps and object keys (1) |
| 3 | `src/AppSelect.tsx` | keep | 120 | 80 | keep | Callback-shaped APIs (1) |
| 3 | `src/gateway-ui.tsx` | keep | 276 | 56 | keep | Async and promises (5); Callback-shaped APIs (5) |
| 4 | `src/hub-overview-model.ts` | ready | 160 | 98 | napi-rs | Insertion-ordered maps and object keys (1) |
| 4 | `src/SubscriptionPolicyFields.tsx` | keep | 56 | 96 | keep | Insertion-ordered maps and object keys (2); Callback-shaped APIs (1) |
| 4 | `src/RequestFlow.tsx` | keep | 96 | 90 | keep | Async and promises (2); Exceptions (1) |
| 4 | `src/ProviderKeyList.tsx` | keep | 107 | 88 | keep | Async and promises (6); Callback-shaped APIs (1) |
| 4 | `src/GatewayModelMatrix.tsx` | keep | 146 | 87 | keep | String offsets and slicing (1); Callback-shaped APIs (1) |
| 4 | `src/ProviderModelsDialog.tsx` | keep | 146 | 84 | keep | Async and promises (2); Callback-shaped APIs (2) |
| 4 | `src/ProviderReasoningDialog.tsx` | keep | 162 | 84 | keep | Dynamic typing (2); Callback-shaped APIs (2) |
| 4 | `src/ProviderTestDialog.tsx` | keep | 212 | 79 | keep | Callback-shaped APIs (4); Async and promises (2) |
| 4 | `src/extension-config-form.tsx` | keep | 211 | 79 | keep | Callback-shaped APIs (5); Insertion-ordered maps and object keys (2) |
| 4 | `src/PluginMarketScreen.tsx` | keep | 319 | 78 | keep | Async and promises (7); Exceptions (3) |
| 4 | `src/AgentDisconnectDialog.tsx` | keep | 141 | 77 | keep | Async and promises (5); Callback-shaped APIs (3) |
| 4 | `src/ConfigDoctorPanel.tsx` | keep | 360 | 77 | keep | Async and promises (8); Exceptions (4) |
| 4 | `src/ComputerUsePreview.tsx` | keep | 244 | 75 | keep | Async and promises (9); Callback-shaped APIs (2) |
| 4 | `src/NativeAuthControls.tsx` | keep | 137 | 75 | keep | String offsets and slicing (1); Async and promises (8) |
| 4 | `src/ProviderMetaSyncDialog.tsx` | keep | 184 | 73 | keep | String offsets and slicing (1); Insertion-ordered maps and object keys (4) |
| 4 | `src/RelayBillingLine.tsx` | keep | 334 | 72 | keep | Callback-shaped APIs (7); Insertion-ordered maps and object keys (2) |
| 4 | `src/ProviderSyncDialog.tsx` | keep | 181 | 71 | keep | String offsets and slicing (1); Insertion-ordered maps and object keys (7) |
| 4 | `src/ProviderExtrasDialog.tsx` | keep | 372 | 66 | keep | Callback-shaped APIs (7); Async and promises (4) |
| 4 | `src/GatewaySessions.tsx` | keep | 220 | 65 | keep | String offsets and slicing (2); Async and promises (18) |
| 5 | `src/ProviderWindowsDialog.tsx` | keep | 192 | 79 | keep | Insertion-ordered maps and object keys (8); Async and promises (2) |
| 5 | `src/GatewayAgents.tsx` | keep | 1340 | 26 | keep | String offsets and slicing (3); Callback-shaped APIs (21) |
| 6 | `src/ProviderFormDialog.tsx` | keep | 408 | 63 | keep | String offsets and slicing (1); Insertion-ordered maps and object keys (6) |
| 7 | `src/GatewayProviders.tsx` | keep | 1451 | 4 | keep | String offsets and slicing (5); Async and promises (50) |
| 8 | `src/HubOverview.tsx` | keep | 474 | 55 | keep | String offsets and slicing (1); Callback-shaped APIs (4) |
| 9 | `src/SessionAgentTabs.tsx` | keep | 28 | 100 | keep | Callback-shaped APIs (1) |
| 9 | `src/AgentFocusDialog.tsx` | keep | 146 | 79 | keep | String offsets and slicing (1); Async and promises (2) |
| 9 | `src/AgentDashboard.tsx` | keep | 214 | 74 | keep | String offsets and slicing (1); Callback-shaped APIs (4) |
| 9 | `src/OnboardingSetup.tsx` | keep | 318 | 71 | keep | Async and promises (10); Callback-shaped APIs (6) |
| 9 | `src/ManagementPages.tsx` | keep | 326 | 47 | keep | Regex features the `regex` crate lacks (1); Async and promises (37) |
| 9 | `src/TrayPanel.tsx` | keep | 937 | 37 | keep | String offsets and slicing (3); Async and promises (25) |
| 10 | `src/ExtensionCard.tsx` | keep | 83 | 86 | keep | String offsets and slicing (1); Callback-shaped APIs (1) |
| 10 | `src/SkillInPlaceUpdateDrawer.tsx` | keep | 64 | 86 | keep | Async and promises (2); Callback-shaped APIs (2) |
| 10 | `src/SkillDuplicatesDrawer.tsx` | keep | 60 | 85 | keep | Async and promises (3); Callback-shaped APIs (2) |
| 10 | `src/agent-cache.tsx` | keep | 201 | 76 | keep | Async and promises (5); Insertion-ordered maps and object keys (2) |
| 10 | `src/ProjectSessionPage.tsx` | keep | 155 | 74 | keep | Insertion-ordered maps and object keys (12); Async and promises (8) |
| 10 | `src/GatewayUsage.tsx` | keep | 396 | 73 | keep | Async and promises (9); Insertion-ordered maps and object keys (1) |
| 10 | `src/SkillSourceMatchDrawer.tsx` | keep | 176 | 73 | keep | Insertion-ordered maps and object keys (6); Async and promises (4) |
| 10 | `src/SkillVersionPicker.tsx` | keep | 55 | 73 | keep | Async and promises (13); Insertion-ordered maps and object keys (3) |
| 10 | `src/AgentCollaborationPage.tsx` | keep | 545 | 68 | keep | Async and promises (8); Insertion-ordered maps and object keys (1) |
| 10 | `src/RemoteControlPage.tsx` | keep | 327 | 65 | keep | String offsets and slicing (1); Async and promises (11) |
| 10 | `src/DshPluginsPanel.tsx` | keep | 492 | 63 | keep | Async and promises (20); Callback-shaped APIs (2) |
| 10 | `src/ComputerUsePage.tsx` | keep | 928 | 44 | keep | String offsets and slicing (3); Async and promises (34) |
| 11 | `src/ExtensionCuration.tsx` | keep | 179 | 72 | keep | String offsets and slicing (3); Callback-shaped APIs (3) |
| 11 | `src/CommunityExtensionsPanel.tsx` | keep | 393 | 55 | keep | String offsets and slicing (5); Async and promises (10) |
| 11 | `src/GatewayPage.tsx` | keep | 978 | 49 | keep | String offsets and slicing (1); Async and promises (15) |
| 11 | `src/ManagementExtra.tsx` | keep | 1087 | 4 | keep | String offsets and slicing (1); Async and promises (92) |
| 12 | `src/MyPublishedPanel.tsx` | keep | 206 | 75 | keep | String offsets and slicing (1); Async and promises (5) |
| 12 | `src/CuratedPackages.tsx` | keep | 211 | 69 | keep | String offsets and slicing (5); Async and promises (5) |
| 13 | `src/provider-api-keys.ts` | ready | 77 | 94 | napi-rs | String offsets and slicing (2); Insertion-ordered maps and object keys (2) |
| 13 | `src/relay-profile-editing.ts` | caution | 1416 | 10 | napi-rs | String offsets and slicing (22); Regex features the `regex` crate lacks (2) |
| 13 | `src/App.tsx` | keep | 18015 | 0 | keep | String offsets and slicing (28); Regex features the `regex` crate lacks (2) |
| 14 | `src/main.tsx` | keep | 66 | 92 | keep | Class inheritance (1) |
| 14 | `web-preview/` | keep | 1051 | 40 | keep | String offsets and slicing (5); Async and promises (18) |

## npm dependencies

| Package | Units | Rust | Fit | Note |
| --- | ---: | --- | --- | --- |
| `clsx` | 1 | ? | unknown |  |
| `esbuild` | 1 | ? | unknown |  |
| `tailwind-merge` | 1 | ? | unknown |  |

## Blocked

- `src/model-picker-views-runtime.manual.ts`: Uses dynamic code (eval, new Function or computed imports).
- `src/runtime-context-capnweb-failure.manual.cjs`: Uses dynamic code (eval, new Function or computed imports).
- `src/skills-batch-react.manual.ts`: Uses dynamic code (eval, new Function or computed imports).

## Risk guide

- **String offsets and slicing** (High): JS indexes strings in UTF-16 code units, Rust in UTF-8 bytes. Offsets that cross the boundary must be converted, or non-ASCII text gets wrong ranges silently.
- **Regex features the `regex` crate lacks** (High): Lookaround, backreferences and the `y` flag are not in the `regex` crate. Use `fancy-regex` for those patterns, or rewrite them, and test them on non-ASCII text: `\w`, `\b` and case-insensitive matching differ between engines even where a pattern compiles. JS `/i` without `u` never matches a non-ASCII letter to an ASCII one, Rust `(?i)` does (`ſ` matches `s`, the Kelvin sign `K` matches `k`): write `(?i-u)` or compare with `eq_ignore_ascii_case`.
- **Regexes built at runtime** (High): Patterns come from data or config in JS syntax. Rust regex syntax differs, so user-visible behavior can change. Pin it with tests on real patterns.
- **Dynamic code (eval, new Function, computed import)** (High): eval, new Function and computed imports have no Rust equivalent. Keep this part in JS or replace it with an explicit registry.
- **Dynamic typing** (Medium): Values whose shape is checked at runtime need an explicit Rust enum or `serde_json::Value`.
- **Insertion-ordered maps and object keys** (Medium): JS Map, Set and object keys keep insertion order. `HashMap` does not: use `indexmap` wherever order reaches output. With serde_json's `preserve_order`, `Map::remove` swaps the last key into the hole: use `shift_remove` for JS `delete`. Sort with `js::sort` when a comparator can return NaN: the order then depends on V8's algorithm.
- **Exceptions** (Low): Thrown errors become `Result`. Errors crossing the bridge must map back to the same JS error shape.
- **Async and promises** (Medium): Promises become futures. Bridge with napi-rs `async fn` or `wasm-bindgen-futures`, or keep the async shell in JS and port the sync core.
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
| `src/model-picker-views-runtime.manual.ts` | server | `createServer(` at src/model-picker-views-runtime.manual.ts:177 |

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
| `react-dom` | yes | 2 | ? | unknown |  |
| `@radix-ui/react-slot` | yes | 1 | ? | unknown |  |
| `class-variance-authority` | yes | 1 | ? | unknown |  |
| `clsx` | yes | 1 | ? | unknown |  |
| `esbuild` | no | 1 | ? | unknown |  |
| `tailwind-merge` | yes | 1 | ? | unknown |  |
| `@fontsource/inter` | yes | 0 | ? | unknown |  |
| `@tauri-apps/cli` | yes | 0 | ? | unknown |  |
| `react` | yes | 0 | ? | unknown |  |
| `typescript` | yes | 0 | oxc | partial | Parsing is covered. Type checking has no Rust equivalent. |

### Pieces, in port order

- wave 0: `src/agent-cache-model.ts` (60 lines)
- wave 0: `src/agent-collaboration-queue.ts` (48 lines)
- wave 0: `src/agent-focus-model.ts` (73 lines)
- wave 0: `src/agent-mcp-model.ts` (66 lines)
- wave 0: `src/aggregate-members.ts` (106 lines)
- wave 0: `src/aggregate-routes.ts` (70 lines)
- wave 0: `src/auto-compact.ts` (40 lines)
- wave 0: `src/code-highlight.ts` (253 lines)
- wave 0: `src/config-doctor-model.ts` (235 lines)
- wave 0: `src/context-config-text.ts` (248 lines)
- wave 0: `src/dictation-settings.ts` (47 lines)
- wave 0: `src/dream-skin.ts` (333 lines)
- wave 0: `src/enhancement-catalog.ts` (161 lines)
- wave 0: `src/enhancement-navigation.ts` (14 lines)
- wave 0: `src/escape-layer.ts` (52 lines)
- wave 0: `src/extension-config-schema.ts` (71 lines)
- wave 0: `src/extension-manager-bridge.ts` (159 lines)
- wave 0: `src/extension-manager-pages.ts` (119 lines)
- wave 0: `src/flow-provider-colors.ts` (31 lines)
- wave 0: `src/github-repository.ts` (73 lines)
- wave 0: `src/goals-config.ts` (88 lines)
- wave 0: `src/history-recovery-report.ts` (15 lines)
- wave 0: `src/i18n-en.ts` (4760 lines)
- wave 0: `src/launch-status.ts` (28 lines)
- wave 0: `src/legacy-webkit.ts` (92 lines)
- wave 0: `src/lib/` (5 lines)
- wave 0: `src/log-view-model.ts` (185 lines)
- wave 0: `src/mac-titlebar.ts` (21 lines)
- wave 0: `src/management-model.ts` (41 lines)
- wave 0: `src/mcp-presets.ts` (82 lines)
- wave 0: `src/model-groups.ts` (70 lines)
- wave 0: `src/model-picker-views-runtime.manual.ts` (915 lines)
- wave 0: `src/model-routes.ts` (167 lines)
- wave 0: `src/multi-launch-model.ts` (85 lines)
- wave 0: `src/native-auth-model.ts` (28 lines)
- wave 0: `src/notice-model.ts` (17 lines)
- wave 0: `src/onboarding-model.ts` (119 lines)
- wave 0: `src/overview-model.ts` (53 lines)
- wave 0: `src/page-groups.ts` (26 lines)
- wave 0: `src/plugin-market-model.ts` (121 lines)
- wave 0: `src/popover-layer.ts` (180 lines)
- wave 0: `src/presets.ts` (273 lines)
- wave 0: `src/provider-brand-icons.ts` (64 lines)
- wave 0: `src/provider-brands.ts` (69 lines)
- wave 0: `src/provider-name.ts` (60 lines)
- wave 0: `src/provider-sync-flow.ts` (63 lines)
- wave 0: `src/provider-sync-model.ts` (68 lines)
- wave 0: `src/provider-sync-target.ts` (21 lines)
- wave 0: `src/provider-test.ts` (76 lines)
- wave 0: `src/relay-billing.ts` (223 lines)
- wave 0: `src/relay-headers.ts` (102 lines)
- wave 0: `src/relay-live-files.ts` (22 lines)
- wave 0: `src/relay-session.ts` (17 lines)
- wave 0: `src/runtime-context-capnweb-failure.manual.cjs` (434 lines)
- wave 0: `src/script-market-model.ts` (103 lines)
- wave 0: `src/settings-events.ts` (8 lines)
- wave 0: `src/settings-rebase.ts` (67 lines)
- wave 0: `src/skill-agent-batch.ts` (142 lines)
- wave 0: `src/skill-groups.ts` (277 lines)
- wave 0: `src/skills-batch-react.manual.ts` (420 lines)
- wave 0: `src/subscription-policy.ts` (45 lines)
- wave 0: `src/theme-mode.ts` (56 lines)
- wave 0: `src/update-notification.ts` (109 lines)
- wave 0: `src/vlm-test-translation.ts` (37 lines)
- wave 0: `src/whale-settings.ts` (45 lines)
- wave 1: `src/extension-host-model.ts` (27 lines)
- wave 1: `src/i18n.ts` (66 lines)
- wave 1: `src/model-metadata.ts` (929 lines)
- wave 1: `src/skill-repository.ts` (18 lines)
- wave 2: `src/gateway-model.ts` (1679 lines)
- wave 2: `src/model-reasoning.ts` (149 lines)
- wave 2: `src/model-windows.ts` (258 lines)
- wave 2: `src/skill-source-scan.ts` (205 lines)
- wave 3: `src/agent-dashboard-model.ts` (59 lines)
- wave 3: `src/gateway-agent-safety.ts` (106 lines)
- wave 3: `src/gateway-auto-compact-model.ts` (25 lines)
- wave 3: `src/onboarding-setup-model.ts` (191 lines)
- wave 3: `src/provider-balances.ts` (64 lines)
- wave 3: `src/provider-form-model.ts` (88 lines)
- wave 3: `src/provider-meta-sync-model.ts` (148 lines)
- wave 3: `src/provider-pool-layout.ts` (19 lines)
- wave 3: `src/request-flow-model.ts` (134 lines)
- wave 3: `src/tray-panel-model.ts` (205 lines)
- wave 4: `src/hub-overview-model.ts` (160 lines)
- wave 13: `src/provider-api-keys.ts` (77 lines)
- wave 13: `src/relay-profile-editing.ts` (1416 lines)

### Decisions for the user

**D1** (shape): The app is a server, entered through src/model-picker-views-runtime.manual.ts. Port every entry point, in that shape?

> Yes: an axum front door (`app front-door`) takes the server over first

**D2** (keep src/AgentCollaborationPage.tsx): src/AgentCollaborationPage.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D3** (keep src/AgentDashboard.tsx): src/AgentDashboard.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D4** (keep src/AgentDisconnectDialog.tsx): src/AgentDisconnectDialog.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D5** (keep src/AgentFocusDialog.tsx): src/AgentFocusDialog.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D6** (keep src/App.tsx): src/App.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D7** (keep src/AppSelect.tsx): src/AppSelect.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D8** (keep src/CodexLaunchButton.tsx): src/CodexLaunchButton.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D9** (keep src/CommandPalette.tsx): src/CommandPalette.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D10** (keep src/CommunityExtensionsPanel.tsx): src/CommunityExtensionsPanel.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D11** (keep src/ComputerUsePage.tsx): src/ComputerUsePage.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D12** (keep src/ComputerUsePreview.tsx): src/ComputerUsePreview.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D13** (keep src/ConfigDoctorPanel.tsx): src/ConfigDoctorPanel.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D14** (keep src/CuPipWindow.tsx): src/CuPipWindow.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D15** (keep src/CuratedPackages.tsx): src/CuratedPackages.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D16** (keep src/DshPluginsPanel.tsx): src/DshPluginsPanel.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D17** (keep src/ExtensionCard.tsx): src/ExtensionCard.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D18** (keep src/ExtensionCuration.tsx): src/ExtensionCuration.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D19** (keep src/ExtensionManagerPanel.tsx): src/ExtensionManagerPanel.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D20** (keep src/ExtensionPublishDialog.tsx): src/ExtensionPublishDialog.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D21** (keep src/GatewayAgents.tsx): src/GatewayAgents.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D22** (keep src/GatewayModelMatrix.tsx): src/GatewayModelMatrix.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D23** (keep src/GatewayPage.tsx): src/GatewayPage.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D24** (keep src/GatewayProviders.tsx): src/GatewayProviders.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D25** (keep src/GatewaySessions.tsx): src/GatewaySessions.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D26** (keep src/GatewayUsage.tsx): src/GatewayUsage.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D27** (keep src/HubOverview.tsx): src/HubOverview.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D28** (keep src/ManagementExtra.tsx): src/ManagementExtra.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D29** (keep src/ManagementPages.tsx): src/ManagementPages.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D30** (keep src/MyPublishedPanel.tsx): src/MyPublishedPanel.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D31** (keep src/NativeAuthControls.tsx): src/NativeAuthControls.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D32** (keep src/OnboardingSetup.tsx): src/OnboardingSetup.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D33** (keep src/OnboardingTour.tsx): src/OnboardingTour.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D34** (keep src/PluginMarketScreen.tsx): src/PluginMarketScreen.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D35** (keep src/ProjectSessionPage.tsx): src/ProjectSessionPage.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D36** (keep src/ProviderExtrasDialog.tsx): src/ProviderExtrasDialog.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D37** (keep src/ProviderFormDialog.tsx): src/ProviderFormDialog.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D38** (keep src/ProviderKeyList.tsx): src/ProviderKeyList.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D39** (keep src/ProviderMetaSyncDialog.tsx): src/ProviderMetaSyncDialog.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D40** (keep src/ProviderModelsDialog.tsx): src/ProviderModelsDialog.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D41** (keep src/ProviderReasoningDialog.tsx): src/ProviderReasoningDialog.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D42** (keep src/ProviderSyncDialog.tsx): src/ProviderSyncDialog.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D43** (keep src/ProviderTestDialog.tsx): src/ProviderTestDialog.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D44** (keep src/ProviderWindowsDialog.tsx): src/ProviderWindowsDialog.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D45** (keep src/RelayBillingLine.tsx): src/RelayBillingLine.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D46** (keep src/RelayHeaderRows.tsx): src/RelayHeaderRows.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D47** (keep src/ReleaseNotes.ts): src/ReleaseNotes.ts stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D48** (keep src/RemoteControlPage.tsx): src/RemoteControlPage.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D49** (keep src/RequestFlow.tsx): src/RequestFlow.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D50** (keep src/SessionAgentTabs.tsx): src/SessionAgentTabs.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D51** (keep src/SkillBackupUsage.tsx): src/SkillBackupUsage.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D52** (keep src/SkillDuplicatesDrawer.tsx): src/SkillDuplicatesDrawer.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D53** (keep src/SkillFileTimes.tsx): src/SkillFileTimes.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D54** (keep src/SkillInPlaceUpdateDrawer.tsx): src/SkillInPlaceUpdateDrawer.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D55** (keep src/SkillListViewport.tsx): src/SkillListViewport.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D56** (keep src/SkillSourceMatchDrawer.tsx): src/SkillSourceMatchDrawer.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D57** (keep src/SkillVersionPicker.tsx): src/SkillVersionPicker.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D58** (keep src/SubscriptionPolicyFields.tsx): src/SubscriptionPolicyFields.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D59** (keep src/TopbarActions.tsx): src/TopbarActions.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D60** (keep src/TrayPanel.tsx): src/TrayPanel.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D61** (keep src/UsageTabs.tsx): src/UsageTabs.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D62** (keep src/agent-cache.tsx): src/agent-cache.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D63** (keep src/agent-focus-store.ts): src/agent-focus-store.ts stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D64** (keep src/app-dialog.tsx): src/app-dialog.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D65** (keep src/command-palette-model.ts): src/command-palette-model.ts stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D66** (keep src/components/): src/components/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D67** (keep src/components/ui/): src/components/ui/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D68** (keep src/cx-layout.tsx): src/cx-layout.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D69** (keep src/extension-config-form.tsx): src/extension-config-form.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D70** (keep src/gateway-store.ts): src/gateway-store.ts stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D71** (keep src/gateway-ui.tsx): src/gateway-ui.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D72** (keep src/main.tsx): src/main.tsx stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D73** (keep src/use-escape-layer.ts): src/use-escape-layer.ts stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D74** (keep src/use-modal-focus.ts): src/use-modal-focus.ts stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D75** (keep src/use-popover-layer.ts): src/use-popover-layer.ts stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D76** (keep web-preview/): web-preview/ stays in the source language (UI code: keep in the original language). What replaces it in the Rust app, or does it stay outside?

> Keep it outside the Rust app, behind the front door, and decide its replacement once the rest is Rust

**D77** (blocked src/model-picker-views-runtime.manual.ts): src/model-picker-views-runtime.manual.ts is blocked: Uses dynamic code (eval, new Function or computed imports). Rework that part in Rust, or keep the unit outside?

> Research the blocker first; then port it with the piece that uses it

**D78** (blocked src/runtime-context-capnweb-failure.manual.cjs): src/runtime-context-capnweb-failure.manual.cjs is blocked: Uses dynamic code (eval, new Function or computed imports). Rework that part in Rust, or keep the unit outside?

> Research the blocker first; then port it with the piece that uses it

**D79** (blocked src/skills-batch-react.manual.ts): src/skills-batch-react.manual.ts is blocked: Uses dynamic code (eval, new Function or computed imports). Rework that part in Rust, or keep the unit outside?

> Research the blocker first; then port it with the piece that uses it

**D80** (dependency vite): No crate does what vite does (declared, no unit imports it; Dev server or UI framework: keep in JS). Reimplement what the app uses, or drop the feature?

> Reimplement only the part the app uses, proved by its recorded cases

**D81** (dependency lucide-react): Which crate replaces lucide-react (used by 55 units)?

> Research it against primary sources into rust/research/lucide-react.md before porting what uses it

**D82** (dependency @tauri-apps/api): Which crate replaces @tauri-apps/api (used by 47 units)?

> Research it against primary sources into rust/research/@tauri-apps-api.md before porting what uses it

**D83** (dependency @tauri-apps/plugin-dialog): Which crate replaces @tauri-apps/plugin-dialog (used by 6 units)?

> Research it against primary sources into rust/research/@tauri-apps-plugin-dialog.md before porting what uses it

**D84** (dependency @dnd-kit/core): Which crate replaces @dnd-kit/core (used by 3 units)?

> Research it against primary sources into rust/research/@dnd-kit-core.md before porting what uses it

**D85** (dependency @dnd-kit/sortable): Which crate replaces @dnd-kit/sortable (used by 3 units)?

> Research it against primary sources into rust/research/@dnd-kit-sortable.md before porting what uses it

**D86** (dependency @dnd-kit/utilities): Which crate replaces @dnd-kit/utilities (used by 3 units)?

> Research it against primary sources into rust/research/@dnd-kit-utilities.md before porting what uses it

**D87** (dependency @fontsource/jetbrains-mono): Which crate replaces @fontsource/jetbrains-mono (used by 2 units)?

> Research it against primary sources into rust/research/@fontsource-jetbrains-mono.md before porting what uses it

**D88** (dependency react-dom): Which crate replaces react-dom (used by 2 units)?

> Research it against primary sources into rust/research/react-dom.md before porting what uses it

**D89** (dependency @radix-ui/react-slot): Which crate replaces @radix-ui/react-slot (used by 1 unit)?

> Research it against primary sources into rust/research/@radix-ui-react-slot.md before porting what uses it

**D90** (dependency class-variance-authority): Which crate replaces class-variance-authority (used by 1 unit)?

> Research it against primary sources into rust/research/class-variance-authority.md before porting what uses it

**D91** (dependency clsx): Which crate replaces clsx (used by 1 unit)?

> Research it against primary sources into rust/research/clsx.md before porting what uses it

**D92** (dependency esbuild): Which crate replaces esbuild (used by 1 unit)?

> Research it against primary sources into rust/research/esbuild.md before porting what uses it

**D93** (dependency tailwind-merge): Which crate replaces tailwind-merge (used by 1 unit)?

> Research it against primary sources into rust/research/tailwind-merge.md before porting what uses it

**D94** (dependency @fontsource/inter): Which crate replaces @fontsource/inter (declared, no unit imports it)?

> Research it against primary sources into rust/research/@fontsource-inter.md before porting what uses it

**D95** (dependency @tauri-apps/cli): Which crate replaces @tauri-apps/cli (declared, no unit imports it)?

> Research it against primary sources into rust/research/@tauri-apps-cli.md before porting what uses it

**D96** (dependency react): Which crate replaces react (declared, no unit imports it)?

> Research it against primary sources into rust/research/react.md before porting what uses it

**D97** (dependency typescript): typescript maps to oxc only in part (declared, no unit imports it; Parsing is covered. Type checking has no Rust equivalent). Use it?

> Use oxc, and record cases for the behavior it does not cover

