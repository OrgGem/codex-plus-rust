// Loads the wasm-bindgen (nodejs target) output built by `rust-anything build`.
const native = require('./pkg/extension_config_schema_rs.js')

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
  schemasteryFields: [[1], 'json', false],
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

// `schemasteryFields`: binary wire; the JSON version handles inputs of other shapes.
{
const json = module.exports.schemasteryFields
const call = native.schemasteryFieldsWire
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
o1["type"] = v1
break
}
case 2: {
let v1
v1 = rDyn()
o1["title"] = v1
break
}
case 3: {
let v1
v1 = rDyn()
o1["description"] = v1
break
}
case 4: {
let v1
v1 = rDyn()
o1["default"] = v1
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
v3 = rDyn()
o3["value"] = v3
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
o1["options"] = v1
break
}
case 6: {
let v1
v1 = rU8() === 1
o1["secret"] = v1
break
}
case 7: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rF64()
}
}
o1["min"] = v1
break
}
case 8: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rF64()
}
}
o1["max"] = v1
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
o3["key"] = v3
break
}
case 1: {
let v3
v3 = rStr()
o3["type"] = v3
break
}
case 2: {
let v3
v3 = rDyn()
o3["title"] = v3
break
}
case 3: {
let v3
v3 = rDyn()
o3["description"] = v3
break
}
case 4: {
let v3
v3 = rDyn()
o3["default"] = v3
break
}
case 5: {
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
v5 = rDyn()
o5["value"] = v5
break
}
case 1: {
let v5
v5 = rStr()
o5["label"] = v5
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
o3["options"] = v3
break
}
case 6: {
let v3
v3 = rU8() === 1
o3["secret"] = v3
break
}
case 7: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
v3 = rF64()
}
}
o3["min"] = v3
break
}
case 8: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
v3 = rF64()
}
}
o3["max"] = v3
break
}
case 9: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
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
o5["type"] = v5
break
}
case 2: {
let v5
v5 = rDyn()
o5["title"] = v5
break
}
case 3: {
let v5
v5 = rDyn()
o5["description"] = v5
break
}
case 4: {
let v5
v5 = rDyn()
o5["default"] = v5
break
}
case 5: {
let v5
if (rU8() === 0) v5 = null
else {
const n6 = rU32()
const a6 = new Array(n6)
for (let i6 = 0; i6 < n6; i6++) {
let v6
{
const o7 = {}
const n7 = rU8()
for (let j7 = 0; j7 < n7; j7++) {
switch (rU8()) {
case 0: {
let v7
v7 = rDyn()
o7["value"] = v7
break
}
case 1: {
let v7
v7 = rDyn()
o7["label"] = v7
break
}
default: {
const k7 = rStr()
o7[k7] = rDyn()
}
}
}
v6 = o7
}
a6[i6] = v6
}
v5 = a6
}
o5["options"] = v5
break
}
case 6: {
let v5
v5 = rU8() === 1
o5["secret"] = v5
break
}
case 7: {
let v5
{
const g6 = rU8()
if (g6 === 0) v5 = undefined
else if (g6 === 2) v5 = null
else {
v5 = rF64()
}
}
o5["min"] = v5
break
}
case 8: {
let v5
{
const g6 = rU8()
if (g6 === 0) v5 = undefined
else if (g6 === 2) v5 = null
else {
v5 = rF64()
}
}
o5["max"] = v5
break
}
case 9: {
let v5
{
const g6 = rU8()
if (g6 === 0) v5 = undefined
else if (g6 === 2) v5 = null
else {
{
const o7 = {}
const n7 = rU8()
for (let j7 = 0; j7 < n7; j7++) {
switch (rU8()) {
case 0: {
let v7
v7 = rStr()
o7["key"] = v7
break
}
case 1: {
let v7
v7 = rStr()
o7["type"] = v7
break
}
case 2: {
let v7
v7 = rDyn()
o7["title"] = v7
break
}
case 3: {
let v7
v7 = rDyn()
o7["description"] = v7
break
}
case 4: {
let v7
v7 = rDyn()
o7["default"] = v7
break
}
case 5: {
let v7
if (rU8() === 0) v7 = null
else {
const n8 = rU32()
const a8 = new Array(n8)
for (let i8 = 0; i8 < n8; i8++) {
let v8
v8 = rDyn()
a8[i8] = v8
}
v7 = a8
}
o7["options"] = v7
break
}
case 6: {
let v7
v7 = rU8() === 1
o7["secret"] = v7
break
}
case 7: {
let v7
{
const g8 = rU8()
if (g8 === 0) v7 = undefined
else if (g8 === 2) v7 = null
else {
v7 = rF64()
}
}
o7["min"] = v7
break
}
case 8: {
let v7
{
const g8 = rU8()
if (g8 === 0) v7 = undefined
else if (g8 === 2) v7 = null
else {
v7 = rF64()
}
}
o7["max"] = v7
break
}
case 9: {
let v7
{
const g8 = rU8()
if (g8 === 0) v7 = undefined
else if (g8 === 2) v7 = null
else {
{
const o9 = {}
const n9 = rU8()
for (let j9 = 0; j9 < n9; j9++) {
switch (rU8()) {
case 0: {
let v9
v9 = rDyn()
o9["key"] = v9
break
}
case 1: {
let v9
v9 = rDyn()
o9["type"] = v9
break
}
case 2: {
let v9
v9 = rDyn()
o9["title"] = v9
break
}
case 3: {
let v9
v9 = rDyn()
o9["description"] = v9
break
}
case 4: {
let v9
v9 = rDyn()
o9["default"] = v9
break
}
case 5: {
let v9
v9 = rDyn()
o9["options"] = v9
break
}
case 6: {
let v9
v9 = rDyn()
o9["secret"] = v9
break
}
case 7: {
let v9
v9 = rDyn()
o9["min"] = v9
break
}
case 8: {
let v9
v9 = rDyn()
o9["max"] = v9
break
}
case 9: {
let v9
v9 = rDyn()
o9["items"] = v9
break
}
case 10: {
let v9
v9 = rDyn()
o9["properties"] = v9
break
}
case 11: {
let v9
v9 = rDyn()
o9["format"] = v9
break
}
case 12: {
let v9
v9 = rDyn()
o9["extensions"] = v9
break
}
default: {
const k9 = rStr()
o9[k9] = rDyn()
}
}
}
v7 = o9
}
}
}
o7["items"] = v7
break
}
case 10: {
let v7
{
const g8 = rU8()
if (g8 === 0) v7 = undefined
else if (g8 === 2) v7 = null
else {
if (rU8() === 0) v7 = null
else {
const n9 = rU32()
const a9 = new Array(n9)
for (let i9 = 0; i9 < n9; i9++) {
let v9
v9 = rDyn()
a9[i9] = v9
}
v7 = a9
}
}
}
o7["properties"] = v7
break
}
case 11: {
let v7
{
const g8 = rU8()
if (g8 === 0) v7 = undefined
else if (g8 === 2) v7 = null
else {
v7 = rStr()
}
}
o7["format"] = v7
break
}
case 12: {
let v7
{
const g8 = rU8()
if (g8 === 0) v7 = undefined
else if (g8 === 2) v7 = null
else {
if (rU8() === 0) v7 = null
else {
const n9 = rU32()
const a9 = new Array(n9)
for (let i9 = 0; i9 < n9; i9++) {
let v9
v9 = rStr()
a9[i9] = v9
}
v7 = a9
}
}
}
o7["extensions"] = v7
break
}
default: {
const k7 = rStr()
o7[k7] = rDyn()
}
}
}
v5 = o7
}
}
}
o5["items"] = v5
break
}
case 10: {
let v5
{
const g6 = rU8()
if (g6 === 0) v5 = undefined
else if (g6 === 2) v5 = null
else {
if (rU8() === 0) v5 = null
else {
const n7 = rU32()
const a7 = new Array(n7)
for (let i7 = 0; i7 < n7; i7++) {
let v7
{
const o8 = {}
const n8 = rU8()
for (let j8 = 0; j8 < n8; j8++) {
switch (rU8()) {
case 0: {
let v8
v8 = rStr()
o8["key"] = v8
break
}
case 1: {
let v8
v8 = rStr()
o8["type"] = v8
break
}
case 2: {
let v8
v8 = rDyn()
o8["title"] = v8
break
}
case 3: {
let v8
v8 = rDyn()
o8["description"] = v8
break
}
case 4: {
let v8
v8 = rDyn()
o8["default"] = v8
break
}
case 5: {
let v8
v8 = rDyn()
o8["options"] = v8
break
}
case 6: {
let v8
v8 = rU8() === 1
o8["secret"] = v8
break
}
case 7: {
let v8
{
const g9 = rU8()
if (g9 === 0) v8 = undefined
else if (g9 === 2) v8 = null
else {
v8 = rF64()
}
}
o8["min"] = v8
break
}
case 8: {
let v8
{
const g9 = rU8()
if (g9 === 0) v8 = undefined
else if (g9 === 2) v8 = null
else {
v8 = rF64()
}
}
o8["max"] = v8
break
}
case 9: {
let v8
v8 = rDyn()
o8["items"] = v8
break
}
case 10: {
let v8
{
const g9 = rU8()
if (g9 === 0) v8 = undefined
else if (g9 === 2) v8 = null
else {
if (rU8() === 0) v8 = null
else {
const n10 = rU32()
const a10 = new Array(n10)
for (let i10 = 0; i10 < n10; i10++) {
let v10
v10 = rDyn()
a10[i10] = v10
}
v8 = a10
}
}
}
o8["properties"] = v8
break
}
case 11: {
let v8
{
const g9 = rU8()
if (g9 === 0) v8 = undefined
else if (g9 === 2) v8 = null
else {
v8 = rStr()
}
}
o8["format"] = v8
break
}
case 12: {
let v8
{
const g9 = rU8()
if (g9 === 0) v8 = undefined
else if (g9 === 2) v8 = null
else {
if (rU8() === 0) v8 = null
else {
const n10 = rU32()
const a10 = new Array(n10)
for (let i10 = 0; i10 < n10; i10++) {
let v10
v10 = rDyn()
a10[i10] = v10
}
v8 = a10
}
}
}
o8["extensions"] = v8
break
}
default: {
const k8 = rStr()
o8[k8] = rDyn()
}
}
}
v7 = o8
}
a7[i7] = v7
}
v5 = a7
}
}
}
o5["properties"] = v5
break
}
case 11: {
let v5
{
const g6 = rU8()
if (g6 === 0) v5 = undefined
else if (g6 === 2) v5 = null
else {
v5 = rStr()
}
}
o5["format"] = v5
break
}
case 12: {
let v5
{
const g6 = rU8()
if (g6 === 0) v5 = undefined
else if (g6 === 2) v5 = null
else {
if (rU8() === 0) v5 = null
else {
const n7 = rU32()
const a7 = new Array(n7)
for (let i7 = 0; i7 < n7; i7++) {
let v7
v7 = rStr()
a7[i7] = v7
}
v5 = a7
}
}
}
o5["extensions"] = v5
break
}
default: {
const k5 = rStr()
o5[k5] = rDyn()
}
}
}
v3 = o5
}
}
}
o3["items"] = v3
break
}
case 10: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
if (rU8() === 0) v3 = null
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
v6 = rStr()
o6["key"] = v6
break
}
case 1: {
let v6
v6 = rStr()
o6["type"] = v6
break
}
case 2: {
let v6
v6 = rDyn()
o6["title"] = v6
break
}
case 3: {
let v6
v6 = rDyn()
o6["description"] = v6
break
}
case 4: {
let v6
v6 = rDyn()
o6["default"] = v6
break
}
case 5: {
let v6
if (rU8() === 0) v6 = null
else {
const n7 = rU32()
const a7 = new Array(n7)
for (let i7 = 0; i7 < n7; i7++) {
let v7
v7 = rDyn()
a7[i7] = v7
}
v6 = a7
}
o6["options"] = v6
break
}
case 6: {
let v6
v6 = rU8() === 1
o6["secret"] = v6
break
}
case 7: {
let v6
{
const g7 = rU8()
if (g7 === 0) v6 = undefined
else if (g7 === 2) v6 = null
else {
v6 = rF64()
}
}
o6["min"] = v6
break
}
case 8: {
let v6
{
const g7 = rU8()
if (g7 === 0) v6 = undefined
else if (g7 === 2) v6 = null
else {
v6 = rF64()
}
}
o6["max"] = v6
break
}
case 9: {
let v6
{
const g7 = rU8()
if (g7 === 0) v6 = undefined
else if (g7 === 2) v6 = null
else {
{
const o8 = {}
const n8 = rU8()
for (let j8 = 0; j8 < n8; j8++) {
switch (rU8()) {
case 0: {
let v8
v8 = rStr()
o8["key"] = v8
break
}
case 1: {
let v8
v8 = rStr()
o8["type"] = v8
break
}
case 2: {
let v8
v8 = rDyn()
o8["title"] = v8
break
}
case 3: {
let v8
v8 = rDyn()
o8["description"] = v8
break
}
case 4: {
let v8
v8 = rDyn()
o8["default"] = v8
break
}
case 5: {
let v8
v8 = rDyn()
o8["options"] = v8
break
}
case 6: {
let v8
v8 = rU8() === 1
o8["secret"] = v8
break
}
case 7: {
let v8
{
const g9 = rU8()
if (g9 === 0) v8 = undefined
else if (g9 === 2) v8 = null
else {
v8 = rF64()
}
}
o8["min"] = v8
break
}
case 8: {
let v8
{
const g9 = rU8()
if (g9 === 0) v8 = undefined
else if (g9 === 2) v8 = null
else {
v8 = rF64()
}
}
o8["max"] = v8
break
}
case 9: {
let v8
v8 = rDyn()
o8["items"] = v8
break
}
case 10: {
let v8
{
const g9 = rU8()
if (g9 === 0) v8 = undefined
else if (g9 === 2) v8 = null
else {
if (rU8() === 0) v8 = null
else {
const n10 = rU32()
const a10 = new Array(n10)
for (let i10 = 0; i10 < n10; i10++) {
let v10
v10 = rDyn()
a10[i10] = v10
}
v8 = a10
}
}
}
o8["properties"] = v8
break
}
case 11: {
let v8
{
const g9 = rU8()
if (g9 === 0) v8 = undefined
else if (g9 === 2) v8 = null
else {
v8 = rStr()
}
}
o8["format"] = v8
break
}
case 12: {
let v8
{
const g9 = rU8()
if (g9 === 0) v8 = undefined
else if (g9 === 2) v8 = null
else {
if (rU8() === 0) v8 = null
else {
const n10 = rU32()
const a10 = new Array(n10)
for (let i10 = 0; i10 < n10; i10++) {
let v10
v10 = rDyn()
a10[i10] = v10
}
v8 = a10
}
}
}
o8["extensions"] = v8
break
}
default: {
const k8 = rStr()
o8[k8] = rDyn()
}
}
}
v6 = o8
}
}
}
o6["items"] = v6
break
}
case 10: {
let v6
{
const g7 = rU8()
if (g7 === 0) v6 = undefined
else if (g7 === 2) v6 = null
else {
if (rU8() === 0) v6 = null
else {
const n8 = rU32()
const a8 = new Array(n8)
for (let i8 = 0; i8 < n8; i8++) {
let v8
{
const o9 = {}
const n9 = rU8()
for (let j9 = 0; j9 < n9; j9++) {
switch (rU8()) {
case 0: {
let v9
v9 = rDyn()
o9["key"] = v9
break
}
case 1: {
let v9
v9 = rDyn()
o9["type"] = v9
break
}
case 2: {
let v9
v9 = rDyn()
o9["title"] = v9
break
}
case 3: {
let v9
v9 = rDyn()
o9["description"] = v9
break
}
case 4: {
let v9
v9 = rDyn()
o9["default"] = v9
break
}
case 5: {
let v9
v9 = rDyn()
o9["options"] = v9
break
}
case 6: {
let v9
v9 = rDyn()
o9["secret"] = v9
break
}
case 7: {
let v9
v9 = rDyn()
o9["min"] = v9
break
}
case 8: {
let v9
v9 = rDyn()
o9["max"] = v9
break
}
case 9: {
let v9
v9 = rDyn()
o9["items"] = v9
break
}
case 10: {
let v9
v9 = rDyn()
o9["properties"] = v9
break
}
case 11: {
let v9
v9 = rDyn()
o9["format"] = v9
break
}
case 12: {
let v9
v9 = rDyn()
o9["extensions"] = v9
break
}
default: {
const k9 = rStr()
o9[k9] = rDyn()
}
}
}
v8 = o9
}
a8[i8] = v8
}
v6 = a8
}
}
}
o6["properties"] = v6
break
}
case 11: {
let v6
{
const g7 = rU8()
if (g7 === 0) v6 = undefined
else if (g7 === 2) v6 = null
else {
v6 = rStr()
}
}
o6["format"] = v6
break
}
case 12: {
let v6
{
const g7 = rU8()
if (g7 === 0) v6 = undefined
else if (g7 === 2) v6 = null
else {
if (rU8() === 0) v6 = null
else {
const n8 = rU32()
const a8 = new Array(n8)
for (let i8 = 0; i8 < n8; i8++) {
let v8
v8 = rStr()
a8[i8] = v8
}
v6 = a8
}
}
}
o6["extensions"] = v6
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
v3 = a5
}
}
}
o3["properties"] = v3
break
}
case 11: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
v3 = rStr()
}
}
o3["format"] = v3
break
}
case 12: {
let v3
{
const g4 = rU8()
if (g4 === 0) v3 = undefined
else if (g4 === 2) v3 = null
else {
if (rU8() === 0) v3 = null
else {
const n5 = rU32()
const a5 = new Array(n5)
for (let i5 = 0; i5 < n5; i5++) {
let v5
v5 = rStr()
a5[i5] = v5
}
v3 = a5
}
}
}
o3["extensions"] = v3
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
o1["items"] = v1
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
o4["type"] = v4
break
}
case 2: {
let v4
v4 = rDyn()
o4["title"] = v4
break
}
case 3: {
let v4
v4 = rDyn()
o4["description"] = v4
break
}
case 4: {
let v4
v4 = rDyn()
o4["default"] = v4
break
}
case 5: {
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
o6["value"] = v6
break
}
case 1: {
let v6
v6 = rDyn()
o6["label"] = v6
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
o4["options"] = v4
break
}
case 6: {
let v4
v4 = rU8() === 1
o4["secret"] = v4
break
}
case 7: {
let v4
{
const g5 = rU8()
if (g5 === 0) v4 = undefined
else if (g5 === 2) v4 = null
else {
v4 = rF64()
}
}
o4["min"] = v4
break
}
case 8: {
let v4
{
const g5 = rU8()
if (g5 === 0) v4 = undefined
else if (g5 === 2) v4 = null
else {
v4 = rF64()
}
}
o4["max"] = v4
break
}
case 9: {
let v4
{
const g5 = rU8()
if (g5 === 0) v4 = undefined
else if (g5 === 2) v4 = null
else {
{
const o6 = {}
const n6 = rU8()
for (let j6 = 0; j6 < n6; j6++) {
switch (rU8()) {
case 0: {
let v6
v6 = rStr()
o6["key"] = v6
break
}
case 1: {
let v6
v6 = rStr()
o6["type"] = v6
break
}
case 2: {
let v6
v6 = rDyn()
o6["title"] = v6
break
}
case 3: {
let v6
v6 = rDyn()
o6["description"] = v6
break
}
case 4: {
let v6
v6 = rDyn()
o6["default"] = v6
break
}
case 5: {
let v6
if (rU8() === 0) v6 = null
else {
const n7 = rU32()
const a7 = new Array(n7)
for (let i7 = 0; i7 < n7; i7++) {
let v7
v7 = rDyn()
a7[i7] = v7
}
v6 = a7
}
o6["options"] = v6
break
}
case 6: {
let v6
v6 = rU8() === 1
o6["secret"] = v6
break
}
case 7: {
let v6
{
const g7 = rU8()
if (g7 === 0) v6 = undefined
else if (g7 === 2) v6 = null
else {
v6 = rF64()
}
}
o6["min"] = v6
break
}
case 8: {
let v6
{
const g7 = rU8()
if (g7 === 0) v6 = undefined
else if (g7 === 2) v6 = null
else {
v6 = rF64()
}
}
o6["max"] = v6
break
}
case 9: {
let v6
{
const g7 = rU8()
if (g7 === 0) v6 = undefined
else if (g7 === 2) v6 = null
else {
{
const o8 = {}
const n8 = rU8()
for (let j8 = 0; j8 < n8; j8++) {
switch (rU8()) {
case 0: {
let v8
v8 = rStr()
o8["key"] = v8
break
}
case 1: {
let v8
v8 = rStr()
o8["type"] = v8
break
}
case 2: {
let v8
v8 = rDyn()
o8["title"] = v8
break
}
case 3: {
let v8
v8 = rDyn()
o8["description"] = v8
break
}
case 4: {
let v8
v8 = rDyn()
o8["default"] = v8
break
}
case 5: {
let v8
v8 = rDyn()
o8["options"] = v8
break
}
case 6: {
let v8
v8 = rU8() === 1
o8["secret"] = v8
break
}
case 7: {
let v8
{
const g9 = rU8()
if (g9 === 0) v8 = undefined
else if (g9 === 2) v8 = null
else {
v8 = rF64()
}
}
o8["min"] = v8
break
}
case 8: {
let v8
{
const g9 = rU8()
if (g9 === 0) v8 = undefined
else if (g9 === 2) v8 = null
else {
v8 = rF64()
}
}
o8["max"] = v8
break
}
case 9: {
let v8
v8 = rDyn()
o8["items"] = v8
break
}
case 10: {
let v8
{
const g9 = rU8()
if (g9 === 0) v8 = undefined
else if (g9 === 2) v8 = null
else {
if (rU8() === 0) v8 = null
else {
const n10 = rU32()
const a10 = new Array(n10)
for (let i10 = 0; i10 < n10; i10++) {
let v10
v10 = rDyn()
a10[i10] = v10
}
v8 = a10
}
}
}
o8["properties"] = v8
break
}
case 11: {
let v8
{
const g9 = rU8()
if (g9 === 0) v8 = undefined
else if (g9 === 2) v8 = null
else {
v8 = rStr()
}
}
o8["format"] = v8
break
}
case 12: {
let v8
{
const g9 = rU8()
if (g9 === 0) v8 = undefined
else if (g9 === 2) v8 = null
else {
if (rU8() === 0) v8 = null
else {
const n10 = rU32()
const a10 = new Array(n10)
for (let i10 = 0; i10 < n10; i10++) {
let v10
v10 = rDyn()
a10[i10] = v10
}
v8 = a10
}
}
}
o8["extensions"] = v8
break
}
default: {
const k8 = rStr()
o8[k8] = rDyn()
}
}
}
v6 = o8
}
}
}
o6["items"] = v6
break
}
case 10: {
let v6
{
const g7 = rU8()
if (g7 === 0) v6 = undefined
else if (g7 === 2) v6 = null
else {
if (rU8() === 0) v6 = null
else {
const n8 = rU32()
const a8 = new Array(n8)
for (let i8 = 0; i8 < n8; i8++) {
let v8
{
const o9 = {}
const n9 = rU8()
for (let j9 = 0; j9 < n9; j9++) {
switch (rU8()) {
case 0: {
let v9
v9 = rDyn()
o9["key"] = v9
break
}
case 1: {
let v9
v9 = rDyn()
o9["type"] = v9
break
}
case 2: {
let v9
v9 = rDyn()
o9["title"] = v9
break
}
case 3: {
let v9
v9 = rDyn()
o9["description"] = v9
break
}
case 4: {
let v9
v9 = rDyn()
o9["default"] = v9
break
}
case 5: {
let v9
v9 = rDyn()
o9["options"] = v9
break
}
case 6: {
let v9
v9 = rDyn()
o9["secret"] = v9
break
}
case 7: {
let v9
v9 = rDyn()
o9["min"] = v9
break
}
case 8: {
let v9
v9 = rDyn()
o9["max"] = v9
break
}
case 9: {
let v9
v9 = rDyn()
o9["items"] = v9
break
}
case 10: {
let v9
v9 = rDyn()
o9["properties"] = v9
break
}
case 11: {
let v9
v9 = rDyn()
o9["format"] = v9
break
}
case 12: {
let v9
v9 = rDyn()
o9["extensions"] = v9
break
}
default: {
const k9 = rStr()
o9[k9] = rDyn()
}
}
}
v8 = o9
}
a8[i8] = v8
}
v6 = a8
}
}
}
o6["properties"] = v6
break
}
case 11: {
let v6
{
const g7 = rU8()
if (g7 === 0) v6 = undefined
else if (g7 === 2) v6 = null
else {
v6 = rStr()
}
}
o6["format"] = v6
break
}
case 12: {
let v6
{
const g7 = rU8()
if (g7 === 0) v6 = undefined
else if (g7 === 2) v6 = null
else {
if (rU8() === 0) v6 = null
else {
const n8 = rU32()
const a8 = new Array(n8)
for (let i8 = 0; i8 < n8; i8++) {
let v8
v8 = rStr()
a8[i8] = v8
}
v6 = a8
}
}
}
o6["extensions"] = v6
break
}
default: {
const k6 = rStr()
o6[k6] = rDyn()
}
}
}
v4 = o6
}
}
}
o4["items"] = v4
break
}
case 10: {
let v4
{
const g5 = rU8()
if (g5 === 0) v4 = undefined
else if (g5 === 2) v4 = null
else {
if (rU8() === 0) v4 = null
else {
const n6 = rU32()
const a6 = new Array(n6)
for (let i6 = 0; i6 < n6; i6++) {
let v6
{
const o7 = {}
const n7 = rU8()
for (let j7 = 0; j7 < n7; j7++) {
switch (rU8()) {
case 0: {
let v7
v7 = rStr()
o7["key"] = v7
break
}
case 1: {
let v7
v7 = rStr()
o7["type"] = v7
break
}
case 2: {
let v7
v7 = rDyn()
o7["title"] = v7
break
}
case 3: {
let v7
v7 = rDyn()
o7["description"] = v7
break
}
case 4: {
let v7
v7 = rDyn()
o7["default"] = v7
break
}
case 5: {
let v7
v7 = rDyn()
o7["options"] = v7
break
}
case 6: {
let v7
v7 = rU8() === 1
o7["secret"] = v7
break
}
case 7: {
let v7
{
const g8 = rU8()
if (g8 === 0) v7 = undefined
else if (g8 === 2) v7 = null
else {
v7 = rF64()
}
}
o7["min"] = v7
break
}
case 8: {
let v7
{
const g8 = rU8()
if (g8 === 0) v7 = undefined
else if (g8 === 2) v7 = null
else {
v7 = rF64()
}
}
o7["max"] = v7
break
}
case 9: {
let v7
{
const g8 = rU8()
if (g8 === 0) v7 = undefined
else if (g8 === 2) v7 = null
else {
{
const o9 = {}
const n9 = rU8()
for (let j9 = 0; j9 < n9; j9++) {
switch (rU8()) {
case 0: {
let v9
v9 = rDyn()
o9["key"] = v9
break
}
case 1: {
let v9
v9 = rDyn()
o9["type"] = v9
break
}
case 2: {
let v9
v9 = rDyn()
o9["title"] = v9
break
}
case 3: {
let v9
v9 = rDyn()
o9["description"] = v9
break
}
case 4: {
let v9
v9 = rDyn()
o9["default"] = v9
break
}
case 5: {
let v9
v9 = rDyn()
o9["options"] = v9
break
}
case 6: {
let v9
v9 = rDyn()
o9["secret"] = v9
break
}
case 7: {
let v9
v9 = rDyn()
o9["min"] = v9
break
}
case 8: {
let v9
v9 = rDyn()
o9["max"] = v9
break
}
case 9: {
let v9
v9 = rDyn()
o9["items"] = v9
break
}
case 10: {
let v9
v9 = rDyn()
o9["properties"] = v9
break
}
case 11: {
let v9
v9 = rDyn()
o9["format"] = v9
break
}
case 12: {
let v9
v9 = rDyn()
o9["extensions"] = v9
break
}
default: {
const k9 = rStr()
o9[k9] = rDyn()
}
}
}
v7 = o9
}
}
}
o7["items"] = v7
break
}
case 10: {
let v7
{
const g8 = rU8()
if (g8 === 0) v7 = undefined
else if (g8 === 2) v7 = null
else {
if (rU8() === 0) v7 = null
else {
const n9 = rU32()
const a9 = new Array(n9)
for (let i9 = 0; i9 < n9; i9++) {
let v9
v9 = rDyn()
a9[i9] = v9
}
v7 = a9
}
}
}
o7["properties"] = v7
break
}
case 11: {
let v7
{
const g8 = rU8()
if (g8 === 0) v7 = undefined
else if (g8 === 2) v7 = null
else {
v7 = rStr()
}
}
o7["format"] = v7
break
}
case 12: {
let v7
{
const g8 = rU8()
if (g8 === 0) v7 = undefined
else if (g8 === 2) v7 = null
else {
if (rU8() === 0) v7 = null
else {
const n9 = rU32()
const a9 = new Array(n9)
for (let i9 = 0; i9 < n9; i9++) {
let v9
v9 = rDyn()
a9[i9] = v9
}
v7 = a9
}
}
}
o7["extensions"] = v7
break
}
default: {
const k7 = rStr()
o7[k7] = rDyn()
}
}
}
v6 = o7
}
a6[i6] = v6
}
v4 = a6
}
}
}
o4["properties"] = v4
break
}
case 11: {
let v4
{
const g5 = rU8()
if (g5 === 0) v4 = undefined
else if (g5 === 2) v4 = null
else {
v4 = rStr()
}
}
o4["format"] = v4
break
}
case 12: {
let v4
{
const g5 = rU8()
if (g5 === 0) v4 = undefined
else if (g5 === 2) v4 = null
else {
if (rU8() === 0) v4 = null
else {
const n6 = rU32()
const a6 = new Array(n6)
for (let i6 = 0; i6 < n6; i6++) {
let v6
v6 = rStr()
a6[i6] = v6
}
v4 = a6
}
}
}
o4["extensions"] = v4
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
o1["properties"] = v1
break
}
case 11: {
let v1
{
const g2 = rU8()
if (g2 === 0) v1 = undefined
else if (g2 === 2) v1 = null
else {
v1 = rStr()
}
}
o1["format"] = v1
break
}
case 12: {
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
o1["extensions"] = v1
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
module.exports.schemasteryFields = function schemasteryFields (a0) {
  try {
    if (arguments.length <= 1 && encode(a0) && wireCall(call)) return decode()
    return json(...arguments)
  } catch (e) {
    throw jsError(e)
  }
}
}
const guards = {

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
