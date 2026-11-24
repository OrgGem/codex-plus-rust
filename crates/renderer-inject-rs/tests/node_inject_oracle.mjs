import fs from "node:fs";
import vm from "node:vm";

const base = "d:/Git/codex-plus/assets/inject/renderer-inject";

// 1. 96-prompt-args.js
const promptSrc = fs.readFileSync(`${base}/96-prompt-args.js`, "utf8");
const promptCtx = vm.createContext({
  codexPlusBackendSettingsLoaded: true,
  codexPlusSettings: () => ({ promptArgs: true }),
  codexPlusComposerEditorFrom: () => null,
  codexPlusClearComposer: () => {},
  showToast: () => {},
  postJson: async () => ({}),
  document: {},
  Date,
  Map,
  Promise,
  setTimeout: () => 0,
});
vm.runInContext(
  `${promptSrc}\nthis.__out = { split: codexPlusPromptArgsSplit, placeholders: codexPlusPromptArgsPlaceholders, expand: codexPlusPromptArgsExpand, pattern: codexPlusPromptArgsPattern };`,
  promptCtx,
);

const splitInputs = [
  `a  b\t"c d" 'e "f"' "g \\"h\\"" ""`,
  `KEY="x y" z`,
  `   `,
  `"unclosed trailing`,
  `plain1 plain2 "tiếng việt" '中文'`,
];
const splitResults = splitInputs.map((s) => [...promptCtx.__out.split(s)]);

const placeholderInputs = [
  "Fix $1 in $2 ($9) costs $$5",
  "All: $ARGUMENTS!",
  "Review $BRANCH against $BASE and $BRANCH",
  "No placeholders at all",
  "Mixed $HOME and $1 with $USER_ID",
];
const placeholderResults = placeholderInputs.map((s) => {
  const r = promptCtx.__out.placeholders(s);
  return { positional: r.positional, named: [...r.named] };
});

const expandCases = [
  ["p", "Fix $1 in $2 ($9) costs $$5", `bug "src/a b.ts"`],
  ["p", "All: $ARGUMENTS!", `one "two three"`],
  ["p", "Review $BRANCH against $BASE", "BRANCH=feat BASE=main"],
  ["p", "Review $BRANCH against $BASE", "BRANCH=feat"],
  ["p", "Review $BRANCH", "BRANCH=feat stray"],
  ["p", "$BRANCH $1", "BRANCH=x"],
  ["p", "Run in $HOME: $1", "build"],
  ["p", "Explain this.", "  foo bar "],
];
const expandResults = expandCases.map(([id, content, argText]) => {
  const r = promptCtx.__out.expand({ id, content }, argText);
  return JSON.parse(JSON.stringify(r));
});

// 2. 34-statsig-overrides.js
const statsigSrc = fs.readFileSync(`${base}/34-statsig-overrides.js`, "utf8");
const statsigCtx = vm.createContext({
  window: { __CODEX_PLUS_STATSIG_OVERRIDES__: null },
  globalThis: null,
  codexPlusBackendSettingsLoaded: true,
  codexPlusBackendSettings: {
    codexAppStatsigGateOverrides: {
      "12345": "off",
      "99999": "on",
      "555": "native",
      bad_id: "on",
      "111": "invalid",
    },
  },
  codexPlusSettings: () => ({ statsigOverrides: true }),
  Date,
  Map,
  Set,
  JSON,
  Object,
  Array,
  String,
  Number,
  Boolean,
  RegExp,
});
statsigCtx.globalThis = statsigCtx;
statsigCtx.window.__CODEX_PLUS_STATSIG_OVERRIDES__ = {
  enabled: true,
  gates: { "12345": true, "555": true, "777": false },
  dynamicConfigs: { cfg1: { timeout: 5000, nested: { a: 1 } } },
  experiments: { exp1: { enabled: true } },
  layers: { lyr1: { v: 2 } },
  gateCatalog: [
    { id: "12345", title: "Gate A" },
    { id: 99999, title: "Num Gate" },
    { id: "not-numeric", title: "Skip" },
  ],
};
vm.runInContext(
  `${statsigSrc}\nthis.__out = { gateCatalog: codexPlusStatsigGateCatalog, userGates: codexPlusStatsigUserGateOverrides, effectiveGates: codexPlusStatsigEffectiveGates, mergeObj: codexPlusStatsigMergeObjectValue, sig: codexPlusStatsigOverridesSignature };`,
  statsigCtx,
);

const statsigCfg = statsigCtx.window.__CODEX_PLUS_STATSIG_OVERRIDES__;
const mergeTarget1 = {
  name: "cfg1",
  value: { timeout: 1000, keep: "orig", nested: { a: 0, b: 2 } },
  __evaluation: { value: { timeout: 1000 }, is_user_in_experiment: false, is_experiment_active: false },
};
const merged1 = statsigCtx.__out.mergeObj({ ...mergeTarget1 }, statsigCfg.dynamicConfigs.cfg1);

const mergeTarget2 = {
  name: "lyr1",
  __value: { v: 1, stay: true },
};
const merged2 = statsigCtx.__out.mergeObj({ ...mergeTarget2 }, statsigCfg.layers.lyr1);

// 3. 70-model-catalog.js (load helper defs bypassing only the __CODEX_PLUS_TEST_SERVICE_TIER__ branch)
const catalogSrc = fs.readFileSync(`${base}/70-model-catalog.js`, "utf8");
const catalogCtx = vm.createContext({
  window: { __CODEX_PLUS_TEST_SERVICE_TIER__: false },
  globalThis: {},
  codexPlusBackendSettingsLoaded: true,
  codexPlusBackendSettings: {
    relayProfilesEnabled: true,
    activeRelayId: "p1",
    activeAggregateRelayId: "agg1",
    activeRelayCodexProvider: "custom",
    modelCatalogSourceIdentity: "src-1",
    relayProfiles: [
      { id: "p1", name: "P1", relayMode: "pureApi", protocol: "responses", model: "gpt-5", modelList: "gpt-5,o3" },
      { id: "p2", name: "P2", relayMode: "official", model: "gpt-5" },
    ],
    aggregateRelayProfiles: [
      { id: "agg1", members: [{ relayId: "p1" }, { relayId: "p2" }] },
    ],
  },
  codexPlusSettings: () => ({ modelWhitelistUnlock: true }),
  codexRemoteSessionActiveProfile: () => null,
  codexServiceTierModelFromValue: (m) => String(m || "").trim(),
  registerCodexPlusExtensionSelector: () => {},
  sendCodexPlusDiagnostic: () => {},
  renderCodexPlusMenu: () => {},
  findCodexQueryClient: () => null,
  WeakMap,
  Map,
  Set,
  Array,
  Object,
  String,
  Number,
  Boolean,
  JSON,
  RegExp,
  Date,
  Math,
  Promise,
  setTimeout: () => 0,
  clearTimeout: () => {},
});
catalogCtx.window.setTimeout = () => 0;
const catApi = vm.runInContext(
  `(function() {\n${catalogSrc}\n;return { uniqueValues, codexModelSettingsSource, codexModelSettingsKey, codexModelCatalogIdentity, catalogBelongsToProfile: codexModelCatalogBelongsToProfile, modelReasoningEfforts, buildDescriptor: codexPlusModelDescriptor, applyMetadata: applyCodexPlusModelMetadata, modelResponseHostId, setCatalog: (c) => { codexModelCatalog = c; } };\n})()`,
  catalogCtx,
);

const sampleCatalog = {
  status: "ok",
  model: "gpt-5.4",
  default_model: "gpt-5.4",
  model_provider: "p1",
  provider_name: "Custom Relay",
  sources: [{ id: "relay-profile:p1" }],
  modelMetadata: {
    "gpt-5.4": {
      displayName: "GPT 5.4",
      description: "Flagship",
      defaultReasoningEffort: "high",
      inputModalities: ["text", "image"],
      supportsImageDetailOriginal: true,
      supportedReasoningEfforts: [
        { reasoningEffort: "low", description: "low d" },
        { reasoningEffort: "high", description: "high d" },
      ],
    },
    "custom-small": {
      supportedReasoningEfforts: [{ reasoningEffort: "low", description: "only low" }],
    },
  },
};
catApi.setCatalog(sampleCatalog);

const appliedDesc = { model: "gpt-5.4", displayName: "old", supportedReasoningEfforts: [] };
const appliedChanged = catApi.applyMetadata(appliedDesc, "GPT-5.4");

// 4. 71-model-picker-groups.js
const pickerSrc = fs.readFileSync(`${base}/71-model-picker-groups.js`, "utf8");
const pickerCtx = vm.createContext({
  window: { setTimeout: () => 0, clearTimeout: () => {}, setInterval: () => 0, clearInterval: () => {} },
  document: {
    documentElement: { lang: "zh-CN", appendChild: () => {} },
    head: { appendChild: () => {} },
    body: { appendChild: () => {} },
    getElementById: () => null,
    createElement: () => ({ id: "", dataset: {}, textContent: "", setAttribute: () => {}, appendChild: () => {} }),
    querySelector: () => null,
    querySelectorAll: () => [],
    addEventListener: () => {},
  },
  navigator: { language: "zh-CN" },
  registerCodexPlusExtensionSelector: () => {},
  MutationObserver: class { observe() {} disconnect() {} },
  setTimeout: () => 0,
  clearTimeout: () => {},
  setInterval: () => 0,
  clearInterval: () => {},
  WeakMap,
  WeakSet,
  Set,
  Map,
  Array,
  Object,
  String,
  Number,
  Boolean,
  JSON,
  RegExp,
});
const pickerApi = vm.runInContext(
  `(function() {\n${pickerSrc}\n;return { splitLabel: codexPlusModelPickerSplitLabel };\n})()`,
  pickerCtx,
);

const pickerSamples = [
  "DeepSeek-V4-Pro（DeepSeek）",
  "Claude Sonnet 4 (Anthropic)",
  "gpt-5.4",
  "Very-Long-Model (ProviderNameThatExceedsTwentyFourCharactersLimitHere)",
  "  spaced   name  ( TeamA ) ",
];
const pickerResults = pickerSamples.map((s) => pickerApi.splitLabel(s));

// 5. 91-extension-api.js routes
const extApiSrc = fs.readFileSync(`${base}/91-extension-api.js`, "utf8");
const extCtx = vm.createContext({
  window: {},
  document: {},
  globalThis: {},
  codexPlusPageClass: "codex-plus-page",
  codexPlusRailSelector: ".codex-rail",
  codexPlusRailDestinationSelector: ".codex-rail-dest",
  actionGroupClass: "codex-session-actions",
  moreMenuClass: "codex-session-more-menu",
  codexPlusRegistry: {
    pages: new Map(),
    rowActions: new Map(),
    actions: new Map(),
    slots: new Map(),
    items: new Map(),
  },
  registerCodexPlusExtensionSelector: () => {},
  codexPlusExtensionItems: () => [],
  codexPlusSettings: () => ({}),
  Set,
  Map,
  Object,
  Array,
  String,
  Number,
  Date,
  Promise,
  Error,
});
const extApi = vm.runInContext(
  `(function() {\n${extApiSrc}\n;return { routes: [...codexPlusExtensionRoutes], coalesce: Object.fromEntries(codexPlusExtensionCoalesceWindowMs) };\n})()`,
  extCtx,
);

process.stdout.write(
  JSON.stringify({
    promptArgs: {
      splitResults,
      placeholderResults,
      expandResults,
      cmdMatchValid: promptCtx.__out.pattern.exec("  /prompts:fix  arg1 \"b c\" ".trim())?.slice(1, 3) ?? null,
      cmdMatchBare: promptCtx.__out.pattern.exec("/prompts:fix"),
    },
    statsig: {
      gateCatalog: statsigCtx.__out.gateCatalog(),
      userGates: statsigCtx.__out.userGates(),
      effectiveGates: statsigCtx.__out.effectiveGates(statsigCfg.gates, statsigCtx.__out.userGates()),
      merged1,
      merged2,
      signatureOn: statsigCtx.__out.sig(),
    },
    modelCatalog: {
      unique: catApi.uniqueValues(["a", "", "  ", "b", "a", null, 123, "c"]),
      settingsSource: catApi.codexModelSettingsSource(catalogCtx.codexPlusBackendSettings),
      settingsKey: catApi.codexModelSettingsKey(catalogCtx.codexPlusBackendSettings),
      identity: catApi.codexModelCatalogIdentity(catalogCtx.codexPlusBackendSettings),
      belongsTrue: catApi.catalogBelongsToProfile({ id: "p1" }),
      belongsFalse: catApi.catalogBelongsToProfile({ id: "p999" }),
      effortsGpt54: catApi.modelReasoningEfforts("gpt-5.4"),
      effortsCustomSmall: catApi.modelReasoningEfforts("custom-small"),
      effortsUnknown: catApi.modelReasoningEfforts("unknown-model"),
      descGpt54: catApi.buildDescriptor("gpt-5.4"),
      appliedChanged,
      appliedDesc,
      hostLocal: catApi.modelResponseHostId("", { hostId: "local" }),
      hostRemote: catApi.modelResponseHostId("local", { host_id: "ssh-1" }),
    },
    pickerResults,
    extApi,
  }),
);
