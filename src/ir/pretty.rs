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
            let child_pad = " ".repeat(indent + 2);
            let _ = writeln!(out, "{}(LEFT", child_pad);
            write_expr(left, indent + 4, out);
            let _ = writeln!(out, "{})", child_pad);

            let _ = writeln!(out, "{}(RIGHT", child_pad);
            write_expr(right, indent + 4, out);
            let _ = writeln!(out, "{})", child_pad);
            let _ = writeln!(out, "{})", pad);
        }
        Expr::UnOp { op, expr } => {
            let _ = writeln!(out, "{}(UNOP {}", pad, op);
            let child_pad = " ".repeat(indent + 2);
            let _ = writeln!(out, "{}(EXPR", child_pad);
            write_expr(expr, indent + 4, out);
            let _ = writeln!(out, "{})", child_pad);
            let _ = writeln!(out, "{})", pad);
        }
        Expr::Mem(addr) => {
            let _ = writeln!(out, "{}(MEM", pad);
            let child_pad = " ".repeat(indent + 2);
            let _ = writeln!(out, "{}(ADDR", child_pad);
            write_expr(addr, indent + 4, out);
            let _ = writeln!(out, "{})", child_pad);
            let _ = writeln!(out, "{})", pad);
        }
        Expr::Call { target, args } => {
            let _ = writeln!(out, "{}(CALL", pad);
            let child_pad = " ".repeat(indent + 2);
            let _ = writeln!(out, "{}(TARGET", child_pad);
            write_expr(target, indent + 4, out);
            let _ = writeln!(out, "{})", child_pad);

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
            let child_pad = " ".repeat(indent + 2);
            let _ = writeln!(out, "{}(STMT", child_pad);
            write_stmt(stmt, indent + 4, out);
            let _ = writeln!(out, "{})", child_pad);

            let _ = writeln!(out, "{}(EXPR", child_pad);
            write_expr(expr, indent + 4, out);
            let _ = writeln!(out, "{})", child_pad);
            let _ = writeln!(out, "{})", pad);
        }
    }
}

fn write_stmt(stmt: &Stmt, indent: usize, out: &mut String) {
    let pad = " ".repeat(indent);
    match stmt {
        Stmt::Move { dst, src } => {
            let _ = writeln!(out, "{}(MOVE", pad);
            let child_pad = " ".repeat(indent + 2);
            let _ = writeln!(out, "{}(DST", child_pad);
            write_expr(dst, indent + 4, out);
            let _ = writeln!(out, "{})", child_pad);

            let _ = writeln!(out, "{}(SRC", child_pad);
            write_expr(src, indent + 4, out);
            let _ = writeln!(out, "{})", child_pad);
            let _ = writeln!(out, "{})", pad);
        }
        Stmt::Seq(stmts) => {
            let _ = writeln!(out, "{}(SEQ", pad);
            let stmt_pad = " ".repeat(indent + 2);
            let _ = writeln!(out, "{}(STMTS", stmt_pad);
            for stmt in stmts {
                write_stmt(stmt, indent + 4, out);
            }
            let _ = writeln!(out, "{})", stmt_pad);
            let _  = writeln!(out, "{})", pad);
        }
        Stmt::Jump(label) => {
            let _ = writeln!(out, "{}(JUMP {})", pad, label);
        }
        Stmt::CJump { op, left, right, t, f } => {
            let _ = writeln!(out, "{}(CJUMP {}", pad, op);

            let child_pad = " ".repeat(indent + 2);

            let _ = writeln!(out, "{}(LEFT", child_pad);
            write_expr(left, indent + 4, out);
            let _ = writeln!(out, "{})", child_pad);

            let _ = writeln!(out, "{}(RIGHT", child_pad);
            write_expr(right, indent + 4, out);
            let _ = writeln!(out, "{})", child_pad);

            let _ = writeln!(out, "{}(TRUE {})", child_pad, t);
            let _ = writeln!(out, "{}(FALSE {})", child_pad, f);

            let _ = writeln!(out, "{})", pad);
        }
        Stmt::Label(label) => {
            let _ = writeln!(out, "{}(LABEL {})", pad, label);
        }
        Stmt::Return(exprs) => {
            let _ = writeln!(out, "{}(RETURN", pad);
            let ret_pad = " ".repeat(indent + 2);
            let _ = writeln!(out, "{}(EXPRS", ret_pad);
            for expr in exprs {
                write_expr(expr, indent + 4, out);
            }
            let _ = writeln!(out, "{})", ret_pad);
            let _ = writeln!(out, "{})", pad);
        }
    }
}
