// Thin JS bridge attaching `renderer-inject-rs` WASM exports for Codex WebView shards
// while preserving `window.codexPlus.constants`, `[data-codex-plus-ext]`, and DOM/V8 runtime shims.
export function installRendererInjectWasmBridge(wasmModule) {
  if (!wasmModule || typeof globalThis === "undefined") return null;
  const api = {
    promptArgsSplit: (text) => JSON.parse(wasmModule.promptArgsSplitJson(String(text ?? ""))),
    promptArgsExpand: (promptId, content, argText) =>
      JSON.parse(
        wasmModule.promptArgsExpandJson(
          String(promptId ?? ""),
          String(content ?? ""),
          String(argText ?? ""),
        ),
      ),
    statsigEffectiveGates: (defaults, backendSettings) =>
      JSON.parse(
        wasmModule.statsigEffectiveGatesJson(
          JSON.stringify(defaults ?? {}),
          JSON.stringify(backendSettings ?? {}),
        ),
      ),
    statsigMergeObject: (target, overrideValue) => {
      if (!target || typeof target !== "object" || Array.isArray(target)) return target;
      if (!overrideValue || typeof overrideValue !== "object" || Array.isArray(overrideValue)) {
        return target;
      }
      const hasObjectValue =
        (target.value && typeof target.value === "object" && !Array.isArray(target.value)) ||
        (target.__value && typeof target.__value === "object" && !Array.isArray(target.__value));
      if (!hasObjectValue) return target;
      const merged = JSON.parse(
        wasmModule.statsigMergeObjectJson(
          JSON.stringify({
            value: target.value,
            __value: target.__value,
            __evaluation: target.__evaluation,
          }),
          JSON.stringify(overrideValue),
        ),
      );
      try {
        Object.assign(target, merged);
      } catch {
        return target;
      }
      if (typeof target.get === "function") {
        const originalGet =
          typeof target.__codexPlusStatsigOriginalGet === "function"
            ? target.__codexPlusStatsigOriginalGet
            : target.get.bind(target);
        const next = merged && merged.value && typeof merged.value === "object" ? merged.value : null;
        if (next) {
          try {
            Object.defineProperty(target, "__codexPlusStatsigOriginalGet", {
              value: originalGet,
              configurable: true,
              writable: true,
              enumerable: false,
            });
            target.get = (key, fallback) =>
              Object.prototype.hasOwnProperty.call(next, key) ? next[key] : originalGet(key, fallback);
          } catch {}
        }
      }
      return target;
    },
    modelCatalogIdentity: (settings) =>
      wasmModule.modelCatalogIdentityJson(JSON.stringify(settings ?? {})),
    buildModelDescriptor: (catalog, modelName) =>
      JSON.parse(
        wasmModule.buildModelDescriptorJson(
          JSON.stringify(catalog ?? {}),
          String(modelName ?? ""),
        ),
      ),
    applyModelMetadata: (descriptor, rawCatalog, modelName) => {
      if (!descriptor || typeof descriptor !== "object" || Array.isArray(descriptor)) return false;
      const res = JSON.parse(
        wasmModule.applyModelMetadataJson(
          JSON.stringify(descriptor),
          JSON.stringify(rawCatalog ?? {}),
          String(modelName ?? ""),
        ),
      );
      if (res && res.changed && res.descriptor && typeof res.descriptor === "object") {
        Object.assign(descriptor, res.descriptor);
      }
      return Boolean(res && res.changed);
    },
    parsePickerGroupSuffix: (label) =>
      JSON.parse(wasmModule.parsePickerGroupSuffixJson(String(label ?? ""))),
    isExtensionRouteAllowed: (route) =>
      Boolean(wasmModule.isExtensionRouteAllowed(String(route ?? ""))),
    escapeHtml: (raw) => wasmModule.escapeHtml(String(raw ?? "")),
    escapeCssIdent: (raw) => wasmModule.escapeCssIdent(String(raw ?? "")),
    sanitizeSvgIcon: (svg) => wasmModule.sanitizeSvgIcon(String(svg ?? "")) ?? null,
    sanitizeExternalHttpUrl: (raw) => wasmModule.sanitizeExternalHttpUrl(String(raw ?? "")) ?? null,
  };
  globalThis.__codexPlusWasmInject = api;
  return api;
}
