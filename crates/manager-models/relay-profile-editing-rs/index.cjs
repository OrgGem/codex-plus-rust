// Loads the wasm-bindgen (nodejs target) output built by `rust-anything build`.
const native = require('./pkg/relay_profile_editing_rs.js')

// Structured values cross as JSON text. JSON has no `undefined`, NaN, -0, bigint, Date, Map, Set
// or bytes, and Rust strings no lone surrogates: those travel as one-key marker objects
// (`{"$undefined": true}`, `{"$num": "NaN"}`, `{"$map": [[k, v]]}`, `{"$bytes": [...]}`, ...),
// the same markers recordings use.
const replacer = function (k, v) {
  const raw = this[k]
  if (raw instanceof Date) return { $date: Number.isNaN(raw.getTime()) ? 'Invalid' : raw.toISOString() }
  if (typeof Buffer !== 'undefined' && Buffer.isBuffer(raw)) return { $buffer: Array.from(raw) }
  if (raw instanceof Uint8Array) return { $bytes: Array.from(raw) }
  if (raw instanceof Map) return { $map: [...raw] }
  if (raw instanceof Set) return { $set: [...raw] }
  return v === undefined ? { $undefined: true }
    : typeof v === 'bigint' ? { $bigint: String(v) }
    : typeof v === 'number' ? (Number.isFinite(v) && !Object.is(v, -0) ? v : { $num: Object.is(v, -0) ? '-0' : String(v) })
    : typeof v === 'string' && !v.isWellFormed() ? { $utf16: Array.from({ length: v.length }, (_, i) => v.charCodeAt(i)) }
    : v
}
const encode = v => JSON.stringify(v, replacer)
const single = (v, key) => Object.keys(v).length === 1 && key in v
const revive = v => Array.isArray(v) ? v.map(revive)
  : v !== null && typeof v === 'object'
    ? (single(v, '$undefined') ? undefined
      : single(v, '$utf16') ? fromUnits(v.$utf16)
      : single(v, '$num') ? Number(v.$num)
      : single(v, '$bigint') ? BigInt(v.$bigint)
      : single(v, '$date') ? new Date(v.$date)
      : single(v, '$bytes') ? Uint8Array.from(v.$bytes)
      : single(v, '$buffer') ? Buffer.from(v.$buffer)
      : single(v, '$map') ? new Map(v.$map.map(([k, x]) => [revive(k), revive(x)]))
      : single(v, '$set') ? new Set(v.$set.map(revive))
      : Object.fromEntries(Object.entries(v).map(([k, x]) => [k, revive(x)])))
    : v
// Most results carry no marker: skip the walk unless the text has one.
const decode = text => {
  const value = JSON.parse(text)
  return text.includes('"$') ? revive(value) : value
}
const fromUnits = (units) => {
  let out = ''
  for (let i = 0; i < units.length; i += 8192)
    out += String.fromCharCode.apply(null, Array.prototype.slice.call(units, i, i + 8192))
  return out
}

module.exports = { ...native }
const specs = {
  withGeneratedRelayFiles: [[1], 'json', false],
  buildRelayConfigToml: [[1, 1], 'raw', false],
  buildRelayAuthJson: [[1], 'raw', false],
  deriveRelayProfileFromFiles: [[1, 1], 'json', false],
  applyRelayProfilePatchToFiles: [[1, 1, 1], 'json', false],
  parseModelSuffix: [[0], 'json', false],
  tomlStructureMask: [[0], 'json', false],
  tomlSectionName: [[0], 'json', false],
  tomlQuotedValue: [[0, 0], 'json', false],
  tomlDottedKeyPath: [[0], 'json', false],
  tomlKeyAssignment: [[0], 'json', false],
  tomlStringAssignmentValue: [[0, 0], 'json', false],
  setCodexProviderStringKey: [[0, 0, 0, 1], 'raw', false],
  setCodexExperimentalBearerToken: [[0, 0, 1], 'raw', false],
  ensureCodexProviderDefaults: [[0, 0, 1], 'raw', false],
  relayProfileSwitchValidation: [[1, 1], 'json', false],
  relayModelRoutesValidation: [[1, 1], 'json', false],
  relayModelRoutesSettingsValidation: [[1], 'json', false],
  relaySettingsValidation: [[1], 'json', false],
  relaySessionProviderValidation: [[1], 'json', false],
  relayModelRouteIssueMessage: [[1], 'json', false],
  relaySettingsWithDraft: [[1, 0, 1, 0], 'json', false],
  relayProfileUsesLiveFiles: [[1], 'raw', false],
  syncLegacyRelayFields: [[1], 'json', false],
  normalizeAggregateProfilesFromRelayProfiles: [[1], 'json', false],
  updateRelayProfile: [[1, 0, 1], 'json', false],
  addRelayProfile: [[1, 1], 'json', false],
  reorderRelayProfiles: [[1, 0, 0], 'json', false],
  removeRelayProfile: [[1, 0], 'json', false],
  relayProfileDeleteIssue: [[1, 0], 'json', false],
  isAggregateRelayProfile: [[1], 'raw', false],
  relayProviderView: [[1], 'raw', false],
  normalizeAggregateRelayProfile: [[1, 1], 'json', false],
  aggregateProfileForMode: [[1, 1, 1], 'json', false],
  normalizeAggregateConfig: [[1, 1, 1], 'json', false],
  aggregateStrategyLabel: [[1], 'raw', false],
  aggregateStrategyHelp: [[1], 'raw', false],
  aggregateRelayProfileValidation: [[1], 'json', false],
  splitTomlRootAndTables: [[0], 'json', false],
  normalizeRelaySessionProvider: [[1], 'json', false],
  relaySessionProviderFromConfig: [[0], 'json', false],
  relaySessionProvider: [[1], 'json', false],
  activeRelayProfile: [[1], 'json', false],
  emptyContextSelection: [[], 'json', false],
  normalizeContextSelection: [[1, 1], 'json', false],
  normalizeRelayMode: [[1], 'json', false],
  normalizeRelayProtocol: [[1], 'json', false],
  normalizeAnthropicAuthHeader: [[1], 'raw', false],
  normalizeAnthropicThinkingMode: [[1], 'raw', false],
  normalizeAnthropicMaxTokens: [[1], 'raw', false],
  hydrateAggregateRelayProfile: [[1, 1], 'json', false],
  normalizeRelayProfile: [[1, 1], 'json', false],
}
const classes = {

}
const convert = result => result === 'json' ? decode : result === 'units' ? fromUnits : x => x
// An omitted structured argument is `undefined`, not a missing value.
const prepare = (a, structured) => Array.from({ length: Math.max(a.length, structured.length) }, (_, i) => structured[i] ? encode(a[i]) : a[i])
// Functions: [structured arguments, result kind, async].
for (const [name, [structured, result, isAsync]] of Object.entries(specs)) {
  const call = native[name]
  const out = convert(result)
  module.exports[name] = isAsync
    ? async (...a) => out(await call(...prepare(a, structured)))
    : (...a) => out(call(...prepare(a, structured)))
}
// Classes: a JS class around the native one, converting arguments and results.
const INNER = Symbol('native')
for (const [name, [ctor, methods]] of Object.entries(classes)) {
  const Native = native[name]
  const Wrapped = { [name]: class { constructor(...a) { this[INNER] = new Native(...prepare(a, ctor)) } } }[name]
  for (const [method, [structured, result]] of Object.entries(methods)) {
    const out = convert(result)
    Wrapped.prototype[method] = function (...a) { return out(this[INNER][method](...prepare(a, structured))) }
  }
  module.exports[name] = Wrapped
}

// The binary wire (see rust-anything's `wire.rs`): arguments are written into `wIn` and copied into
// wasm memory in one step; results are read where the Rust side left them.
let wIn = new Uint8Array(1 << 16), iv = new DataView(wIn.buffer), at = 0, p = 0
const mem = native.rustAnythingWireMemory()
let wOut = new Uint8Array(mem.buffer), ov = new DataView(mem.buffer)
const regrow = () => {}
// Growing wasm memory replaces its buffer: views are made again when that happened.
const views = () => {
  if (wOut.buffer !== mem.buffer) {
    wOut = new Uint8Array(mem.buffer)
    ov = new DataView(mem.buffer)
  }
}
const wireCall = fn => {
  const ptr = native.rustAnythingWireIn(at)
  views()
  wOut.set(wIn.subarray(0, at), ptr)
  if (fn(at) === 0xFFFFFFFF) return false
  views()
  p = native.rustAnythingWireOut()
  return true
}

const need = n => {
  if (at + n <= wIn.length) return
  const grown = new Uint8Array(Math.max(wIn.length * 2, at + n))
  grown.set(wIn)
  wIn = grown
  iv = new DataView(wIn.buffer)
  regrow()
}
const wU8 = v => { need(1); iv.setUint8(at++, v) }
const wU32 = v => { need(4); iv.setUint32(at, v, true); at += 4 }
const wF64 = v => { need(8); iv.setFloat64(at, v, true); at += 8 }
const wUnits = s => {
  need(4 + 2 * s.length)
  iv.setUint32(at, s.length, true)
  at += 4
  for (let i = 0; i < s.length; i++) { iv.setUint16(at, s.charCodeAt(i), true); at += 2 }
}
const wStr = s => { wU8(4); wUnits(s) }
const wDyn = v => {
  switch (typeof v) {
    case 'undefined': wU8(0); return true
    case 'boolean': wU8(v ? 3 : 2); return true
    case 'number': wU8(4); wF64(v); return true
    case 'string': wU8(5); wUnits(v); return true
    case 'object':
      if (v === null) { wU8(1); return true }
      if (Array.isArray(v)) {
        wU8(6); wU32(v.length)
        for (let i = 0; i < v.length; i++) if (!wDyn(v[i])) return false
        return true
      }
      if (Object.getPrototypeOf(v) !== Object.prototype && Object.getPrototypeOf(v) !== null) return false
      {
        const keys = Object.keys(v)
        wU8(7); wU32(keys.length)
        for (const k of keys) { wUnits(k); if (!wDyn(v[k])) return false }
        return true
      }
  }
  return false
}
const rU8 = () => ov.getUint8(p++)
const rU32 = () => { const v = ov.getUint32(p, true); p += 4; return v }
const rF64 = () => { const v = ov.getFloat64(p, true); p += 8; return v }
// ignoreBOM: a leading U+FEFF is part of the string, not a byte-order mark.
const utf16 = new TextDecoder('utf-16le', { ignoreBOM: true })
const rUnits = (wellFormed) => {
  const n = rU32()
  let s
  if (n > 24 && wellFormed) s = utf16.decode(wOut.subarray(p, p + 2 * n))
  else {
    s = ''
    for (let i = 0; i < n; i++) s += String.fromCharCode(ov.getUint16(p + 2 * i, true))
  }
  p += 2 * n
  return s
}
// Tag 1: a string from Rust (well-formed); 4: raw code units, lone surrogates included.
const rStr = () => rUnits(rU8() === 1)
const rDyn = () => {
  switch (rU8()) {
    case 0: return undefined
    case 1: return null
    case 2: return false
    case 3: return true
    case 4: return rF64()
    case 5: return rUnits(false)
    case 6: { const n = rU32(); const a = new Array(n); for (let i = 0; i < n; i++) a[i] = rDyn(); return a }
    case 7: { const n = rU32(); const o = {}; for (let i = 0; i < n; i++) { const k = rUnits(false); o[k] = rDyn() } return o }
  }
}

// `withGeneratedRelayFiles`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.withGeneratedRelayFiles
const call = native.withGeneratedRelayFilesWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
return true
}
const decode = () => {
let t
t = rDyn()
return t
}
module.exports.withGeneratedRelayFiles = function withGeneratedRelayFiles (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `buildRelayConfigToml`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.buildRelayConfigToml
const call = native.buildRelayConfigTomlWire
const encode = (a0, a1) => {
at = 0
if (!wDyn(a0)) return false
if (!wDyn(a1)) return false
return true
}
const decode = () => {
let t
t = rStr()
return t
}
module.exports.buildRelayConfigToml = function buildRelayConfigToml (a0, a1) {
  try {
    if (arguments.length <= 2 && encode(a0, a1) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `buildRelayAuthJson`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.buildRelayAuthJson
const call = native.buildRelayAuthJsonWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
return true
}
const decode = () => {
let t
t = rStr()
return t
}
module.exports.buildRelayAuthJson = function buildRelayAuthJson (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `deriveRelayProfileFromFiles`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.deriveRelayProfileFromFiles
const call = native.deriveRelayProfileFromFilesWire
const encode = (a0, a1) => {
at = 0
if (!wDyn(a0)) return false
if (!wDyn(a1)) return false
return true
}
const decode = () => {
let t
t = rDyn()
return t
}
module.exports.deriveRelayProfileFromFiles = function deriveRelayProfileFromFiles (a0, a1) {
  try {
    if (arguments.length <= 2 && encode(a0, a1) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `applyRelayProfilePatchToFiles`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.applyRelayProfilePatchToFiles
const call = native.applyRelayProfilePatchToFilesWire
const encode = (a0, a1, a2) => {
at = 0
if (!wDyn(a0)) return false
if (!wDyn(a1)) return false
if (!wDyn(a2)) return false
return true
}
const decode = () => {
let t
t = rDyn()
return t
}
module.exports.applyRelayProfilePatchToFiles = function applyRelayProfilePatchToFiles (a0, a1, a2) {
  try {
    if (arguments.length <= 3 && encode(a0, a1, a2) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `parseModelSuffix`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.parseModelSuffix
const call = native.parseModelSuffixWire
const encode = (a0) => {
at = 0
if (typeof a0 !== 'string') return false
wStr(a0)
return true
}
const decode = () => {
let t
{
const o0 = {}
const n0 = rU8()
for (let j0 = 0; j0 < n0; j0++) {
switch (rU8()) {
case 0: {
let v0
v0 = rStr()
o0["slug"] = v0
break
}
case 1: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
v0 = rF64()
}
}
o0["window"] = v0
break
}
default: {
const k0 = rStr()
o0[k0] = rDyn()
}
}
}
t = o0
}
return t
}
module.exports.parseModelSuffix = function parseModelSuffix (a0) {
  if (illFormed(a0)) throw new TypeError('argument 1 of parseModelSuffix has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `tomlStructureMask`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.tomlStructureMask
const call = native.tomlStructureMaskWire
const encode = (a0) => {
at = 0
if (!Array.isArray(a0)) return false
wU8(1)
wU32(a0.length)
for (let i0 = 0; i0 < a0.length; i0++) {
const v0 = a0[i0]
if (typeof v0 !== 'string') return false
wStr(v0)
}
return true
}
const decode = () => {
let t
if (rU8() === 0) t = null
else {
const n0 = rU32()
const a0 = new Array(n0)
for (let i0 = 0; i0 < n0; i0++) {
let v0
v0 = rU8() === 1
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.tomlStructureMask = function tomlStructureMask (a0) {
  if (illFormed(a0)) throw new TypeError('argument 1 of tomlStructureMask has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `tomlSectionName`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.tomlSectionName
const call = native.tomlSectionNameWire
const encode = (a0) => {
at = 0
if (typeof a0 !== 'string') return false
wStr(a0)
return true
}
const decode = () => {
let t
{
const g0 = rU8()
if (g0 === 0) t = undefined
else if (g0 === 2) t = null
else {
t = rStr()
}
}
return t
}
module.exports.tomlSectionName = function tomlSectionName (a0) {
  if (illFormed(a0)) throw new TypeError('argument 1 of tomlSectionName has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `tomlQuotedValue`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.tomlQuotedValue
const call = native.tomlQuotedValueWire
const encode = (a0, a1) => {
at = 0
if (typeof a0 !== 'string') return false
wStr(a0)
if (typeof a1 !== 'number') return false
wF64(a1)
return true
}
const decode = () => {
let t
{
const g0 = rU8()
if (g0 === 0) t = undefined
else if (g0 === 2) t = null
else {
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["value"] = v1
break
}
case 1: {
let v1
v1 = rF64()
o1["end"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
t = o1
}
}
}
return t
}
module.exports.tomlQuotedValue = function tomlQuotedValue (a0, a1) {
  if (illFormed(a0)) throw new TypeError('argument 1 of tomlQuotedValue has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 2 && encode(a0, a1) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `tomlDottedKeyPath`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.tomlDottedKeyPath
const call = native.tomlDottedKeyPathWire
const encode = (a0) => {
at = 0
if (typeof a0 !== 'string') return false
wStr(a0)
return true
}
const decode = () => {
let t
{
const g0 = rU8()
if (g0 === 0) t = undefined
else if (g0 === 2) t = null
else {
if (rU8() === 0) t = null
else {
const n1 = rU32()
const a1 = new Array(n1)
for (let i1 = 0; i1 < n1; i1++) {
let v1
v1 = rStr()
a1[i1] = v1
}
t = a1
}
}
}
return t
}
module.exports.tomlDottedKeyPath = function tomlDottedKeyPath (a0) {
  if (illFormed(a0)) throw new TypeError('argument 1 of tomlDottedKeyPath has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `tomlKeyAssignment`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.tomlKeyAssignment
const call = native.tomlKeyAssignmentWire
const encode = (a0) => {
at = 0
if (typeof a0 !== 'string') return false
wStr(a0)
return true
}
const decode = () => {
let t
{
const g0 = rU8()
if (g0 === 0) t = undefined
else if (g0 === 2) t = null
else {
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
if (rU8() === 0) v1 = null
else {
const n2 = rU32()
const a2 = new Array(n2)
for (let i2 = 0; i2 < n2; i2++) {
let v2
v2 = rStr()
a2[i2] = v2
}
v1 = a2
}
o1["path"] = v1
break
}
case 1: {
let v1
v1 = rF64()
o1["valueStart"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
t = o1
}
}
}
return t
}
module.exports.tomlKeyAssignment = function tomlKeyAssignment (a0) {
  if (illFormed(a0)) throw new TypeError('argument 1 of tomlKeyAssignment has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `tomlStringAssignmentValue`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.tomlStringAssignmentValue
const call = native.tomlStringAssignmentValueWire
const encode = (a0, a1) => {
at = 0
if (typeof a0 !== 'string') return false
wStr(a0)
if (typeof a1 !== 'string') return false
wStr(a1)
return true
}
const decode = () => {
let t
{
const g0 = rU8()
if (g0 === 0) t = undefined
else if (g0 === 2) t = null
else {
t = rStr()
}
}
return t
}
module.exports.tomlStringAssignmentValue = function tomlStringAssignmentValue (a0, a1) {
  if (illFormed(a0)) throw new TypeError('argument 1 of tomlStringAssignmentValue has a lone surrogate, which a Rust String cannot hold')
  if (illFormed(a1)) throw new TypeError('argument 2 of tomlStringAssignmentValue has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 2 && encode(a0, a1) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `setCodexProviderStringKey`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.setCodexProviderStringKey
const call = native.setCodexProviderStringKeyWire
const encode = (a0, a1, a2, a3) => {
at = 0
if (typeof a0 !== 'string') return false
wStr(a0)
if (typeof a1 !== 'string') return false
wStr(a1)
if (typeof a2 !== 'string') return false
wStr(a2)
if (!wDyn(a3)) return false
return true
}
const decode = () => {
let t
t = rStr()
return t
}
module.exports.setCodexProviderStringKey = function setCodexProviderStringKey (a0, a1, a2, a3) {
  if (illFormed(a0)) throw new TypeError('argument 1 of setCodexProviderStringKey has a lone surrogate, which a Rust String cannot hold')
  if (illFormed(a1)) throw new TypeError('argument 2 of setCodexProviderStringKey has a lone surrogate, which a Rust String cannot hold')
  if (illFormed(a2)) throw new TypeError('argument 3 of setCodexProviderStringKey has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 4 && encode(a0, a1, a2, a3) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `setCodexExperimentalBearerToken`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.setCodexExperimentalBearerToken
const call = native.setCodexExperimentalBearerTokenWire
const encode = (a0, a1, a2) => {
at = 0
if (typeof a0 !== 'string') return false
wStr(a0)
if (typeof a1 !== 'string') return false
wStr(a1)
if (!wDyn(a2)) return false
return true
}
const decode = () => {
let t
t = rStr()
return t
}
module.exports.setCodexExperimentalBearerToken = function setCodexExperimentalBearerToken (a0, a1, a2) {
  if (illFormed(a0)) throw new TypeError('argument 1 of setCodexExperimentalBearerToken has a lone surrogate, which a Rust String cannot hold')
  if (illFormed(a1)) throw new TypeError('argument 2 of setCodexExperimentalBearerToken has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 3 && encode(a0, a1, a2) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `ensureCodexProviderDefaults`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.ensureCodexProviderDefaults
const call = native.ensureCodexProviderDefaultsWire
const encode = (a0, a1, a2) => {
at = 0
if (typeof a0 !== 'string') return false
wStr(a0)
if (typeof a1 !== 'string') return false
wStr(a1)
if (!wDyn(a2)) return false
return true
}
const decode = () => {
let t
t = rStr()
return t
}
module.exports.ensureCodexProviderDefaults = function ensureCodexProviderDefaults (a0, a1, a2) {
  if (illFormed(a0)) throw new TypeError('argument 1 of ensureCodexProviderDefaults has a lone surrogate, which a Rust String cannot hold')
  if (illFormed(a1)) throw new TypeError('argument 2 of ensureCodexProviderDefaults has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 3 && encode(a0, a1, a2) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `relayProfileSwitchValidation`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.relayProfileSwitchValidation
const call = native.relayProfileSwitchValidationWire
const encode = (a0, a1) => {
at = 0
if (!wDyn(a0)) return false
if (!wDyn(a1)) return false
return true
}
const decode = () => {
let t
{
const g0 = rU8()
if (g0 === 0) t = undefined
else if (g0 === 2) t = null
else {
t = rStr()
}
}
return t
}
module.exports.relayProfileSwitchValidation = function relayProfileSwitchValidation (a0, a1) {
  try {
    if (arguments.length <= 2 && encode(a0, a1) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `relayModelRoutesValidation`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.relayModelRoutesValidation
const call = native.relayModelRoutesValidationWire
const encode = (a0, a1) => {
at = 0
if (!wDyn(a0)) return false
if (!wDyn(a1)) return false
return true
}
const decode = () => {
let t
{
const g0 = rU8()
if (g0 === 0) t = undefined
else if (g0 === 2) t = null
else {
t = rStr()
}
}
return t
}
module.exports.relayModelRoutesValidation = function relayModelRoutesValidation (a0, a1) {
  try {
    if (arguments.length <= 2 && encode(a0, a1) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `relayModelRoutesSettingsValidation`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.relayModelRoutesSettingsValidation
const call = native.relayModelRoutesSettingsValidationWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
return true
}
const decode = () => {
let t
{
const g0 = rU8()
if (g0 === 0) t = undefined
else if (g0 === 2) t = null
else {
t = rStr()
}
}
return t
}
module.exports.relayModelRoutesSettingsValidation = function relayModelRoutesSettingsValidation (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `relaySettingsValidation`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.relaySettingsValidation
const call = native.relaySettingsValidationWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
return true
}
const decode = () => {
let t
{
const g0 = rU8()
if (g0 === 0) t = undefined
else if (g0 === 2) t = null
else {
t = rStr()
}
}
return t
}
module.exports.relaySettingsValidation = function relaySettingsValidation (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `relaySessionProviderValidation`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.relaySessionProviderValidation
const call = native.relaySessionProviderValidationWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
return true
}
const decode = () => {
let t
{
const g0 = rU8()
if (g0 === 0) t = undefined
else if (g0 === 2) t = null
else {
t = rStr()
}
}
return t
}
module.exports.relaySessionProviderValidation = function relaySessionProviderValidation (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `relayModelRouteIssueMessage`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.relayModelRouteIssueMessage
const call = native.relayModelRouteIssueMessageWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
return true
}
const decode = () => {
let t
{
const g0 = rU8()
if (g0 === 0) t = undefined
else if (g0 === 2) t = null
else {
t = rStr()
}
}
return t
}
module.exports.relayModelRouteIssueMessage = function relayModelRouteIssueMessage (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `relaySettingsWithDraft`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.relaySettingsWithDraft
const call = native.relaySettingsWithDraftWire
const encode = (a0, a1, a2, a3) => {
at = 0
if (!wDyn(a0)) return false
if (typeof a1 !== 'string') return false
wStr(a1)
if (!wDyn(a2)) return false
if (typeof a3 !== 'boolean') return false
wU8(a3 ? 1 : 0)
return true
}
const decode = () => {
let t
t = rDyn()
return t
}
module.exports.relaySettingsWithDraft = function relaySettingsWithDraft (a0, a1, a2, a3) {
  if (illFormed(a1)) throw new TypeError('argument 2 of relaySettingsWithDraft has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 4 && encode(a0, a1, a2, a3) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `relayProfileUsesLiveFiles`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.relayProfileUsesLiveFiles
const call = native.relayProfileUsesLiveFilesWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.relayProfileUsesLiveFiles = function relayProfileUsesLiveFiles (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `syncLegacyRelayFields`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.syncLegacyRelayFields
const call = native.syncLegacyRelayFieldsWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
return true
}
const decode = () => {
let t
t = rDyn()
return t
}
module.exports.syncLegacyRelayFields = function syncLegacyRelayFields (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `normalizeAggregateProfilesFromRelayProfiles`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.normalizeAggregateProfilesFromRelayProfiles
const call = native.normalizeAggregateProfilesFromRelayProfilesWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
return true
}
const decode = () => {
let t
if (rU8() === 0) t = null
else {
const n0 = rU32()
const a0 = new Array(n0)
for (let i0 = 0; i0 < n0; i0++) {
let v0
v0 = rDyn()
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.normalizeAggregateProfilesFromRelayProfiles = function normalizeAggregateProfilesFromRelayProfiles (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `updateRelayProfile`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.updateRelayProfile
const call = native.updateRelayProfileWire
const encode = (a0, a1, a2) => {
at = 0
if (!wDyn(a0)) return false
if (typeof a1 !== 'string') return false
wStr(a1)
if (!wDyn(a2)) return false
return true
}
const decode = () => {
let t
t = rDyn()
return t
}
module.exports.updateRelayProfile = function updateRelayProfile (a0, a1, a2) {
  if (illFormed(a1)) throw new TypeError('argument 2 of updateRelayProfile has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 3 && encode(a0, a1, a2) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `addRelayProfile`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.addRelayProfile
const call = native.addRelayProfileWire
const encode = (a0, a1) => {
at = 0
if (!wDyn(a0)) return false
if (!wDyn(a1)) return false
return true
}
const decode = () => {
let t
t = rDyn()
return t
}
module.exports.addRelayProfile = function addRelayProfile (a0, a1) {
  try {
    if (arguments.length <= 2 && encode(a0, a1) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `reorderRelayProfiles`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.reorderRelayProfiles
const call = native.reorderRelayProfilesWire
const encode = (a0, a1, a2) => {
at = 0
if (!wDyn(a0)) return false
if (typeof a1 !== 'string') return false
wStr(a1)
if (typeof a2 !== 'string') return false
wStr(a2)
return true
}
const decode = () => {
let t
t = rDyn()
return t
}
module.exports.reorderRelayProfiles = function reorderRelayProfiles (a0, a1, a2) {
  if (illFormed(a1)) throw new TypeError('argument 2 of reorderRelayProfiles has a lone surrogate, which a Rust String cannot hold')
  if (illFormed(a2)) throw new TypeError('argument 3 of reorderRelayProfiles has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 3 && encode(a0, a1, a2) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `removeRelayProfile`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.removeRelayProfile
const call = native.removeRelayProfileWire
const encode = (a0, a1) => {
at = 0
if (!wDyn(a0)) return false
if (typeof a1 !== 'string') return false
wStr(a1)
return true
}
const decode = () => {
let t
t = rDyn()
return t
}
module.exports.removeRelayProfile = function removeRelayProfile (a0, a1) {
  if (illFormed(a1)) throw new TypeError('argument 2 of removeRelayProfile has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 2 && encode(a0, a1) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `relayProfileDeleteIssue`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.relayProfileDeleteIssue
const call = native.relayProfileDeleteIssueWire
const encode = (a0, a1) => {
at = 0
if (!wDyn(a0)) return false
if (typeof a1 !== 'string') return false
wStr(a1)
return true
}
const decode = () => {
let t
{
const g0 = rU8()
if (g0 === 0) t = undefined
else if (g0 === 2) t = null
else {
t = rStr()
}
}
return t
}
module.exports.relayProfileDeleteIssue = function relayProfileDeleteIssue (a0, a1) {
  if (illFormed(a1)) throw new TypeError('argument 2 of relayProfileDeleteIssue has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 2 && encode(a0, a1) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `isAggregateRelayProfile`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.isAggregateRelayProfile
const call = native.isAggregateRelayProfileWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.isAggregateRelayProfile = function isAggregateRelayProfile (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `relayProviderView`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.relayProviderView
const call = native.relayProviderViewWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
return true
}
const decode = () => {
let t
t = rStr()
return t
}
module.exports.relayProviderView = function relayProviderView (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `normalizeAggregateRelayProfile`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.normalizeAggregateRelayProfile
const call = native.normalizeAggregateRelayProfileWire
const encode = (a0, a1) => {
at = 0
if (!wDyn(a0)) return false
if (!wDyn(a1)) return false
return true
}
const decode = () => {
let t
t = rDyn()
return t
}
module.exports.normalizeAggregateRelayProfile = function normalizeAggregateRelayProfile (a0, a1) {
  try {
    if (arguments.length <= 2 && encode(a0, a1) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `aggregateProfileForMode`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.aggregateProfileForMode
const call = native.aggregateProfileForModeWire
const encode = (a0, a1, a2) => {
at = 0
if (!wDyn(a0)) return false
if (!wDyn(a1)) return false
if (!wDyn(a2)) return false
return true
}
const decode = () => {
let t
t = rDyn()
return t
}
module.exports.aggregateProfileForMode = function aggregateProfileForMode (a0, a1, a2) {
  try {
    if (arguments.length <= 3 && encode(a0, a1, a2) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `normalizeAggregateConfig`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.normalizeAggregateConfig
const call = native.normalizeAggregateConfigWire
const encode = (a0, a1, a2) => {
at = 0
if (!wDyn(a0)) return false
if (!wDyn(a1)) return false
if (!wDyn(a2)) return false
return true
}
const decode = () => {
let t
t = rDyn()
return t
}
module.exports.normalizeAggregateConfig = function normalizeAggregateConfig (a0, a1, a2) {
  try {
    if (arguments.length <= 3 && encode(a0, a1, a2) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `aggregateStrategyLabel`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.aggregateStrategyLabel
const call = native.aggregateStrategyLabelWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
return true
}
const decode = () => {
let t
t = rStr()
return t
}
module.exports.aggregateStrategyLabel = function aggregateStrategyLabel (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `aggregateStrategyHelp`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.aggregateStrategyHelp
const call = native.aggregateStrategyHelpWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
return true
}
const decode = () => {
let t
t = rStr()
return t
}
module.exports.aggregateStrategyHelp = function aggregateStrategyHelp (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `aggregateRelayProfileValidation`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.aggregateRelayProfileValidation
const call = native.aggregateRelayProfileValidationWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
return true
}
const decode = () => {
let t
{
const g0 = rU8()
if (g0 === 0) t = undefined
else if (g0 === 2) t = null
else {
t = rStr()
}
}
return t
}
module.exports.aggregateRelayProfileValidation = function aggregateRelayProfileValidation (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `splitTomlRootAndTables`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.splitTomlRootAndTables
const call = native.splitTomlRootAndTablesWire
const encode = (a0) => {
at = 0
if (typeof a0 !== 'string') return false
wStr(a0)
return true
}
const decode = () => {
let t
{
const o0 = {}
const n0 = rU8()
for (let j0 = 0; j0 < n0; j0++) {
switch (rU8()) {
case 0: {
let v0
v0 = rStr()
o0["root"] = v0
break
}
case 1: {
let v0
v0 = rStr()
o0["tables"] = v0
break
}
default: {
const k0 = rStr()
o0[k0] = rDyn()
}
}
}
t = o0
}
return t
}
module.exports.splitTomlRootAndTables = function splitTomlRootAndTables (a0) {
  if (illFormed(a0)) throw new TypeError('argument 1 of splitTomlRootAndTables has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `normalizeRelaySessionProvider`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.normalizeRelaySessionProvider
const call = native.normalizeRelaySessionProviderWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
return true
}
const decode = () => {
let t
t = rDyn()
return t
}
module.exports.normalizeRelaySessionProvider = function normalizeRelaySessionProvider (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `relaySessionProviderFromConfig`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.relaySessionProviderFromConfig
const call = native.relaySessionProviderFromConfigWire
const encode = (a0) => {
at = 0
if (typeof a0 !== 'string') return false
wStr(a0)
return true
}
const decode = () => {
let t
t = rDyn()
return t
}
module.exports.relaySessionProviderFromConfig = function relaySessionProviderFromConfig (a0) {
  if (illFormed(a0)) throw new TypeError('argument 1 of relaySessionProviderFromConfig has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `relaySessionProvider`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.relaySessionProvider
const call = native.relaySessionProviderWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
return true
}
const decode = () => {
let t
t = rDyn()
return t
}
module.exports.relaySessionProvider = function relaySessionProvider (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `activeRelayProfile`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.activeRelayProfile
const call = native.activeRelayProfileWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
return true
}
const decode = () => {
let t
t = rDyn()
return t
}
module.exports.activeRelayProfile = function activeRelayProfile (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `emptyContextSelection`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.emptyContextSelection
const call = native.emptyContextSelectionWire
const encode = () => {
at = 0
return true
}
const decode = () => {
let t
t = rDyn()
return t
}
module.exports.emptyContextSelection = function emptyContextSelection () {
  try {
    if (arguments.length <= 0 && encode() && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `normalizeContextSelection`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.normalizeContextSelection
const call = native.normalizeContextSelectionWire
const encode = (a0, a1) => {
at = 0
if (!wDyn(a0)) return false
if (!wDyn(a1)) return false
return true
}
const decode = () => {
let t
t = rDyn()
return t
}
module.exports.normalizeContextSelection = function normalizeContextSelection (a0, a1) {
  try {
    if (arguments.length <= 2 && encode(a0, a1) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `normalizeRelayMode`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.normalizeRelayMode
const call = native.normalizeRelayModeWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
return true
}
const decode = () => {
let t
t = rDyn()
return t
}
module.exports.normalizeRelayMode = function normalizeRelayMode (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `normalizeRelayProtocol`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.normalizeRelayProtocol
const call = native.normalizeRelayProtocolWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
return true
}
const decode = () => {
let t
t = rDyn()
return t
}
module.exports.normalizeRelayProtocol = function normalizeRelayProtocol (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `normalizeAnthropicAuthHeader`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.normalizeAnthropicAuthHeader
const call = native.normalizeAnthropicAuthHeaderWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
return true
}
const decode = () => {
let t
t = rStr()
return t
}
module.exports.normalizeAnthropicAuthHeader = function normalizeAnthropicAuthHeader (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `normalizeAnthropicThinkingMode`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.normalizeAnthropicThinkingMode
const call = native.normalizeAnthropicThinkingModeWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
return true
}
const decode = () => {
let t
t = rStr()
return t
}
module.exports.normalizeAnthropicThinkingMode = function normalizeAnthropicThinkingMode (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `normalizeAnthropicMaxTokens`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.normalizeAnthropicMaxTokens
const call = native.normalizeAnthropicMaxTokensWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
return true
}
const decode = () => {
let t
t = rF64()
return t
}
module.exports.normalizeAnthropicMaxTokens = function normalizeAnthropicMaxTokens (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `hydrateAggregateRelayProfile`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.hydrateAggregateRelayProfile
const call = native.hydrateAggregateRelayProfileWire
const encode = (a0, a1) => {
at = 0
if (!wDyn(a0)) return false
if (!wDyn(a1)) return false
return true
}
const decode = () => {
let t
t = rDyn()
return t
}
module.exports.hydrateAggregateRelayProfile = function hydrateAggregateRelayProfile (a0, a1) {
  try {
    if (arguments.length <= 2 && encode(a0, a1) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `normalizeRelayProfile`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.normalizeRelayProfile
const call = native.normalizeRelayProfileWire
const encode = (a0, a1) => {
at = 0
if (!wDyn(a0)) return false
if (!wDyn(a1)) return false
return true
}
const decode = () => {
let t
t = rDyn()
return t
}
module.exports.normalizeRelayProfile = function normalizeRelayProfile (a0, a1) {
  try {
    if (arguments.length <= 2 && encode(a0, a1) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}
const guards = {
  buildOfficialRelayAuthJson: [false, [0]],
  codexModelFromConfig: [false, [0]],
  codexBaseUrlFromConfig: [false, [0]],
  codexExperimentalBearerTokenFromConfig: [false, [0]],
  codexProviderStringFromConfig: [false, [0, 1]],
  codexApiKeyFromAuth: [false, [0]],
  codexTopLevelIntFromConfig: [false, [0, 1]],
  rootTomlStringValue: [false, [0, 1]],
  tomlKeyPathName: [false, [0]],
  codexProviderSectionName: [false, [0]],
  tomlInlineComment: [false, [0]],
  setAuthOpenAiApiKey: [false, [0, 1]],
  setRootTomlStringKey: [false, [0, 1, 2]],
  setManagedOpenAiBaseUrl: [false, [0]],
  setRootTomlIntKey: [false, [0, 1, 2]],
  setRootTomlLine: [false, [0, 1, 2]],
  codexRequiresOpenAiAuthFromConfig: [false, [0]],
  removeCodexExperimentalBearerToken: [false, [0]],
  setTomlSectionBoolKey: [false, [0, 1, 2]],
  setTomlSectionStringKey: [false, [0, 1, 2, 3]],
  setTomlSectionRawKey: [false, [0, 1, 2, 3]],
  removeTomlSectionKey: [false, [0, 1, 2]],
  authJsonHasOpenAiApiKey: [false, [0]],
  tomlString: [false, [0]],
  clampAggregateWeight: [false, []],
  ensureTrailingNewline: [false, [0]],
  removeRootTomlKey: [false, [0, 1]],
  joinTomlSections: [false, [0]],
}
const JS_ERRORS = /^(TypeError|RangeError|SyntaxError|ReferenceError|URIError|EvalError): ([\s\S]*)$/
const jsError = e => {
  const message = typeof e?.message === 'string' ? e.message : ''
  const typed = JS_ERRORS.exec(message)
  if (typed) return new globalThis[typed[1]](typed[2])
  if (/Expected$/.test(e?.code)) return Object.assign(new TypeError(message), { code: e.code })
  if (message.startsWith('Rust panic:')) e.code = 'RUST_PANIC'
  return e
}
const illFormed = v => typeof v === 'string' ? !v.isWellFormed() : Array.isArray(v) && v.some(illFormed)
for (const [name, [isAsync, strings]] of Object.entries(guards)) {
  const f = module.exports[name]
  if (typeof f !== 'function') continue
  const check = a => {
    for (const i of strings) {
      if (illFormed(a[i])) throw new TypeError(`argument ${i + 1} of ${name} has a lone surrogate, which a Rust String cannot hold`)
    }
  }
  module.exports[name] = isAsync
    ? { [name]: async function (...a) { check(a); try { return await f(...a) } catch (e) { throw jsError(e) } } }[name]
    : { [name]: function (...a) { check(a); try { return f(...a) } catch (e) { throw jsError(e) } } }[name]
}
