use crate::checker::context::Context;
use crate::parser::ast::{AssignTarget, BinOp, Expr, IdentStmtRest, Stmt, Type, UnaryOp, Program, TopLevelItem, FuncDef, Block};

#[derive(Clone, Debug)]
pub enum SemanticError {
    // 1. Scope Errors
    UndeclaredIdentifier {
        name: String,
    },
    DuplicateDeclaration {
        name: String,
    },

    // 2. Type Errors
    TypeMismatch {
        expected: SemanticType,
        found: SemanticType,
    },
    ExpectedArray {
        found: SemanticType,
    },
    InvalidBinaryOp {
        op: BinOp,
        left: SemanticType,
        right: SemanticType,
    },
    InvalidUnaryOp {
        op: UnaryOp,
        exp: SemanticType,
    },
    ConditionNotBoolean {
        found: SemanticType,
    },

    // 3. Function Errors
    NotCallable {
        found: SemanticType,
    },
    ArgumentCountMismatch {
        expected: usize,
        found: usize,
    },
    ArgumentTypeMismatch {
        param_index: usize,
        expected: SemanticType,
        found: SemanticType,
    },
    InvalidReturnType {
        found: SemanticType,
    },
    InvalidBaseType {
        found: SemanticType,
    },

    // 4. Control Flow Errors
    BreakOutsideLoop,
    MissingReturn {
        expected: SemanticType,
    },
}

#[derive(PartialEq, Clone, Debug)]
pub enum SemanticType {
    Single(Type),
    Tuple(Vec<Type>),
    Func {
        args: Vec<Type>,
        ret: Box<SemanticType>,
    },
    EmptyArray,

    Unit,
    Void,

    Error,
}

pub struct TypeChecker {
    errors: Vec<SemanticError>,
    context: Context,
    current_return_type: Option<SemanticType>,
}
impl TypeChecker {
    fn check_program(&mut self, program: &Program) {
        // Pass 1: collect all signatures into context
        for item in &program.items {
            match item {
                TopLevelItem::Func(func) => {
                    if self.context.search(&func.name).is_some() {
                        self.errors.push(SemanticError::DuplicateDeclaration {
                            name: func.name.clone(),
                        });
                        continue;
                    }
                    let ret = if func.returns.is_empty() {
                        SemanticType::Tuple(vec![]) // procedure
                    } else {
                        SemanticType::Tuple(func.returns.clone())
                    };
                    self.context.push(
                        &func.name,
                        &SemanticType::Func {
                            args: func.params.iter().map(|p| p.ty.clone()).collect(),
                            ret: Box::new(ret),
                        },
                    );
                }
                TopLevelItem::Global(global) => {
                    if self.context.search(&global.name).is_some() {
                        self.errors.push(SemanticError::DuplicateDeclaration {
                            name: global.name.clone(),
                        });
                        continue;
                    }
                    self.context
                        .push(&global.name, &SemanticType::Single(global.ty.clone()));
                }
            }
        }

        // Pass 2: check each body
        for item in &program.items {
            match item {
                TopLevelItem::Func(func) => self.check_func_def(func),
                TopLevelItem::Global(global) => {
                    // spec: init must be a literal
                    if let Some(init) = &global.init {
                        let t = self.check_expr(init);
                        let expected = SemanticType::Single(global.ty.clone());
                        if !self.types_compatible(&t, &expected) {
                            self.errors
                                .push(SemanticError::TypeMismatch { expected, found: t });
                        }
                    }
                }
            }
        }
    }


    fn types_compatible(&self, actual: &SemanticType, expected: &SemanticType) -> bool {
        if actual == expected {
            return true;
        }
        match (actual, expected) {
            (SemanticType::EmptyArray, SemanticType::Single(Type::Array(_, _))) => true,
            (SemanticType::Error, _) | (_, SemanticType::Error) => true, // suppress cascading
            _ => false,
        }
    }

    fn check_func_def(&mut self, func: &FuncDef) {
        // #[derive(Clone, Debug)]
        // pub struct FuncDef {
        //     pub name: String,
        //     pub params: Vec<Param>,
        //     pub returns: Vec<Type>,
        //     pub body: Block,
        // }

        // parameters and types
        // #[derive(Clone, Debug)]
        // pub struct Param {
        //     pub name: String,
        //     pub ty: Type,
        // }

        let initial_context = self.context.clone();

        for param in &func.params {
            let name = param.name.clone();
            let typ = param.ty.clone();
            self.context.push(&name, &SemanticType::Single(typ));
        }

        let ret = if func.returns.len() > 0 {
            SemanticType::Tuple(func.returns.clone())
        } else {
            SemanticType::Tuple(vec![])
        };

        self.current_return_type = Some(ret.clone());
        let stmt_type = self.check_stmt(&Stmt::Block(func.body.clone()));

        if func.returns.len() > 0 && stmt_type != SemanticType::Void {
            self.errors.push(SemanticError::InvalidReturnType { found: stmt_type });
        }
        self.context = initial_context.clone();

        self.current_return_type = None;
    }

    pub fn check_expr(&mut self, exp: &Expr) -> SemanticType {
        match exp {
            Expr::Var(var) => match self.context.search(var) {
                Some(typ) => typ.clone(),
                None => {
                    self.errors
                        .push(SemanticError::UndeclaredIdentifier { name: var.clone() });
                    SemanticType::Error
                }
            },
            Expr::IntLit(_) | Expr::CharLit(_) => SemanticType::Single(Type::Int),
            Expr::StringLit(_) => SemanticType::Single(Type::Array(Box::new(Type::Int), None)),
            Expr::BoolLit(_) => SemanticType::Single(Type::Bool),

            Expr::BinOp(op, expr1, expr2) => {
                // let typ1 = self.check_expr(expr1);
                // let typ2 = self.check_expr(expr2);

                // // 1. Ensure both sides are single types (Eta operators don't work on tuples)
                // if typ1 != typ2 {
                //     self.errors.push(SemanticError::InvalidBinaryOp {
                //         op: op.clone(),
                //         left: typ1.clone(),
                //         right: typ2.clone(),
                //     });
                //     return SemanticType::Error;
                // }
                let typ1 = self.check_expr(expr1);
                let typ2 = self.check_expr(expr2);

                // Helper to stop cascading error spam: if a child expression already
                // threw an error, we don't need to throw another one here.
                let is_err = typ1 == SemanticType::Error || typ2 == SemanticType::Error;

                match op {
                    // match op {
                    BinOp::Add => {
                        match (&typ1, &typ2) {
                            // 1. Int + Int
                            (SemanticType::Single(Type::Int), SemanticType::Single(Type::Int)) => {
                                SemanticType::Single(Type::Int)
                            }

                            // 2. Typed Array + Typed Array
                            (
                                SemanticType::Single(Type::Array(base1, _)),
                                SemanticType::Single(Type::Array(base2, _)),
                            ) => {
                                if base1 != base2 {
                                    self.errors.push(SemanticError::TypeMismatch {
                                        expected: typ1.clone(),
                                        found: typ2.clone(),
                                    });
                                    return SemanticType::Error;
                                }
                                SemanticType::Single(Type::Array(base1.clone(), None))
                            }

                            // 3. Typed Array + Empty Array (and vice versa)
                            (SemanticType::Single(Type::Array(_, _)), SemanticType::EmptyArray) => {
                                typ1.clone()
                            }
                            (SemanticType::EmptyArray, SemanticType::Single(Type::Array(_, _))) => {
                                typ2.clone()
                            }

                            // 4. Empty Array + Empty Array
                            (SemanticType::EmptyArray, SemanticType::EmptyArray) => {
                                SemanticType::EmptyArray
                            }

                            // Invalid Addition
                            _ => {
                                if !is_err {
                                    self.errors.push(SemanticError::InvalidBinaryOp {
                                        op: op.clone(),
                                        left: typ1.clone(),
                                        right: typ2.clone(),
                                    });
                                }
                                SemanticType::Error
                            }
                        }
                    }
                    BinOp::Sub | BinOp::Mul | BinOp::HighMul | BinOp::Div | BinOp::Mod => {
                        if typ1 == SemanticType::Single(Type::Int)
                            && typ2 == SemanticType::Single(Type::Int)
                        {
                            SemanticType::Single(Type::Int)
                        } else {
                            if !is_err {
                                self.errors.push(SemanticError::InvalidBinaryOp {
                                    op: op.clone(),
                                    left: typ1.clone(),
                                    right: typ2.clone(),
                                });
                            }
                            SemanticType::Error
                        }
                    }
                    BinOp::Lt | BinOp::Le | BinOp::Gt | BinOp::Ge => {
                        if typ1 == SemanticType::Single(Type::Int)
                            && typ2 == SemanticType::Single(Type::Int)
                        {
                            SemanticType::Single(Type::Bool)
                        } else {
                            if !is_err {
                                self.errors.push(SemanticError::InvalidBinaryOp {
                                    op: op.clone(),
                                    left: typ1.clone(),
                                    right: typ2.clone(),
                                });
                            }
                            SemanticType::Error
                        }
                    }
                    BinOp::Eq | BinOp::Ne => {
                        match (&typ1, &typ2) {
                            // Exact matches (Int == Int, Bool == Bool, Array == Array)
                            (t1, t2) if t1 == t2 && !is_err => SemanticType::Single(Type::Bool),

                            // Comparing typed arrays with empty arrays
                            (SemanticType::Single(Type::Array(_, _)), SemanticType::EmptyArray) => {
                                SemanticType::Single(Type::Bool)
                            }
                            (SemanticType::EmptyArray, SemanticType::Single(Type::Array(_, _))) => {
                                SemanticType::Single(Type::Bool)
                            }

                            _ => {
                                if !is_err {
                                    self.errors.push(SemanticError::TypeMismatch {
                                        expected: typ1.clone(),
                                        found: typ2.clone(),
                                    });
                                }
                                SemanticType::Error
                            }
                        }
                    }
                    BinOp::Or | BinOp::And => {
                        if typ1 == SemanticType::Single(Type::Bool)
                            && typ2 == SemanticType::Single(Type::Bool)
                        {
                            SemanticType::Single(Type::Bool)
                        } else {
                            if !is_err {
                                self.errors.push(SemanticError::InvalidBinaryOp {
                                    op: op.clone(),
                                    left: typ1.clone(),
                                    right: typ2.clone(),
                                });
                            }
                            SemanticType::Error
                        }
                    }
                }
            }
            Expr::UnaryOp(op, exp) => {
                let typ = self.check_expr(exp);
                match op {
                    UnaryOp::Neg => {
                        if typ != SemanticType::Single(Type::Int) {
                            self.errors.push(SemanticError::InvalidUnaryOp {
                                op: op.clone(),
                                exp: typ.clone(),
                            });
                            return SemanticType::Error;
                        }
                        SemanticType::Single(Type::Int)
                    }
                    UnaryOp::Not => {
                        if typ != SemanticType::Single(Type::Bool) {
                            self.errors.push(SemanticError::InvalidUnaryOp {
                                op: op.clone(),
                                exp: typ.clone(),
                            });
                            return SemanticType::Error;
                        }
                        SemanticType::Single(Type::Bool)
                    }
                }
            }
            // }
            // }
            // },
            Expr::FuncCall(s, e_vec) => {
                let func = self.context.search(s).cloned();
                match func {
                    Some(SemanticType::Func { args, ret }) => {
                        match *ret {
                            SemanticType::Single(_) => {
                                let arg_len = args.len();
                                let exp_len = e_vec.len();

                                if arg_len != exp_len {
                                    self.errors.push(SemanticError::ArgumentCountMismatch {
                                        expected: arg_len,
                                        found: exp_len,
                                    });
                                    return SemanticType::Error;
                                }

                                for i in 0..exp_len {
                                    let actual_typ = self.check_expr(&e_vec[i]);
                                    let expected_typ = SemanticType::Single(args[i].clone());
                                    if actual_typ != expected_typ {
                                        self.errors.push(SemanticError::ArgumentTypeMismatch {
                                            param_index: i,
                                            expected: expected_typ,
                                            found: actual_typ,
                                        });
                                        return SemanticType::Error;
                                    }
                                }
                            }
                            _ => {
                                self.errors
                                    .push(SemanticError::InvalidReturnType { found: *ret });
                                return SemanticType::Error;
                            }
                        }

                        *ret
                    }
                    Some(x) => {
                        self.errors
                            .push(SemanticError::NotCallable { found: x.clone() });
                        SemanticType::Error
                    }

                    None => {
                        self.errors
                            .push(SemanticError::UndeclaredIdentifier { name: s.clone() });
                        SemanticType::Error
                    }
                }
            }

            Expr::Length(exp) => {
                let typ = self.check_expr(exp);
                match typ {
                    SemanticType::Single(Type::Array(_, _)) | SemanticType::EmptyArray => {
                        SemanticType::Single(Type::Int)
                    }
                    _ => {
                        self.errors
                            .push(SemanticError::ExpectedArray { found: typ });
                        SemanticType::Error
                    }
                }
            }
            Expr::Index(exp1, exp2) => {
                let typ1 = self.check_expr(exp1);
                let typ2 = self.check_expr(exp2);

                match typ1 {
                    SemanticType::Single(Type::Array(base, _)) => match typ2 {
                        SemanticType::Single(Type::Int) => SemanticType::Single(*base),
                        _ => {
                            self.errors.push(SemanticError::TypeMismatch {
                                expected: SemanticType::Single(Type::Int),
                                found: typ2,
                            });
                            SemanticType::Error
                        }
                    },
                    _ => {
                        self.errors
                            .push(SemanticError::ExpectedArray { found: typ1 });
                        SemanticType::Error
                    }
                }
            }

            Expr::ArrayConstructor(exp_lst) => {
                if exp_lst.len() == 0 {
                    return SemanticType::EmptyArray;
                }

                let base_typ = self.check_expr(&exp_lst[0]).clone();

                for i in 1..exp_lst.len() {
                    let typ = self.check_expr(&exp_lst[i]);

                    if typ != base_typ {
                        self.errors.push(SemanticError::TypeMismatch {
                            expected: base_typ,
                            found: typ,
                        });
                        return SemanticType::Error;
                    }
                }

                match base_typ {
                    SemanticType::Single(t) => SemanticType::Single(Type::Array(Box::new(t), None)),
                    SemanticType::EmptyArray => SemanticType::EmptyArray,
                    _ => {
                        self.errors
                            .push(SemanticError::InvalidBaseType { found: base_typ });
                        SemanticType::Error
                    }
                }
            }
        }
    }

    pub fn check_stmt(&mut self, stmt: &Stmt) -> SemanticType {
        // If(Expr, Box<Stmt>, Option<Box<Stmt>>),
        match stmt {
            Stmt::If(expr, t_stmt, f_stmt) => {
                let initial_context = self.context.clone();
                let guard = self.check_expr(expr);

                if guard != SemanticType::Single(Type::Bool) {
                    self.errors.push(SemanticError::TypeMismatch {
                        expected: SemanticType::Single(Type::Bool),
                        found: guard,
                    });
                    return SemanticType::Error;
                }

                let t_return = self.check_stmt(&*t_stmt);
                self.context = initial_context.clone();

                match f_stmt {
                    Some(s) => {
                        let f_return = self.check_stmt(&*s);
                        self.context = initial_context;

                        if t_return == SemanticType::Void && f_return == SemanticType::Void {
                            return SemanticType::Void;
                        }
                        SemanticType::Unit
                    }
                    None => SemanticType::Unit,
                }
            }
            // While(Expr, Box<Stmt>),
            Stmt::While(expr, s) => {
                let initial_context = self.context.clone();
                let guard = self.check_expr(expr);

                if guard != SemanticType::Single(Type::Bool) {
                    self.errors.push(SemanticError::TypeMismatch {
                        expected: SemanticType::Single(Type::Bool),
                        found: guard,
                    });
                    return SemanticType::Error;
                }
                self.check_stmt(s);
                self.context = initial_context.clone();

                SemanticType::Unit
            }
            // Block(Block),
            Stmt::Block(Block { stmts, return_val }) => {
                let initial_context = self.context.clone();
                let mut result = SemanticType::Unit;

                for (i, s) in stmts.iter().enumerate() {
                    let s_type = self.check_stmt(s);

                    if i == stmts.len() - 1 && return_val.is_none() {
                        result = s_type;
                    }
                }

                if let Some(ret_exprs) = return_val {
                    let expected_types = match &self.current_return_type {
                        Some(SemanticType::Tuple(types)) => Some(types.clone()),
                        _ => None,
                    };

                    if let Some(types) = expected_types {
                        if ret_exprs.len() != types.len() {
                            self.errors.push(SemanticError::ArgumentCountMismatch {
                                expected: types.len(),
                                found: ret_exprs.len(),
                            });
                        } else {
                            for (i, expr) in ret_exprs.iter().enumerate() {
                                let actual = self.check_expr(expr);
                                let expected = SemanticType::Single(types[i].clone());
                                if !self.types_compatible(&actual, &expected) {
                                    self.errors.push(SemanticError::TypeMismatch {
                                        expected,
                                        found: actual,
                                    });
                                }
                            }
                        }
                    }

                    result = SemanticType::Void;
                }

                self.context = initial_context;
                result
            },
            _ => SemanticType::Error,
        }

        // Assign(Vec<AssignTarget>, Vec<Expr>),
    }

    pub fn check_assign_target(&mut self, target: &AssignTarget) -> SemanticType {
        // Discard,
        // Var(String),
        // Decl(String, Type),
        // ArrayIndex(String, Vec<Expr>),
        SemanticType::Error
    }

    pub fn check_iden_stmt(&mut self, stmt: &IdentStmtRest) -> SemanticType {
        // ProcCall(Vec<Expr>),
        // Assign(Expr),
        // ArrayAssign(Vec<Expr>, Expr),
        // UnifiedDecl(Type, Vec<Option<Expr>>, DeclSuffix),
        // MultiAssign(Vec<AssignTarget>, Vec<Expr>),
        // MultiArrayAssign(Vec<Expr>, Vec<AssignTarget>, Vec<Expr>),
        // CallIndexAssign(Vec<Expr>, Vec<Expr>, Expr),
        SemanticType::Error
    }
}
