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

pub enum Definition { // a definition is a method or a global declaration
    Method(Method),
    GlobalDeclaration(Declaration),
}

pub struct Method { // a method is a function, with a name, parameters, return types, and a block
    pub id: String,
    pub declarations: Vec<Declaration>, // a parameter in a function..., no initialization
    pub return_tys: Vec<Type>,
    pub block: Block,
}

pub struct Declaration {
    pub id: String,
    pub ty: Type,
}

// i think this can also be applied to declarations within a method, which are 'global' in that scope
pub struct GlobalDeclaration {
    pub id: String,
    pub ty: Type,
    pub init: Option<Expr>,
}

pub enum Type {
    Bool,
    Int,
    Array(Box<Type>),
}

pub struct Block {
    pub statements: Vec<Stmt>,
}

pub enum Stmt {
    Expression(Expr),
    Declaration(Declaration),
    Assignment(Assignment),
    If {cond: Expr, then_block: Block, else: Option<Block>}, // can Optionally be chained with an else
    While {cond: Expr, body: Block}, // should just contain an expression i think...
    Return(Vec<Expr>), // list of expressions
    Proc {id: String, args: Vec<Expr>} , // name and args i believe
}

// assignment can be a list of expressions, procedure calls, or types...
pub enum BinaryOp {
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

// not sure how we wanna represent strings but fine rn...
pub enum Literal {
    Int(i64),
    Bool(bool),
    Char(char),
    String(String),
}

pub enum UnaryOp {
    Neg,
    Not,
}

// expressions can exist after an equals sign, after an if / while statement...
pub enum Expr {
    Literal(Literal),
    Identifier(String),
    Grouped(Box<Expr>), // ?
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    Unary {
        op: UnaryOp,
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
