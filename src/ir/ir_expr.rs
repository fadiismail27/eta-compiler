use super::ir_op::{BinOp, UnOp};
use super::ir_stmt::Stmt;
use super::symbol::{Label, Temp};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    // literal
    Const(i64),
    // Variable
    Temp(Temp),
    // Logical Operations
    BinOp {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    UnOp {
        op: UnOp,
        expr: Box<Expr>,
    },
    // Content of Memory Address
    Mem(Box<Expr>),
    // Function Call (target: func, args: parameters)
    Call {
        target: Box<Expr>,
        args: Vec<Expr>,
    },
    // Address of label
    Name(Label),
    // (s ; e)
    ESeq {
        stmt: Box<Stmt>,
        expr: Box<Expr>,
    },
}