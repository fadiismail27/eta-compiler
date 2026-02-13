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
    Global(DeclData),
}

pub struct Method { // a method is a function, with a name, parameters, return types, and a block
    pub id: String,
    pub params: Vec<Param>, // a parameter in a function..., no initialization
    pub return_tys: Vec<Type>,
    pub block: Block,
}

pub struct Param {
    pub id: String,
    pub ty: Type,
}

pub struct DeclData {
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
    VarDecl(DeclData),
    Assignment {lhs: Expr, rhs: Expr}, // assign only works for existing types
    If {cond: Expr, then_block: Block, else_block: Option<Block>}, // can Optionally be chained with an else
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
    And,
    Or,
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
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    Unary {
        op: UnaryOp,
        right: Box<Expr>,
    },
    FuncCall { id: String, args: Vec<Expr> },
    ArrayIndex { array: Box<Expr>, index: Box<Expr> },
    ArrayLit(Vec<Expr>),
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
