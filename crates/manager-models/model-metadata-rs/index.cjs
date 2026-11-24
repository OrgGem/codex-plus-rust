// Loads the wasm-bindgen (nodejs target) output built by `rust-anything build`.
const native = require('./pkg/model_metadata_rs.js')

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
  parseModelMetadataMap: [[0], 'json', false],
  serializeModelMetadataMap: [[1], 'raw', false],
  serializeModelMetadataDocument: [[0, 1, 0, 0], 'raw', false],
  parseModelRowName: [[0], 'json', false],
  suffixWindowTokens: [[0], 'json', false],
  suffixWindowString: [[0], 'json', false],
  builtinRowBackfillValue: [[0, 0, 0, 1], 'json', false],
  createActiveImportDraft: [[1], 'json', false],
  updateActiveImportDraft: [[1, 1], 'json', false],
  cancelActiveImportDraft: [[1], 'json', false],
  rematchActiveImportDraft: [[1, 0, 1], 'json', false],
  builtinEntryToImportDocument: [[1, 0], 'raw', false],
  metadataSourceTags: [[1], 'json', false],
  importDocumentSyncPatch: [[1, 1], 'json', false],
  metadataMatchesBuiltin: [[1, 1], 'raw', false],
  importSaveDecision: [[1], 'json', false],
  importPanelControls: [[1], 'json', false],
  replaceModelMetadataForSlug: [[0, 0, 1], 'raw', false],
  resolveModelMetadataRowKey: [[1, 1], 'json', false],
  remapModelMetadataSlugs: [[0, 1], 'raw', false],
  retainModelMetadataForSlugs: [[0, 1], 'raw', false],
  modelFastSupportMode: [[1], 'raw', false],
  setModelFastSupportMode: [[1, 0], 'json', false],
  synchronizeModelMetadataFastSupport: [[0, 0, 0], 'json', false],
  synchronizeModelMetadataLiteSupport: [[0, 0, 0], 'json', false],
  modelMultiAgentMode: [[1], 'raw', false],
  synchronizeModelMetadataMultiAgentVersion: [[0, 0, 0], 'json', false],
  synchronizeModelMetadataDocumentContextWindow: [[0, 0, 0], 'json', false],
  synchronizeModelMetadataDocumentLimits: [[0, 0, 0, 0], 'json', false],
  synchronizeModelMetadataDocumentLimitsPreview: [[0, 0, 0, 0], 'json', false],
  validateModelCapabilities: [[1], 'json', false],
  parseModelMetadataDocument: [[0, 0], 'json', false],
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

// `parseModelMetadataMap`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.parseModelMetadataMap
const call = native.parseModelMetadataMapWire
const encode = (a0) => {
at = 0
if (typeof a0 !== 'string') return false
wStr(a0)
return true
}
const decode = () => {
let t
if (rU8() === 0) t = null
else {
const n0 = rU32()
const o0 = {}
for (let i0 = 0; i0 < n0; i0++) {
const k0 = rStr()
let v0
if (rU8() === 0) v0 = null
else {
const n1 = rU32()
const o1 = {}
for (let i1 = 0; i1 < n1; i1++) {
const k1 = rStr()
let v1
v1 = rDyn()
o1[k1] = v1
}
v0 = o1
}
o0[k0] = v0
}
t = o0
}
return t
}
module.exports.parseModelMetadataMap = function parseModelMetadataMap (a0) {
  if (illFormed(a0)) throw new TypeError('argument 1 of parseModelMetadataMap has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `serializeModelMetadataMap`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.serializeModelMetadataMap
const call = native.serializeModelMetadataMapWire
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
module.exports.serializeModelMetadataMap = function serializeModelMetadataMap (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `serializeModelMetadataDocument`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.serializeModelMetadataDocument
const call = native.serializeModelMetadataDocumentWire
const encode = (a0, a1, a2, a3) => {
at = 0
if (typeof a0 !== 'string') return false
wStr(a0)
if (!wDyn(a1)) return false
if (typeof a2 !== 'string') return false
wStr(a2)
if (a3 === undefined) {
wU8(0)
} else if (a3 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof a3 !== 'string') return false
wStr(a3)
}
return true
}
const decode = () => {
let t
t = rStr()
return t
}
module.exports.serializeModelMetadataDocument = function serializeModelMetadataDocument (a0, a1, a2, a3) {
  if (illFormed(a0)) throw new TypeError('argument 1 of serializeModelMetadataDocument has a lone surrogate, which a Rust String cannot hold')
  if (illFormed(a2)) throw new TypeError('argument 3 of serializeModelMetadataDocument has a lone surrogate, which a Rust String cannot hold')
  if (illFormed(a3)) throw new TypeError('argument 4 of serializeModelMetadataDocument has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 4 && encode(a0, a1, a2, a3) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `parseModelRowName`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.parseModelRowName
const call = native.parseModelRowNameWire
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
o0["rawName"] = v0
break
}
case 1: {
let v0
v0 = rStr()
o0["canonicalSlug"] = v0
break
}
case 2: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
v0 = rStr()
}
}
o0["suffixWindow"] = v0
break
}
case 3: {
let v0
v0 = rStr()
o0["key"] = v0
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
module.exports.parseModelRowName = function parseModelRowName (a0) {
  if (illFormed(a0)) throw new TypeError('argument 1 of parseModelRowName has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `suffixWindowTokens`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.suffixWindowTokens
const call = native.suffixWindowTokensWire
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
module.exports.suffixWindowTokens = function suffixWindowTokens (a0) {
  if (illFormed(a0)) throw new TypeError('argument 1 of suffixWindowTokens has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `suffixWindowString`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.suffixWindowString
const call = native.suffixWindowStringWire
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
module.exports.suffixWindowString = function suffixWindowString (a0) {
  if (illFormed(a0)) throw new TypeError('argument 1 of suffixWindowString has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `builtinRowBackfillValue`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.builtinRowBackfillValue
const call = native.builtinRowBackfillValueWire
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
o0["window"] = v0
break
}
case 1: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
v0 = rStr()
}
}
o0["autoCompact"] = v0
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
module.exports.builtinRowBackfillValue = function builtinRowBackfillValue (a0, a1, a2, a3) {
  if (illFormed(a0)) throw new TypeError('argument 1 of builtinRowBackfillValue has a lone surrogate, which a Rust String cannot hold')
  if (illFormed(a1)) throw new TypeError('argument 2 of builtinRowBackfillValue has a lone surrogate, which a Rust String cannot hold')
  if (illFormed(a2)) throw new TypeError('argument 3 of builtinRowBackfillValue has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 4 && encode(a0, a1, a2, a3) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `createActiveImportDraft`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.createActiveImportDraft
const call = native.createActiveImportDraftWire
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
o0["index"] = v0
break
}
case 1: {
let v0
v0 = rStr()
o0["originalSlug"] = v0
break
}
case 2: {
let v0
v0 = rStr()
o0["originalWindow"] = v0
break
}
case 3: {
let v0
v0 = rStr()
o0["originalAutoCompact"] = v0
break
}
case 4: {
let v0
v0 = rStr()
o0["document"] = v0
break
}
case 5: {
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
o2["slug"] = v2
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
v3 = rDyn()
o3[k3] = v3
}
v2 = o3
}
o2["metadata"] = v2
break
}
case 2: {
let v2
if (rU8() === 0) v2 = null
else {
const n3 = rU32()
const o3 = {}
for (let i3 = 0; i3 < n3; i3++) {
const k3 = rStr()
let v3
v3 = rDyn()
o3[k3] = v3
}
v2 = o3
}
o2["documentEntry"] = v2
break
}
case 3: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rStr()
}
}
o2["contextWindow"] = v2
break
}
case 4: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rStr()
}
}
o2["autoCompactPercent"] = v2
break
}
case 5: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rStr()
}
}
o2["autoCompactCalculationPercent"] = v2
break
}
case 6: {
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
o2["ignoredFields"] = v2
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
o0["preview"] = v0
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
module.exports.createActiveImportDraft = function createActiveImportDraft (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `updateActiveImportDraft`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.updateActiveImportDraft
const call = native.updateActiveImportDraftWire
const encode = (a0, a1) => {
at = 0
if (!wDyn(a0)) return false
if (!wDyn(a1)) return false
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
o0["index"] = v0
break
}
case 1: {
let v0
v0 = rStr()
o0["originalSlug"] = v0
break
}
case 2: {
let v0
v0 = rStr()
o0["originalWindow"] = v0
break
}
case 3: {
let v0
v0 = rStr()
o0["originalAutoCompact"] = v0
break
}
case 4: {
let v0
v0 = rStr()
o0["document"] = v0
break
}
case 5: {
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
o2["slug"] = v2
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
v3 = rDyn()
o3[k3] = v3
}
v2 = o3
}
o2["metadata"] = v2
break
}
case 2: {
let v2
if (rU8() === 0) v2 = null
else {
const n3 = rU32()
const o3 = {}
for (let i3 = 0; i3 < n3; i3++) {
const k3 = rStr()
let v3
v3 = rDyn()
o3[k3] = v3
}
v2 = o3
}
o2["documentEntry"] = v2
break
}
case 3: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rStr()
}
}
o2["contextWindow"] = v2
break
}
case 4: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rStr()
}
}
o2["autoCompactPercent"] = v2
break
}
case 5: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rStr()
}
}
o2["autoCompactCalculationPercent"] = v2
break
}
case 6: {
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
o2["ignoredFields"] = v2
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
o0["preview"] = v0
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
module.exports.updateActiveImportDraft = function updateActiveImportDraft (a0, a1) {
  try {
    if (arguments.length <= 2 && encode(a0, a1) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `cancelActiveImportDraft`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.cancelActiveImportDraft
const call = native.cancelActiveImportDraftWire
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
v0 = rDyn()
o0["draft"] = v0
break
}
case 1: {
let v0
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["window"] = v1
break
}
case 1: {
let v1
v1 = rStr()
o1["autoCompact"] = v1
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
o0["rowPatch"] = v0
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
module.exports.cancelActiveImportDraft = function cancelActiveImportDraft (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `rematchActiveImportDraft`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.rematchActiveImportDraft
const call = native.rematchActiveImportDraftWire
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
v0 = rF64()
o0["index"] = v0
break
}
case 1: {
let v0
v0 = rStr()
o0["originalSlug"] = v0
break
}
case 2: {
let v0
v0 = rStr()
o0["originalWindow"] = v0
break
}
case 3: {
let v0
v0 = rStr()
o0["originalAutoCompact"] = v0
break
}
case 4: {
let v0
v0 = rStr()
o0["document"] = v0
break
}
case 5: {
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
o2["slug"] = v2
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
v3 = rDyn()
o3[k3] = v3
}
v2 = o3
}
o2["metadata"] = v2
break
}
case 2: {
let v2
if (rU8() === 0) v2 = null
else {
const n3 = rU32()
const o3 = {}
for (let i3 = 0; i3 < n3; i3++) {
const k3 = rStr()
let v3
v3 = rDyn()
o3[k3] = v3
}
v2 = o3
}
o2["documentEntry"] = v2
break
}
case 3: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rStr()
}
}
o2["contextWindow"] = v2
break
}
case 4: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rStr()
}
}
o2["autoCompactPercent"] = v2
break
}
case 5: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rStr()
}
}
o2["autoCompactCalculationPercent"] = v2
break
}
case 6: {
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
o2["ignoredFields"] = v2
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
o0["preview"] = v0
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
module.exports.rematchActiveImportDraft = function rematchActiveImportDraft (a0, a1, a2) {
  if (illFormed(a1)) throw new TypeError('argument 2 of rematchActiveImportDraft has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 3 && encode(a0, a1, a2) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `builtinEntryToImportDocument`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.builtinEntryToImportDocument
const call = native.builtinEntryToImportDocumentWire
const encode = (a0, a1) => {
at = 0
if (!wDyn(a0)) return false
if (a1 === undefined) {
wU8(0)
} else if (a1 === null) {
return false
wU8(2)
} else {
wU8(1)
if (typeof a1 !== 'string') return false
wStr(a1)
}
return true
}
const decode = () => {
let t
t = rStr()
return t
}
module.exports.builtinEntryToImportDocument = function builtinEntryToImportDocument (a0, a1) {
  if (illFormed(a1)) throw new TypeError('argument 2 of builtinEntryToImportDocument has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 2 && encode(a0, a1) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `metadataSourceTags`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.metadataSourceTags
const call = native.metadataSourceTagsWire
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
module.exports.metadataSourceTags = function metadataSourceTags (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `importDocumentSyncPatch`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.importDocumentSyncPatch
const call = native.importDocumentSyncPatchWire
const encode = (a0, a1) => {
at = 0
if (!wDyn(a0)) return false
if (!wDyn(a1)) return false
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
o0["window"] = v0
break
}
case 1: {
let v0
{
const g1 = rU8()
if (g1 === 0) v0 = undefined
else if (g1 === 2) v0 = null
else {
v0 = rStr()
}
}
o0["autoCompact"] = v0
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
module.exports.importDocumentSyncPatch = function importDocumentSyncPatch (a0, a1) {
  try {
    if (arguments.length <= 2 && encode(a0, a1) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `metadataMatchesBuiltin`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.metadataMatchesBuiltin
const call = native.metadataMatchesBuiltinWire
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
module.exports.metadataMatchesBuiltin = function metadataMatchesBuiltin (a0, a1) {
  try {
    if (arguments.length <= 2 && encode(a0, a1) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `importSaveDecision`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.importSaveDecision
const call = native.importSaveDecisionWire
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
v0 = rU8() === 1
o0["needsSave"] = v0
break
}
case 1: {
let v0
v0 = rStr()
o0["effect"] = v0
break
}
case 2: {
let v0
v0 = rStr()
o0["label"] = v0
break
}
case 3: {
let v0
v0 = rStr()
o0["title"] = v0
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
module.exports.importSaveDecision = function importSaveDecision (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `importPanelControls`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.importPanelControls
const call = native.importPanelControlsWire
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
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rU8() === 1
o1["disabled"] = v1
break
}
case 1: {
let v1
v1 = rStr()
o1["title"] = v1
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
o0["rematch"] = v0
break
}
case 1: {
let v0
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rU8() === 1
o1["disabled"] = v1
break
}
case 1: {
let v1
v1 = rStr()
o1["title"] = v1
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
o0["clear"] = v0
break
}
case 2: {
let v0
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rU8() === 1
o1["disabled"] = v1
break
}
case 1: {
let v1
v1 = rStr()
o1["title"] = v1
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
o0["cancel"] = v0
break
}
case 3: {
let v0
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rU8() === 1
o1["disabled"] = v1
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
v1 = rStr()
o1["title"] = v1
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
o0["save"] = v0
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
module.exports.importPanelControls = function importPanelControls (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `replaceModelMetadataForSlug`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.replaceModelMetadataForSlug
const call = native.replaceModelMetadataForSlugWire
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
module.exports.replaceModelMetadataForSlug = function replaceModelMetadataForSlug (a0, a1, a2) {
  if (illFormed(a0)) throw new TypeError('argument 1 of replaceModelMetadataForSlug has a lone surrogate, which a Rust String cannot hold')
  if (illFormed(a1)) throw new TypeError('argument 2 of replaceModelMetadataForSlug has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 3 && encode(a0, a1, a2) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `resolveModelMetadataRowKey`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.resolveModelMetadataRowKey
const call = native.resolveModelMetadataRowKeyWire
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
module.exports.resolveModelMetadataRowKey = function resolveModelMetadataRowKey (a0, a1) {
  try {
    if (arguments.length <= 2 && encode(a0, a1) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `remapModelMetadataSlugs`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.remapModelMetadataSlugs
const call = native.remapModelMetadataSlugsWire
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
module.exports.remapModelMetadataSlugs = function remapModelMetadataSlugs (a0, a1) {
  if (illFormed(a0)) throw new TypeError('argument 1 of remapModelMetadataSlugs has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 2 && encode(a0, a1) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `retainModelMetadataForSlugs`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.retainModelMetadataForSlugs
const call = native.retainModelMetadataForSlugsWire
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
module.exports.retainModelMetadataForSlugs = function retainModelMetadataForSlugs (a0, a1) {
  if (illFormed(a0)) throw new TypeError('argument 1 of retainModelMetadataForSlugs has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 2 && encode(a0, a1) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `modelFastSupportMode`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.modelFastSupportMode
const call = native.modelFastSupportModeWire
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
module.exports.modelFastSupportMode = function modelFastSupportMode (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `setModelFastSupportMode`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.setModelFastSupportMode
const call = native.setModelFastSupportModeWire
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
const o0 = {}
for (let i0 = 0; i0 < n0; i0++) {
const k0 = rStr()
let v0
v0 = rDyn()
o0[k0] = v0
}
t = o0
}
return t
}
module.exports.setModelFastSupportMode = function setModelFastSupportMode (a0, a1) {
  if (illFormed(a1)) throw new TypeError('argument 2 of setModelFastSupportMode has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 2 && encode(a0, a1) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `synchronizeModelMetadataFastSupport`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.synchronizeModelMetadataFastSupport
const call = native.synchronizeModelMetadataFastSupportWire
const encode = (a0, a1, a2) => {
at = 0
if (typeof a0 !== 'string') return false
wStr(a0)
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
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["document"] = v1
break
}
case 1: {
let v1
{
const o2 = {}
const n2 = rU8()
for (let j2 = 0; j2 < n2; j2++) {
switch (rU8()) {
case 0: {
let v2
v2 = rStr()
o2["slug"] = v2
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
v3 = rDyn()
o3[k3] = v3
}
v2 = o3
}
o2["metadata"] = v2
break
}
case 2: {
let v2
if (rU8() === 0) v2 = null
else {
const n3 = rU32()
const o3 = {}
for (let i3 = 0; i3 < n3; i3++) {
const k3 = rStr()
let v3
v3 = rDyn()
o3[k3] = v3
}
v2 = o3
}
o2["documentEntry"] = v2
break
}
case 3: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rStr()
}
}
o2["contextWindow"] = v2
break
}
case 4: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rStr()
}
}
o2["autoCompactPercent"] = v2
break
}
case 5: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rStr()
}
}
o2["autoCompactCalculationPercent"] = v2
break
}
case 6: {
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
o2["ignoredFields"] = v2
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
o1["preview"] = v1
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
module.exports.synchronizeModelMetadataFastSupport = function synchronizeModelMetadataFastSupport (a0, a1, a2) {
  if (illFormed(a0)) throw new TypeError('argument 1 of synchronizeModelMetadataFastSupport has a lone surrogate, which a Rust String cannot hold')
  if (illFormed(a1)) throw new TypeError('argument 2 of synchronizeModelMetadataFastSupport has a lone surrogate, which a Rust String cannot hold')
  if (illFormed(a2)) throw new TypeError('argument 3 of synchronizeModelMetadataFastSupport has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 3 && encode(a0, a1, a2) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `synchronizeModelMetadataLiteSupport`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.synchronizeModelMetadataLiteSupport
const call = native.synchronizeModelMetadataLiteSupportWire
const encode = (a0, a1, a2) => {
at = 0
if (typeof a0 !== 'string') return false
wStr(a0)
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
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["document"] = v1
break
}
case 1: {
let v1
{
const o2 = {}
const n2 = rU8()
for (let j2 = 0; j2 < n2; j2++) {
switch (rU8()) {
case 0: {
let v2
v2 = rStr()
o2["slug"] = v2
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
v3 = rDyn()
o3[k3] = v3
}
v2 = o3
}
o2["metadata"] = v2
break
}
case 2: {
let v2
if (rU8() === 0) v2 = null
else {
const n3 = rU32()
const o3 = {}
for (let i3 = 0; i3 < n3; i3++) {
const k3 = rStr()
let v3
v3 = rDyn()
o3[k3] = v3
}
v2 = o3
}
o2["documentEntry"] = v2
break
}
case 3: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rStr()
}
}
o2["contextWindow"] = v2
break
}
case 4: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rStr()
}
}
o2["autoCompactPercent"] = v2
break
}
case 5: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rStr()
}
}
o2["autoCompactCalculationPercent"] = v2
break
}
case 6: {
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
o2["ignoredFields"] = v2
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
o1["preview"] = v1
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
module.exports.synchronizeModelMetadataLiteSupport = function synchronizeModelMetadataLiteSupport (a0, a1, a2) {
  if (illFormed(a0)) throw new TypeError('argument 1 of synchronizeModelMetadataLiteSupport has a lone surrogate, which a Rust String cannot hold')
  if (illFormed(a1)) throw new TypeError('argument 2 of synchronizeModelMetadataLiteSupport has a lone surrogate, which a Rust String cannot hold')
  if (illFormed(a2)) throw new TypeError('argument 3 of synchronizeModelMetadataLiteSupport has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 3 && encode(a0, a1, a2) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `modelMultiAgentMode`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.modelMultiAgentMode
const call = native.modelMultiAgentModeWire
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
module.exports.modelMultiAgentMode = function modelMultiAgentMode (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `synchronizeModelMetadataMultiAgentVersion`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.synchronizeModelMetadataMultiAgentVersion
const call = native.synchronizeModelMetadataMultiAgentVersionWire
const encode = (a0, a1, a2) => {
at = 0
if (typeof a0 !== 'string') return false
wStr(a0)
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
{
const o1 = {}
const n1 = rU8()
for (let j1 = 0; j1 < n1; j1++) {
switch (rU8()) {
case 0: {
let v1
v1 = rStr()
o1["document"] = v1
break
}
case 1: {
let v1
{
const o2 = {}
const n2 = rU8()
for (let j2 = 0; j2 < n2; j2++) {
switch (rU8()) {
case 0: {
let v2
v2 = rStr()
o2["slug"] = v2
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
v3 = rDyn()
o3[k3] = v3
}
v2 = o3
}
o2["metadata"] = v2
break
}
case 2: {
let v2
if (rU8() === 0) v2 = null
else {
const n3 = rU32()
const o3 = {}
for (let i3 = 0; i3 < n3; i3++) {
const k3 = rStr()
let v3
v3 = rDyn()
o3[k3] = v3
}
v2 = o3
}
o2["documentEntry"] = v2
break
}
case 3: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rStr()
}
}
o2["contextWindow"] = v2
break
}
case 4: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rStr()
}
}
o2["autoCompactPercent"] = v2
break
}
case 5: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rStr()
}
}
o2["autoCompactCalculationPercent"] = v2
break
}
case 6: {
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
o2["ignoredFields"] = v2
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
o1["preview"] = v1
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
module.exports.synchronizeModelMetadataMultiAgentVersion = function synchronizeModelMetadataMultiAgentVersion (a0, a1, a2) {
  if (illFormed(a0)) throw new TypeError('argument 1 of synchronizeModelMetadataMultiAgentVersion has a lone surrogate, which a Rust String cannot hold')
  if (illFormed(a1)) throw new TypeError('argument 2 of synchronizeModelMetadataMultiAgentVersion has a lone surrogate, which a Rust String cannot hold')
  if (illFormed(a2)) throw new TypeError('argument 3 of synchronizeModelMetadataMultiAgentVersion has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 3 && encode(a0, a1, a2) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `synchronizeModelMetadataDocumentContextWindow`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.synchronizeModelMetadataDocumentContextWindow
const call = native.synchronizeModelMetadataDocumentContextWindowWire
const encode = (a0, a1, a2) => {
at = 0
if (typeof a0 !== 'string') return false
wStr(a0)
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
t = rStr()
}
}
return t
}
module.exports.synchronizeModelMetadataDocumentContextWindow = function synchronizeModelMetadataDocumentContextWindow (a0, a1, a2) {
  if (illFormed(a0)) throw new TypeError('argument 1 of synchronizeModelMetadataDocumentContextWindow has a lone surrogate, which a Rust String cannot hold')
  if (illFormed(a1)) throw new TypeError('argument 2 of synchronizeModelMetadataDocumentContextWindow has a lone surrogate, which a Rust String cannot hold')
  if (illFormed(a2)) throw new TypeError('argument 3 of synchronizeModelMetadataDocumentContextWindow has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 3 && encode(a0, a1, a2) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `synchronizeModelMetadataDocumentLimits`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.synchronizeModelMetadataDocumentLimits
const call = native.synchronizeModelMetadataDocumentLimitsWire
const encode = (a0, a1, a2, a3) => {
at = 0
if (typeof a0 !== 'string') return false
wStr(a0)
if (typeof a1 !== 'string') return false
wStr(a1)
if (typeof a2 !== 'string') return false
wStr(a2)
if (typeof a3 !== 'string') return false
wStr(a3)
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
module.exports.synchronizeModelMetadataDocumentLimits = function synchronizeModelMetadataDocumentLimits (a0, a1, a2, a3) {
  if (illFormed(a0)) throw new TypeError('argument 1 of synchronizeModelMetadataDocumentLimits has a lone surrogate, which a Rust String cannot hold')
  if (illFormed(a1)) throw new TypeError('argument 2 of synchronizeModelMetadataDocumentLimits has a lone surrogate, which a Rust String cannot hold')
  if (illFormed(a2)) throw new TypeError('argument 3 of synchronizeModelMetadataDocumentLimits has a lone surrogate, which a Rust String cannot hold')
  if (illFormed(a3)) throw new TypeError('argument 4 of synchronizeModelMetadataDocumentLimits has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 4 && encode(a0, a1, a2, a3) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `synchronizeModelMetadataDocumentLimitsPreview`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.synchronizeModelMetadataDocumentLimitsPreview
const call = native.synchronizeModelMetadataDocumentLimitsPreviewWire
const encode = (a0, a1, a2, a3) => {
at = 0
if (typeof a0 !== 'string') return false
wStr(a0)
if (typeof a1 !== 'string') return false
wStr(a1)
if (typeof a2 !== 'string') return false
wStr(a2)
if (typeof a3 !== 'string') return false
wStr(a3)
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
o1["document"] = v1
break
}
case 1: {
let v1
{
const o2 = {}
const n2 = rU8()
for (let j2 = 0; j2 < n2; j2++) {
switch (rU8()) {
case 0: {
let v2
v2 = rStr()
o2["slug"] = v2
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
v3 = rDyn()
o3[k3] = v3
}
v2 = o3
}
o2["metadata"] = v2
break
}
case 2: {
let v2
if (rU8() === 0) v2 = null
else {
const n3 = rU32()
const o3 = {}
for (let i3 = 0; i3 < n3; i3++) {
const k3 = rStr()
let v3
v3 = rDyn()
o3[k3] = v3
}
v2 = o3
}
o2["documentEntry"] = v2
break
}
case 3: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rStr()
}
}
o2["contextWindow"] = v2
break
}
case 4: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rStr()
}
}
o2["autoCompactPercent"] = v2
break
}
case 5: {
let v2
{
const g3 = rU8()
if (g3 === 0) v2 = undefined
else if (g3 === 2) v2 = null
else {
v2 = rStr()
}
}
o2["autoCompactCalculationPercent"] = v2
break
}
case 6: {
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
o2["ignoredFields"] = v2
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
o1["preview"] = v1
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
module.exports.synchronizeModelMetadataDocumentLimitsPreview = function synchronizeModelMetadataDocumentLimitsPreview (a0, a1, a2, a3) {
  if (illFormed(a0)) throw new TypeError('argument 1 of synchronizeModelMetadataDocumentLimitsPreview has a lone surrogate, which a Rust String cannot hold')
  if (illFormed(a1)) throw new TypeError('argument 2 of synchronizeModelMetadataDocumentLimitsPreview has a lone surrogate, which a Rust String cannot hold')
  if (illFormed(a2)) throw new TypeError('argument 3 of synchronizeModelMetadataDocumentLimitsPreview has a lone surrogate, which a Rust String cannot hold')
  if (illFormed(a3)) throw new TypeError('argument 4 of synchronizeModelMetadataDocumentLimitsPreview has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 4 && encode(a0, a1, a2, a3) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `validateModelCapabilities`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.validateModelCapabilities
const call = native.validateModelCapabilitiesWire
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
module.exports.validateModelCapabilities = function validateModelCapabilities (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}

// `parseModelMetadataDocument`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.parseModelMetadataDocument
const call = native.parseModelMetadataDocumentWire
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
t = rDyn()
return t
}
module.exports.parseModelMetadataDocument = function parseModelMetadataDocument (a0, a1) {
  if (illFormed(a0)) throw new TypeError('argument 1 of parseModelMetadataDocument has a lone surrogate, which a Rust String cannot hold')
  if (illFormed(a1)) throw new TypeError('argument 2 of parseModelMetadataDocument has a lone surrogate, which a Rust String cannot hold')
  try {
    if (arguments.length <= 2 && encode(a0, a1) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}
const guards = {
  modelMetadataKey: [false, [0]],
  modelSlugFromRowName: [false, [0]],
  normalizeTokenCountInput: [false, [0]],
  clearModelMetadataForSlug: [false, [0, 1]],
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
