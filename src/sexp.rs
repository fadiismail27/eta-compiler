//! S-expression printer for the Eta AST.
//!
//! Produces the S-expression output format required by the PA2 test harness.
//! Depends only on `crate::parser::ast`.

use crate::parser::ast::*;

// ── intermediate S-expression tree ─────────────────────────────────

/// A simple S-expression: either an atom or a list of sub-expressions.
#[derive(Clone, Debug)]
enum SExp {
    Atom(String),
    List(Vec<SExp>),
}

// ── public entry points ────────────────────────────────────────────

/// Convert a Program AST to its S-expression string.
/// Format: `((use*) (definition*))`
pub fn sexp_program(p: &Program) -> String {
    let uses: Vec<SExp> = p
        .uses
        .iter()
        .map(|u| SExp::List(vec![SExp::Atom("use".into()), SExp::Atom(u.clone())]))
        .collect();

    let defs: Vec<SExp> = p
        .items
        .iter()
        .map(|item| match item {
            TopLevelItem::Func(f) => sexp_func_def(f),
            TopLevelItem::Global(g) => sexp_global_decl(g),
        })
        .collect();

    let tree = SExp::List(vec![SExp::List(uses), SExp::List(defs)]);
    render(&tree, 0)
}

/// Convert an Interface AST (.eti file) to its S-expression string.
/// Format: `((method_interface*))`
pub fn sexp_interface(iface: &Interface) -> String {
    let methods: Vec<SExp> = iface.decls.iter().map(sexp_interface_decl).collect();
    let tree = SExp::List(vec![SExp::List(methods)]);
    render(&tree, 0)
}

// ── top-level items ────────────────────────────────────────────────

/// `(name (params*) (returns*) block)`
fn sexp_func_def(f: &FuncDef) -> SExp {
    let mut elems = vec![SExp::Atom(f.name.clone())];
    elems.push(SExp::List(f.params.iter().map(sexp_param).collect()));
    elems.push(SExp::List(f.returns.iter().map(sexp_type).collect()));
    elems.push(sexp_block(&f.body));
    SExp::List(elems)
}

/// `(:global name type)` or `(:global name type value)`
fn sexp_global_decl(g: &GlobalDecl) -> SExp {
    let mut elems = vec![
        SExp::Atom(":global".into()),
        SExp::Atom(g.name.clone()),
        sexp_type(&g.ty),
    ];
    if let Some(init) = &g.init {
        elems.push(sexp_expr(init));
    }
    SExp::List(elems)
}

/// `(name (params*) (returns*))`
fn sexp_interface_decl(d: &InterfaceDecl) -> SExp {
    let mut elems = vec![SExp::Atom(d.name.clone())];
    elems.push(SExp::List(d.params.iter().map(sexp_param).collect()));
    elems.push(SExp::List(d.returns.iter().map(sexp_type).collect()));
    SExp::List(elems)
}

// ── parameters and types ───────────────────────────────────────────

/// `(name type)`
fn sexp_param(p: &Param) -> SExp {
    SExp::List(vec![SExp::Atom(p.name.clone()), sexp_type(&p.ty)])
}

/// `int`, `bool`, `([] inner)` or `([] inner size)`
fn sexp_type(t: &Type) -> SExp {
    match t {
        Type::Int => SExp::Atom("int".into()),
        Type::Bool => SExp::Atom("bool".into()),
        Type::Array(inner, size) => {
            let mut elems = vec![SExp::Atom("[]".into()), sexp_type(inner)];
            if let Some(sz) = size {
                elems.push(sexp_expr(sz));
            }
            SExp::List(elems)
        }
    }
}

// ── blocks ─────────────────────────────────────────────────────────

/// `(stmt1 stmt2 ... [return ...])`
fn sexp_block(block: &Block) -> SExp {
    let mut elems: Vec<SExp> = block.stmts.iter().map(sexp_stmt).collect();
    if let Some(ret) = &block.return_val {
        let mut ret_elems = vec![SExp::Atom("return".into())];
        ret_elems.extend(ret.iter().map(sexp_expr));
        elems.push(SExp::List(ret_elems));
    }
    SExp::List(elems)
}

// ── statements ─────────────────────────────────────────────────────

fn sexp_stmt(s: &Stmt) -> SExp {
    match s {
        Stmt::If(cond, then_branch, else_branch) => {
            let mut elems = vec![
                SExp::Atom("if".into()),
                sexp_expr(cond),
                sexp_stmt_body(then_branch),
            ];
            if let Some(el) = else_branch {
                elems.push(sexp_stmt_body(el));
            }
            SExp::List(elems)
        }
        Stmt::While(cond, body) => SExp::List(vec![
            SExp::Atom("while".into()),
            sexp_expr(cond),
            sexp_stmt_body(body),
        ]),
        Stmt::Block(block) => sexp_block(block),
        Stmt::Assign(targets, values) => sexp_assign(targets, values),
    }
}

/// For if/while bodies: if it's a Block, render as `(stmts...)`;
/// otherwise render the single statement directly.
fn sexp_stmt_body(s: &Stmt) -> SExp {
    match s {
        Stmt::Block(block) => sexp_block(block),
        other => sexp_stmt(other),
    }
}

/// Handle the many cases of `Stmt::Assign(targets, values)`.
fn sexp_assign(targets: &[AssignTarget], values: &[Expr]) -> SExp {
    // Procedure call: no targets, just the call expression
    if targets.is_empty() {
        if let Some(val) = values.first() {
            return sexp_expr(val);
        }
        return SExp::List(vec![]);
    }

    // Declaration only (no initialiser): single Decl target, no values
    //   `x: int` → `(x int)`
    //   `a: int[][]` → `(a ([] ([] int)))`
    if values.is_empty() && targets.len() == 1 {
        return sexp_decl_target(&targets[0]);
    }

    // Assignment: `(= lhs rhs)`
    let lhs = if targets.len() == 1 {
        sexp_assign_target(&targets[0])
    } else {
        SExp::List(targets.iter().map(sexp_assign_target).collect())
    };

    // Values: should be exactly one expression for assignment
    let rhs = if values.len() == 1 {
        sexp_expr(&values[0])
    } else {
        // Multiple values shouldn't normally occur; render as a list
        SExp::List(values.iter().map(sexp_expr).collect())
    };

    SExp::List(vec![SExp::Atom("=".into()), lhs, rhs])
}

/// Format an AssignTarget for the LHS of an assignment `(= lhs rhs)`.
fn sexp_assign_target(t: &AssignTarget) -> SExp {
    match t {
        AssignTarget::Discard => SExp::Atom("_".into()),
        AssignTarget::Var(name) => SExp::Atom(name.clone()),
        AssignTarget::Decl(name, ty) => {
            SExp::List(vec![SExp::Atom(name.clone()), sexp_type(ty)])
        }
        AssignTarget::ArrayIndex(name, indices) => {
            let mut result = SExp::Atom(name.clone());
            for idx in indices {
                result = SExp::List(vec![SExp::Atom("[]".into()), result, sexp_expr(idx)]);
            }
            result
        }
    }
}

/// Format a declaration-only target (no assignment, just the declaration).
/// `Decl("x", Int)` → `(x int)`
/// `Var("x")` → `x`  (bare variable name, though unusual without init)
fn sexp_decl_target(t: &AssignTarget) -> SExp {
    match t {
        AssignTarget::Decl(name, ty) => {
            SExp::List(vec![SExp::Atom(name.clone()), sexp_type(ty)])
        }
        other => sexp_assign_target(other),
    }
}

// ── expressions ────────────────────────────────────────────────────

fn sexp_expr(e: &Expr) -> SExp {
    match e {
        Expr::IntLit(n) => SExp::Atom(n.to_string()),
        Expr::BoolLit(b) => SExp::Atom(b.to_string()),
        Expr::CharLit(c) => SExp::Atom(format_char_lit(*c)),
        Expr::StringLit(s) => SExp::Atom(format_string_lit(s)),
        Expr::Var(name) => SExp::Atom(name.clone()),

        Expr::BinOp(op, lhs, rhs) => SExp::List(vec![
            SExp::Atom(binop_str(op).into()),
            sexp_expr(lhs),
            sexp_expr(rhs),
        ]),

        Expr::UnaryOp(op, inner) => {
            let sym = match op {
                UnaryOp::Neg => "-",
                UnaryOp::Not => "!",
            };
            SExp::List(vec![SExp::Atom(sym.into()), sexp_expr(inner)])
        }

        Expr::FuncCall(name, args) => {
            let mut elems = vec![SExp::Atom(name.clone())];
            elems.extend(args.iter().map(sexp_expr));
            SExp::List(elems)
        }

        Expr::Index(arr, idx) => {
            SExp::List(vec![SExp::Atom("[]".into()), sexp_expr(arr), sexp_expr(idx)])
        }

        Expr::Length(inner) => {
            SExp::List(vec![SExp::Atom("length".into()), sexp_expr(inner)])
        }

        Expr::ArrayConstructor(elems) => {
            SExp::List(elems.iter().map(sexp_expr).collect())
        }
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

// ── character / string formatting ──────────────────────────────────

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
            c => {
                out.push_str(&format!("\\x{{{:X}}}", c as u32));
            }
        }
    }
    out.push('"');
    out
}

// ── rendering ──────────────────────────────────────────────────────

/// Render an S-expression tree as a string with indentation.
fn render(sexp: &SExp, indent: usize) -> String {
    match sexp {
        SExp::Atom(s) => s.clone(),
        SExp::List(elems) => {
            if elems.is_empty() {
                return "()".to_string();
            }

            let flat = render_flat(sexp);
            if flat.len() + indent <= 80 {
                return flat;
            }

            // Multi-line rendering: each element on its own line,
            // indented one space past the opening paren.
            let inner_indent = indent + 1;
            let mut lines = Vec::new();
            for (i, elem) in elems.iter().enumerate() {
                let rendered = render(elem, inner_indent);
                if i == 0 {
                    lines.push(format!("({}", rendered));
                } else {
                    lines.push(format!("{}{}", " ".repeat(inner_indent), rendered));
                }
            }
            let last = lines.len() - 1;
            lines[last].push(')');
            lines.join("\n")
        }
    }
}

/// Render an S-expression on a single line (no line breaks).
fn render_flat(sexp: &SExp) -> String {
    match sexp {
        SExp::Atom(s) => s.clone(),
        SExp::List(elems) => {
            if elems.is_empty() {
                return "()".to_string();
            }
            let parts: Vec<String> = elems.iter().map(|e| render_flat(e)).collect();
            format!("({})", parts.join(" "))
        }
    }
}

// ── tests ──────────────────────────────────────────────────────────

#[cfg(test)]
#[path = "sexp_tests.rs"]
mod sexp_tests;
