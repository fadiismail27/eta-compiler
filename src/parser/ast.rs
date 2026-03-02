// token enum for the LALRPOP extern block
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Use,
    If,
    Else,
    While,
    Return,
    Length,
    IntType,
    BoolType,
    True,
    False,

    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,

    Colon,
    Semicolon,
    Comma,
    Assign,
    Underscore,

    Plus,
    Minus,
    Mul,
    HighMul,
    Div,
    Mod,
    Not,
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    Ne,
    And,
    Or,

    Identifier(String),
    IntLiteral(i64),
    CharLiteral(i64),
    StringLiteral(String),
}

// ── span / location types ─────────────────────────────────────────

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn dummy() -> Self {
        Span { start: 0, end: 0 }
    }
}

#[derive(Clone, Debug)]
pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}

impl<T: PartialEq> PartialEq for Spanned<T> {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node
    }
}

impl<T> Spanned<T> {
    pub fn dummy(node: T) -> Self {
        Spanned { node, span: Span::dummy() }
    }
}

pub fn spanned<T>(node: T, start: usize, end: usize) -> Spanned<T> {
    Spanned { node, span: Span { start, end } }
}

pub type Expr = Spanned<ExprKind>;
pub type Stmt = Spanned<StmtKind>;

// ── program ───────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct Program {
    pub uses: Vec<String>,
    pub items: Vec<TopLevelItem>,
}

// top-level definitions
#[derive(Clone, Debug)]
pub enum TopLevelItem {
    Func(FuncDef),
    Global(GlobalDecl),
}

#[derive(Clone, Debug)]
pub struct FuncDef {
    pub name: String,
    pub params: Vec<Param>,
    pub returns: Vec<Type>,
    pub body: Block,
}

#[derive(Clone, Debug)]
pub struct GlobalDecl {
    pub name: String,
    pub ty: Type,
    pub init: Option<Expr>,
}

// parameters and types
#[derive(Clone, Debug)]
pub struct Param {
    pub name: String,
    pub ty: Type,
}

#[derive(Clone, Debug)]
pub enum Type {
    Int,
    Bool,
    /// `Array(element_type, optional_size)`.
    /// Size is `None` for unsized arrays like `int[]`, `Some(expr)` for sized like `int[n]`.
    Array(Box<Type>, Option<Box<Expr>>),
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Type::Int, Type::Int) => true,
            (Type::Bool, Type::Bool) => true,
            (Type::Array(base1, _), Type::Array(base2, _)) => base1 == base2,
            _ => false,
        }
    }
}

// block
#[derive(Clone, Debug)]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub return_val: Option<Vec<Expr>>,
}

// statements
#[derive(Clone, Debug)]
pub enum StmtKind {
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    While(Expr, Box<Stmt>),
    Block(Block),
    Assign(Vec<AssignTarget>, Vec<Expr>),
}

// assignment targets
#[derive(Clone, Debug)]
pub enum AssignTarget {
    // this is '_'
    Discard,
    // x
    Var(String),
    // x:t
    Decl(String, Type),
    // x[][]...[]
    ArrayIndex(String, Vec<Expr>),
}

#[derive(Clone, Debug)]
pub enum IdentStmtRest {
    ProcCall(Vec<Expr>),
    Assign(Expr),
    ArrayAssign(Vec<Expr>, Expr),
    UnifiedDecl(Type, Vec<Option<Expr>>, DeclSuffix),
    MultiAssign(Vec<AssignTarget>, Vec<Expr>),
    MultiArrayAssign(Vec<Expr>, Vec<AssignTarget>, Vec<Expr>),
    CallIndexAssign(Vec<Expr>, Vec<Expr>, Expr),
}

// expressions
#[derive(PartialEq, Clone, Debug)]
pub enum ExprKind {
    BinOp(BinOp, Box<Expr>, Box<Expr>),
    UnaryOp(UnaryOp, Box<Expr>),
    Var(String),
    IntLit(i64),
    CharLit(i64),
    StringLit(String),
    BoolLit(bool),
    FuncCall(String, Vec<Expr>),
    Index(Box<Expr>, Box<Expr>),
    Length(Box<Expr>),
    ArrayConstructor(Vec<Expr>),
}

// operators
#[derive(PartialEq, Clone, Debug)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    HighMul,
    Div,
    Mod,
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    Ne,
    And,
    Or,
}

#[derive(PartialEq, Clone, Debug)]
pub enum UnaryOp {
    Neg,
    Not,
}

// .eti interface file types
#[derive(Clone, Debug)]
pub struct Interface {
    pub decls: Vec<InterfaceDecl>,
}

#[derive(Clone, Debug)]
pub struct InterfaceDecl {
    pub name: String,
    pub params: Vec<Param>,
    pub returns: Vec<Type>,
}

// parsing helpers -- only used during parse to resolve ambiguities
// with identifier-started statements, never appear in the final AST

#[derive(Clone, Debug)]
pub enum DeclSuffix {
    None,
    Init(Expr),
    Multi(Vec<AssignTarget>, Vec<Expr>),
}

impl IdentStmtRest {
    /// Converts an ident-started statement fragment into a StmtKind.
    /// `name` is the leading IDENT already consumed by the grammar.
    /// `span` is the overall span of the statement (used for internally
    /// constructed expression nodes that lack their own position).
    pub fn into_stmt(self, name: String, span: Span) -> StmtKind {
        match self {
            IdentStmtRest::ProcCall(args) => {
                let call = Spanned { node: ExprKind::FuncCall(name, args), span };
                StmtKind::Assign(vec![], vec![call])
            }
            IdentStmtRest::Assign(e) => {
                StmtKind::Assign(vec![AssignTarget::Var(name)], vec![e])
            }
            IdentStmtRest::ArrayAssign(indices, e) => {
                StmtKind::Assign(vec![AssignTarget::ArrayIndex(name, indices)], vec![e])
            }
            IdentStmtRest::UnifiedDecl(base_ty, dims, suffix) => {
                // Dimensions are parsed left-to-right: int[3][4] → dims=[Some(3), Some(4)].
                // But the first bracket is the outermost Array level, so we reverse:
                // inner = Array(Int, 4), outer = Array(inner, 3) → ([] ([] int 4) 3).
                let mut ty = base_ty;
                for dim in dims.iter().rev() {
                    ty = Type::Array(Box::new(ty), dim.clone().map(Box::new));
                }
                match suffix {
                    DeclSuffix::None => StmtKind::Assign(vec![AssignTarget::Decl(name, ty)], vec![]),
                    DeclSuffix::Init(e) => {
                        StmtKind::Assign(vec![AssignTarget::Decl(name, ty)], vec![e])
                    }
                    DeclSuffix::Multi(rest, vals) => {
                        let mut targets = vec![AssignTarget::Decl(name, ty)];
                        targets.extend(rest);
                        StmtKind::Assign(targets, vals)
                    }
                }
            }
            IdentStmtRest::MultiAssign(rest, vals) => {
                let mut targets = vec![AssignTarget::Var(name)];
                targets.extend(rest);
                StmtKind::Assign(targets, vals)
            }
            IdentStmtRest::MultiArrayAssign(indices, rest, vals) => {
                let mut targets = vec![AssignTarget::ArrayIndex(name, indices)];
                targets.extend(rest);
                StmtKind::Assign(targets, vals)
            }
            IdentStmtRest::CallIndexAssign(args, indices, e) => {
                let call = Spanned { node: ExprKind::FuncCall(name, args), span };
                let mut target = call;
                for idx in indices {
                    target = Spanned {
                        node: ExprKind::Index(Box::new(target), Box::new(idx)),
                        span,
                    };
                }
                StmtKind::Assign(vec![AssignTarget::Discard], vec![e])
            }
        }
    }
}
