use crate::lexer::Token;
// snake case for everything other than enums and structs

// spanning allows us to track the location of each node in the source code
pub type SpannedAST = Spanned<Program>; // i think i use like this...

// "when I match on this variant, do I need to extract any data from it?"
// If yes, give it values. If every variant is interchangeable and
// carries no unique data, keep them bare.

pub struct Program {
    pub uses: Option<Vec<Token::Identifier>>, // uses is a vector of identifiers, when we pretty print, we precede them with "use"
    pub definitions: Option<Vec<Definition>>,
}

pub enum Definition { // toplevelitem in grammar
    Method(Method),
    GlobalDeclaration(Declaration),
}

pub struct Method {
    pub name: Token::Identifier,
    pub declarations: Option<Vec<Declaration>>,
    pub return_types: Option<Vec<Type>>,
    pub block: Block,
}

pub enum Type {
    // one of bool, int...
    Bool,
    Int,
    // int[]... not recursive but i dont rlly get it ....
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
