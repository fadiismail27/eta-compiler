use super::op::{RelOp};
use super::expr::Expr;
use super::symbol::{Label};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt {
    // (dest <- e)
    Move {
        dst: Box<Expr>,
        src: Box<Expr>,
    },
    // s1 ... sn
    Seq(Vec<Stmt>),
    Jump(Label),
    // CJUMP(e, L1, L2)
    CJump {
        op: RelOp,
        left: Box<Expr>,
        right: Box<Expr>,
        t: Label,
        f: Label,
    },
    Label(Label),
    // Return(e1,...,en)
    Return(Vec<Expr>),
}