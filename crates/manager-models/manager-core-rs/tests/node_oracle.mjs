import fs from "node:fs";
import vm from "node:vm";

// Stub tối thiểu các global trình duyệt khi chạy trong Node CLI oracle.
const store = new Map();
globalThis.localStorage = {
  getItem: (k) => (store.has(k) ? store.get(k) : null),
  setItem: (k, v) => store.set(k, String(v)),
  removeItem: (k) => store.delete(k),
  clear: () => store.clear(),
};
globalThis.window = {
  location: { search: "" },
  addEventListener: () => {},
  removeEventListener: () => {},
  dispatchEvent: () => true,
  matchMedia: () => ({ matches: false, addEventListener: () => {}, removeEventListener: () => {} }),
  localStorage: globalThis.localStorage,
};
globalThis.document = {
  addEventListener: () => {},
  removeEventListener: () => {},
  documentElement: {
    classList: { toggle: () => {}, add: () => {}, remove: () => {} },
    setAttribute: () => {},
    style: {},
  },
};
Object.defineProperty(globalThis, "navigator", {
  value: { platform: "MacIntel", userAgent: "Mozilla/5.0" },
  configurable: true,
});
globalThis.CustomEvent = class CustomEvent {
  constructor(type, init) {
    this.type = type;
    this.detail = init?.detail;
  }
};

const srcBase = "file:///d:/Git/codex-plus/apps/codex-plus-manager/src";

const [
  presetsMod,
  i18nEnMod,
  i18nMod,
  mcpMod,
  vlmMod,
  colorsMod,
  skillScanMod,
  escapeMod,
  macTitlebarMod,
  brandIconsMod,
  settingsEventsMod,
  gatewayMod,
  metaSyncMod,
  requestFlowMod,
  hubMod,
  apiKeysMod,
  relayEditMod,
  updateMod,
  popoverMod,
  dreamMod,
] = await Promise.all([
  import(`${srcBase}/presets.ts`),
  import(`${srcBase}/i18n-en.ts`),
  import(`${srcBase}/i18n.ts`),
  import(`${srcBase}/mcp-presets.ts`),
  import(`${srcBase}/vlm-test-translation.ts`),
  import(`${srcBase}/flow-provider-colors.ts`),
  import(`${srcBase}/skill-source-scan.ts`),
  import(`${srcBase}/escape-layer.ts`),
  import(`${srcBase}/mac-titlebar.ts`),
  import(`${srcBase}/provider-brand-icons.ts`),
  import(`${srcBase}/settings-events.ts`),
  import(`${srcBase}/gateway-model.ts`),
  import(`${srcBase}/provider-meta-sync-model.ts`),
  import(`${srcBase}/request-flow-model.ts`),
  import(`${srcBase}/hub-overview-model.ts`),
  import(`${srcBase}/provider-api-keys.ts`),
  import(`${srcBase}/relay-profile-editing.ts`),
  import(`${srcBase}/update-notification.ts`),
  import(`${srcBase}/popover-layer.ts`),
  import(`${srcBase}/dream-skin.ts`),
]);

// Tải thêm bundle injected JS 42-provider-editing.generated.js qua vm.
const bundleCode = fs.readFileSync(
  "d:/Git/codex-plus/assets/inject/renderer-inject/42-provider-editing.generated.js",
  "utf8",
);
const vmCtx = {
  globalThis: null,
  window: null,
  self: null,
  console,
  Map,
  Set,
  Array,
  Object,
  Number,
  String,
  Boolean,
  Math,
  JSON,
  RegExp,
  Date,
  Error,
  TypeError,
  URL,
  URLSearchParams,
};
vmCtx.globalThis = vmCtx;
vmCtx.window = vmCtx;
vmCtx.self = vmCtx;
vm.runInNewContext(bundleCode, vmCtx);
const bundle = vmCtx.codexPlusProviderEditing;

const fixedNow = 1_700_000_000_000;
const origDateNow = Date.now;
Date.now = () => fixedNow;

try {
  // 1. i18n-en dictionary & pattern sample probes
  i18nMod.setLanguage("en");
  const plainKeys = Object.keys(i18nEnMod.EN_PLAIN);
  const templateKeys = Object.keys(i18nEnMod.EN_TEMPLATE);
  const backendKeys = Object.keys(i18nEnMod.EN_BACKEND);
  const patternCount = i18nEnMod.EN_BACKEND_PATTERNS.length;
  const backendSamples = [
    "启动 Codex 失败（code=1）：boom",
    "无法解析目录 C:\\work",
    "没有找到账号 foo",
    "未发现可用路径 /tmp/x",
    "HTTP 502 错误",
    "请求超时（30s）",
    "端口 8787 已被占用",
    "文件 /tmp/a.json 不存在",
  ];
  const backendTranslated = backendSamples.map((s) => i18nMod.t(s));
  i18nMod.setLanguage("zh");

  // 2. MCP presets & TOML bodies
  const mcpPresets = mcpMod.MCP_PRESETS;
  const mcpTomls = mcpPresets.map((p) => ({
    id: p.id,
    unix: p.tomlBody({ windows: false }),
    windows: p.tomlBody({ windows: true }),
  }));

  // 3. Static / DOM / callback probes
  const zhTr = (tmpl, args = []) =>
    tmpl.replace(/\{(\d+)\}/g, (m, i) => (args[Number(i)] === undefined ? m : String(args[Number(i)])));
  const vlmCases = [
    ["ok", undefined, 345],
    ["http_error", 401, 120],
    ["http_error", 404, 100],
    ["http_error", 500, 1850],
    ["timeout", undefined, 0],
  ].map(([status, httpCode, durationMs]) =>
    vlmMod.vlmTestTranslation(status, httpCode, durationMs, zhTr),
  );

  // flow-provider-colors
  const colorSlot1 = colorsMod.flowProviderSlot("p-alpha");
  const colorSlot2 = colorsMod.flowProviderSlot("p-beta");
  const colorSlot3 = colorsMod.flowProviderSlot("p-gamma");
  const colorSlot4 = colorsMod.flowProviderSlot("p-delta");
  const colorSlot1Again = colorsMod.flowProviderSlot("p-alpha");
  const colorSlotEmpty = colorsMod.flowProviderSlot("");

  // escape-layer
  let closedA = 0;
  let closedB = 0;
  const noop = () => {};
  const unregA = escapeMod.pushEscapeLayer(() => {
    closedA += 1;
  });
  const unregB = escapeMod.pushEscapeLayer(() => {
    closedB += 1;
  });
  const escHandled1 = escapeMod.handleEscapeKey({
    key: "Escape",
    isComposing: false,
    preventDefault: noop,
    stopImmediatePropagation: noop,
  });
  const escHandledComp = escapeMod.handleEscapeKey({
    key: "Escape",
    isComposing: true,
    preventDefault: noop,
    stopImmediatePropagation: noop,
  });
  unregB();
  const escHandled2 = escapeMod.handleEscapeKey({
    key: "Escape",
    isComposing: false,
    preventDefault: noop,
    stopImmediatePropagation: noop,
  });
  unregA();

  // mac-titlebar
  const macQueryOn = macTitlebarMod.usesMacOverlayTitlebar({
    location: { search: "?macTitlebar=1" },
    navigator: { platform: "Win32", userAgent: "" },
  });
  const macNoTauri = macTitlebarMod.usesMacOverlayTitlebar({
    location: { search: "" },
    navigator: { platform: "MacIntel", userAgent: "Mozilla/5.0" },
  });
  const macTauriDarwin = macTitlebarMod.usesMacOverlayTitlebar({
    __TAURI_INTERNALS__: {},
    location: { search: "" },
    navigator: { platform: "MacIntel", userAgent: "Mozilla/5.0" },
  });

  // js_only helpers
  const sampleGatewayPayload = {
    gateway: { autoModels: true },
    groups: [
      { id: "g1", disabled: false, hideMembers: true, members: [{ model: "gpt-4o-mini" }, { model: "group/x" }, { model: "claude-*" }] },
      { id: "g2", disabled: true, hideMembers: true, members: [{ model: "secret-disabled" }] },
    ],
    autoGroups: [
      { id: "ag1", hideMembers: true, hidden: false, members: [{ model: "Auto-Hidden-1" }] },
      { id: "ag2", hideMembers: true, hidden: true, members: [{ model: "ignored-hidden" }] },
    ],
    pool: [
      { id: "p1", name: "alpha@example.com", baseUrl: "https://a.test/v1", enabled: true, keyCount: 2 },
      { id: "p2", name: "disabled", baseUrl: "https://b.test/v1", enabled: false, keyCount: 1 },
    ],
    health: { providers: [], keys: [] },
    active: { relayId: "p1" },
    activity: {
      agents: [
        {
          agent: "codex",
          inFlight: [],
          last: { finishedAtMs: 1_700_000_002_000, relayId: "p1", model: "gpt-5" },
        },
      ],
    },
  };
  const unlisted = Array.from(gatewayMod.unlistedModels(sampleGatewayPayload)).sort();

  const metaPreview = {
    windows: [
      { model: "gpt-5", manual: false },
      { model: "o3", manual: true },
    ],
    reasoning: [
      { model: "gpt-5", manual: false },
      { model: "o1", manual: true },
    ],
  };
  const metaPickedWin = Array.from(metaSyncMod.defaultPicked(metaPreview, "windows")).sort();
  const metaPickedReason = Array.from(metaSyncMod.defaultPicked(metaPreview, "reasoning")).sort();

  const flowEntries = [
    { id: "e1", agent: "codex", status: "running", phase: "generating", relayId: "p1", timestampMs: 1_700_000_000_500 },
    { id: "e2", agent: "codex", status: "running", phase: "waiting", relayId: "p2", timestampMs: 1_700_000_000_900 },
    { id: "e3", agent: "codex", status: "done", relayId: "p1", timestampMs: 1_700_000_001_000 },
  ];
  const busy = Array.from(requestFlowMod.busyLinks(flowEntries)).sort();
  const returning = Array.from(requestFlowMod.returningLinks(flowEntries)).sort();
  const routeRecs = Object.fromEntries(hubMod.routeRecordsByEntry(sampleGatewayPayload, flowEntries));
  const hubProvs = hubMod.hubProviders(sampleGatewayPayload, new Map([["p1", 42]]), 1_700_000_000_000);

  const createdKey = apiKeysMod.createRelayApiKey([{ id: "key-loy3v6tc", name: "Existing" }]);
  relayEditMod.configureRelayProfileEditing({
    defaultRelayBaseUrl: () => "https://relay.default/v1",
    contextSelectionForAllEntries: () => ({ window: true }),
  });
  const createdProfile = relayEditMod.createRelayProfile({ relayProfiles: [{ id: "p0" }] });
  const baseRelay = relayEditMod.normalizeRelayProfile({
    ...createdProfile,
    id: "relay-1",
    name: "One",
    model: "gpt-5",
    relayMode: "official",
    apiKey: "k1",
    baseUrl: "https://u1.test/v1",
    upstreamBaseUrl: "https://u1.test/v1",
  });
  const createdAgg = relayEditMod.createAggregateRelayProfile({
    relayProfiles: [baseRelay],
  });
  const duplicated = relayEditMod.duplicateRelayProfile(
    {
      activeRelayId: "relay-1",
      relayProfiles: [baseRelay],
    },
    "relay-1",
  );

  // update-notification
  const updSkipCases = [
    ["0.12.0", "0.12.0"],
    ["v0.11.9", "0.12.0"],
    ["0.13.0", "0.12.0"],
    ["0.12.1", "v0.12.1-beta"],
    ["", "0.12.0"],
    ["0.12.0", ""],
  ].map(([cand, sk]) => updateMod.isVersionSkipped(cand, sk));

  // dream-skin
  Object.defineProperty(globalThis, "navigator", {
    value: { platform: "MacIntel", userAgent: "Mozilla/5.0 (Macintosh)" },
    configurable: true,
  });
  const dreamThemeMac = dreamMod.defaultDreamSkinTheme();
  Object.defineProperty(globalThis, "navigator", {
    value: { platform: "Win32", userAgent: "Mozilla/5.0 (Windows NT 10.0)" },
    configurable: true,
  });
  const dreamThemeWin = dreamMod.defaultDreamSkinTheme();

  // popover-layer
  popoverMod.resetOpenPopovers();
  popoverMod.registerOpenPopover("pop-a", () => {});
  popoverMod.registerOpenPopover("pop-b", () => {});
  const popCountAfterB = popoverMod.openPopoverCount();
  popoverMod.releaseOpenPopover("pop-b");
  const popCountAfterRelease = popoverMod.openPopoverCount();

  // bundle check
  const sampleProfile = {
    ...createdProfile,
    id: "relay-sample",
    name: "Sample",
    baseUrl: "https://api.example.com/v1",
    upstreamBaseUrl: "https://api.example.com/v1",
    apiKey: "sk-test-123",
    model: "gpt-5",
    protocol: "responses",
    relayMode: "third_party",
    sessionProvider: "custom",
    customHeaders: [{ name: "X-Custom", value: "abc" }],
  };
  const bundleConfigToml = bundle.buildRelayConfigToml(sampleProfile, {});
  const bundleAuthJson = bundle.buildRelayAuthJson(sampleProfile);
  const bundleNorm = bundle.normalizeRelayProfile(sampleProfile);
  const bundleDerived = bundle.deriveRelayProfileFromFiles(
    { ...sampleProfile, configContents: bundleConfigToml, authContents: bundleAuthJson },
    "config",
  );
  const bundleMask = bundle.gateway.maskAccountName("alice@example.com");

  const report = {
    presets: presetsMod.PRESETS,
    i18nCounts: {
      plain: plainKeys.length,
      template: templateKeys.length,
      backend: backendKeys.length,
      patterns: patternCount,
    },
    i18nPlain: i18nEnMod.EN_PLAIN,
    i18nTemplate: i18nEnMod.EN_TEMPLATE,
    i18nBackend: i18nEnMod.EN_BACKEND,
    backendSamples,
    backendTranslated,
    mcpPresets,
    mcpTomls,
    vlmCases,
    colorSlots: [colorSlot1, colorSlot2, colorSlot3, colorSlot4, colorSlot1Again, colorSlotEmpty],
    escapeChecks: { closedA, closedB, escHandled1, escHandledComp, escHandled2 },
    macChecks: {
      classConst: macTitlebarMod.MAC_TITLEBAR_CLASS,
      macQueryOn,
      macNoTauri,
      macTauriDarwin,
    },
    brandIconKeys: Object.keys(brandIconsMod.BRAND_ICON_URLS),
    settingsEventName: settingsEventsMod.BACKEND_SETTINGS_CHANGED_EVENT,
    unlisted,
    metaPickedWin,
    metaPickedReason,
    busy,
    returning,
    routeRecs,
    hubProvs,
    createdKey,
    createdProfile,
    createdAgg,
    duplicated,
    updSkipCases,
    dreamThemeMac,
    dreamThemeWin,
    popChecks: { popCountAfterB, popCountAfterRelease },
    skillRepoKeySample: skillScanMod.skillSourceRepoKey({
      owner: "anthropics",
      name: "skills",
      branch: "main",
      subdir: "sub",
    }),
    bundleChecks: {
      exportCount: Object.keys(bundle).length,
      configToml: bundleConfigToml,
      authJson: bundleAuthJson,
      normalized: bundleNorm,
      derived: bundleDerived,
      maskAccountName: bundleMask,
    },
  };

  process.stdout.write(JSON.stringify(report));
} finally {
  Date.now = origDateNow;
}
