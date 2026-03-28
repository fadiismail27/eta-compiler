use std::fmt;

// ── Binary Operators ──────────────────────────────────────────────────
/// Binary operators for expression nodes like `BinOp(op, left, right)`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    HMul, // High Multiply, (high bits)
    Div,
    Mod,
    And,
    Or,
    Xor,
    RShift,
    LShift,
    ARShift,
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            BinOp::Add => "ADD",
            BinOp::Sub => "SUB",
            BinOp::Mul => "MUL",
            BinOp::HMul => "HMUL",
            BinOp::Div => "DIV",
            BinOp::Mod => "MOD",
            BinOp::And => "AND",
            BinOp::Or => "OR",
            BinOp::Xor => "XOR",
            BinOp::RShift => "RSHIFT",
            BinOp::LShift => "LSHIFT",
            BinOp::ARShift => "ARSHIFT",
        };
        f.write_str(s)
    }    
}

// ── Relational Operators ──────────────────────────────────────────────────
// Relational operators for Conditional branches '(CJump)'
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RelOp {
    Eq,
    Neq,
    Lt,
    Leq,
    Gt,
    Geq,
    Ult, // Unsigned Less Than
    Uleq, // Unsigned Less or Equal
    Ugt, // Unsigned Greater Than
    Ugeq, // Unsigned Greater Than or Equal
}

impl fmt::Display for RelOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            RelOp::Eq => "EQ",
            RelOp::Neq => "NEQ",
            RelOp::Lt => "LT",
            RelOp::Leq => "LEQ",
            RelOp::Gt => "GT",
            RelOp::Geq => "GEQ",
            RelOp::Ult => "ULT",
            RelOp::Uleq => "ULEQ",
            RelOp::Ugt => "UGT",
            RelOp::Ugeq => "UGEQ",
        };
        f.write_str(s)
    }    
}

// ── Unary Operators ──────────────────────────────────────────────────
// Unary operators '(NOT)'
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum UnOp {
    Not,
}

impl fmt::Display for UnOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            UnOp::Not => "NOT",
        };
        f.write_str(s)
    }    
}