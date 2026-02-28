//! Pretty-printer for the Eta AST.
//!
//! Produces deterministic, human-readable Eta source from AST nodes.
//! Depends only on `crate::parser::ast`.

use crate::parser::ast::*;
use std::fmt::Write as _;

// ── public entry points ────────────────────────────────────────────

/// Pretty-print a full program.
pub fn pretty_program(p: &Program) -> String {
    let mut buf = String::new();
    for u in &p.uses {
        writeln!(buf, "use {}", u).unwrap();
    }
    if !p.uses.is_empty() && !p.items.is_empty() {
        buf.push('\n');
    }
    for (i, item) in p.items.iter().enumerate() {
        if i > 0 {
            buf.push('\n');
        }
        match item {
            TopLevelItem::Func(f) => pretty_func_def(&mut buf, f, 0),
            TopLevelItem::Global(g) => pretty_global_decl(&mut buf, g, 0),
        }
    }
    buf
}

/// Pretty-print a single expression.
pub fn pretty_expr(e: &Expr) -> String {
    format_expr(e, Prec::Top)
}

/// Pretty-print a single statement (at indentation level 0).
/// The returned string does NOT have a trailing newline.
pub fn pretty_stmt(s: &Stmt) -> String {
    let mut buf = String::new();
    write_stmt(&mut buf, s, 0);
    // write_stmt always appends '\n'; strip trailing newlines for the public API.
    let trimmed = buf.trim_end_matches('\n');
    trimmed.to_string()
}

// ── Display impls ──────────────────────────────────────────────────

impl std::fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&pretty_program(self))
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&pretty_expr(self))
    }
}

impl std::fmt::Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&pretty_stmt(self))
    }
}

// ── indentation helper ─────────────────────────────────────────────

const INDENT: &str = "    "; // 4 spaces

fn indent(buf: &mut String, level: usize) {
    for _ in 0..level {
        buf.push_str(INDENT);
    }
}

// ── top-level items ────────────────────────────────────────────────

fn pretty_func_def(buf: &mut String, f: &FuncDef, level: usize) {
    indent(buf, level);
    buf.push_str(&f.name);
    buf.push('(');
    for (i, p) in f.params.iter().enumerate() {
        if i > 0 {
            buf.push_str(", ");
        }
        write!(buf, "{}: {}", p.name, format_type(&p.ty)).unwrap();
    }
    buf.push(')');

    if !f.returns.is_empty() {
        buf.push_str(": ");
        for (i, t) in f.returns.iter().enumerate() {
            if i > 0 {
                buf.push_str(", ");
            }
            buf.push_str(&format_type(t));
        }
    }

    buf.push(' ');
    write_block(buf, &f.body, level);
    buf.push('\n');
}

fn pretty_global_decl(buf: &mut String, g: &GlobalDecl, level: usize) {
    indent(buf, level);
    write!(buf, "{}: {}", g.name, format_type(&g.ty)).unwrap();
    if let Some(init) = &g.init {
        write!(buf, " = {}", format_expr(init, Prec::Top)).unwrap();
    }
    buf.push_str(";\n");
}

// ── types ──────────────────────────────────────────────────────────

fn format_type(t: &Type) -> String {
    match t {
        Type::Int => "int".to_string(),
        Type::Bool => "bool".to_string(),
        Type::Array(inner, _size) => format!("{}[]", format_type(inner)),
    }
}

// ── blocks ─────────────────────────────────────────────────────────

fn write_block(buf: &mut String, block: &Block, level: usize) {
    buf.push_str("{\n");
    for s in &block.stmts {
        write_stmt(buf, s, level + 1);
    }
    if let Some(ret) = &block.return_val {
        indent(buf, level + 1);
        buf.push_str("return");
        if !ret.is_empty() {
            buf.push(' ');
            for (i, e) in ret.iter().enumerate() {
                if i > 0 {
                    buf.push_str(", ");
                }
                buf.push_str(&format_expr(e, Prec::Top));
            }
        }
        buf.push_str(";\n");
    }
    indent(buf, level);
    buf.push('}');
}

// ── statements ─────────────────────────────────────────────────────

fn write_stmt(buf: &mut String, s: &Stmt, level: usize) {
    match s {
        Stmt::If(cond, then_branch, else_branch) => {
            indent(buf, level);
            write!(buf, "if {} ", format_expr(cond, Prec::Top)).unwrap();
            write_stmt_body(buf, then_branch, level);
            if let Some(el) = else_branch {
                buf.push_str(" else ");
                write_stmt_body(buf, el, level);
            }
            buf.push('\n');
        }
        Stmt::While(cond, body) => {
            indent(buf, level);
            write!(buf, "while {} ", format_expr(cond, Prec::Top)).unwrap();
            write_stmt_body(buf, body, level);
            buf.push('\n');
        }
        Stmt::Block(block) => {
            indent(buf, level);
            write_block(buf, block, level);
            buf.push('\n');
        }
        Stmt::Assign(targets, values) => {
            write_assign(buf, targets, values, level);
        }
    }
}

/// Writes the body of an `if` or `while`. If it is already a block, writes
/// the block directly; otherwise wraps the single statement in braces.
fn write_stmt_body(buf: &mut String, s: &Stmt, level: usize) {
    match s {
        Stmt::Block(block) => {
            write_block(buf, block, level);
        }
        _ => {
            // Wrap single statement in a block for clarity.
            buf.push_str("{\n");
            write_stmt(buf, s, level + 1);
            indent(buf, level);
            buf.push('}');
        }
    }
}

/// Format an assignment / declaration / procedure-call statement.
fn write_assign(buf: &mut String, targets: &[AssignTarget], values: &[Expr], level: usize) {
    indent(buf, level);

    // Procedure call: no targets – the AST invariant is exactly one FuncCall
    // expression, but we print all values defensively.
    if targets.is_empty() {
        debug_assert!(
            values.len() == 1,
            "expected exactly 1 value in procedure-call statement, got {}",
            values.len()
        );
        for (i, val) in values.iter().enumerate() {
            if i > 0 {
                buf.push_str(", ");
            }
            buf.push_str(&format_expr(val, Prec::Top));
        }
        buf.push_str(";\n");
        return;
    }

    // Single declaration with no initialiser.
    if values.is_empty() && targets.len() == 1 {
        if let AssignTarget::Decl(name, ty) = &targets[0] {
            write!(buf, "{}: {}", name, format_type(ty)).unwrap();
            buf.push_str(";\n");
            return;
        }
    }

    // General case: targets = values
    for (i, t) in targets.iter().enumerate() {
        if i > 0 {
            buf.push_str(", ");
        }
        buf.push_str(&format_assign_target(t));
    }

    if !values.is_empty() {
        buf.push_str(" = ");
        for (i, v) in values.iter().enumerate() {
            if i > 0 {
                buf.push_str(", ");
            }
            buf.push_str(&format_expr(v, Prec::Top));
        }
    }

    buf.push_str(";\n");
}

fn format_assign_target(t: &AssignTarget) -> String {
    match t {
        AssignTarget::Discard => "_".to_string(),
        AssignTarget::Var(name) => name.clone(),
        AssignTarget::Decl(name, ty) => format!("{}: {}", name, format_type(ty)),
        AssignTarget::ArrayIndex(name, indices) => {
            let mut s = name.clone();
            for idx in indices {
                write!(s, "[{}]", format_expr(idx, Prec::Top)).unwrap();
            }
            s
        }
    }
}

// ── expressions ────────────────────────────────────────────────────

/// Operator precedence levels (higher = tighter binding).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Prec {
    Top = 0,
    Or = 1,
    And = 2,
    Eq = 3,
    Rel = 4,
    Add = 5,
    Mul = 6,
    Unary = 7,
    Postfix = 8,
    Atom = 9,
}

fn binop_prec(op: &BinOp) -> Prec {
    match op {
        BinOp::Or => Prec::Or,
        BinOp::And => Prec::And,
        BinOp::Eq | BinOp::Ne => Prec::Eq,
        BinOp::Lt | BinOp::Le | BinOp::Gt | BinOp::Ge => Prec::Rel,
        BinOp::Add | BinOp::Sub => Prec::Add,
        BinOp::Mul | BinOp::HighMul | BinOp::Div | BinOp::Mod => Prec::Mul,
    }
}

fn binop_str(op: &BinOp) -> &'static str {
    match op {
        BinOp::Add => "+",
        BinOp::Sub => "-",
        BinOp::Mul => "*",
        BinOp::HighMul => "*>>",
        BinOp::Div => "/",
        BinOp::Mod => "%",
        BinOp::Lt => "<",
        BinOp::Le => "<=",
        BinOp::Gt => ">",
        BinOp::Ge => ">=",
        BinOp::Eq => "==",
        BinOp::Ne => "!=",
        BinOp::And => "&",
        BinOp::Or => "|",
    }
}

/// Format an expression, adding parentheses only when the surrounding
/// context has higher precedence than this expression.
fn format_expr(e: &Expr, ctx: Prec) -> String {
    let (s, prec) = format_expr_inner(e);
    if prec < ctx {
        format!("({})", s)
    } else {
        s
    }
}

/// Returns (formatted_string, precedence_of_this_node).
fn format_expr_inner(e: &Expr) -> (String, Prec) {
    match e {
        Expr::IntLit(n) => (n.to_string(), Prec::Atom),
        Expr::BoolLit(b) => (b.to_string(), Prec::Atom),
        Expr::CharLit(c) => (format_char_lit(*c), Prec::Atom),
        Expr::StringLit(s) => (format_string_lit(s), Prec::Atom),
        Expr::Var(name) => (name.clone(), Prec::Atom),

        Expr::ArrayConstructor(elems) => {
            let mut s = String::from("{");
            for (i, elem) in elems.iter().enumerate() {
                if i > 0 {
                    s.push_str(", ");
                }
                s.push_str(&format_expr(elem, Prec::Top));
            }
            s.push('}');
            (s, Prec::Atom)
        }

        Expr::FuncCall(name, args) => {
            let mut s = name.clone();
            s.push('(');
            for (i, a) in args.iter().enumerate() {
                if i > 0 {
                    s.push_str(", ");
                }
                s.push_str(&format_expr(a, Prec::Top));
            }
            s.push(')');
            (s, Prec::Postfix)
        }

        Expr::Index(arr, idx) => {
            // The array sub-expression needs at least Postfix precedence
            // so that lower-precedence exprs get parenthesised.
            let arr_s = format_expr(arr, Prec::Postfix);
            let idx_s = format_expr(idx, Prec::Top);
            (format!("{}[{}]", arr_s, idx_s), Prec::Postfix)
        }

        Expr::Length(inner) => {
            let inner_s = format_expr(inner, Prec::Top);
            (format!("length({})", inner_s), Prec::Postfix)
        }

        Expr::UnaryOp(op, inner) => {
            let sym = match op {
                UnaryOp::Neg => "-",
                UnaryOp::Not => "!",
            };
            let inner_s = format_expr(inner, Prec::Unary);
            (format!("{}{}", sym, inner_s), Prec::Unary)
        }

        Expr::BinOp(op, lhs, rhs) => {
            let prec = binop_prec(op);
            // Left-associative: left child needs parens only if strictly
            // lower precedence; right child needs parens if lower *or equal*
            // (to force left-associativity).
            let lhs_s = format_expr(lhs, prec);
            let rhs_s = format_expr(rhs, next_prec(prec));
            (format!("{} {} {}", lhs_s, binop_str(op), rhs_s), prec)
        }
    }
}

/// Return the next-higher precedence level (used for the right operand
/// of a left-associative binary operator).
fn next_prec(p: Prec) -> Prec {
    match p {
        Prec::Top => Prec::Or,
        Prec::Or => Prec::And,
        Prec::And => Prec::Eq,
        Prec::Eq => Prec::Rel,
        Prec::Rel => Prec::Add,
        Prec::Add => Prec::Mul,
        Prec::Mul => Prec::Unary,
        Prec::Unary => Prec::Postfix,
        Prec::Postfix => Prec::Atom,
        Prec::Atom => Prec::Atom,
    }
}

// ── character / string escaping ────────────────────────────────────

fn format_char_lit(code: i64) -> String {
    let ch = char::from_u32(code as u32);
    match ch {
        Some('\n') => "'\\n'".to_string(),
        Some('\t') => "'\\t'".to_string(),
        Some('\r') => "'\\r'".to_string(),
        Some('\\') => "'\\\\'".to_string(),
        Some('\'') => "'\\''".to_string(),
        Some(c) if c.is_ascii_graphic() || c == ' ' => format!("'{}'", c),
        _ => format!("'\\x{{{:X}}}'", code),
    }
}

fn format_string_lit(s: &str) -> String {
    let mut out = String::from('"');
    for ch in s.chars() {
        match ch {
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            '\r' => out.push_str("\\r"),
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            c if c.is_ascii_graphic() || c == ' ' => out.push(c),
            c => write!(out, "\\x{{{:X}}}", c as u32).unwrap(),
        }
    }
    out.push('"');
    out
}

#[cfg(test)]
#[path = "pretty_tests.rs"]
mod pretty_tests;
