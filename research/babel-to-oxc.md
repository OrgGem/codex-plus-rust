# Primary-Source Research: `@babel/parser` + `@babel/traverse` + `supports-color` -> Rust `oxc`

## Scope
Applies to `assets/native-browser/inspect-service.mjs`, `assets/native-browser/require-identification.mjs`, and `scripts/assemble-renderer-inject.mjs`.

## Crate Selection

| Original JS Dependency | Replacement Rust Crate | Rationale |
| :--- | :--- | :--- |
| `@babel/parser` | `oxc_parser = "0.160"` (`default-features = false`), `oxc_allocator = "0.160"`, `oxc_ast = "0.160"`, `oxc_span = "0.160"` | Pure Rust ES module parser (`SourceType::mjs()`), compiles cleanly to both `x86_64-pc-windows-gnu` and `wasm32-unknown-unknown`. Setting `ParseOptions { preserve_parens: false, ..ParseOptions::default() }` omits `ParenthesizedExpression` wrappers to match Babel's default AST structure. `Span::start`/`end` are UTF-8 byte offsets, matching `Buffer.byteLength(source.slice(0, pos))`. |
| `@babel/traverse` | `oxc_semantic = "0.160"` (`SemanticBuilder`, `Scoping`, `AstNodes`) | Eliminates handwritten JS scope binding (`var` hoisting, `let`/`const`/`class`/`import` lexical scoping, `constant` mutability tracking via `symbol_is_mutated` + `symbol_redeclarations`, and AST subtree/parent queries via `AstNodes`) with full ES module fidelity. |
| `supports-color` | *(Not needed)* | Only present inside bundled `@babel/code-frame` diagnostics inside `inspect-service.cjs`; never called by `inspect-service.mjs`. Error redaction intentionally suppresses source excerpts and ANSI formatting. |

## Verified AST & Serialization Equivalences
- `oxc_ast 0.160` encodes computed keys inside `PropertyKey::INHERIT(Expression<'a>)` instead of a `.computed` boolean field on `ObjectProperty`, `MethodDefinition`, `PropertyDefinition`, or `BindingProperty` (`key.as_expression().is_some()`).
- Babel AST `shape()` JSON field order is preserved explicitly via `serde_json` (`preserve_order` / `indexmap`), including `AssignmentExpression` (`type`, `operator`, `left`, `right`), `ClassMethod`, `FunctionDeclaration`, `OptionalMemberExpression`, and `TryStatement`/`CatchClause`.
