use crate::lexer::Token;
// snake case for everything other than enums and structs

// spanning allows us to track the location of each node in the source code
pub type SpannedAST = Spanned<Program>; // i think i use like this...

// "when I match on this variant, do I need to extract any data from it?"
// If yes, give it values. If every variant is interchangeable and
// carries no unique data, keep them bare.

pub struct Program {
    pub uses: Vec<String>, // uses is a vector of identifiers, when we pretty print, we precede them with "use"
    pub definitions: Vec<Definition>,
}

pub enum Definition { // toplevelitem in grammar
    Method(Method),
    GlobalDeclaration(Declaration),
}

pub struct Method {
    pub name: String,
    pub declarations: <Vec<Declaration>>,
    pub return_types: <Vec<Type>>,
    pub block: Block,
}

pub enum Type {
    // one of bool, int...
    Bool,
    Int,
    Array(Box<Type>),
}

pub struct Block {
    pub statements: Vec<Stmt>,
}

pub enum Stmt {
    Expression(Expression),
    Declaration(Declaration),
    Assignment(Assignment),
    //
}

pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,
}
pub enum Expr {
    Literal(Literal),
    Identifier(String),
    Grouped(Box<Expr>),
    Binary {
        left: Box<Expr>,
        op: Op,
        right: Box<Expr>,
    },
    Unary {
        op: Op,
        right: Box<Expr>,
    },
}
// expr
pub struct Span {
    pub start: usize,  // byte offset
    pub end: usize,
}

pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}

pub struct Argument {

}
