//! The Rust port of `inspect-service.mjs` using `oxc_parser` and `oxc_semantic`.

#![forbid(unsafe_code)]

use crate::js;
use oxc_allocator::Allocator;
use oxc_ast::{AstKind, ast::*};
use oxc_parser::{ParseOptions, Parser};
use oxc_semantic::{AstNode, AstNodes, Scoping, SemanticBuilder};
use oxc_span::SourceType;
use oxc_syntax::{node::NodeId, symbol::SymbolId};
use serde_json::{Map, Value, json};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

const PROTO_A: &str = "5bf64da3b8386af46a6fb2c2d8829a507130ba9896237a813b8e04c4ce8eb515";
const PROTO_B: &str = "eae1b49427aebf3ed3d1119de1c303125c4f78b3b0ca644f055ed07ec2c2be30";
const META_SHA: &str = "96146f3d4c51e5707a7cb38783b06294b1f60e9bbc1f0b0cba93c63d23222ce8";
const POLICY_SHA: &str = "3c64bbeed6876be6468d1700b8f63c217b458ac1f495856ede7e7f939c9b3df2";

fn req(cond: bool, msg: &str) -> Result<(), String> {
    cond.then_some(()).ok_or_else(|| format!("Error: {msg}"))
}

fn id_json(name: &str) -> Value {
    json!({ "type": "Identifier", "name": name })
}

struct Sem<'a> {
    sc: Scoping,
    nodes: AstNodes<'a>,
}

impl<'a> Sem<'a> {
    fn build(prog: &'a Program<'a>) -> Self {
        let ret = SemanticBuilder::new().with_build_nodes(true).build(prog);
        let (sc, nodes) = ret.semantic.into_scoping_and_nodes();
        Self { sc, nodes }
    }

    fn ref_sym(&self, id: &IdentifierReference) -> Option<SymbolId> {
        self.sc.get_reference(id.reference_id.get()?).symbol_id()
    }

    fn pat_sym(p: &BindingPattern) -> Option<SymbolId> {
        match p {
            BindingPattern::BindingIdentifier(id) => id.symbol_id.get(),
            _ => None,
        }
    }

    fn expr_sym(&self, e: &Expression) -> Option<SymbolId> {
        match e {
            Expression::Identifier(id) => self.ref_sym(id),
            _ => None,
        }
    }

    fn is_const(&self, sym: SymbolId) -> bool {
        !self.sc.symbol_is_mutated(sym) && self.sc.symbol_redeclarations(sym).is_empty()
    }

    fn decl_kind(&self, sym: SymbolId) -> AstKind<'a> {
        let id = self.sc.symbol_declaration(sym);
        match self.nodes.kind(id) {
            AstKind::BindingIdentifier(_) => self.nodes.parent_kind(id),
            k => k,
        }
    }

    fn sub(&self, root: NodeId) -> impl Iterator<Item = &AstNode<'a>> + '_ {
        self.nodes
            .into_iter()
            .filter(move |n| n.id() == root || self.nodes.ancestor_ids(n.id()).any(|a| a == root))
    }

    fn is_awaited(&self, call_id: NodeId) -> bool {
        matches!(self.nodes.parent_kind(call_id), AstKind::AwaitExpression(_))
    }

    fn enclosing_decl(&self, id: NodeId) -> Option<&'a VariableDeclarator<'a>> {
        self.nodes.ancestors(id).find_map(|a| match a.kind() {
            AstKind::VariableDeclarator(d) => Some(d),
            _ => None,
        })
    }
}

fn parse_mjs<'a>(alloc: &'a Allocator, src: &'a str) -> Result<Program<'a>, String> {
    let opts = ParseOptions {
        preserve_parens: false,
        ..ParseOptions::default()
    };
    let ret = Parser::new(alloc, src, SourceType::mjs())
        .with_options(opts)
        .parse();
    if !ret.diagnostics.is_empty() || ret.fatal_error {
        let msg = ret.diagnostics.first().map(ToString::to_string);
        return Err(msg.unwrap_or_else(|| "SyntaxError".into()));
    }
    Ok(ret.program)
}

fn member_this(e: &Expression, prop: &str) -> bool {
    let Expression::StaticMemberExpression(m) = e else {
        return false;
    };
    matches!(m.object, Expression::ThisExpression(_)) && m.property.name == prop
}

/// Port of `inspectEntry` (`inspect-service.mjs:143`).
pub fn inspect_entry(source: Value) -> Result<Value, String> {
    let src = source
        .as_str()
        .filter(|s| s.len() <= 1024 * 1024)
        .ok_or("Error: Invalid entry size")?;
    let alloc = Allocator::default();
    let prog = parse_mjs(&alloc, src)?;
    let sem = Sem::build(&prog);
    let mut imported: Option<SymbolId> = None;
    let mut launches = 0usize;
    for node in &sem.nodes {
        match node.kind() {
            AstKind::ImportDeclaration(i) if i.source.value == "@oai/cua-repl" => {
                let spec = i
                    .specifiers
                    .as_ref()
                    .filter(|s| imported.is_none() && s.len() == 1);
                let Some(ImportDeclarationSpecifier::ImportNamespaceSpecifier(ns)) =
                    spec.and_then(|s| s.first())
                else {
                    return Err("Error: Ambiguous CUA entry import".into());
                };
                imported = ns.local.symbol_id.get();
            }
            AstKind::CallExpression(c) => {
                if let Expression::StaticMemberExpression(m) = &c.callee
                    && m.property.name == "launch"
                    && imported.is_some()
                    && sem.expr_sym(&m.object) == imported
                {
                    launches += 1;
                    let ok = c.arguments.is_empty() && sem.is_awaited(node.id());
                    req(ok, "Changed CUA launch signature")?;
                }
            }
            _ => {}
        }
    }
    let ok = imported.is_some_and(|s| sem.is_const(s)) && launches == 1;
    req(ok, "Missing stable CUA launch entry")?;
    Ok(js::undefined())
}

struct Shaper<'a, 'b> {
    sem: &'b Sem<'a>,
    names: HashMap<SymbolId, String>,
}

impl<'a, 'b> Shaper<'a, 'b> {
    fn rename(&mut self, sym: Option<SymbolId>, raw: &str) -> Value {
        let out = sym.map_or_else(
            || raw.to_string(),
            |s| {
                let n = self.names.len();
                self.names
                    .entry(s)
                    .or_insert_with(|| format!("local{n}"))
                    .clone()
            },
        );
        id_json(&out)
    }

    fn bind_id(&mut self, id: &BindingIdentifier) -> Value {
        self.rename(id.symbol_id.get(), id.name.as_str())
    }

    fn ref_id(&mut self, id: &IdentifierReference) -> Value {
        self.rename(self.sem.ref_sym(id), id.name.as_str())
    }

    fn pat(&mut self, p: &BindingPattern) -> Value {
        match p {
            BindingPattern::BindingIdentifier(id) => self.bind_id(id),
            _ => Value::Null,
        }
    }

    fn has_opt(e: &Expression) -> bool {
        match e {
            Expression::StaticMemberExpression(m) => m.optional || Self::has_opt(&m.object),
            Expression::ComputedMemberExpression(m) => m.optional || Self::has_opt(&m.object),
            Expression::CallExpression(c) => c.optional || Self::has_opt(&c.callee),
            _ => false,
        }
    }

    fn block(&mut self, stmts: &[Statement]) -> Value {
        let body: Vec<Value> = stmts.iter().map(|s| self.stmt(s)).collect();
        json!({ "type": "BlockStatement", "body": body, "directives": [] })
    }

    fn member(&mut self, obj: Value, computed: bool, prop: Value, opt: Option<bool>) -> Value {
        if let Some(o) = opt {
            json!({ "type": "OptionalMemberExpression", "object": obj, "computed": computed, "property": prop, "optional": o })
        } else {
            json!({ "type": "MemberExpression", "object": obj, "computed": computed, "property": prop })
        }
    }

    fn call_node(&mut self, ty: &str, cal: Value, args: &[Argument], opt: Option<bool>) -> Value {
        let argv: Vec<Value> = args.iter().map(|a| self.arg(a)).collect();
        if let Some(o) = opt {
            json!({ "type": "OptionalCallExpression", "callee": cal, "arguments": argv, "optional": o })
        } else {
            json!({ "type": ty, "callee": cal, "arguments": argv })
        }
    }

    fn bin(&mut self, ty: &str, l: &Expression, op: &str, r: &Expression) -> Value {
        let (lv, rv) = (self.expr(l, false), self.expr(r, false));
        json!({ "type": ty, "left": lv, "operator": op, "right": rv })
    }

    fn stmt(&mut self, st: &Statement) -> Value {
        match st {
            Statement::BlockStatement(b) => self.block(&b.body),
            Statement::ExpressionStatement(e) => {
                let x = self.expr(&e.expression, false);
                json!({ "type": "ExpressionStatement", "expression": x })
            }
            Statement::ReturnStatement(r) => {
                let a = r.argument.as_ref().map(|x| self.expr(x, false));
                json!({ "type": "ReturnStatement", "argument": a })
            }
            Statement::ThrowStatement(t) => {
                let a = self.expr(&t.argument, false);
                json!({ "type": "ThrowStatement", "argument": a })
            }
            Statement::IfStatement(i) => {
                let (t, c) = (self.expr(&i.test, false), self.stmt(&i.consequent));
                let a = i.alternate.as_ref().map(|x| self.stmt(x));
                json!({ "type": "IfStatement", "test": t, "consequent": c, "alternate": a })
            }
            Statement::TryStatement(t) => {
                let blk = self.block(&t.block.body);
                let h = t.handler.as_ref().map(|c| {
                    let p = c.param.as_ref().map(|x| self.pat(&x.pattern));
                    let b = self.block(&c.body.body);
                    json!({ "type": "CatchClause", "param": p, "body": b })
                });
                let f = t.finalizer.as_ref().map(|x| self.block(&x.body));
                json!({ "type": "TryStatement", "block": blk, "handler": h, "finalizer": f })
            }
            Statement::VariableDeclaration(v) => {
                let ds: Vec<Value> = v
                    .declarations
                    .iter()
                    .map(|d| {
                        let (id, init) = (
                            self.pat(&d.id),
                            d.init.as_ref().map(|x| self.expr(x, false)),
                        );
                        json!({ "type": "VariableDeclarator", "id": id, "init": init })
                    })
                    .collect();
                let kind = match v.kind {
                    VariableDeclarationKind::Let => "let",
                    VariableDeclarationKind::Const => "const",
                    _ => "var",
                };
                json!({ "type": "VariableDeclaration", "declarations": ds, "kind": kind })
            }
            _ => Value::Null,
        }
    }

    fn assign_left(&mut self, t: &AssignmentTarget) -> Value {
        match t {
            AssignmentTarget::AssignmentTargetIdentifier(id) => self.ref_id(id),
            AssignmentTarget::StaticMemberExpression(m) => {
                let o = self.expr(&m.object, false);
                self.member(o, false, id_json(m.property.name.as_str()), None)
            }
            AssignmentTarget::ComputedMemberExpression(m) => {
                let (o, p) = (self.expr(&m.object, false), self.expr(&m.expression, false));
                self.member(o, true, p, None)
            }
            _ => Value::Null,
        }
    }

    fn arg(&mut self, a: &Argument) -> Value {
        match a {
            Argument::SpreadElement(sp) => {
                let x = self.expr(&sp.argument, false);
                json!({ "type": "SpreadElement", "argument": x })
            }
            _ => a
                .as_expression()
                .map_or(Value::Null, |e| self.expr(e, false)),
        }
    }

    fn prop(&mut self, p: &ObjectPropertyKind) -> Value {
        match p {
            ObjectPropertyKind::ObjectProperty(x) => {
                let k = match &x.key {
                    PropertyKey::StaticIdentifier(id) => id_json(id.name.as_str()),
                    PropertyKey::StaticStringLiteral(s) => {
                        json!({ "type": "StringLiteral", "value": s.value.as_str() })
                    }
                    _ => x
                        .key
                        .as_expression()
                        .map_or(Value::Null, |e| self.expr(e, false)),
                };
                let (c, v) = (x.key.as_expression().is_some(), self.expr(&x.value, false));
                json!({ "type": "ObjectProperty", "method": x.method, "key": k, "computed": c, "shorthand": x.shorthand, "value": v })
            }
            ObjectPropertyKind::SpreadProperty(sp) => {
                let a = self.expr(&sp.argument, false);
                json!({ "type": "SpreadElement", "argument": a })
            }
        }
    }

    fn expr(&mut self, e: &Expression, in_chain: bool) -> Value {
        let opt = in_chain && Self::has_opt(e);
        match e {
            Expression::Identifier(id) => self.ref_id(id),
            Expression::ThisExpression(_) => json!({ "type": "ThisExpression" }),
            Expression::NullLiteral(_) => json!({ "type": "NullLiteral" }),
            Expression::BooleanLiteral(b) => json!({ "type": "BooleanLiteral", "value": b.value }),
            Expression::StringLiteral(s) => {
                json!({ "type": "StringLiteral", "value": s.value.as_str() })
            }
            Expression::NumericLiteral(n) => {
                let v = if n.value.fract() == 0.0 && n.value.abs() < 9_007_199_254_740_992.0 {
                    Value::from(n.value as i64)
                } else {
                    Value::from(n.value)
                };
                json!({ "type": "NumericLiteral", "value": v })
            }
            Expression::ChainExpression(c) => match &c.expression {
                ChainElement::StaticMemberExpression(m) => {
                    let o = self.expr(&m.object, true);
                    self.member(
                        o,
                        false,
                        id_json(m.property.name.as_str()),
                        Some(m.optional),
                    )
                }
                ChainElement::ComputedMemberExpression(m) => {
                    let (o, p) = (self.expr(&m.object, true), self.expr(&m.expression, false));
                    self.member(o, true, p, Some(m.optional))
                }
                ChainElement::CallExpression(x) => {
                    let cal = self.expr(&x.callee, true);
                    self.call_node("CallExpression", cal, &x.arguments, Some(x.optional))
                }
                _ => Value::Null,
            },
            Expression::StaticMemberExpression(m) => {
                let o = self.expr(&m.object, opt);
                self.member(
                    o,
                    false,
                    id_json(m.property.name.as_str()),
                    opt.then_some(m.optional),
                )
            }
            Expression::ComputedMemberExpression(m) => {
                let (o, p) = (self.expr(&m.object, opt), self.expr(&m.expression, false));
                self.member(o, true, p, opt.then_some(m.optional))
            }
            Expression::CallExpression(c) => {
                let cal = self.expr(&c.callee, opt);
                self.call_node(
                    "CallExpression",
                    cal,
                    &c.arguments,
                    opt.then_some(c.optional),
                )
            }
            Expression::NewExpression(n) => {
                let cal = self.expr(&n.callee, false);
                self.call_node("NewExpression", cal, &n.arguments, None)
            }
            Expression::BinaryExpression(b) => {
                self.bin("BinaryExpression", &b.left, b.operator.as_str(), &b.right)
            }
            Expression::LogicalExpression(l) => {
                self.bin("LogicalExpression", &l.left, l.operator.as_str(), &l.right)
            }
            Expression::UnaryExpression(u) => {
                let a = self.expr(&u.argument, false);
                json!({ "type": "UnaryExpression", "operator": u.operator.as_str(), "prefix": true, "argument": a })
            }
            Expression::AssignmentExpression(a) => {
                let (l, r) = (self.assign_left(&a.left), self.expr(&a.right, false));
                json!({ "type": "AssignmentExpression", "operator": a.operator.as_str(), "left": l, "right": r })
            }
            Expression::AwaitExpression(a) => {
                let x = self.expr(&a.argument, false);
                json!({ "type": "AwaitExpression", "argument": x })
            }
            Expression::SequenceExpression(q) => {
                let xs: Vec<Value> = q.expressions.iter().map(|x| self.expr(x, false)).collect();
                json!({ "type": "SequenceExpression", "expressions": xs })
            }
            Expression::ObjectExpression(o) => {
                let ps: Vec<Value> = o.properties.iter().map(|p| self.prop(p)).collect();
                json!({ "type": "ObjectExpression", "properties": ps })
            }
            Expression::ArrayExpression(a) => {
                let els: Vec<Value> = a
                    .elements
                    .iter()
                    .map(|el| match el {
                        ArrayExpressionElement::SpreadElement(sp) => {
                            let x = self.expr(&sp.argument, false);
                            json!({ "type": "SpreadElement", "argument": x })
                        }
                        ArrayExpressionElement::Elision(_) => Value::Null,
                        _ => el
                            .as_expression()
                            .map_or(Value::Null, |x| self.expr(x, false)),
                    })
                    .collect();
                json!({ "type": "ArrayExpression", "elements": els })
            }
            _ => Value::Null,
        }
    }

    fn sha(val: Value) -> String {
        let s = serde_json::to_string(&val).unwrap_or_default();
        format!("{:x}", Sha256::digest(s.as_bytes()))
    }
}

fn shape_fn<'a>(sem: &Sem<'a>, f: &Function) -> String {
    let mut sh = Shaper {
        sem,
        names: HashMap::new(),
    };
    let id = f.id.as_ref().map(|x| sh.bind_id(x));
    let ps: Vec<Value> = f.params.items.iter().map(|p| sh.pat(&p.pattern)).collect();
    let body = f.body.as_ref().map(|b| sh.block(&b.statements));
    let (g, a) = (f.generator, f.r#async);
    Shaper::sha(
        json!({ "type": "FunctionDeclaration", "id": id, "generator": g, "async": a, "params": ps, "body": body }),
    )
}

fn shape_method<'a>(sem: &Sem<'a>, m: &MethodDefinition) -> String {
    let mut sh = Shaper {
        sem,
        names: HashMap::new(),
    };
    let key = match &m.key {
        PropertyKey::StaticIdentifier(id) => id_json(id.name.as_str()),
        _ => Value::Null,
    };
    let ps: Vec<Value> = m
        .value
        .params
        .items
        .iter()
        .map(|p| sh.pat(&p.pattern))
        .collect();
    let body = m.value.body.as_ref().map(|b| sh.block(&b.statements));
    let (st, comp, g, a) = (
        m.r#static,
        m.key.as_expression().is_some(),
        m.value.generator,
        m.value.r#async,
    );
    Shaper::sha(
        json!({ "type": "ClassMethod", "static": st, "key": key, "computed": comp, "kind": "method", "id": null, "generator": g, "async": a, "params": ps, "body": body }),
    )
}

fn has_str_lit<'a>(sem: &Sem<'a>, f: &Function<'a>, target: &str) -> bool {
    sem.sub(f.node_id.get())
        .any(|n| matches!(n.kind(), AstKind::StringLiteral(s) if s.value == target))
}

fn adaptive_request<'a>(sem: &Sem<'a>, send: &MethodDefinition<'a>) -> Result<(), String> {
    let f = &send.value;
    let p_ok = f.r#async
        && f.params.items.len() >= 2
        && f.params.items[..2]
            .iter()
            .all(|p| matches!(p.pattern, BindingPattern::BindingIdentifier(_)));
    req(p_ok, "Changed browser request signature")?;
    let (Some(cmd), Some(pay)) = (
        Sem::pat_sym(&f.params.items[0].pattern),
        Sem::pat_sym(&f.params.items[1].pattern),
    ) else {
        return Err("Error: Rebound request arguments".into());
    };
    req(
        sem.is_const(cmd) && sem.is_const(pay),
        "Rebound request arguments",
    )?;
    let root = send.node_id.get();
    let mut session: Option<SymbolId> = None;
    for node in sem.sub(root) {
        if let AstKind::VariableDeclarator(d) = node.kind()
            && let Some(Expression::CallExpression(c)) = &d.init
            && member_this(&c.callee, "getSessionParams")
        {
            let sid = Sem::pat_sym(&d.id);
            let ok = session.is_none() && sid.is_some() && c.arguments.is_empty();
            req(ok, "Ambiguous session parameters")?;
            session = sid;
        }
    }
    req(
        session.is_some_and(|s| sem.is_const(s)),
        "Missing stable session parameters",
    )?;
    let (mut hdr, mut cb, mut fwd, mut wr) = (None, 0usize, 0usize, 0usize);
    for node in sem.sub(root) {
        if let AstKind::CallExpression(c) = node.kind() {
            if member_this(&c.callee, "readRequestHeaderEnabled") {
                cb += 1;
                req(
                    c.arguments.is_empty() && sem.is_awaited(node.id()),
                    "Changed policy invocation",
                )?;
                let decl = sem.enclosing_decl(node.id());
                let hb = decl
                    .and_then(|d| Sem::pat_sym(&d.id))
                    .ok_or("Error: Missing header decision")?;
                req(sem.is_const(hb), "Rebound header decision")?;
                hdr = Some(hb);
            }
            if member_this(&c.callee, "sendRequest") {
                fwd += 1;
                let a0_ok = c.arguments.len() == 2
                    && c.arguments[0].as_expression().and_then(|e| sem.expr_sym(e)) == Some(cmd);
                let Some(Expression::ObjectExpression(obj)) = (c.arguments.len() == 2)
                    .then(|| c.arguments[1].as_expression())
                    .flatten()
                else {
                    return Err("Error: Changed browser request forwarding".into());
                };
                req(a0_ok, "Changed browser request forwarding")?;
                let sp: Vec<&Expression> = obj
                    .properties
                    .iter()
                    .filter_map(|p| match p {
                        ObjectPropertyKind::SpreadProperty(s) => Some(&s.argument),
                        _ => None,
                    })
                    .collect();
                let ok = obj.properties.len() == 2
                    && sp.len() == 2
                    && sem.expr_sym(sp[0]) == Some(pay)
                    && sem.expr_sym(sp[1]) == session;
                req(ok, "Missing request or session parameters")?;
            }
        }
    }
    for node in sem.sub(root) {
        if let AstKind::AssignmentExpression(a) = node.kind()
            && let AssignmentTarget::StaticMemberExpression(m) = &a.left
            && m.property.name == "agent_request_header_enabled"
            && sem.expr_sym(&m.object) == session
        {
            wr += 1;
            req(
                hdr.is_some() && sem.expr_sym(&a.right) == hdr,
                "Changed request identification data flow",
            )?;
        }
    }
    req(
        cb == 1 && fwd == 1 && wr == 1,
        "Ambiguous request identification contract",
    )
}

fn find_method<'a>(
    methods: &'a [ClassElement<'a>],
    name: Option<&str>,
) -> Option<&'a MethodDefinition<'a>> {
    let mut found = methods.iter().filter_map(|el| match el {
        ClassElement::MethodDefinition(m)
            if matches!(&m.key, PropertyKey::StaticIdentifier(id) if Some(id.name.as_str()) == name) =>
        {
            Some(m.as_ref())
        }
        _ => None,
    });
    let first = found.next()?;
    found.next().is_none().then_some(first)
}

fn request_method<'a>(
    sem: &Sem<'a>,
    methods: &'a [ClassElement<'a>],
) -> Result<&'a MethodDefinition<'a>, String> {
    let info = find_method(methods, Some("getInfo"))
        .filter(|m| m.value.r#async)
        .ok_or("Error: Ambiguous browser handshake")?;
    let root = info.node_id.get();
    let (mut mname, mut res, mut calls, mut stores) = (None, None, 0usize, 0usize);
    for node in sem.sub(root) {
        if let AstKind::VariableDeclarator(d) = node.kind()
            && let Some(Expression::AwaitExpression(aw)) = &d.init
            && let Expression::CallExpression(c) = &aw.argument
            && matches!(c.arguments.first().and_then(|a| a.as_expression()), Some(Expression::StringLiteral(s)) if s.value == "getInfo")
        {
            let (Expression::StaticMemberExpression(m), Some(sid)) =
                (&c.callee, Sem::pat_sym(&d.id))
            else {
                return Err("Error: Changed browser handshake request".into());
            };
            req(
                matches!(m.object, Expression::ThisExpression(_)),
                "Changed browser handshake request",
            )?;
            calls += 1;
            mname = Some(m.property.name.as_str());
            res = Some(sid);
        }
    }
    for node in sem.sub(root) {
        if let AstKind::AssignmentExpression(a) = node.kind()
            && let AssignmentTarget::StaticMemberExpression(m) = &a.left
            && matches!(m.object, Expression::ThisExpression(_))
            && m.property.name == "clientInfo"
        {
            stores += 1;
            req(
                res.is_some() && sem.expr_sym(&a.right) == res,
                "Changed browser handshake result",
            )?;
        }
    }
    let ok = calls == 1 && stores == 1 && res.is_some_and(|s| sem.is_const(s));
    req(ok, "Changed browser client handshake")?;
    find_method(methods, mname).ok_or_else(|| "Error: Ambiguous browser request method".into())
}

fn validate_candidate<'a>(
    node: &AstNode<'a>,
    n: &'a NewExpression<'a>,
    sem: &Sem<'a>,
) -> Result<Value, String> {
    let Expression::Identifier(callee) = &n.callee else {
        return Err("Error: Unsupported constructor".into());
    };
    let Some(cb) = sem.ref_sym(callee).filter(|&s| sem.is_const(s)) else {
        return Err("Error: Unstable browser constructor".into());
    };
    let AstKind::VariableDeclarator(decl) = sem.decl_kind(cb) else {
        return Err("Error: Unstable browser constructor".into());
    };
    let Some(Expression::ClassExpression(cls)) = &decl.init else {
        return Err("Error: Unstable browser constructor".into());
    };
    let ctor = cls
        .body
        .body
        .iter()
        .find_map(|el| match el {
            ClassElement::MethodDefinition(m) if m.kind == MethodDefinitionKind::Constructor => {
                Some(m.as_ref())
            }
            _ => None,
        })
        .ok_or("Error: Unsupported constructor arguments")?;
    let cp_ok = ctor.value.params.items.len() == 5
        && ctor.value.params.rest.is_none()
        && ctor
            .value
            .params
            .items
            .iter()
            .all(|p| matches!(p.pattern, BindingPattern::BindingIdentifier(_)));
    req(cp_ok, "Unsupported constructor arguments")?;
    let c_ids: Vec<(SymbolId, &str)> = ctor
        .value
        .params
        .items
        .iter()
        .filter_map(|p| match &p.pattern {
            BindingPattern::BindingIdentifier(id) => {
                id.symbol_id.get().map(|s| (s, id.name.as_str()))
            }
            _ => None,
        })
        .collect();
    req(c_ids.len() == 5, "Unsupported constructor arguments")?;
    let mut assigned: HashMap<&str, Option<usize>> = HashMap::new();
    let mut bound = false;
    for sub in sem.sub(ctor.node_id.get()) {
        if let AstKind::AssignmentExpression(a) = sub.kind()
            && let AssignmentTarget::StaticMemberExpression(m) = &a.left
            && matches!(m.object, Expression::ThisExpression(_))
            && let Expression::Identifier(r) = &a.right
        {
            let prop = m.property.name.as_str();
            req(!assigned.contains_key(prop), "Duplicate constructor field")?;
            let idx = sem
                .ref_sym(r)
                .and_then(|s| c_ids.iter().position(|&(cs, _)| cs == s));
            assigned.insert(prop, idx);
        } else if let AstKind::CallExpression(c) = sub.kind()
            && member_this(&c.callee, "registerRequestHandlerObject")
            && c.arguments.len() == 1
            && matches!(c.arguments[0].as_expression(), Some(Expression::Identifier(id)) if id.name.as_str() == c_ids[1].1)
        {
            bound = true;
        }
    }
    let fields = [
        ("apiTransport", 0),
        ("getTurnMetadata", 2),
        ("turnEndedTracker", 3),
        ("readRequestHeaderEnabled", 4),
    ];
    req(
        bound
            && fields
                .iter()
                .all(|&(k, i)| assigned.get(k) == Some(&Some(i))),
        "Changed callback contract",
    )?;
    let Some(Expression::ArrowFunctionExpression(arr)) = n.arguments[2].as_expression() else {
        return Err("Error: Changed browser arguments".into());
    };
    let arg_ok = n.arguments[1]
        .as_expression()
        .is_some_and(|e| member_this(e, "clientApi"))
        && arr.params.items.is_empty()
        && arr.params.rest.is_none();
    req(arg_ok, "Changed browser arguments")?;
    let Some(Expression::CallExpression(call)) = arr.get_expression() else {
        return Err("Error: Changed callback arguments".into());
    };
    let (Expression::Identifier(meta_id), Some(Expression::Identifier(pol_id))) =
        (&call.callee, n.arguments[4].as_expression())
    else {
        return Err("Error: Changed callback arguments".into());
    };
    let call_ok = call.arguments.len() == 1
        && call.arguments[0]
            .as_expression()
            .is_some_and(|e| member_this(e, "runtime"));
    req(call_ok, "Changed callback arguments")?;
    let (Some(mb), Some(pb)) = (sem.ref_sym(meta_id), sem.ref_sym(pol_id)) else {
        return Err("Error: Unstable callback bindings".into());
    };
    let (AstKind::Function(meta_fn), AstKind::Function(pol_fn)) =
        (sem.decl_kind(mb), sem.decl_kind(pb))
    else {
        return Err("Error: Unstable callback bindings".into());
    };
    req(
        sem.is_const(mb) && sem.is_const(pb),
        "Unstable callback bindings",
    )?;
    let proto_ok = has_str_lit(sem, meta_fn, "x-codex-turn-metadata")
        && meta_fn.params.items.len() == 1
        && meta_fn.params.rest.is_none()
        && pol_fn.r#async
        && pol_fn.params.items.is_empty()
        && has_str_lit(sem, pol_fn, "codex_browser_use_agent_request_header");
    req(proto_ok, "Changed metadata or policy protocol")?;
    req(
        shape_fn(sem, meta_fn) == META_SHA && shape_fn(sem, pol_fn) == POLICY_SHA,
        "Changed metadata or policy semantics",
    )?;
    let send = request_method(sem, &cls.body.body)?;
    let contract_sha = shape_method(sem, send);
    if contract_sha != PROTO_A && contract_sha != PROTO_B {
        adaptive_request(sem, send)?;
    }
    let scope = node.scope_id();
    let shadow_ok = sem.sc.find_binding(scope, meta_id.name) == Some(mb)
        && sem
            .sc
            .find_binding(scope, "cppNativeIdentificationReader".into())
            .is_none();
    req(shadow_ok, "Shadowed metadata binding")?;
    Ok(Value::Object(Map::from_iter([
        ("start".into(), pol_id.span.start.into()),
        ("end".into(), pol_id.span.end.into()),
        ("policy".into(), pol_id.name.as_str().into()),
        ("metadata".into(), meta_id.name.as_str().into()),
        ("contractSha".into(), contract_sha.into()),
    ])))
}

/// Port of `inspectService` (`inspect-service.mjs:171`).
pub fn inspect_service(source: Value) -> Result<Value, String> {
    let src = source
        .as_str()
        .filter(|s| s.len() <= 32 * 1024 * 1024)
        .ok_or("Error: Invalid service size")?;
    let alloc = Allocator::default();
    let prog = parse_mjs(&alloc, src)?;
    let sem = Sem::build(&prog);
    let mut cands = Vec::new();
    for node in &sem.nodes {
        let conflicting = match node.kind() {
            AstKind::IdentifierReference(id) => id.name == "cppNativeIdentificationReader",
            AstKind::BindingIdentifier(id) => id.name == "cppNativeIdentificationReader",
            AstKind::IdentifierName(id) => id.name == "cppNativeIdentificationReader",
            AstKind::LabelIdentifier(id) => id.name == "cppNativeIdentificationReader",
            _ => false,
        };
        req(!conflicting, "Conflicting adapter")?;
        if let AstKind::NewExpression(n) = node.kind()
            && n.arguments.len() == 5
            && n.arguments[3]
                .as_expression()
                .is_some_and(|e| member_this(e, "turnEndedTracker"))
        {
            cands.push(validate_candidate(node, n, &sem)?);
        }
    }
    req(cands.len() == 1, "Expected one browser callback contract")?;
    Ok(cands.swap_remove(0))
}
