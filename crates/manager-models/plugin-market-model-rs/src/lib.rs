//! JS bindings for `src/plugin-market-model.ts`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;
pub mod utf16;

/// `isPluginMarketSuccess`.
fn is_plugin_market_success_inner(status: String) -> Result<bool, String> {
    let out = port::is_plugin_market_success(status)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "isPluginMarketSuccess")]
pub fn is_plugin_market_success(status: String) -> Result<bool, JsError> {
    is_plugin_market_success_inner(status).map_err(|e| JsError::new(&e))
}

/// `mergePluginMarketCatalog`.
fn merge_plugin_market_catalog_inner(
    catalogs: String,
    source: String,
    result: String,
) -> Result<String, String> {
    let catalogs = crate::js::from_json(&catalogs)?;
    let result = crate::js::from_json(&result)?;
    let out = port::merge_plugin_market_catalog(catalogs, source, result)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "mergePluginMarketCatalog")]
pub fn merge_plugin_market_catalog(
    catalogs: String,
    source: String,
    result: String,
) -> Result<String, JsError> {
    merge_plugin_market_catalog_inner(catalogs, source, result).map_err(|e| JsError::new(&e))
}

/// `filterMarketPlugins`.
fn filter_market_plugins_inner(
    plugins: String,
    query: String,
    filter: String,
) -> Result<String, String> {
    let plugins = crate::js::from_json(&plugins)?;
    let out = port::filter_market_plugins(plugins, query, filter)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "filterMarketPlugins")]
pub fn filter_market_plugins(
    plugins: String,
    query: String,
    filter: String,
) -> Result<String, JsError> {
    filter_market_plugins_inner(plugins, query, filter).map_err(|e| JsError::new(&e))
}

/// `pluginMarketPage`.
fn plugin_market_page_inner(
    plugins: String,
    page: f64,
    page_size: Option<f64>,
) -> Result<String, String> {
    let plugins = crate::js::from_json(&plugins)?;
    let out = port::plugin_market_page(plugins, page, page_size)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "pluginMarketPage")]
pub fn plugin_market_page(
    plugins: String,
    page: f64,
    page_size: Option<f64>,
) -> Result<String, JsError> {
    plugin_market_page_inner(plugins, page, page_size).map_err(|e| JsError::new(&e))
}

/// `applyPluginInstallation`.
fn apply_plugin_installation_inner(
    catalogs: String,
    source: String,
    installed: String,
) -> Result<String, String> {
    let catalogs = crate::js::from_json(&catalogs)?;
    let installed = crate::js::from_json(&installed)?;
    let out = port::apply_plugin_installation(catalogs, source, installed)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "applyPluginInstallation")]
pub fn apply_plugin_installation(
    catalogs: String,
    source: String,
    installed: String,
) -> Result<String, JsError> {
    apply_plugin_installation_inner(catalogs, source, installed).map_err(|e| JsError::new(&e))
}

fn plugin_market_requests_new_inner() -> Result<port::PluginMarketRequests, String> {
    let out = port::PluginMarketRequests::new()?;
    Ok(out)
}

fn plugin_market_requests_activate_inner(
    inner: &mut port::PluginMarketRequests,
) -> Result<String, String> {
    let out = inner.activate()?;
    crate::js::to_json(&out)
}

fn plugin_market_requests_dispose_inner(
    inner: &mut port::PluginMarketRequests,
) -> Result<String, String> {
    let out = inner.dispose()?;
    crate::js::to_json(&out)
}

fn plugin_market_requests_begin_catalog_inner(
    inner: &mut port::PluginMarketRequests,
) -> Result<String, String> {
    let out = inner.begin_catalog()?;
    crate::js::to_json(&out)
}

fn plugin_market_requests_is_current_catalog_inner(
    inner: &mut port::PluginMarketRequests,
    sequence: f64,
) -> Result<bool, String> {
    let out = inner.is_current_catalog(sequence)?;
    Ok(out)
}

fn plugin_market_requests_invalidate_catalog_inner(
    inner: &mut port::PluginMarketRequests,
) -> Result<String, String> {
    let out = inner.invalidate_catalog()?;
    crate::js::to_json(&out)
}

fn plugin_market_requests_finish_catalog_inner(
    inner: &mut port::PluginMarketRequests,
    sequence: f64,
) -> Result<bool, String> {
    let out = inner.finish_catalog(sequence)?;
    Ok(out)
}

fn plugin_market_requests_begin_install_inner(
    inner: &mut port::PluginMarketRequests,
) -> Result<String, String> {
    let out = inner.begin_install()?;
    crate::js::to_json(&out)
}

fn plugin_market_requests_is_current_install_inner(
    inner: &mut port::PluginMarketRequests,
    sequence: f64,
) -> Result<bool, String> {
    let out = inner.is_current_install(sequence)?;
    Ok(out)
}

fn plugin_market_requests_finish_install_inner(
    inner: &mut port::PluginMarketRequests,
    sequence: f64,
) -> Result<bool, String> {
    let out = inner.finish_install(sequence)?;
    Ok(out)
}

fn plugin_market_requests_can_select_source_inner(
    inner: &mut port::PluginMarketRequests,
) -> Result<bool, String> {
    let out = inner.can_select_source()?;
    Ok(out)
}

/// Class `PluginMarketRequests`.
#[wasm_bindgen(js_name = "PluginMarketRequests")]
pub struct PluginMarketRequests {
    inner: port::PluginMarketRequests,
}

#[wasm_bindgen(js_class = "PluginMarketRequests")]
impl PluginMarketRequests {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<PluginMarketRequests, JsError> {
        let inner = plugin_market_requests_new_inner().map_err(|e| JsError::new(&e))?;
        Ok(Self { inner })
    }

    #[wasm_bindgen(js_name = "activate")]
    pub fn activate(&mut self) -> Result<String, JsError> {
        plugin_market_requests_activate_inner(&mut self.inner).map_err(|e| JsError::new(&e))
    }

    #[wasm_bindgen(js_name = "dispose")]
    pub fn dispose(&mut self) -> Result<String, JsError> {
        plugin_market_requests_dispose_inner(&mut self.inner).map_err(|e| JsError::new(&e))
    }

    #[wasm_bindgen(js_name = "beginCatalog")]
    pub fn begin_catalog(&mut self) -> Result<String, JsError> {
        plugin_market_requests_begin_catalog_inner(&mut self.inner).map_err(|e| JsError::new(&e))
    }

    #[wasm_bindgen(js_name = "isCurrentCatalog")]
    pub fn is_current_catalog(&mut self, sequence: f64) -> Result<bool, JsError> {
        plugin_market_requests_is_current_catalog_inner(&mut self.inner, sequence)
            .map_err(|e| JsError::new(&e))
    }

    #[wasm_bindgen(js_name = "invalidateCatalog")]
    pub fn invalidate_catalog(&mut self) -> Result<String, JsError> {
        plugin_market_requests_invalidate_catalog_inner(&mut self.inner)
            .map_err(|e| JsError::new(&e))
    }

    #[wasm_bindgen(js_name = "finishCatalog")]
    pub fn finish_catalog(&mut self, sequence: f64) -> Result<bool, JsError> {
        plugin_market_requests_finish_catalog_inner(&mut self.inner, sequence)
            .map_err(|e| JsError::new(&e))
    }

    #[wasm_bindgen(js_name = "beginInstall")]
    pub fn begin_install(&mut self) -> Result<String, JsError> {
        plugin_market_requests_begin_install_inner(&mut self.inner).map_err(|e| JsError::new(&e))
    }

    #[wasm_bindgen(js_name = "isCurrentInstall")]
    pub fn is_current_install(&mut self, sequence: f64) -> Result<bool, JsError> {
        plugin_market_requests_is_current_install_inner(&mut self.inner, sequence)
            .map_err(|e| JsError::new(&e))
    }

    #[wasm_bindgen(js_name = "finishInstall")]
    pub fn finish_install(&mut self, sequence: f64) -> Result<bool, JsError> {
        plugin_market_requests_finish_install_inner(&mut self.inner, sequence)
            .map_err(|e| JsError::new(&e))
    }

    #[wasm_bindgen(js_name = "canSelectSource")]
    pub fn can_select_source(&mut self) -> Result<bool, JsError> {
        plugin_market_requests_can_select_source_inner(&mut self.inner)
            .map_err(|e| JsError::new(&e))
    }
}

// ---- the binary wire ------------------------------------------------------------------------

thread_local! {
    static WIRE_IN: std::cell::RefCell<Vec<u8>> = const { std::cell::RefCell::new(Vec::new()) };
    static WIRE_OUT: std::cell::RefCell<Vec<u8>> = const { std::cell::RefCell::new(Vec::new()) };
}

/// This module's memory, where `index.cjs` reads and writes the wire buffers.
#[wasm_bindgen(js_name = "rustAnythingWireMemory")]
pub fn rust_anything_wire_memory() -> JsValue {
    wasm_bindgen::memory()
}

/// The address of an argument buffer of `len` bytes.
#[wasm_bindgen(js_name = "rustAnythingWireIn")]
pub fn rust_anything_wire_in(len: u32) -> u32 {
    WIRE_IN.with(|b| {
        let mut b = b.borrow_mut();
        b.resize(len as usize, 0);
        b.as_mut_ptr() as usize as u32
    })
}

/// The address of the last result.
#[wasm_bindgen(js_name = "rustAnythingWireOut")]
pub fn rust_anything_wire_out() -> u32 {
    WIRE_OUT.with(|b| b.borrow().as_ptr() as usize as u32)
}

/// Reads `len` argument bytes, runs `f`, and keeps its result for `rustAnythingWireOut`.
fn wire_io(
    len: u32,
    f: impl FnOnce(&mut wire::Reader, &mut wire::Writer) -> Result<(), String>,
) -> Result<u32, JsError> {
    let input = WIRE_IN.with(|b| std::mem::take(&mut *b.borrow_mut()));
    let mut w = wire::Writer::default();
    let result = f(
        &mut wire::Reader::new(&input[..(len as usize).min(input.len())]),
        &mut w,
    );
    WIRE_IN.with(|b| *b.borrow_mut() = input);
    match result {
        // A result of another shape than the TS type: JS runs the call through JSON.
        Err(e) if e.starts_with(wire::MISMATCH) => return Ok(u32::MAX),
        other => other.map_err(|e| JsError::new(&e))?,
    }
    let n = w.0.len();
    WIRE_OUT.with(|b| *b.borrow_mut() = w.0);
    Ok(u32::try_from(n).unwrap_or(u32::MAX))
}

static S_MERGE_PLUGIN_MARKET_CATALOG_0: wire::Schema = wire::Schema::Dyn;
static S_MERGE_PLUGIN_MARKET_CATALOG_2: wire::Schema = wire::Schema::Dyn;
static S_MERGE_PLUGIN_MARKET_CATALOG_OUT: wire::Schema = wire::Schema::Dyn;

/// `mergePluginMarketCatalog` over the binary wire.
#[wasm_bindgen(js_name = "mergePluginMarketCatalogWire")]
pub fn merge_plugin_market_catalog_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let catalogs: Value = wire::read_value(r, &S_MERGE_PLUGIN_MARKET_CATALOG_0)?;
        let source: String = r.string16()?;
        let result: Value = wire::read_value(r, &S_MERGE_PLUGIN_MARKET_CATALOG_2)?;
        let out = &port::merge_plugin_market_catalog(catalogs, source, result)?;
        wire::write_value(w, out, &S_MERGE_PLUGIN_MARKET_CATALOG_OUT)?;
        Ok(())
    })
}

static S_FILTER_MARKET_PLUGINS_0: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
    ("displayName", wire::Schema::Str16),
    ("description", wire::Schema::Str16),
    ("version", wire::Schema::Str16),
    ("author", wire::Schema::Str16),
    ("tags", wire::Schema::List(&wire::Schema::Str16)),
    ("license", wire::Schema::Str16),
    ("path", wire::Schema::Str16),
    ("tree", wire::Schema::Str16),
    ("bytes", wire::Schema::Num),
    ("skills", wire::Schema::Num),
    ("requiresAuth", wire::Schema::Bool),
    ("installed", wire::Schema::Bool),
    ("installedVersion", wire::Schema::Str16),
    ("updateAvailable", wire::Schema::Bool),
]));
static S_FILTER_MARKET_PLUGINS_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("id", wire::Schema::Str16),
    ("name", wire::Schema::Str16),
    ("displayName", wire::Schema::Str16),
    ("description", wire::Schema::Str16),
    ("version", wire::Schema::Str16),
    ("author", wire::Schema::Str16),
    ("tags", wire::Schema::List(&wire::Schema::Str16)),
    ("license", wire::Schema::Str16),
    ("path", wire::Schema::Str16),
    ("tree", wire::Schema::Str16),
    ("bytes", wire::Schema::Num),
    ("skills", wire::Schema::Num),
    ("requiresAuth", wire::Schema::Bool),
    ("installed", wire::Schema::Bool),
    ("installedVersion", wire::Schema::Str16),
    ("updateAvailable", wire::Schema::Bool),
]));

/// `filterMarketPlugins` over the binary wire.
#[wasm_bindgen(js_name = "filterMarketPluginsWire")]
pub fn filter_market_plugins_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let plugins: Vec<Value> =
            r.list(|r| wire::read_value(r, S_FILTER_MARKET_PLUGINS_0.inner()))?;
        let query: String = r.string16()?;
        let filter: String = r.string16()?;
        let out = &port::filter_market_plugins(plugins, query, filter)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_FILTER_MARKET_PLUGINS_OUT.inner())?;
        }
        Ok(())
    })
}

static S_PLUGIN_MARKET_PAGE_0: wire::Schema = wire::Schema::Dyn;
static S_PLUGIN_MARKET_PAGE_OUT: wire::Schema = wire::Schema::Dyn;

/// `pluginMarketPage` over the binary wire.
#[wasm_bindgen(js_name = "pluginMarketPageWire")]
pub fn plugin_market_page_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let plugins: Value = wire::read_value(r, &S_PLUGIN_MARKET_PAGE_0)?;
        let page: f64 = r.f64()?;
        let page_size: Option<f64> = r.opt_js(|r| r.f64())?;
        let out = &port::plugin_market_page(plugins, page, page_size)?;
        wire::write_value(w, out, &S_PLUGIN_MARKET_PAGE_OUT)?;
        Ok(())
    })
}

static S_APPLY_PLUGIN_INSTALLATION_0: wire::Schema = wire::Schema::Dyn;
static S_APPLY_PLUGIN_INSTALLATION_2: wire::Schema = wire::Schema::Dyn;
static S_APPLY_PLUGIN_INSTALLATION_OUT: wire::Schema = wire::Schema::Dyn;

/// `applyPluginInstallation` over the binary wire.
#[wasm_bindgen(js_name = "applyPluginInstallationWire")]
pub fn apply_plugin_installation_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let catalogs: Value = wire::read_value(r, &S_APPLY_PLUGIN_INSTALLATION_0)?;
        let source: String = r.string16()?;
        let installed: Value = wire::read_value(r, &S_APPLY_PLUGIN_INSTALLATION_2)?;
        let out = &port::apply_plugin_installation(catalogs, source, installed)?;
        wire::write_value(w, out, &S_APPLY_PLUGIN_INSTALLATION_OUT)?;
        Ok(())
    })
}
