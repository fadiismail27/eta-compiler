use std::fmt::Write;

use super::expr::Expr;
use super::stmt::Stmt;

/// Pretty-print an IR expression tree.
pub fn pretty_expr(expr: &Expr) -> String {
    let mut out = String::new();
    write_expr(expr, 0, &mut out);
    out
}

fn write_expr(expr: &Expr, indent: usize, out: &mut String) {
    let pad = " ".repeat(indent);
    match expr {
        Expr::Const(value) => {
            let _ = writeln!(out, "{}(CONST {})", pad, value);
        }
        Expr::Temp(temp) => {
            let _ = writeln!(out, "{}(TEMP {})", pad, temp);
        }
        Expr::BinOp { op, left, right } => {
            let _ = writeln!(out, "{}(BINOP {}", pad, op);
            write_expr(left, indent + 2, out);
            write_expr(right, indent + 2, out);
            let _ = writeln!(out, "{})", pad);
        }
        Expr::UnOp { op, expr } => {
            let _ = writeln!(out, "{}(UNOP {}", pad, op);
            write_expr(expr, indent + 2, out);
            let _ = writeln!(out, "{})", pad);
        }
        Expr::Mem(addr) => {
            let _ = writeln!(out, "{}(MEM", pad);
            write_expr(addr, indent + 2, out);
            let _ = writeln!(out, "{})", pad);
        }
        Expr::Call { target, args } => {
            let _ = writeln!(out, "{}(CALL", pad);
            write_expr(target, indent + 2, out);
            let args_pad = " ".repeat(indent + 2);
            let _ = writeln!(out, "{}(ARGS", args_pad);
            for arg in args {
                write_expr(arg, indent + 4, out);
            }
            let _ = writeln!(out, "{})", args_pad);
            let _ = writeln!(out, "{})", pad);
        }
        Expr::Name(label) => {
            let _ = writeln!(out, "{}(NAME {})", pad, label);
        }
        Expr::ESeq { stmt, expr } => {
            let _ = writeln!(out, "{}(ESEQ", pad);
            write_stmt_placeholder(stmt, indent + 2, out);
            write_expr(expr, indent + 2, out);
            let _ = writeln!(out, "{})", pad);
        }
    }
}

fn write_stmt_placeholder(stmt: &Stmt, indent: usize, out: &mut String) {
    let pad = " ".repeat(indent);
    let _ = writeln!(out, "{}(STMT {:?})", pad, stmt);
}
