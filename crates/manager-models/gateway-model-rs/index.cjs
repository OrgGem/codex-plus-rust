// Loads the wasm-bindgen (nodejs target) output built by `rust-anything build`.
const native = require('./pkg/gateway_model_rs.js')

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
  keyLabel: [[1, 0], 'raw', false],
  blankBinding: [[], 'json', false],
  draftFromBinding: [[1], 'json', false],
  defaultProviderAgents: [[1], 'json', false],
  skippedAgents: [[1, 1], 'json', false],
  poolExcludeToggle: [[1, 0, 0], 'raw', false],
  parseContextWindow: [[0], 'json', false],
  parseKeyLimit: [[0], 'json', false],
  parseCompactionNote: [[0], 'json', false],
  compactionNote: [[1], 'json', false],
  foreignCompactionDropped: [[1], 'raw', false],
  queueWaitMs: [[1], 'json', false],
  bindingFromDraft: [[1, 1, 1], 'json', false],
  disabledGroup: [[1, 0, 1], 'json', false],
  providerEnabled: [[1], 'raw', false],
  agentPoolProviders: [[1, 1], 'json', false],
  agentModels: [[1, 1], 'raw', false],
  qualifiedProvider: [[1, 0, 1], 'json', false],
  modelRoutable: [[1, 0, 1], 'raw', false],
  validateDraft: [[1, 1, 1, 1], 'json', false],
  protocolLabel: [[1], 'raw', false],
  glanceAgents: [[1], 'json', false],
  rememberAgentNames: [[1], 'json', false],
  visibleOwnModels: [[1, 1], 'json', false],
  servingProviders: [[1, 0, 1], 'json', false],
  groupDisplayName: [[1, 0], 'json', false],
  modelDisplay: [[1, 0, 1, 1], 'json', false],
  parseWeight: [[0], 'json', false],
  parseGatewayPort: [[0], 'json', false],
  patternMatches: [[1, 0], 'raw', false],
  speedOf: [[1, 0, 0], 'json', false],
  quotaWindowLabel: [[1], 'json', false],
  quotaSupported: [[1], 'raw', false],
  quotaEntries: [[1], 'json', false],
  quotaRemaining: [[1, 0], 'raw', false],
  quotaWindowFresh: [[1, 1, 0, 1], 'raw', false],
  subscriptionBlockReason: [[1, 1], 'json', false],
  memberLabel: [[1, 0], 'raw', false],
  memberStatus: [[1, 0, 1], 'json', false],
  dispatchStatus: [[1, 0, 0, 1], 'json', false],
  routeNarrative: [[0, 1, 1], 'json', false],
  connectedAgents: [[1], 'json', false],
  modelDispatch: [[1], 'json', false],
  activeSponsorIds: [[1], 'raw', false],
  gatewayPresets: [[1], 'json', false],
  presetAdded: [[1, 1], 'raw', false],
  providerDraftFromPreset: [[1], 'json', false],
  providerDraftFromEntry: [[1], 'json', false],
  providerInput: [[1], 'json', false],
  priceFromDraft: [[1], 'json', false],
  proxyAddressError: [[0], 'json', false],
  validateProviderDraft: [[1, 1], 'json', false],
  usageRangeStart: [[0, 1], 'json', false],
  relayFilterFromFocus: [[1], 'json', false],
  relayFilterArgs: [[1], 'json', false],
  formatRequestCost: [[1], 'raw', false],
  cacheHitRate: [[1], 'json', false],
  averageDuration: [[1], 'json', false],
  costText: [[1], 'raw', false],
  usageDayStats: [[1], 'json', false],
  axisLabelIndexes: [[0, 1], 'raw', false],
  shortDate: [[0, 1], 'raw', false],
  filterSessions: [[1, 1, 0], 'json', false],
  relativeTime: [[0, 0], 'json', false],
  importCandidates: [[1], 'json', false],
  pendingImportCount: [[1], 'raw', false],
  isFavoriteModel: [[1, 0, 1], 'raw', false],
  toggleFavoriteModel: [[1, 0, 1], 'json', false],
  activeAutoGroup: [[1, 0], 'json', false],
  memberTarget: [[1, 0], 'json', false],
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

// `keyLabel`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.keyLabel
const call = native.keyLabelWire
const encode = (a0, a1) => {
at = 0
if (!wDyn(a0)) return false
if (typeof a1 !== 'string') return false
wStr(a1)
return true
}
const decode = () => {
let t
t = rStr()
return t
}
module.exports.keyLabel = function keyLabel (a0, a1) {
  if (illFormed(a1)) throw new TypeError('argument 2 of keyLabel has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 2 && encode(a0, a1) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `blankBinding`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.blankBinding
const call = native.blankBindingWire
const encode = () => {
at = 0
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
v0 = rU8() === 1
o0["enabled"] = v0
break
}
case 1: {
let v0
v0 = rStr()
o0["model"] = v0
break
}
case 2: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
{
const o2 = {}
const n2 = rU8()
for (let j2 = 0; j2 < n2; j2++) {
switch (rU8()) {
case 0: {
let v2
v2 = rStr()
o2["providerId"] = v2
break
}
case 1: {
let v2
v2 = rStr()
o2["upstreamModel"] = v2
break
}
case 2: {
let v2
v2 = rStr()
o2["providerSource"] = v2
break
}
case 3: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rU8() === 1
}
}
o2["providerRemoved"] = v2
break
}
default: {
const k2 = rStr()
o2[k2] = rDyn()
}
}
}
v0 = o2
}
}
}
o0["defaultModelSelection"] = v0
break
}
case 3: {
let v0
if (rU8() === 0) v0 = null
else {
const n1 = rU32()
const a1 = new Array(n1)
for (let i1 = 0; i1 < n1; i1++) {
let v1
v1 = rStr()
a1[i1] = v1
}
v0 = a1
}
o0["extraModels"] = v0
break
}
case 4: {
let v0
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["opus"] = v1
break
}
case 1: {
let v1
v1 = rStr()
o1["sonnet"] = v1
break
}
case 2: {
let v1
v1 = rStr()
o1["haiku"] = v1
break
}
case 3: {
let v1
v1 = rStr()
o1["subagent"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
v0 = o1
}
o0["claudeRoles"] = v0
break
}
case 5: {
let v0
v0 = rStr()
o0["unknownModel"] = v0
break
}
case 6: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
v0 = rStr()
}
}
o0["autoReviewModel"] = v0
break
}
case 7: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
v0 = rStr()
}
}
o0["titleModel"] = v0
break
}
case 8: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
v0 = rStr()
}
}
o0["compactionModel"] = v0
break
}
case 9: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
v0 = rStr()
}
}
o0["compactionReasoningEffort"] = v0
break
}
case 10: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
v0 = rF64()
}
}
o0["contextWindow"] = v0
break
}
case 11: {
let v0
v0 = rU8() === 1
o0["claudeEnableEffort"] = v0
break
}
case 12: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
if (rU8() === 0) v0 = null
else {
const n2 = rU32()
const a2 = new Array(n2)
for (let i2 = 0; i2 < n2; i2++) {
let v2
v2 = rStr()
a2[i2] = v2
}
v0 = a2
}
}
}
o0["poolExclude"] = v0
break
}
case 13: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
v0 = rU8() === 1
}
}
o0["autoJoinProviders"] = v0
break
}
case 14: {
let v0
v0 = rStr()
o0["token"] = v0
break
}
case 15: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
{
const o2 = {}
const n2 = rU8()
for (let j2 = 0; j2 < n2; j2++) {
switch (rU8()) {
case 0: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rStr()
}
}
o2["claudePreviousProviderId"] = v2
break
}
case 1: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rF64()
}
}
o2["appliedPort"] = v2
break
}
default: {
const k2 = rStr()
o2[k2] = rDyn()
}
}
}
v0 = o2
}
}
}
o0["restore"] = v0
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
module.exports.blankBinding = function blankBinding () {
  try {
    if (arguments.length <= 0 && encode() && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `draftFromBinding`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.draftFromBinding
const call = native.draftFromBindingWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
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
o0["model"] = v0
break
}
case 1: {
let v0
v0 = rStr()
o0["extraModels"] = v0
break
}
case 2: {
let v0
v0 = rStr()
o0["opus"] = v0
break
}
case 3: {
let v0
v0 = rStr()
o0["sonnet"] = v0
break
}
case 4: {
let v0
v0 = rStr()
o0["haiku"] = v0
break
}
case 5: {
let v0
v0 = rStr()
o0["subagent"] = v0
break
}
case 6: {
let v0
v0 = rStr()
o0["unknownModel"] = v0
break
}
case 7: {
let v0
v0 = rStr()
o0["autoReviewModel"] = v0
break
}
case 8: {
let v0
v0 = rStr()
o0["titleModel"] = v0
break
}
case 9: {
let v0
v0 = rStr()
o0["compactionModel"] = v0
break
}
case 10: {
let v0
v0 = rStr()
o0["compactionReasoningEffort"] = v0
break
}
case 11: {
let v0
v0 = rStr()
o0["contextWindow"] = v0
break
}
case 12: {
let v0
v0 = rU8() === 1
o0["claudeEnableEffort"] = v0
break
}
case 13: {
let v0
if (rU8() === 0) v0 = null
else {
const n1 = rU32()
const a1 = new Array(n1)
for (let i1 = 0; i1 < n1; i1++) {
let v1
v1 = rStr()
a1[i1] = v1
}
v0 = a1
}
o0["poolExclude"] = v0
break
}
case 14: {
let v0
v0 = rU8() === 1
o0["autoJoinProviders"] = v0
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
module.exports.draftFromBinding = function draftFromBinding (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `defaultProviderAgents`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.defaultProviderAgents
const call = native.defaultProviderAgentsWire
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
v0 = rStr()
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.defaultProviderAgents = function defaultProviderAgents (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `skippedAgents`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.skippedAgents
const call = native.skippedAgentsWire
const encode = (a0, a1) => {
at = 0
if (!wDyn(a0)) return false
if (!wDyn(a1)) return false
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
v0 = rStr()
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.skippedAgents = function skippedAgents (a0, a1) {
  try {
    if (arguments.length <= 2 && encode(a0, a1) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `poolExcludeToggle`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.poolExcludeToggle
const call = native.poolExcludeToggleWire
const encode = (a0, a1, a2) => {
at = 0
if (!wDyn(a0)) return false
if (typeof a1 !== 'string') return false
wStr(a1)
if (typeof a2 !== 'boolean') return false
wU8(a2 ? 1 : 0)
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
v0 = rStr()
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.poolExcludeToggle = function poolExcludeToggle (a0, a1, a2) {
  if (illFormed(a1)) throw new TypeError('argument 2 of poolExcludeToggle has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 3 && encode(a0, a1, a2) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `parseContextWindow`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.parseContextWindow
const call = native.parseContextWindowWire
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
t = rF64()
}
}
return t
}
module.exports.parseContextWindow = function parseContextWindow (a0) {
  if (illFormed(a0)) throw new TypeError('argument 1 of parseContextWindow has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `parseKeyLimit`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.parseKeyLimit
const call = native.parseKeyLimitWire
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
t = rF64()
}
}
return t
}
module.exports.parseKeyLimit = function parseKeyLimit (a0) {
  if (illFormed(a0)) throw new TypeError('argument 1 of parseKeyLimit has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `parseCompactionNote`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.parseCompactionNote
const call = native.parseCompactionNoteWire
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
module.exports.parseCompactionNote = function parseCompactionNote (a0) {
  if (illFormed(a0)) throw new TypeError('argument 1 of parseCompactionNote has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `compactionNote`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.compactionNote
const call = native.compactionNoteWire
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
module.exports.compactionNote = function compactionNote (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `foreignCompactionDropped`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.foreignCompactionDropped
const call = native.foreignCompactionDroppedWire
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
module.exports.foreignCompactionDropped = function foreignCompactionDropped (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `queueWaitMs`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.queueWaitMs
const call = native.queueWaitMsWire
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
t = rF64()
}
}
return t
}
module.exports.queueWaitMs = function queueWaitMs (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `bindingFromDraft`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.bindingFromDraft
const call = native.bindingFromDraftWire
const encode = (a0, a1, a2) => {
at = 0
if (!wDyn(a0)) return false
if (!wDyn(a1)) return false
if (!wDyn(a2)) return false
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
v0 = rU8() === 1
o0["enabled"] = v0
break
}
case 1: {
let v0
v0 = rStr()
o0["model"] = v0
break
}
case 2: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
{
const o2 = {}
const n2 = rU8()
for (let j2 = 0; j2 < n2; j2++) {
switch (rU8()) {
case 0: {
let v2
v2 = rStr()
o2["providerId"] = v2
break
}
case 1: {
let v2
v2 = rStr()
o2["upstreamModel"] = v2
break
}
case 2: {
let v2
v2 = rStr()
o2["providerSource"] = v2
break
}
case 3: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rU8() === 1
}
}
o2["providerRemoved"] = v2
break
}
default: {
const k2 = rStr()
o2[k2] = rDyn()
}
}
}
v0 = o2
}
}
}
o0["defaultModelSelection"] = v0
break
}
case 3: {
let v0
if (rU8() === 0) v0 = null
else {
const n1 = rU32()
const a1 = new Array(n1)
for (let i1 = 0; i1 < n1; i1++) {
let v1
v1 = rStr()
a1[i1] = v1
}
v0 = a1
}
o0["extraModels"] = v0
break
}
case 4: {
let v0
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["opus"] = v1
break
}
case 1: {
let v1
v1 = rStr()
o1["sonnet"] = v1
break
}
case 2: {
let v1
v1 = rStr()
o1["haiku"] = v1
break
}
case 3: {
let v1
v1 = rStr()
o1["subagent"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
v0 = o1
}
o0["claudeRoles"] = v0
break
}
case 5: {
let v0
v0 = rStr()
o0["unknownModel"] = v0
break
}
case 6: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
v0 = rStr()
}
}
o0["autoReviewModel"] = v0
break
}
case 7: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
v0 = rStr()
}
}
o0["titleModel"] = v0
break
}
case 8: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
v0 = rStr()
}
}
o0["compactionModel"] = v0
break
}
case 9: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
v0 = rStr()
}
}
o0["compactionReasoningEffort"] = v0
break
}
case 10: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
v0 = rF64()
}
}
o0["contextWindow"] = v0
break
}
case 11: {
let v0
v0 = rU8() === 1
o0["claudeEnableEffort"] = v0
break
}
case 12: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
if (rU8() === 0) v0 = null
else {
const n2 = rU32()
const a2 = new Array(n2)
for (let i2 = 0; i2 < n2; i2++) {
let v2
v2 = rStr()
a2[i2] = v2
}
v0 = a2
}
}
}
o0["poolExclude"] = v0
break
}
case 13: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
v0 = rU8() === 1
}
}
o0["autoJoinProviders"] = v0
break
}
case 14: {
let v0
v0 = rStr()
o0["token"] = v0
break
}
case 15: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
{
const o2 = {}
const n2 = rU8()
for (let j2 = 0; j2 < n2; j2++) {
switch (rU8()) {
case 0: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rStr()
}
}
o2["claudePreviousProviderId"] = v2
break
}
case 1: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rF64()
}
}
o2["appliedPort"] = v2
break
}
default: {
const k2 = rStr()
o2[k2] = rDyn()
}
}
}
v0 = o2
}
}
}
o0["restore"] = v0
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
module.exports.bindingFromDraft = function bindingFromDraft (a0, a1, a2) {
  try {
    if (arguments.length <= 3 && encode(a0, a1, a2) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `disabledGroup`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.disabledGroup
const call = native.disabledGroupWire
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
o1["id"] = v1
break
}
case 1: {
let v1
v1 = rStr()
o1["name"] = v1
break
}
case 2: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["disabled"] = v1
break
}
case 3: {
let v1
v1 = rStr()
o1["mode"] = v1
break
}
case 4: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["sticky"] = v1
break
}
case 5: {
let v1
if (rU8() === 0) v1 = null
else {
const n2 = rU32()
const a2 = new Array(n2)
for (let i2 = 0; i2 < n2; i2++) {
let v2
{
const o3 = {}
const n3 = rU8()
for (let j3 = 0; j3 < n3; j3++) {
switch (rU8()) {
case 0: {
let v3
v3 = rStr()
o3["model"] = v3
break
}
case 1: {
let v3
v3 = rF64()
o3["weight"] = v3
break
}
default: {
const k3 = rStr()
o3[k3] = rDyn()
}
}
}
v2 = o3
}
a2[i2] = v2
}
v1 = a2
}
o1["members"] = v1
break
}
case 6: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["hideMembers"] = v1
break
}
case 7: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["pick"] = v1
break
}
case 8: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["imageInputModel"] = v1
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
module.exports.disabledGroup = function disabledGroup (a0, a1, a2) {
  if (illFormed(a1)) throw new TypeError('argument 2 of disabledGroup has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 3 && encode(a0, a1, a2) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `providerEnabled`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.providerEnabled
const call = native.providerEnabledWire
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
module.exports.providerEnabled = function providerEnabled (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `agentPoolProviders`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.agentPoolProviders
const call = native.agentPoolProvidersWire
const encode = (a0, a1) => {
at = 0
if (!wDyn(a0)) return false
if (!wDyn(a1)) return false
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
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["id"] = v1
break
}
case 1: {
let v1
v1 = rStr()
o1["name"] = v1
break
}
case 2: {
let v1
v1 = rStr()
o1["baseUrl"] = v1
break
}
case 3: {
let v1
v1 = rStr()
o1["protocol"] = v1
break
}
case 4: {
let v1
v1 = rU8() === 1
o1["official"] = v1
break
}
case 5: {
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
o1["models"] = v1
break
}
case 6: {
let v1
v1 = rF64()
o1["keyCount"] = v1
break
}
case 7: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["enabled"] = v1
break
}
case 8: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["source"] = v1
break
}
case 9: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
{
const o3 = {}
const n3 = rU8()
for (let j3 = 0; j3 < n3; j3++) {
switch (rU8()) {
case 0: {
let v3
v3 = rStr()
o3["currency"] = v3
break
}
case 1: {
let v3
v3 = rF64()
o3["input"] = v3
break
}
case 2: {
let v3
v3 = rF64()
o3["cachedInput"] = v3
break
}
case 3: {
let v3
v3 = rF64()
o3["output"] = v3
break
}
default: {
const k3 = rStr()
o3[k3] = rDyn()
}
}
}
v1 = o3
}
}
}
o1["price"] = v1
break
}
case 10: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
if (rU8() === 0) v1 = null
else {
const n3 = rU32()
const o3 = {}
for (let i3 = 0; i3 < n3; i3++) {
const k3 = rStr()
let v3
{
const o4 = {}
const n4 = rU8()
for (let j4 = 0; j4 < n4; j4++) {
switch (rU8()) {
case 0: {
let v4
v4 = rStr()
o4["currency"] = v4
break
}
case 1: {
let v4
v4 = rF64()
o4["input"] = v4
break
}
case 2: {
let v4
v4 = rF64()
o4["cachedInput"] = v4
break
}
case 3: {
let v4
v4 = rF64()
o4["output"] = v4
break
}
default: {
const k4 = rStr()
o4[k4] = rDyn()
}
}
}
v3 = o4
}
o3[k3] = v3
}
v1 = o3
}
}
}
o1["modelPrices"] = v1
break
}
case 11: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
if (rU8() === 0) v1 = null
else {
const n3 = rU32()
const a3 = new Array(n3)
for (let i3 = 0; i3 < n3; i3++) {
let v3
v3 = rStr()
a3[i3] = v3
}
v1 = a3
}
}
}
o1["imageModels"] = v1
break
}
case 12: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["modelMetadata"] = v1
break
}
case 13: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
if (rU8() === 0) v1 = null
else {
const n3 = rU32()
const o3 = {}
for (let i3 = 0; i3 < n3; i3++) {
const k3 = rStr()
let v3
v3 = rF64()
o3[k3] = v3
}
v1 = o3
}
}
}
o1["modelWindows"] = v1
break
}
case 14: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rF64()
}
}
o1["contextWindow"] = v1
break
}
case 15: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
{
const o3 = {}
const n3 = rU8()
for (let j3 = 0; j3 < n3; j3++) {
switch (rU8()) {
case 0: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
if (rU8() === 0) v3 = null
else {
const n5 = rU32()
const o5 = {}
for (let i5 = 0; i5 < n5; i5++) {
const k5 = rStr()
let v5
v5 = rF64()
o5[k5] = v5
}
v3 = o5
}
}
}
o3["models"] = v3
break
}
case 1: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
v3 = rF64()
}
}
o3["fallback"] = v3
break
}
default: {
const k3 = rStr()
o3[k3] = rDyn()
}
}
}
v1 = o3
}
}
}
o1["autoCompact"] = v1
break
}
case 16: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["proxy"] = v1
break
}
case 17: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rF64()
}
}
o1["keyRpm"] = v1
break
}
case 18: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rF64()
}
}
o1["keyConcurrency"] = v1
break
}
case 19: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["stripImageGenerationTool"] = v1
break
}
case 20: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["stripResponsesImageDetail"] = v1
break
}
case 21: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["disableToolSearch"] = v1
break
}
case 22: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["account"] = v1
break
}
case 23: {
let v1
v1 = rDyn()
o1["subscriptionPolicy"] = v1
break
}
case 24: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["subscriptionBlockReason"] = v1
break
}
case 25: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["plan"] = v1
break
}
case 26: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
if (rU8() === 0) v1 = null
else {
const n3 = rU32()
const a3 = new Array(n3)
for (let i3 = 0; i3 < n3; i3++) {
let v3
v3 = rStr()
a3[i3] = v3
}
v1 = a3
}
}
}
o1["upstreamNew"] = v1
break
}
case 27: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
if (rU8() === 0) v1 = null
else {
const n3 = rU32()
const a3 = new Array(n3)
for (let i3 = 0; i3 < n3; i3++) {
let v3
v3 = rStr()
a3[i3] = v3
}
v1 = a3
}
}
}
o1["upstreamGone"] = v1
break
}
case 28: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["lentToCodex"] = v1
break
}
case 29: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
{
const o3 = {}
const n3 = rU8()
for (let j3 = 0; j3 < n3; j3++) {
switch (rU8()) {
case 0: {
let v3
if (rU8() === 0) v3 = null
else {
const n4 = rU32()
const a4 = new Array(n4)
for (let i4 = 0; i4 < n4; i4++) {
let v4
{
const o5 = {}
const n5 = rU8()
for (let j5 = 0; j5 < n5; j5++) {
switch (rU8()) {
case 0: {
let v5
{
const g6 = rU8()
if (g6 === 0) v5 = undefined
else if (g6 === 2) v5 = null
else {
v5 = rF64()
}
}
o5["windowMinutes"] = v5
break
}
case 1: {
let v5
v5 = rF64()
o5["usedPercent"] = v5
break
}
case 2: {
let v5
{
const g6 = rU8()
if (g6 === 0) v5 = undefined
else if (g6 === 2) v5 = null
else {
v5 = rF64()
}
}
o5["resetsAtMs"] = v5
break
}
case 3: {
let v5
{
const g6 = rU8()
if (g6 === 0) v5 = undefined
else if (g6 === 2) v5 = null
else {
v5 = rF64()
}
}
o5["observedAtMs"] = v5
break
}
default: {
const k5 = rStr()
o5[k5] = rDyn()
}
}
}
v4 = o5
}
a4[i4] = v4
}
v3 = a4
}
o3["windows"] = v3
break
}
case 1: {
let v3
v3 = rF64()
o3["updatedAtMs"] = v3
break
}
case 2: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
v3 = rU8() === 1
}
}
o3["windowsComplete"] = v3
break
}
case 3: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
v3 = rU8() === 1
}
}
o3["officialAllowed"] = v3
break
}
case 4: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
v3 = rU8() === 1
}
}
o3["officialLimitReached"] = v3
break
}
case 5: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
v3 = rU8() === 1
}
}
o3["creditsAvailable"] = v3
break
}
default: {
const k3 = rStr()
o3[k3] = rDyn()
}
}
}
v1 = o3
}
}
}
o1["quota"] = v1
break
}
case 30: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["quotaApi"] = v1
break
}
case 31: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
if (rU8() === 0) v1 = null
else {
const n3 = rU32()
const a3 = new Array(n3)
for (let i3 = 0; i3 < n3; i3++) {
let v3
{
const o4 = {}
const n4 = rU8()
for (let j4 = 0; j4 < n4; j4++) {
switch (rU8()) {
case 0: {
let v4
v4 = rStr()
o4["id"] = v4
break
}
case 1: {
let v4
v4 = rStr()
o4["masked"] = v4
break
}
default: {
const k4 = rStr()
o4[k4] = rDyn()
}
}
}
v3 = o4
}
a3[i3] = v3
}
v1 = a3
}
}
}
o1["keys"] = v1
break
}
case 32: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["balanceApi"] = v1
break
}
case 33: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
{
const o3 = {}
const n3 = rU8()
for (let j3 = 0; j3 < n3; j3++) {
switch (rU8()) {
case 0: {
let v3
v3 = rStr()
o3["masked"] = v3
break
}
case 1: {
let v3
v3 = rStr()
o3["userId"] = v3
break
}
default: {
const k3 = rStr()
o3[k3] = rDyn()
}
}
}
v1 = o3
}
}
}
o1["newapiAccess"] = v1
break
}
case 34: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
if (rU8() === 0) v1 = null
else {
const n3 = rU32()
const a3 = new Array(n3)
for (let i3 = 0; i3 < n3; i3++) {
let v3
{
const o4 = {}
const n4 = rU8()
for (let j4 = 0; j4 < n4; j4++) {
switch (rU8()) {
case 0: {
let v4
v4 = rStr()
o4["key"] = v4
break
}
case 1: {
let v4
v4 = rStr()
o4["value"] = v4
break
}
case 2: {
let v4
{
const g5 = rU8()
if (g5 === 0) v4 = undefined
else if (g5 === 2) v4 = null
else {
v4 = rStr()
}
}
o4["masked"] = v4
break
}
default: {
const k4 = rStr()
o4[k4] = rDyn()
}
}
}
v3 = o4
}
a3[i3] = v3
}
v1 = a3
}
}
}
o1["customHeaders"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
v0 = o1
}
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.agentPoolProviders = function agentPoolProviders (a0, a1) {
  try {
    if (arguments.length <= 2 && encode(a0, a1) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `agentModels`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.agentModels
const call = native.agentModelsWire
const encode = (a0, a1) => {
at = 0
if (!wDyn(a0)) return false
if (!wDyn(a1)) return false
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
v0 = rStr()
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.agentModels = function agentModels (a0, a1) {
  try {
    if (arguments.length <= 2 && encode(a0, a1) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `qualifiedProvider`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.qualifiedProvider
const call = native.qualifiedProviderWire
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
{
const o2 = {}
const n2 = rU8()
for (let j2 = 0; j2 < n2; j2++) {
switch (rU8()) {
case 0: {
let v2
v2 = rStr()
o2["id"] = v2
break
}
case 1: {
let v2
v2 = rStr()
o2["name"] = v2
break
}
case 2: {
let v2
v2 = rStr()
o2["baseUrl"] = v2
break
}
case 3: {
let v2
v2 = rStr()
o2["protocol"] = v2
break
}
case 4: {
let v2
v2 = rU8() === 1
o2["official"] = v2
break
}
case 5: {
let v2
if (rU8() === 0) v2 = null
else {
const n3 = rU32()
const a3 = new Array(n3)
for (let i3 = 0; i3 < n3; i3++) {
let v3
v3 = rStr()
a3[i3] = v3
}
v2 = a3
}
o2["models"] = v2
break
}
case 6: {
let v2
v2 = rF64()
o2["keyCount"] = v2
break
}
case 7: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rU8() === 1
}
}
o2["enabled"] = v2
break
}
case 8: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rStr()
}
}
o2["source"] = v2
break
}
case 9: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
{
const o4 = {}
const n4 = rU8()
for (let j4 = 0; j4 < n4; j4++) {
switch (rU8()) {
case 0: {
let v4
v4 = rStr()
o4["currency"] = v4
break
}
case 1: {
let v4
v4 = rF64()
o4["input"] = v4
break
}
case 2: {
let v4
v4 = rF64()
o4["cachedInput"] = v4
break
}
case 3: {
let v4
v4 = rF64()
o4["output"] = v4
break
}
default: {
const k4 = rStr()
o4[k4] = rDyn()
}
}
}
v2 = o4
}
}
}
o2["price"] = v2
break
}
case 10: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
if (rU8() === 0) v2 = null
else {
const n4 = rU32()
const o4 = {}
for (let i4 = 0; i4 < n4; i4++) {
const k4 = rStr()
let v4
{
const o5 = {}
const n5 = rU8()
for (let j5 = 0; j5 < n5; j5++) {
switch (rU8()) {
case 0: {
let v5
v5 = rStr()
o5["currency"] = v5
break
}
case 1: {
let v5
v5 = rF64()
o5["input"] = v5
break
}
case 2: {
let v5
v5 = rF64()
o5["cachedInput"] = v5
break
}
case 3: {
let v5
v5 = rF64()
o5["output"] = v5
break
}
default: {
const k5 = rStr()
o5[k5] = rDyn()
}
}
}
v4 = o5
}
o4[k4] = v4
}
v2 = o4
}
}
}
o2["modelPrices"] = v2
break
}
case 11: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
if (rU8() === 0) v2 = null
else {
const n4 = rU32()
const a4 = new Array(n4)
for (let i4 = 0; i4 < n4; i4++) {
let v4
v4 = rStr()
a4[i4] = v4
}
v2 = a4
}
}
}
o2["imageModels"] = v2
break
}
case 12: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rStr()
}
}
o2["modelMetadata"] = v2
break
}
case 13: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
if (rU8() === 0) v2 = null
else {
const n4 = rU32()
const o4 = {}
for (let i4 = 0; i4 < n4; i4++) {
const k4 = rStr()
let v4
v4 = rF64()
o4[k4] = v4
}
v2 = o4
}
}
}
o2["modelWindows"] = v2
break
}
case 14: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rF64()
}
}
o2["contextWindow"] = v2
break
}
case 15: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
{
const o4 = {}
const n4 = rU8()
for (let j4 = 0; j4 < n4; j4++) {
switch (rU8()) {
case 0: {
let v4
{
const g5 = rU8()
if (g5 === 0) v4 = undefined
else if (g5 === 2) v4 = null
else {
if (rU8() === 0) v4 = null
else {
const n6 = rU32()
const o6 = {}
for (let i6 = 0; i6 < n6; i6++) {
const k6 = rStr()
let v6
v6 = rDyn()
o6[k6] = v6
}
v4 = o6
}
}
}
o4["models"] = v4
break
}
case 1: {
let v4
{
const g5 = rU8()
if (g5 === 0) v4 = undefined
else if (g5 === 2) v4 = null
else {
v4 = rF64()
}
}
o4["fallback"] = v4
break
}
default: {
const k4 = rStr()
o4[k4] = rDyn()
}
}
}
v2 = o4
}
}
}
o2["autoCompact"] = v2
break
}
case 16: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rStr()
}
}
o2["proxy"] = v2
break
}
case 17: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rF64()
}
}
o2["keyRpm"] = v2
break
}
case 18: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rF64()
}
}
o2["keyConcurrency"] = v2
break
}
case 19: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rU8() === 1
}
}
o2["stripImageGenerationTool"] = v2
break
}
case 20: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rU8() === 1
}
}
o2["stripResponsesImageDetail"] = v2
break
}
case 21: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rU8() === 1
}
}
o2["disableToolSearch"] = v2
break
}
case 22: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rStr()
}
}
o2["account"] = v2
break
}
case 23: {
let v2
v2 = rDyn()
o2["subscriptionPolicy"] = v2
break
}
case 24: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rStr()
}
}
o2["subscriptionBlockReason"] = v2
break
}
case 25: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rStr()
}
}
o2["plan"] = v2
break
}
case 26: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
if (rU8() === 0) v2 = null
else {
const n4 = rU32()
const a4 = new Array(n4)
for (let i4 = 0; i4 < n4; i4++) {
let v4
v4 = rStr()
a4[i4] = v4
}
v2 = a4
}
}
}
o2["upstreamNew"] = v2
break
}
case 27: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
if (rU8() === 0) v2 = null
else {
const n4 = rU32()
const a4 = new Array(n4)
for (let i4 = 0; i4 < n4; i4++) {
let v4
v4 = rStr()
a4[i4] = v4
}
v2 = a4
}
}
}
o2["upstreamGone"] = v2
break
}
case 28: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rU8() === 1
}
}
o2["lentToCodex"] = v2
break
}
case 29: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
{
const o4 = {}
const n4 = rU8()
for (let j4 = 0; j4 < n4; j4++) {
switch (rU8()) {
case 0: {
let v4
if (rU8() === 0) v4 = null
else {
const n5 = rU32()
const a5 = new Array(n5)
for (let i5 = 0; i5 < n5; i5++) {
let v5
{
const o6 = {}
const n6 = rU8()
for (let j6 = 0; j6 < n6; j6++) {
switch (rU8()) {
case 0: {
let v6
v6 = rDyn()
o6["windowMinutes"] = v6
break
}
case 1: {
let v6
v6 = rDyn()
o6["usedPercent"] = v6
break
}
case 2: {
let v6
v6 = rDyn()
o6["resetsAtMs"] = v6
break
}
case 3: {
let v6
v6 = rDyn()
o6["observedAtMs"] = v6
break
}
default: {
const k6 = rStr()
o6[k6] = rDyn()
}
}
}
v5 = o6
}
a5[i5] = v5
}
v4 = a5
}
o4["windows"] = v4
break
}
case 1: {
let v4
v4 = rF64()
o4["updatedAtMs"] = v4
break
}
case 2: {
let v4
{
const g5 = rU8()
if (g5 === 0) v4 = undefined
else if (g5 === 2) v4 = null
else {
v4 = rU8() === 1
}
}
o4["windowsComplete"] = v4
break
}
case 3: {
let v4
{
const g5 = rU8()
if (g5 === 0) v4 = undefined
else if (g5 === 2) v4 = null
else {
v4 = rU8() === 1
}
}
o4["officialAllowed"] = v4
break
}
case 4: {
let v4
{
const g5 = rU8()
if (g5 === 0) v4 = undefined
else if (g5 === 2) v4 = null
else {
v4 = rU8() === 1
}
}
o4["officialLimitReached"] = v4
break
}
case 5: {
let v4
{
const g5 = rU8()
if (g5 === 0) v4 = undefined
else if (g5 === 2) v4 = null
else {
v4 = rU8() === 1
}
}
o4["creditsAvailable"] = v4
break
}
default: {
const k4 = rStr()
o4[k4] = rDyn()
}
}
}
v2 = o4
}
}
}
o2["quota"] = v2
break
}
case 30: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rU8() === 1
}
}
o2["quotaApi"] = v2
break
}
case 31: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
if (rU8() === 0) v2 = null
else {
const n4 = rU32()
const a4 = new Array(n4)
for (let i4 = 0; i4 < n4; i4++) {
let v4
{
const o5 = {}
const n5 = rU8()
for (let j5 = 0; j5 < n5; j5++) {
switch (rU8()) {
case 0: {
let v5
v5 = rStr()
o5["id"] = v5
break
}
case 1: {
let v5
v5 = rStr()
o5["masked"] = v5
break
}
default: {
const k5 = rStr()
o5[k5] = rDyn()
}
}
}
v4 = o5
}
a4[i4] = v4
}
v2 = a4
}
}
}
o2["keys"] = v2
break
}
case 32: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rU8() === 1
}
}
o2["balanceApi"] = v2
break
}
case 33: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
{
const o4 = {}
const n4 = rU8()
for (let j4 = 0; j4 < n4; j4++) {
switch (rU8()) {
case 0: {
let v4
v4 = rStr()
o4["masked"] = v4
break
}
case 1: {
let v4
v4 = rStr()
o4["userId"] = v4
break
}
default: {
const k4 = rStr()
o4[k4] = rDyn()
}
}
}
v2 = o4
}
}
}
o2["newapiAccess"] = v2
break
}
case 34: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
if (rU8() === 0) v2 = null
else {
const n4 = rU32()
const a4 = new Array(n4)
for (let i4 = 0; i4 < n4; i4++) {
let v4
{
const o5 = {}
const n5 = rU8()
for (let j5 = 0; j5 < n5; j5++) {
switch (rU8()) {
case 0: {
let v5
v5 = rStr()
o5["key"] = v5
break
}
case 1: {
let v5
v5 = rStr()
o5["value"] = v5
break
}
case 2: {
let v5
{
const g6 = rU8()
if (g6 === 0) v5 = undefined
else if (g6 === 2) v5 = null
else {
v5 = rStr()
}
}
o5["masked"] = v5
break
}
default: {
const k5 = rStr()
o5[k5] = rDyn()
}
}
}
v4 = o5
}
a4[i4] = v4
}
v2 = a4
}
}
}
o2["customHeaders"] = v2
break
}
default: {
const k2 = rStr()
o2[k2] = rDyn()
}
}
}
v1 = o2
}
o1["entry"] = v1
break
}
case 1: {
let v1
v1 = rStr()
o1["model"] = v1
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
module.exports.qualifiedProvider = function qualifiedProvider (a0, a1, a2) {
  if (illFormed(a1)) throw new TypeError('argument 2 of qualifiedProvider has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 3 && encode(a0, a1, a2) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `modelRoutable`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.modelRoutable
const call = native.modelRoutableWire
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
t = rU8() === 1
return t
}
module.exports.modelRoutable = function modelRoutable (a0, a1, a2) {
  if (illFormed(a1)) throw new TypeError('argument 2 of modelRoutable has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 3 && encode(a0, a1, a2) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `validateDraft`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.validateDraft
const call = native.validateDraftWire
const encode = (a0, a1, a2, a3) => {
at = 0
if (!wDyn(a0)) return false
if (!wDyn(a1)) return false
if (!wDyn(a2)) return false
if (!wDyn(a3)) return false
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
module.exports.validateDraft = function validateDraft (a0, a1, a2, a3) {
  try {
    if (arguments.length <= 4 && encode(a0, a1, a2, a3) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `protocolLabel`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.protocolLabel
const call = native.protocolLabelWire
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
module.exports.protocolLabel = function protocolLabel (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `glanceAgents`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.glanceAgents
const call = native.glanceAgentsWire
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
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["agent"] = v1
break
}
case 1: {
let v1
v1 = rU8() === 1
o1["enabled"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
v0 = o1
}
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.glanceAgents = function glanceAgents (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `rememberAgentNames`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.rememberAgentNames
const call = native.rememberAgentNamesWire
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
module.exports.rememberAgentNames = function rememberAgentNames (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `visibleOwnModels`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.visibleOwnModels
const call = native.visibleOwnModelsWire
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
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["vendor"] = v1
break
}
case 1: {
let v1
if (rU8() === 0) v1 = null
else {
const n2 = rU32()
const a2 = new Array(n2)
for (let i2 = 0; i2 < n2; i2++) {
let v2
{
const o3 = {}
const n3 = rU8()
for (let j3 = 0; j3 < n3; j3++) {
switch (rU8()) {
case 0: {
let v3
v3 = rStr()
o3["id"] = v3
break
}
case 1: {
let v3
v3 = rStr()
o3["label"] = v3
break
}
default: {
const k3 = rStr()
o3[k3] = rDyn()
}
}
}
v2 = o3
}
a2[i2] = v2
}
v1 = a2
}
o1["models"] = v1
break
}
case 2: {
let v1
v1 = rStr()
o1["current"] = v1
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
module.exports.visibleOwnModels = function visibleOwnModels (a0, a1) {
  try {
    if (arguments.length <= 2 && encode(a0, a1) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `servingProviders`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.servingProviders
const call = native.servingProvidersWire
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
if (rU8() === 0) t = null
else {
const n0 = rU32()
const a0 = new Array(n0)
for (let i0 = 0; i0 < n0; i0++) {
let v0
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["id"] = v1
break
}
case 1: {
let v1
v1 = rStr()
o1["name"] = v1
break
}
case 2: {
let v1
v1 = rStr()
o1["baseUrl"] = v1
break
}
case 3: {
let v1
v1 = rStr()
o1["protocol"] = v1
break
}
case 4: {
let v1
v1 = rU8() === 1
o1["official"] = v1
break
}
case 5: {
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
o1["models"] = v1
break
}
case 6: {
let v1
v1 = rF64()
o1["keyCount"] = v1
break
}
case 7: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["enabled"] = v1
break
}
case 8: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["source"] = v1
break
}
case 9: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
{
const o3 = {}
const n3 = rU8()
for (let j3 = 0; j3 < n3; j3++) {
switch (rU8()) {
case 0: {
let v3
v3 = rStr()
o3["currency"] = v3
break
}
case 1: {
let v3
v3 = rF64()
o3["input"] = v3
break
}
case 2: {
let v3
v3 = rF64()
o3["cachedInput"] = v3
break
}
case 3: {
let v3
v3 = rF64()
o3["output"] = v3
break
}
default: {
const k3 = rStr()
o3[k3] = rDyn()
}
}
}
v1 = o3
}
}
}
o1["price"] = v1
break
}
case 10: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
if (rU8() === 0) v1 = null
else {
const n3 = rU32()
const o3 = {}
for (let i3 = 0; i3 < n3; i3++) {
const k3 = rStr()
let v3
{
const o4 = {}
const n4 = rU8()
for (let j4 = 0; j4 < n4; j4++) {
switch (rU8()) {
case 0: {
let v4
v4 = rStr()
o4["currency"] = v4
break
}
case 1: {
let v4
v4 = rF64()
o4["input"] = v4
break
}
case 2: {
let v4
v4 = rF64()
o4["cachedInput"] = v4
break
}
case 3: {
let v4
v4 = rF64()
o4["output"] = v4
break
}
default: {
const k4 = rStr()
o4[k4] = rDyn()
}
}
}
v3 = o4
}
o3[k3] = v3
}
v1 = o3
}
}
}
o1["modelPrices"] = v1
break
}
case 11: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
if (rU8() === 0) v1 = null
else {
const n3 = rU32()
const a3 = new Array(n3)
for (let i3 = 0; i3 < n3; i3++) {
let v3
v3 = rStr()
a3[i3] = v3
}
v1 = a3
}
}
}
o1["imageModels"] = v1
break
}
case 12: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["modelMetadata"] = v1
break
}
case 13: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
if (rU8() === 0) v1 = null
else {
const n3 = rU32()
const o3 = {}
for (let i3 = 0; i3 < n3; i3++) {
const k3 = rStr()
let v3
v3 = rF64()
o3[k3] = v3
}
v1 = o3
}
}
}
o1["modelWindows"] = v1
break
}
case 14: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rF64()
}
}
o1["contextWindow"] = v1
break
}
case 15: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
{
const o3 = {}
const n3 = rU8()
for (let j3 = 0; j3 < n3; j3++) {
switch (rU8()) {
case 0: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
if (rU8() === 0) v3 = null
else {
const n5 = rU32()
const o5 = {}
for (let i5 = 0; i5 < n5; i5++) {
const k5 = rStr()
let v5
v5 = rF64()
o5[k5] = v5
}
v3 = o5
}
}
}
o3["models"] = v3
break
}
case 1: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
v3 = rF64()
}
}
o3["fallback"] = v3
break
}
default: {
const k3 = rStr()
o3[k3] = rDyn()
}
}
}
v1 = o3
}
}
}
o1["autoCompact"] = v1
break
}
case 16: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["proxy"] = v1
break
}
case 17: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rF64()
}
}
o1["keyRpm"] = v1
break
}
case 18: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rF64()
}
}
o1["keyConcurrency"] = v1
break
}
case 19: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["stripImageGenerationTool"] = v1
break
}
case 20: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["stripResponsesImageDetail"] = v1
break
}
case 21: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["disableToolSearch"] = v1
break
}
case 22: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["account"] = v1
break
}
case 23: {
let v1
v1 = rDyn()
o1["subscriptionPolicy"] = v1
break
}
case 24: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["subscriptionBlockReason"] = v1
break
}
case 25: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["plan"] = v1
break
}
case 26: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
if (rU8() === 0) v1 = null
else {
const n3 = rU32()
const a3 = new Array(n3)
for (let i3 = 0; i3 < n3; i3++) {
let v3
v3 = rStr()
a3[i3] = v3
}
v1 = a3
}
}
}
o1["upstreamNew"] = v1
break
}
case 27: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
if (rU8() === 0) v1 = null
else {
const n3 = rU32()
const a3 = new Array(n3)
for (let i3 = 0; i3 < n3; i3++) {
let v3
v3 = rStr()
a3[i3] = v3
}
v1 = a3
}
}
}
o1["upstreamGone"] = v1
break
}
case 28: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["lentToCodex"] = v1
break
}
case 29: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
{
const o3 = {}
const n3 = rU8()
for (let j3 = 0; j3 < n3; j3++) {
switch (rU8()) {
case 0: {
let v3
if (rU8() === 0) v3 = null
else {
const n4 = rU32()
const a4 = new Array(n4)
for (let i4 = 0; i4 < n4; i4++) {
let v4
{
const o5 = {}
const n5 = rU8()
for (let j5 = 0; j5 < n5; j5++) {
switch (rU8()) {
case 0: {
let v5
{
const g6 = rU8()
if (g6 === 0) v5 = undefined
else if (g6 === 2) v5 = null
else {
v5 = rF64()
}
}
o5["windowMinutes"] = v5
break
}
case 1: {
let v5
v5 = rF64()
o5["usedPercent"] = v5
break
}
case 2: {
let v5
{
const g6 = rU8()
if (g6 === 0) v5 = undefined
else if (g6 === 2) v5 = null
else {
v5 = rF64()
}
}
o5["resetsAtMs"] = v5
break
}
case 3: {
let v5
{
const g6 = rU8()
if (g6 === 0) v5 = undefined
else if (g6 === 2) v5 = null
else {
v5 = rF64()
}
}
o5["observedAtMs"] = v5
break
}
default: {
const k5 = rStr()
o5[k5] = rDyn()
}
}
}
v4 = o5
}
a4[i4] = v4
}
v3 = a4
}
o3["windows"] = v3
break
}
case 1: {
let v3
v3 = rF64()
o3["updatedAtMs"] = v3
break
}
case 2: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
v3 = rU8() === 1
}
}
o3["windowsComplete"] = v3
break
}
case 3: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
v3 = rU8() === 1
}
}
o3["officialAllowed"] = v3
break
}
case 4: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
v3 = rU8() === 1
}
}
o3["officialLimitReached"] = v3
break
}
case 5: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
v3 = rU8() === 1
}
}
o3["creditsAvailable"] = v3
break
}
default: {
const k3 = rStr()
o3[k3] = rDyn()
}
}
}
v1 = o3
}
}
}
o1["quota"] = v1
break
}
case 30: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["quotaApi"] = v1
break
}
case 31: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
if (rU8() === 0) v1 = null
else {
const n3 = rU32()
const a3 = new Array(n3)
for (let i3 = 0; i3 < n3; i3++) {
let v3
{
const o4 = {}
const n4 = rU8()
for (let j4 = 0; j4 < n4; j4++) {
switch (rU8()) {
case 0: {
let v4
v4 = rStr()
o4["id"] = v4
break
}
case 1: {
let v4
v4 = rStr()
o4["masked"] = v4
break
}
default: {
const k4 = rStr()
o4[k4] = rDyn()
}
}
}
v3 = o4
}
a3[i3] = v3
}
v1 = a3
}
}
}
o1["keys"] = v1
break
}
case 32: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["balanceApi"] = v1
break
}
case 33: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
{
const o3 = {}
const n3 = rU8()
for (let j3 = 0; j3 < n3; j3++) {
switch (rU8()) {
case 0: {
let v3
v3 = rStr()
o3["masked"] = v3
break
}
case 1: {
let v3
v3 = rStr()
o3["userId"] = v3
break
}
default: {
const k3 = rStr()
o3[k3] = rDyn()
}
}
}
v1 = o3
}
}
}
o1["newapiAccess"] = v1
break
}
case 34: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
if (rU8() === 0) v1 = null
else {
const n3 = rU32()
const a3 = new Array(n3)
for (let i3 = 0; i3 < n3; i3++) {
let v3
{
const o4 = {}
const n4 = rU8()
for (let j4 = 0; j4 < n4; j4++) {
switch (rU8()) {
case 0: {
let v4
v4 = rStr()
o4["key"] = v4
break
}
case 1: {
let v4
v4 = rStr()
o4["value"] = v4
break
}
case 2: {
let v4
{
const g5 = rU8()
if (g5 === 0) v4 = undefined
else if (g5 === 2) v4 = null
else {
v4 = rStr()
}
}
o4["masked"] = v4
break
}
default: {
const k4 = rStr()
o4[k4] = rDyn()
}
}
}
v3 = o4
}
a3[i3] = v3
}
v1 = a3
}
}
}
o1["customHeaders"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
v0 = o1
}
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.servingProviders = function servingProviders (a0, a1, a2) {
  if (illFormed(a1)) throw new TypeError('argument 2 of servingProviders has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 3 && encode(a0, a1, a2) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `groupDisplayName`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.groupDisplayName
const call = native.groupDisplayNameWire
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
module.exports.groupDisplayName = function groupDisplayName (a0, a1) {
  if (illFormed(a1)) throw new TypeError('argument 2 of groupDisplayName has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 2 && encode(a0, a1) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `modelDisplay`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.modelDisplay
const call = native.modelDisplayWire
const encode = (a0, a1, a2, a3) => {
at = 0
if (!wDyn(a0)) return false
if (typeof a1 !== 'string') return false
wStr(a1)
if (!wDyn(a2)) return false
if (!wDyn(a3)) return false
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
o0["title"] = v0
break
}
case 1: {
let v0
v0 = rStr()
o0["sub"] = v0
break
}
case 2: {
let v0
v0 = rU8() === 1
o0["group"] = v0
break
}
case 3: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
v0 = rU8() === 1
}
}
o0["auto"] = v0
break
}
case 4: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
v0 = rStr()
}
}
o0["issue"] = v0
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
module.exports.modelDisplay = function modelDisplay (a0, a1, a2, a3) {
  if (illFormed(a1)) throw new TypeError('argument 2 of modelDisplay has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 4 && encode(a0, a1, a2, a3) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `parseWeight`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.parseWeight
const call = native.parseWeightWire
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
t = rF64()
}
}
return t
}
module.exports.parseWeight = function parseWeight (a0) {
  if (illFormed(a0)) throw new TypeError('argument 1 of parseWeight has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `parseGatewayPort`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.parseGatewayPort
const call = native.parseGatewayPortWire
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
t = rF64()
}
}
return t
}
module.exports.parseGatewayPort = function parseGatewayPort (a0) {
  if (illFormed(a0)) throw new TypeError('argument 1 of parseGatewayPort has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `patternMatches`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.patternMatches
const call = native.patternMatchesWire
const encode = (a0, a1) => {
at = 0
if (!wDyn(a0)) return false
if (typeof a1 !== 'string') return false
wStr(a1)
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
v0 = rStr()
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.patternMatches = function patternMatches (a0, a1) {
  if (illFormed(a1)) throw new TypeError('argument 2 of patternMatches has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 2 && encode(a0, a1) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `speedOf`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.speedOf
const call = native.speedOfWire
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
{
const g0 = rU8()
if (g0 === 0) t = undefined
else if (g0 === 2) t = null
else {
t = rF64()
}
}
return t
}
module.exports.speedOf = function speedOf (a0, a1, a2) {
  if (illFormed(a1)) throw new TypeError('argument 2 of speedOf has a lone surrogate, which a Rust String cannot hold')
  if (illFormed(a2)) throw new TypeError('argument 3 of speedOf has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 3 && encode(a0, a1, a2) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `quotaWindowLabel`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.quotaWindowLabel
const call = native.quotaWindowLabelWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
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
o0["text"] = v0
break
}
case 1: {
let v0
if (rU8() === 0) v0 = null
else {
const n1 = rU32()
const a1 = new Array(n1)
for (let i1 = 0; i1 < n1; i1++) {
let v1
v1 = rF64()
a1[i1] = v1
}
v0 = a1
}
o0["args"] = v0
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
module.exports.quotaWindowLabel = function quotaWindowLabel (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `quotaSupported`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.quotaSupported
const call = native.quotaSupportedWire
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
module.exports.quotaSupported = function quotaSupported (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `quotaEntries`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.quotaEntries
const call = native.quotaEntriesWire
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
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["id"] = v1
break
}
case 1: {
let v1
v1 = rStr()
o1["name"] = v1
break
}
case 2: {
let v1
v1 = rStr()
o1["baseUrl"] = v1
break
}
case 3: {
let v1
v1 = rStr()
o1["protocol"] = v1
break
}
case 4: {
let v1
v1 = rU8() === 1
o1["official"] = v1
break
}
case 5: {
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
o1["models"] = v1
break
}
case 6: {
let v1
v1 = rF64()
o1["keyCount"] = v1
break
}
case 7: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["enabled"] = v1
break
}
case 8: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["source"] = v1
break
}
case 9: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
{
const o3 = {}
const n3 = rU8()
for (let j3 = 0; j3 < n3; j3++) {
switch (rU8()) {
case 0: {
let v3
v3 = rStr()
o3["currency"] = v3
break
}
case 1: {
let v3
v3 = rF64()
o3["input"] = v3
break
}
case 2: {
let v3
v3 = rF64()
o3["cachedInput"] = v3
break
}
case 3: {
let v3
v3 = rF64()
o3["output"] = v3
break
}
default: {
const k3 = rStr()
o3[k3] = rDyn()
}
}
}
v1 = o3
}
}
}
o1["price"] = v1
break
}
case 10: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
if (rU8() === 0) v1 = null
else {
const n3 = rU32()
const o3 = {}
for (let i3 = 0; i3 < n3; i3++) {
const k3 = rStr()
let v3
{
const o4 = {}
const n4 = rU8()
for (let j4 = 0; j4 < n4; j4++) {
switch (rU8()) {
case 0: {
let v4
v4 = rStr()
o4["currency"] = v4
break
}
case 1: {
let v4
v4 = rF64()
o4["input"] = v4
break
}
case 2: {
let v4
v4 = rF64()
o4["cachedInput"] = v4
break
}
case 3: {
let v4
v4 = rF64()
o4["output"] = v4
break
}
default: {
const k4 = rStr()
o4[k4] = rDyn()
}
}
}
v3 = o4
}
o3[k3] = v3
}
v1 = o3
}
}
}
o1["modelPrices"] = v1
break
}
case 11: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
if (rU8() === 0) v1 = null
else {
const n3 = rU32()
const a3 = new Array(n3)
for (let i3 = 0; i3 < n3; i3++) {
let v3
v3 = rStr()
a3[i3] = v3
}
v1 = a3
}
}
}
o1["imageModels"] = v1
break
}
case 12: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["modelMetadata"] = v1
break
}
case 13: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
if (rU8() === 0) v1 = null
else {
const n3 = rU32()
const o3 = {}
for (let i3 = 0; i3 < n3; i3++) {
const k3 = rStr()
let v3
v3 = rF64()
o3[k3] = v3
}
v1 = o3
}
}
}
o1["modelWindows"] = v1
break
}
case 14: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rF64()
}
}
o1["contextWindow"] = v1
break
}
case 15: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
{
const o3 = {}
const n3 = rU8()
for (let j3 = 0; j3 < n3; j3++) {
switch (rU8()) {
case 0: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
if (rU8() === 0) v3 = null
else {
const n5 = rU32()
const o5 = {}
for (let i5 = 0; i5 < n5; i5++) {
const k5 = rStr()
let v5
v5 = rF64()
o5[k5] = v5
}
v3 = o5
}
}
}
o3["models"] = v3
break
}
case 1: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
v3 = rF64()
}
}
o3["fallback"] = v3
break
}
default: {
const k3 = rStr()
o3[k3] = rDyn()
}
}
}
v1 = o3
}
}
}
o1["autoCompact"] = v1
break
}
case 16: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["proxy"] = v1
break
}
case 17: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rF64()
}
}
o1["keyRpm"] = v1
break
}
case 18: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rF64()
}
}
o1["keyConcurrency"] = v1
break
}
case 19: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["stripImageGenerationTool"] = v1
break
}
case 20: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["stripResponsesImageDetail"] = v1
break
}
case 21: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["disableToolSearch"] = v1
break
}
case 22: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["account"] = v1
break
}
case 23: {
let v1
v1 = rDyn()
o1["subscriptionPolicy"] = v1
break
}
case 24: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["subscriptionBlockReason"] = v1
break
}
case 25: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["plan"] = v1
break
}
case 26: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
if (rU8() === 0) v1 = null
else {
const n3 = rU32()
const a3 = new Array(n3)
for (let i3 = 0; i3 < n3; i3++) {
let v3
v3 = rStr()
a3[i3] = v3
}
v1 = a3
}
}
}
o1["upstreamNew"] = v1
break
}
case 27: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
if (rU8() === 0) v1 = null
else {
const n3 = rU32()
const a3 = new Array(n3)
for (let i3 = 0; i3 < n3; i3++) {
let v3
v3 = rStr()
a3[i3] = v3
}
v1 = a3
}
}
}
o1["upstreamGone"] = v1
break
}
case 28: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["lentToCodex"] = v1
break
}
case 29: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
{
const o3 = {}
const n3 = rU8()
for (let j3 = 0; j3 < n3; j3++) {
switch (rU8()) {
case 0: {
let v3
if (rU8() === 0) v3 = null
else {
const n4 = rU32()
const a4 = new Array(n4)
for (let i4 = 0; i4 < n4; i4++) {
let v4
{
const o5 = {}
const n5 = rU8()
for (let j5 = 0; j5 < n5; j5++) {
switch (rU8()) {
case 0: {
let v5
{
const g6 = rU8()
if (g6 === 0) v5 = undefined
else if (g6 === 2) v5 = null
else {
v5 = rF64()
}
}
o5["windowMinutes"] = v5
break
}
case 1: {
let v5
v5 = rF64()
o5["usedPercent"] = v5
break
}
case 2: {
let v5
{
const g6 = rU8()
if (g6 === 0) v5 = undefined
else if (g6 === 2) v5 = null
else {
v5 = rF64()
}
}
o5["resetsAtMs"] = v5
break
}
case 3: {
let v5
{
const g6 = rU8()
if (g6 === 0) v5 = undefined
else if (g6 === 2) v5 = null
else {
v5 = rF64()
}
}
o5["observedAtMs"] = v5
break
}
default: {
const k5 = rStr()
o5[k5] = rDyn()
}
}
}
v4 = o5
}
a4[i4] = v4
}
v3 = a4
}
o3["windows"] = v3
break
}
case 1: {
let v3
v3 = rF64()
o3["updatedAtMs"] = v3
break
}
case 2: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
v3 = rU8() === 1
}
}
o3["windowsComplete"] = v3
break
}
case 3: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
v3 = rU8() === 1
}
}
o3["officialAllowed"] = v3
break
}
case 4: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
v3 = rU8() === 1
}
}
o3["officialLimitReached"] = v3
break
}
case 5: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
v3 = rU8() === 1
}
}
o3["creditsAvailable"] = v3
break
}
default: {
const k3 = rStr()
o3[k3] = rDyn()
}
}
}
v1 = o3
}
}
}
o1["quota"] = v1
break
}
case 30: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["quotaApi"] = v1
break
}
case 31: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
if (rU8() === 0) v1 = null
else {
const n3 = rU32()
const a3 = new Array(n3)
for (let i3 = 0; i3 < n3; i3++) {
let v3
{
const o4 = {}
const n4 = rU8()
for (let j4 = 0; j4 < n4; j4++) {
switch (rU8()) {
case 0: {
let v4
v4 = rStr()
o4["id"] = v4
break
}
case 1: {
let v4
v4 = rStr()
o4["masked"] = v4
break
}
default: {
const k4 = rStr()
o4[k4] = rDyn()
}
}
}
v3 = o4
}
a3[i3] = v3
}
v1 = a3
}
}
}
o1["keys"] = v1
break
}
case 32: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["balanceApi"] = v1
break
}
case 33: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
{
const o3 = {}
const n3 = rU8()
for (let j3 = 0; j3 < n3; j3++) {
switch (rU8()) {
case 0: {
let v3
v3 = rStr()
o3["masked"] = v3
break
}
case 1: {
let v3
v3 = rStr()
o3["userId"] = v3
break
}
default: {
const k3 = rStr()
o3[k3] = rDyn()
}
}
}
v1 = o3
}
}
}
o1["newapiAccess"] = v1
break
}
case 34: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
if (rU8() === 0) v1 = null
else {
const n3 = rU32()
const a3 = new Array(n3)
for (let i3 = 0; i3 < n3; i3++) {
let v3
{
const o4 = {}
const n4 = rU8()
for (let j4 = 0; j4 < n4; j4++) {
switch (rU8()) {
case 0: {
let v4
v4 = rStr()
o4["key"] = v4
break
}
case 1: {
let v4
v4 = rStr()
o4["value"] = v4
break
}
case 2: {
let v4
{
const g5 = rU8()
if (g5 === 0) v4 = undefined
else if (g5 === 2) v4 = null
else {
v4 = rStr()
}
}
o4["masked"] = v4
break
}
default: {
const k4 = rStr()
o4[k4] = rDyn()
}
}
}
v3 = o4
}
a3[i3] = v3
}
v1 = a3
}
}
}
o1["customHeaders"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
v0 = o1
}
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.quotaEntries = function quotaEntries (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `quotaRemaining`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.quotaRemaining
const call = native.quotaRemainingWire
const encode = (a0, a1) => {
at = 0
if (!wDyn(a0)) return false
if (typeof a1 !== 'number') return false
wF64(a1)
return true
}
const decode = () => {
let t
t = rF64()
return t
}
module.exports.quotaRemaining = function quotaRemaining (a0, a1) {
  try {
    if (arguments.length <= 2 && encode(a0, a1) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `quotaWindowFresh`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.quotaWindowFresh
const call = native.quotaWindowFreshWire
const encode = (a0, a1, a2, a3) => {
at = 0
if (!wDyn(a0)) return false
if (!wDyn(a1)) return false
if (typeof a2 !== 'number') return false
wF64(a2)
if (!wDyn(a3)) return false
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.quotaWindowFresh = function quotaWindowFresh (a0, a1, a2, a3) {
  try {
    if (arguments.length <= 4 && encode(a0, a1, a2, a3) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `subscriptionBlockReason`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.subscriptionBlockReason
const call = native.subscriptionBlockReasonWire
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
module.exports.subscriptionBlockReason = function subscriptionBlockReason (a0, a1) {
  try {
    if (arguments.length <= 2 && encode(a0, a1) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `memberLabel`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.memberLabel
const call = native.memberLabelWire
const encode = (a0, a1) => {
at = 0
if (!wDyn(a0)) return false
if (typeof a1 !== 'string') return false
wStr(a1)
return true
}
const decode = () => {
let t
t = rStr()
return t
}
module.exports.memberLabel = function memberLabel (a0, a1) {
  if (illFormed(a1)) throw new TypeError('argument 2 of memberLabel has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 2 && encode(a0, a1) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `memberStatus`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.memberStatus
const call = native.memberStatusWire
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
{
const o0 = {}
const n0 = rU8()
for (let j0 = 0; j0 < n0; j0++) {
switch (rU8()) {
case 0: {
let v0
v0 = rStr()
o0["state"] = v0
break
}
case 1: {
let v0
v0 = rStr()
o0["text"] = v0
break
}
case 2: {
let v0
if (rU8() === 0) v0 = null
else {
const n1 = rU32()
const a1 = new Array(n1)
for (let i1 = 0; i1 < n1; i1++) {
let v1
v1 = rDyn()
a1[i1] = v1
}
v0 = a1
}
o0["args"] = v0
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
module.exports.memberStatus = function memberStatus (a0, a1, a2) {
  if (illFormed(a1)) throw new TypeError('argument 2 of memberStatus has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 3 && encode(a0, a1, a2) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `dispatchStatus`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.dispatchStatus
const call = native.dispatchStatusWire
const encode = (a0, a1, a2, a3) => {
at = 0
if (!wDyn(a0)) return false
if (typeof a1 !== 'string') return false
wStr(a1)
if (typeof a2 !== 'string') return false
wStr(a2)
if (!wDyn(a3)) return false
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
o1["state"] = v1
break
}
case 1: {
let v1
v1 = rStr()
o1["text"] = v1
break
}
case 2: {
let v1
if (rU8() === 0) v1 = null
else {
const n2 = rU32()
const a2 = new Array(n2)
for (let i2 = 0; i2 < n2; i2++) {
let v2
v2 = rDyn()
a2[i2] = v2
}
v1 = a2
}
o1["args"] = v1
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
module.exports.dispatchStatus = function dispatchStatus (a0, a1, a2, a3) {
  if (illFormed(a1)) throw new TypeError('argument 2 of dispatchStatus has a lone surrogate, which a Rust String cannot hold')
  if (illFormed(a2)) throw new TypeError('argument 3 of dispatchStatus has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 4 && encode(a0, a1, a2, a3) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `routeNarrative`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.routeNarrative
const call = native.routeNarrativeWire
const encode = (a0, a1, a2) => {
at = 0
if (typeof a0 !== 'string') return false
wStr(a0)
if (!wDyn(a1)) return false
if (!wDyn(a2)) return false
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
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["tone"] = v1
break
}
case 1: {
let v1
v1 = rStr()
o1["text"] = v1
break
}
case 2: {
let v1
if (rU8() === 0) v1 = null
else {
const n2 = rU32()
const a2 = new Array(n2)
for (let i2 = 0; i2 < n2; i2++) {
let v2
v2 = rDyn()
a2[i2] = v2
}
v1 = a2
}
o1["args"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
v0 = o1
}
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.routeNarrative = function routeNarrative (a0, a1, a2) {
  if (illFormed(a0)) throw new TypeError('argument 1 of routeNarrative has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 3 && encode(a0, a1, a2) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `connectedAgents`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.connectedAgents
const call = native.connectedAgentsWire
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
v0 = rStr()
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.connectedAgents = function connectedAgents (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `modelDispatch`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.modelDispatch
const call = native.modelDispatchWire
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
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["model"] = v1
break
}
case 1: {
let v1
if (rU8() === 0) v1 = null
else {
const n2 = rU32()
const a2 = new Array(n2)
for (let i2 = 0; i2 < n2; i2++) {
let v2
{
const o3 = {}
const n3 = rU8()
for (let j3 = 0; j3 < n3; j3++) {
switch (rU8()) {
case 0: {
let v3
v3 = rStr()
o3["id"] = v3
break
}
case 1: {
let v3
v3 = rStr()
o3["name"] = v3
break
}
case 2: {
let v3
v3 = rStr()
o3["baseUrl"] = v3
break
}
case 3: {
let v3
if (rU8() === 0) v3 = null
else {
const n4 = rU32()
const a4 = new Array(n4)
for (let i4 = 0; i4 < n4; i4++) {
let v4
v4 = rStr()
a4[i4] = v4
}
v3 = a4
}
o3["excludedBy"] = v3
break
}
default: {
const k3 = rStr()
o3[k3] = rDyn()
}
}
}
v2 = o3
}
a2[i2] = v2
}
v1 = a2
}
o1["providers"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
v0 = o1
}
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.modelDispatch = function modelDispatch (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `activeSponsorIds`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.activeSponsorIds
const call = native.activeSponsorIdsWire
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
v0 = rStr()
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.activeSponsorIds = function activeSponsorIds (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `gatewayPresets`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.gatewayPresets
const call = native.gatewayPresetsWire
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
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["key"] = v1
break
}
case 1: {
let v1
v1 = rStr()
o1["name"] = v1
break
}
case 2: {
let v1
v1 = rStr()
o1["baseUrl"] = v1
break
}
case 3: {
let v1
v1 = rDyn()
o1["protocol"] = v1
break
}
case 4: {
let v1
v1 = rStr()
o1["model"] = v1
break
}
case 5: {
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
o1["modelList"] = v1
break
}
case 6: {
let v1
v1 = rStr()
o1["section"] = v1
break
}
case 7: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["defaultKey"] = v1
break
}
case 8: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["apiKeyUrl"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
v0 = o1
}
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.gatewayPresets = function gatewayPresets (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `presetAdded`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.presetAdded
const call = native.presetAddedWire
const encode = (a0, a1) => {
at = 0
if (!wDyn(a0)) return false
if (!wDyn(a1)) return false
return true
}
const decode = () => {
let t
t = rU8() === 1
return t
}
module.exports.presetAdded = function presetAdded (a0, a1) {
  try {
    if (arguments.length <= 2 && encode(a0, a1) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `providerDraftFromPreset`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.providerDraftFromPreset
const call = native.providerDraftFromPresetWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
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
o0["name"] = v0
break
}
case 1: {
let v0
v0 = rStr()
o0["baseUrl"] = v0
break
}
case 2: {
let v0
v0 = rStr()
o0["keys"] = v0
break
}
case 3: {
let v0
v0 = rDyn()
o0["protocol"] = v0
break
}
case 4: {
let v0
v0 = rStr()
o0["model"] = v0
break
}
case 5: {
let v0
v0 = rStr()
o0["modelList"] = v0
break
}
case 6: {
let v0
v0 = rStr()
o0["priceCurrency"] = v0
break
}
case 7: {
let v0
v0 = rStr()
o0["priceInput"] = v0
break
}
case 8: {
let v0
v0 = rStr()
o0["priceCached"] = v0
break
}
case 9: {
let v0
v0 = rStr()
o0["priceOutput"] = v0
break
}
case 10: {
let v0
v0 = rStr()
o0["proxy"] = v0
break
}
case 11: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
if (rU8() === 0) v0 = null
else {
const n2 = rU32()
const o2 = {}
for (let i2 = 0; i2 < n2; i2++) {
const k2 = rStr()
let v2
{
const o3 = {}
const n3 = rU8()
for (let j3 = 0; j3 < n3; j3++) {
switch (rU8()) {
case 0: {
let v3
v3 = rStr()
o3["currency"] = v3
break
}
case 1: {
let v3
v3 = rF64()
o3["input"] = v3
break
}
case 2: {
let v3
v3 = rF64()
o3["cachedInput"] = v3
break
}
case 3: {
let v3
v3 = rF64()
o3["output"] = v3
break
}
default: {
const k3 = rStr()
o3[k3] = rDyn()
}
}
}
v2 = o3
}
o2[k2] = v2
}
v0 = o2
}
}
}
o0["modelPrices"] = v0
break
}
case 12: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
v0 = rU8() === 1
}
}
o0["stripImageGenerationTool"] = v0
break
}
case 13: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
{
const o2 = {}
const n2 = rU8()
for (let j2 = 0; j2 < n2; j2++) {
switch (rU8()) {
case 0: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rF64()
}
}
o2["defaultUsedPercentLimit"] = v2
break
}
case 1: {
let v2
if (rU8() === 0) v2 = null
else {
const n3 = rU32()
const o3 = {}
for (let i3 = 0; i3 < n3; i3++) {
const k3 = rStr()
let v3
v3 = rF64()
o3[k3] = v3
}
v2 = o3
}
o2["windowUsedPercentLimits"] = v2
break
}
case 2: {
let v2
v2 = rStr()
o2["onLimit"] = v2
break
}
case 3: {
let v2
v2 = rStr()
o2["unknownUsage"] = v2
break
}
case 4: {
let v2
v2 = rF64()
o2["maxAgeSeconds"] = v2
break
}
default: {
const k2 = rStr()
o2[k2] = rDyn()
}
}
}
v0 = o2
}
}
}
o0["subscriptionPolicy"] = v0
break
}
case 14: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
v0 = rU8() === 1
}
}
o0["stripResponsesImageDetail"] = v0
break
}
case 15: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
v0 = rU8() === 1
}
}
o0["disableToolSearch"] = v0
break
}
case 16: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
if (rU8() === 0) v0 = null
else {
const n2 = rU32()
const a2 = new Array(n2)
for (let i2 = 0; i2 < n2; i2++) {
let v2
{
const o3 = {}
const n3 = rU8()
for (let j3 = 0; j3 < n3; j3++) {
switch (rU8()) {
case 0: {
let v3
v3 = rStr()
o3["key"] = v3
break
}
case 1: {
let v3
v3 = rStr()
o3["value"] = v3
break
}
case 2: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
v3 = rStr()
}
}
o3["masked"] = v3
break
}
case 3: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
v3 = rStr()
}
}
o3["savedKey"] = v3
break
}
default: {
const k3 = rStr()
o3[k3] = rDyn()
}
}
}
v2 = o3
}
a2[i2] = v2
}
v0 = a2
}
}
}
o0["customHeaders"] = v0
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
module.exports.providerDraftFromPreset = function providerDraftFromPreset (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `providerDraftFromEntry`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.providerDraftFromEntry
const call = native.providerDraftFromEntryWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
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
o0["name"] = v0
break
}
case 1: {
let v0
v0 = rStr()
o0["baseUrl"] = v0
break
}
case 2: {
let v0
v0 = rStr()
o0["keys"] = v0
break
}
case 3: {
let v0
v0 = rDyn()
o0["protocol"] = v0
break
}
case 4: {
let v0
v0 = rStr()
o0["model"] = v0
break
}
case 5: {
let v0
v0 = rStr()
o0["modelList"] = v0
break
}
case 6: {
let v0
v0 = rStr()
o0["priceCurrency"] = v0
break
}
case 7: {
let v0
v0 = rStr()
o0["priceInput"] = v0
break
}
case 8: {
let v0
v0 = rStr()
o0["priceCached"] = v0
break
}
case 9: {
let v0
v0 = rStr()
o0["priceOutput"] = v0
break
}
case 10: {
let v0
v0 = rStr()
o0["proxy"] = v0
break
}
case 11: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
if (rU8() === 0) v0 = null
else {
const n2 = rU32()
const o2 = {}
for (let i2 = 0; i2 < n2; i2++) {
const k2 = rStr()
let v2
{
const o3 = {}
const n3 = rU8()
for (let j3 = 0; j3 < n3; j3++) {
switch (rU8()) {
case 0: {
let v3
v3 = rStr()
o3["currency"] = v3
break
}
case 1: {
let v3
v3 = rF64()
o3["input"] = v3
break
}
case 2: {
let v3
v3 = rF64()
o3["cachedInput"] = v3
break
}
case 3: {
let v3
v3 = rF64()
o3["output"] = v3
break
}
default: {
const k3 = rStr()
o3[k3] = rDyn()
}
}
}
v2 = o3
}
o2[k2] = v2
}
v0 = o2
}
}
}
o0["modelPrices"] = v0
break
}
case 12: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
v0 = rU8() === 1
}
}
o0["stripImageGenerationTool"] = v0
break
}
case 13: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
{
const o2 = {}
const n2 = rU8()
for (let j2 = 0; j2 < n2; j2++) {
switch (rU8()) {
case 0: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rF64()
}
}
o2["defaultUsedPercentLimit"] = v2
break
}
case 1: {
let v2
if (rU8() === 0) v2 = null
else {
const n3 = rU32()
const o3 = {}
for (let i3 = 0; i3 < n3; i3++) {
const k3 = rStr()
let v3
v3 = rF64()
o3[k3] = v3
}
v2 = o3
}
o2["windowUsedPercentLimits"] = v2
break
}
case 2: {
let v2
v2 = rStr()
o2["onLimit"] = v2
break
}
case 3: {
let v2
v2 = rStr()
o2["unknownUsage"] = v2
break
}
case 4: {
let v2
v2 = rF64()
o2["maxAgeSeconds"] = v2
break
}
default: {
const k2 = rStr()
o2[k2] = rDyn()
}
}
}
v0 = o2
}
}
}
o0["subscriptionPolicy"] = v0
break
}
case 14: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
v0 = rU8() === 1
}
}
o0["stripResponsesImageDetail"] = v0
break
}
case 15: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
v0 = rU8() === 1
}
}
o0["disableToolSearch"] = v0
break
}
case 16: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
if (rU8() === 0) v0 = null
else {
const n2 = rU32()
const a2 = new Array(n2)
for (let i2 = 0; i2 < n2; i2++) {
let v2
{
const o3 = {}
const n3 = rU8()
for (let j3 = 0; j3 < n3; j3++) {
switch (rU8()) {
case 0: {
let v3
v3 = rStr()
o3["key"] = v3
break
}
case 1: {
let v3
v3 = rStr()
o3["value"] = v3
break
}
case 2: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
v3 = rStr()
}
}
o3["masked"] = v3
break
}
case 3: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
v3 = rStr()
}
}
o3["savedKey"] = v3
break
}
default: {
const k3 = rStr()
o3[k3] = rDyn()
}
}
}
v2 = o3
}
a2[i2] = v2
}
v0 = a2
}
}
}
o0["customHeaders"] = v0
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
module.exports.providerDraftFromEntry = function providerDraftFromEntry (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `providerInput`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.providerInput
const call = native.providerInputWire
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
module.exports.providerInput = function providerInput (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `priceFromDraft`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.priceFromDraft
const call = native.priceFromDraftWire
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
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["currency"] = v1
break
}
case 1: {
let v1
v1 = rF64()
o1["input"] = v1
break
}
case 2: {
let v1
v1 = rF64()
o1["cachedInput"] = v1
break
}
case 3: {
let v1
v1 = rF64()
o1["output"] = v1
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
module.exports.priceFromDraft = function priceFromDraft (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `proxyAddressError`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.proxyAddressError
const call = native.proxyAddressErrorWire
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
module.exports.proxyAddressError = function proxyAddressError (a0) {
  if (illFormed(a0)) throw new TypeError('argument 1 of proxyAddressError has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `validateProviderDraft`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.validateProviderDraft
const call = native.validateProviderDraftWire
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
module.exports.validateProviderDraft = function validateProviderDraft (a0, a1) {
  try {
    if (arguments.length <= 2 && encode(a0, a1) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `usageRangeStart`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.usageRangeStart
const call = native.usageRangeStartWire
const encode = (a0, a1) => {
at = 0
if (typeof a0 !== 'string') return false
wStr(a0)
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
t = rF64()
}
}
return t
}
module.exports.usageRangeStart = function usageRangeStart (a0, a1) {
  if (illFormed(a0)) throw new TypeError('argument 1 of usageRangeStart has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 2 && encode(a0, a1) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `relayFilterFromFocus`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.relayFilterFromFocus
const call = native.relayFilterFromFocusWire
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
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["id"] = v1
break
}
case 1: {
let v1
v1 = rStr()
o1["label"] = v1
break
}
case 2: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["unknown"] = v1
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
module.exports.relayFilterFromFocus = function relayFilterFromFocus (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `relayFilterArgs`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.relayFilterArgs
const call = native.relayFilterArgsWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
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
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
v0 = rStr()
}
}
o0["relayId"] = v0
break
}
case 1: {
let v0
v0 = rU8() === 1
o0["relayUnknown"] = v0
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
module.exports.relayFilterArgs = function relayFilterArgs (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `formatRequestCost`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.formatRequestCost
const call = native.formatRequestCostWire
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
module.exports.formatRequestCost = function formatRequestCost (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `cacheHitRate`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.cacheHitRate
const call = native.cacheHitRateWire
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
t = rF64()
}
}
return t
}
module.exports.cacheHitRate = function cacheHitRate (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `averageDuration`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.averageDuration
const call = native.averageDurationWire
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
t = rF64()
}
}
return t
}
module.exports.averageDuration = function averageDuration (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `costText`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.costText
const call = native.costTextWire
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
module.exports.costText = function costText (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `usageDayStats`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.usageDayStats
const call = native.usageDayStatsWire
const encode = (a0) => {
at = 0
if (!wDyn(a0)) return false
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
v0 = rF64()
o0["activeDays"] = v0
break
}
case 1: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
{
const o2 = {}
const n2 = rU8()
for (let j2 = 0; j2 < n2; j2++) {
switch (rU8()) {
case 0: {
let v2
v2 = rF64()
o2["calls"] = v2
break
}
case 1: {
let v2
v2 = rF64()
o2["errors"] = v2
break
}
case 2: {
let v2
v2 = rF64()
o2["inputTokens"] = v2
break
}
case 3: {
let v2
v2 = rF64()
o2["outputTokens"] = v2
break
}
case 4: {
let v2
v2 = rF64()
o2["cachedTokens"] = v2
break
}
case 5: {
let v2
v2 = rF64()
o2["reasoningTokens"] = v2
break
}
case 6: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rF64()
}
}
o2["durationMs"] = v2
break
}
case 7: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rF64()
}
}
o2["timedCalls"] = v2
break
}
case 8: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rF64()
}
}
o2["images"] = v2
break
}
case 9: {
let v2
v2 = rStr()
o2["date"] = v2
break
}
default: {
const k2 = rStr()
o2[k2] = rDyn()
}
}
}
v0 = o2
}
}
}
o0["peak"] = v0
break
}
case 2: {
let v0
v0 = rF64()
o0["dailyAverage"] = v0
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
module.exports.usageDayStats = function usageDayStats (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `axisLabelIndexes`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.axisLabelIndexes
const call = native.axisLabelIndexesWire
const encode = (a0, a1) => {
at = 0
if (typeof a0 !== 'number') return false
wF64(a0)
if (!wDyn(a1)) return false
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
v0 = rF64()
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.axisLabelIndexes = function axisLabelIndexes (a0, a1) {
  try {
    if (arguments.length <= 2 && encode(a0, a1) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `shortDate`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.shortDate
const call = native.shortDateWire
const encode = (a0, a1) => {
at = 0
if (typeof a0 !== 'string') return false
wStr(a0)
if (!wDyn(a1)) return false
return true
}
const decode = () => {
let t
t = rStr()
return t
}
module.exports.shortDate = function shortDate (a0, a1) {
  if (illFormed(a0)) throw new TypeError('argument 1 of shortDate has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 2 && encode(a0, a1) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `filterSessions`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.filterSessions
const call = native.filterSessionsWire
const encode = (a0, a1, a2) => {
at = 0
if (!wDyn(a0)) return false
if (!wDyn(a1)) return false
if (typeof a2 !== 'string') return false
wStr(a2)
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
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["agent"] = v1
break
}
case 1: {
let v1
v1 = rStr()
o1["id"] = v1
break
}
case 2: {
let v1
v1 = rStr()
o1["title"] = v1
break
}
case 3: {
let v1
v1 = rStr()
o1["cwd"] = v1
break
}
case 4: {
let v1
v1 = rF64()
o1["updatedAtMs"] = v1
break
}
case 5: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["surface"] = v1
break
}
case 6: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["archived"] = v1
break
}
case 7: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["resumeId"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
v0 = o1
}
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.filterSessions = function filterSessions (a0, a1, a2) {
  if (illFormed(a2)) throw new TypeError('argument 3 of filterSessions has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 3 && encode(a0, a1, a2) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `relativeTime`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.relativeTime
const call = native.relativeTimeWire
const encode = (a0, a1) => {
at = 0
if (typeof a0 !== 'number') return false
wF64(a0)
if (typeof a1 !== 'number') return false
wF64(a1)
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
o0["text"] = v0
break
}
case 1: {
let v0
if (rU8() === 0) v0 = null
else {
const n1 = rU32()
const a1 = new Array(n1)
for (let i1 = 0; i1 < n1; i1++) {
let v1
v1 = rDyn()
a1[i1] = v1
}
v0 = a1
}
o0["args"] = v0
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
module.exports.relativeTime = function relativeTime (a0, a1) {
  try {
    if (arguments.length <= 2 && encode(a0, a1) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `importCandidates`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.importCandidates
const call = native.importCandidatesWire
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
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["source"] = v1
break
}
case 1: {
let v1
v1 = rStr()
o1["id"] = v1
break
}
case 2: {
let v1
v1 = rStr()
o1["name"] = v1
break
}
case 3: {
let v1
v1 = rU8() === 1
o1["imported"] = v1
break
}
default: {
const k1 = rStr()
o1[k1] = rDyn()
}
}
}
v0 = o1
}
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.importCandidates = function importCandidates (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `pendingImportCount`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.pendingImportCount
const call = native.pendingImportCountWire
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
module.exports.pendingImportCount = function pendingImportCount (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `isFavoriteModel`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.isFavoriteModel
const call = native.isFavoriteModelWire
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
t = rU8() === 1
return t
}
module.exports.isFavoriteModel = function isFavoriteModel (a0, a1, a2) {
  if (illFormed(a1)) throw new TypeError('argument 2 of isFavoriteModel has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 3 && encode(a0, a1, a2) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `toggleFavoriteModel`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.toggleFavoriteModel
const call = native.toggleFavoriteModelWire
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
if (rU8() === 0) t = null
else {
const n0 = rU32()
const a0 = new Array(n0)
for (let i0 = 0; i0 < n0; i0++) {
let v0
v0 = rStr()
a0[i0] = v0
}
t = a0
}
return t
}
module.exports.toggleFavoriteModel = function toggleFavoriteModel (a0, a1, a2) {
  if (illFormed(a1)) throw new TypeError('argument 2 of toggleFavoriteModel has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 3 && encode(a0, a1, a2) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `activeAutoGroup`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.activeAutoGroup
const call = native.activeAutoGroupWire
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
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["id"] = v1
break
}
case 1: {
let v1
v1 = rStr()
o1["name"] = v1
break
}
case 2: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["disabled"] = v1
break
}
case 3: {
let v1
v1 = rStr()
o1["mode"] = v1
break
}
case 4: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["sticky"] = v1
break
}
case 5: {
let v1
if (rU8() === 0) v1 = null
else {
const n2 = rU32()
const a2 = new Array(n2)
for (let i2 = 0; i2 < n2; i2++) {
let v2
{
const o3 = {}
const n3 = rU8()
for (let j3 = 0; j3 < n3; j3++) {
switch (rU8()) {
case 0: {
let v3
v3 = rStr()
o3["model"] = v3
break
}
case 1: {
let v3
v3 = rF64()
o3["weight"] = v3
break
}
default: {
const k3 = rStr()
o3[k3] = rDyn()
}
}
}
v2 = o3
}
a2[i2] = v2
}
v1 = a2
}
o1["members"] = v1
break
}
case 6: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rU8() === 1
}
}
o1["hideMembers"] = v1
break
}
case 7: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["pick"] = v1
break
}
case 8: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["imageInputModel"] = v1
break
}
case 9: {
let v1
v1 = rU8() === 1
o1["hidden"] = v1
break
}
case 10: {
let v1
v1 = rU8() === 1
o1["customized"] = v1
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
module.exports.activeAutoGroup = function activeAutoGroup (a0, a1) {
  if (illFormed(a1)) throw new TypeError('argument 2 of activeAutoGroup has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 2 && encode(a0, a1) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `memberTarget`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.memberTarget
const call = native.memberTargetWire
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
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
{
const o2 = {}
const n2 = rU8()
for (let j2 = 0; j2 < n2; j2++) {
switch (rU8()) {
case 0: {
let v2
v2 = rStr()
o2["id"] = v2
break
}
case 1: {
let v2
v2 = rStr()
o2["name"] = v2
break
}
case 2: {
let v2
v2 = rStr()
o2["baseUrl"] = v2
break
}
case 3: {
let v2
v2 = rStr()
o2["protocol"] = v2
break
}
case 4: {
let v2
v2 = rU8() === 1
o2["official"] = v2
break
}
case 5: {
let v2
if (rU8() === 0) v2 = null
else {
const n3 = rU32()
const a3 = new Array(n3)
for (let i3 = 0; i3 < n3; i3++) {
let v3
v3 = rStr()
a3[i3] = v3
}
v2 = a3
}
o2["models"] = v2
break
}
case 6: {
let v2
v2 = rF64()
o2["keyCount"] = v2
break
}
case 7: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rU8() === 1
}
}
o2["enabled"] = v2
break
}
case 8: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rStr()
}
}
o2["source"] = v2
break
}
case 9: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
{
const o4 = {}
const n4 = rU8()
for (let j4 = 0; j4 < n4; j4++) {
switch (rU8()) {
case 0: {
let v4
v4 = rStr()
o4["currency"] = v4
break
}
case 1: {
let v4
v4 = rF64()
o4["input"] = v4
break
}
case 2: {
let v4
v4 = rF64()
o4["cachedInput"] = v4
break
}
case 3: {
let v4
v4 = rF64()
o4["output"] = v4
break
}
default: {
const k4 = rStr()
o4[k4] = rDyn()
}
}
}
v2 = o4
}
}
}
o2["price"] = v2
break
}
case 10: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
if (rU8() === 0) v2 = null
else {
const n4 = rU32()
const o4 = {}
for (let i4 = 0; i4 < n4; i4++) {
const k4 = rStr()
let v4
{
const o5 = {}
const n5 = rU8()
for (let j5 = 0; j5 < n5; j5++) {
switch (rU8()) {
case 0: {
let v5
v5 = rStr()
o5["currency"] = v5
break
}
case 1: {
let v5
v5 = rF64()
o5["input"] = v5
break
}
case 2: {
let v5
v5 = rF64()
o5["cachedInput"] = v5
break
}
case 3: {
let v5
v5 = rF64()
o5["output"] = v5
break
}
default: {
const k5 = rStr()
o5[k5] = rDyn()
}
}
}
v4 = o5
}
o4[k4] = v4
}
v2 = o4
}
}
}
o2["modelPrices"] = v2
break
}
case 11: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
if (rU8() === 0) v2 = null
else {
const n4 = rU32()
const a4 = new Array(n4)
for (let i4 = 0; i4 < n4; i4++) {
let v4
v4 = rStr()
a4[i4] = v4
}
v2 = a4
}
}
}
o2["imageModels"] = v2
break
}
case 12: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rStr()
}
}
o2["modelMetadata"] = v2
break
}
case 13: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
if (rU8() === 0) v2 = null
else {
const n4 = rU32()
const o4 = {}
for (let i4 = 0; i4 < n4; i4++) {
const k4 = rStr()
let v4
v4 = rF64()
o4[k4] = v4
}
v2 = o4
}
}
}
o2["modelWindows"] = v2
break
}
case 14: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rF64()
}
}
o2["contextWindow"] = v2
break
}
case 15: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
{
const o4 = {}
const n4 = rU8()
for (let j4 = 0; j4 < n4; j4++) {
switch (rU8()) {
case 0: {
let v4
{
const g5 = rU8()
if (g5 === 0) v4 = undefined
else if (g5 === 2) v4 = null
else {
if (rU8() === 0) v4 = null
else {
const n6 = rU32()
const o6 = {}
for (let i6 = 0; i6 < n6; i6++) {
const k6 = rStr()
let v6
v6 = rDyn()
o6[k6] = v6
}
v4 = o6
}
}
}
o4["models"] = v4
break
}
case 1: {
let v4
{
const g5 = rU8()
if (g5 === 0) v4 = undefined
else if (g5 === 2) v4 = null
else {
v4 = rF64()
}
}
o4["fallback"] = v4
break
}
default: {
const k4 = rStr()
o4[k4] = rDyn()
}
}
}
v2 = o4
}
}
}
o2["autoCompact"] = v2
break
}
case 16: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rStr()
}
}
o2["proxy"] = v2
break
}
case 17: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rF64()
}
}
o2["keyRpm"] = v2
break
}
case 18: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rF64()
}
}
o2["keyConcurrency"] = v2
break
}
case 19: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rU8() === 1
}
}
o2["stripImageGenerationTool"] = v2
break
}
case 20: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rU8() === 1
}
}
o2["stripResponsesImageDetail"] = v2
break
}
case 21: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rU8() === 1
}
}
o2["disableToolSearch"] = v2
break
}
case 22: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rStr()
}
}
o2["account"] = v2
break
}
case 23: {
let v2
v2 = rDyn()
o2["subscriptionPolicy"] = v2
break
}
case 24: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rStr()
}
}
o2["subscriptionBlockReason"] = v2
break
}
case 25: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rStr()
}
}
o2["plan"] = v2
break
}
case 26: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
if (rU8() === 0) v2 = null
else {
const n4 = rU32()
const a4 = new Array(n4)
for (let i4 = 0; i4 < n4; i4++) {
let v4
v4 = rStr()
a4[i4] = v4
}
v2 = a4
}
}
}
o2["upstreamNew"] = v2
break
}
case 27: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
if (rU8() === 0) v2 = null
else {
const n4 = rU32()
const a4 = new Array(n4)
for (let i4 = 0; i4 < n4; i4++) {
let v4
v4 = rStr()
a4[i4] = v4
}
v2 = a4
}
}
}
o2["upstreamGone"] = v2
break
}
case 28: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rU8() === 1
}
}
o2["lentToCodex"] = v2
break
}
case 29: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
{
const o4 = {}
const n4 = rU8()
for (let j4 = 0; j4 < n4; j4++) {
switch (rU8()) {
case 0: {
let v4
if (rU8() === 0) v4 = null
else {
const n5 = rU32()
const a5 = new Array(n5)
for (let i5 = 0; i5 < n5; i5++) {
let v5
{
const o6 = {}
const n6 = rU8()
for (let j6 = 0; j6 < n6; j6++) {
switch (rU8()) {
case 0: {
let v6
v6 = rDyn()
o6["windowMinutes"] = v6
break
}
case 1: {
let v6
v6 = rDyn()
o6["usedPercent"] = v6
break
}
case 2: {
let v6
v6 = rDyn()
o6["resetsAtMs"] = v6
break
}
case 3: {
let v6
v6 = rDyn()
o6["observedAtMs"] = v6
break
}
default: {
const k6 = rStr()
o6[k6] = rDyn()
}
}
}
v5 = o6
}
a5[i5] = v5
}
v4 = a5
}
o4["windows"] = v4
break
}
case 1: {
let v4
v4 = rF64()
o4["updatedAtMs"] = v4
break
}
case 2: {
let v4
{
const g5 = rU8()
if (g5 === 0) v4 = undefined
else if (g5 === 2) v4 = null
else {
v4 = rU8() === 1
}
}
o4["windowsComplete"] = v4
break
}
case 3: {
let v4
{
const g5 = rU8()
if (g5 === 0) v4 = undefined
else if (g5 === 2) v4 = null
else {
v4 = rU8() === 1
}
}
o4["officialAllowed"] = v4
break
}
case 4: {
let v4
{
const g5 = rU8()
if (g5 === 0) v4 = undefined
else if (g5 === 2) v4 = null
else {
v4 = rU8() === 1
}
}
o4["officialLimitReached"] = v4
break
}
case 5: {
let v4
{
const g5 = rU8()
if (g5 === 0) v4 = undefined
else if (g5 === 2) v4 = null
else {
v4 = rU8() === 1
}
}
o4["creditsAvailable"] = v4
break
}
default: {
const k4 = rStr()
o4[k4] = rDyn()
}
}
}
v2 = o4
}
}
}
o2["quota"] = v2
break
}
case 30: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rU8() === 1
}
}
o2["quotaApi"] = v2
break
}
case 31: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
if (rU8() === 0) v2 = null
else {
const n4 = rU32()
const a4 = new Array(n4)
for (let i4 = 0; i4 < n4; i4++) {
let v4
{
const o5 = {}
const n5 = rU8()
for (let j5 = 0; j5 < n5; j5++) {
switch (rU8()) {
case 0: {
let v5
v5 = rStr()
o5["id"] = v5
break
}
case 1: {
let v5
v5 = rStr()
o5["masked"] = v5
break
}
default: {
const k5 = rStr()
o5[k5] = rDyn()
}
}
}
v4 = o5
}
a4[i4] = v4
}
v2 = a4
}
}
}
o2["keys"] = v2
break
}
case 32: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rU8() === 1
}
}
o2["balanceApi"] = v2
break
}
case 33: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
{
const o4 = {}
const n4 = rU8()
for (let j4 = 0; j4 < n4; j4++) {
switch (rU8()) {
case 0: {
let v4
v4 = rStr()
o4["masked"] = v4
break
}
case 1: {
let v4
v4 = rStr()
o4["userId"] = v4
break
}
default: {
const k4 = rStr()
o4[k4] = rDyn()
}
}
}
v2 = o4
}
}
}
o2["newapiAccess"] = v2
break
}
case 34: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
if (rU8() === 0) v2 = null
else {
const n4 = rU32()
const a4 = new Array(n4)
for (let i4 = 0; i4 < n4; i4++) {
let v4
{
const o5 = {}
const n5 = rU8()
for (let j5 = 0; j5 < n5; j5++) {
switch (rU8()) {
case 0: {
let v5
v5 = rStr()
o5["key"] = v5
break
}
case 1: {
let v5
v5 = rStr()
o5["value"] = v5
break
}
case 2: {
let v5
{
const g6 = rU8()
if (g6 === 0) v5 = undefined
else if (g6 === 2) v5 = null
else {
v5 = rStr()
}
}
o5["masked"] = v5
break
}
default: {
const k5 = rStr()
o5[k5] = rDyn()
}
}
}
v4 = o5
}
a4[i4] = v4
}
v2 = a4
}
}
}
o2["customHeaders"] = v2
break
}
default: {
const k2 = rStr()
o2[k2] = rDyn()
}
}
}
v1 = o2
}
o1["entry"] = v1
break
}
case 1: {
let v1
v1 = rStr()
o1["model"] = v1
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
module.exports.memberTarget = function memberTarget (a0, a1) {
  if (illFormed(a1)) throw new TypeError('argument 2 of memberTarget has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 2 && encode(a0, a1) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}
const guards = {
  imageModelLabel: [false, [0]],
  formatQueueWait: [false, []],
  splitModels: [false, [0]],
  groupModelId: [false, [0]],
  slugify: [false, [0]],
  failureLabel: [false, [0]],
  formatRemaining: [false, []],
  agentName: [false, [0]],
  prettyModelName: [false, [0]],
  groupModeLabel: [false, [0]],
  groupModeDescription: [false, [0]],
  groupStickyLabel: [false, [0]],
  groupStickyDescription: [false, [0]],
  isPattern: [false, [0]],
  globMatches: [false, [0, 1]],
  formatElapsed: [false, []],
  presetHost: [false, [0]],
  isDirectProxy: [false, [0]],
  formatTokens: [false, []],
  currencySymbol: [false, [0]],
  formatMoney: [false, [1]],
  niceCeiling: [false, []],
  shortPath: [false, [0, 1]],
  maskAccountName: [false, [0]],
  maskAccountText: [false, [0]],
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
