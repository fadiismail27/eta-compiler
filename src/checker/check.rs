use crate::checker::context::Context;
use crate::parser::ast::{
    AssignTarget, BinOp, Expr, ExprKind, Block, FuncDef, Program, Span, Spanned,
    StmtKind, TopLevelItem, Type, UnaryOp,
};

#[derive(Clone, Debug)]
pub struct SemanticError {
    pub span: Span,
    pub kind: SemanticErrorKind,
}

#[derive(Clone, Debug)]
pub enum SemanticErrorKind {
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

impl SemanticErrorKind {
    pub fn at(self, span: Span) -> SemanticError {
        SemanticError { span, kind: self }
    }
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
}

type TypeResult = Result<SemanticType, SemanticError>;

pub struct TypeChecker {
    context: Context,
    current_return_type: Option<SemanticType>,
}

impl TypeChecker {
    pub fn new() -> Self {
        TypeChecker {
            context: Context::new(),
            current_return_type: None,
        }
    }

    pub fn check_program(&mut self, program: &Program) -> Result<(), SemanticError> {
        // Pass 1: collect all signatures into context
        for item in &program.items {
            match item {
                TopLevelItem::Func(func) => {
                    if self.context.search(&func.name).is_some() {
                        return Err(SemanticErrorKind::DuplicateDeclaration {
                            name: func.name.clone(),
                        }.at(Span::dummy()));
                    }

                    let ret = SemanticType::Tuple(func.returns.clone());

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
                        return Err(SemanticErrorKind::DuplicateDeclaration {
                            name: global.name.clone(),
                        }.at(Span::dummy()));
                    }

                    self.context
                        .push(&global.name, &SemanticType::Single(global.ty.clone()));
                }
            }
        }

        // Pass 2: check each body
        for item in &program.items {
            match item {
                TopLevelItem::Func(func) => self.check_func_def(func)?,
                TopLevelItem::Global(global) => {
                    if let Some(init) = &global.init {
                        let t = self.check_expr(init)?;
                        let expected = SemanticType::Single(global.ty.clone());
                        if !self.types_compatible(&t, &expected) {
                            return Err(SemanticErrorKind::TypeMismatch { expected, found: t }
                                .at(init.span));
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn types_compatible(&self, actual: &SemanticType, expected: &SemanticType) -> bool {
        match (actual, expected) {
            (a, b) if a == b => true,
            (SemanticType::EmptyArray, SemanticType::Single(Type::Array(_, _))) => true,
            _ => false,
        }
    }

    fn check_func_def(&mut self, func: &FuncDef) -> Result<(), SemanticError> {
        let initial_context = self.context.clone();

        for param in &func.params {
            if self.context.search(&param.name).is_some() {
                return Err(SemanticErrorKind::DuplicateDeclaration {
                    name: param.name.clone(),
                }.at(Span::dummy()));
            }
            self.context
                .push(&param.name, &SemanticType::Single(param.ty.clone()));
        }

        let ret = SemanticType::Tuple(func.returns.clone());

        self.current_return_type = Some(ret.clone());
        let body_stmt = Spanned::dummy(StmtKind::Block(func.body.clone()));
        let stmt_type = self.check_stmt(&body_stmt)?;

        if !func.returns.is_empty() && stmt_type != SemanticType::Void {
            return Err(SemanticErrorKind::MissingReturn { expected: ret }
                .at(Span::dummy()));
        }

        self.context = initial_context;
        self.current_return_type = None;
        Ok(())
    }

    fn check_expr(&mut self, exp: &Expr) -> TypeResult {
        let span = exp.span;
        match &exp.node {
            ExprKind::Var(var) => match self.context.search(var) {
                Some(typ) => Ok(typ.clone()),
                None => Err(SemanticErrorKind::UndeclaredIdentifier { name: var.clone() }
                    .at(span)),
            },
            ExprKind::IntLit(_) | ExprKind::CharLit(_) => {
                Ok(SemanticType::Single(Type::Int))
            }
            ExprKind::StringLit(_) => {
                Ok(SemanticType::Single(Type::Array(Box::new(Type::Int), None)))
            }
            ExprKind::BoolLit(_) => Ok(SemanticType::Single(Type::Bool)),

            ExprKind::BinOp(op, expr1, expr2) => {
                let typ1 = self.check_expr(expr1)?;
                let typ2 = self.check_expr(expr2)?;

                match op {
                    BinOp::Add => match (&typ1, &typ2) {
                        (
                            SemanticType::Single(Type::Int),
                            SemanticType::Single(Type::Int),
                        ) => Ok(SemanticType::Single(Type::Int)),

                        (
                            SemanticType::Single(Type::Array(base1, _)),
                            SemanticType::Single(Type::Array(base2, _)),
                        ) => {
                            if base1 != base2 {
                                return Err(SemanticErrorKind::TypeMismatch {
                                    expected: typ1.clone(),
                                    found: typ2.clone(),
                                }.at(span));
                            }
                            Ok(SemanticType::Single(Type::Array(base1.clone(), None)))
                        }

                        (
                            SemanticType::Single(Type::Array(_, _)),
                            SemanticType::EmptyArray,
                        ) => Ok(typ1.clone()),
                        (
                            SemanticType::EmptyArray,
                            SemanticType::Single(Type::Array(_, _)),
                        ) => Ok(typ2.clone()),

                        (SemanticType::EmptyArray, SemanticType::EmptyArray) => {
                            Ok(SemanticType::EmptyArray)
                        }

                        _ => Err(SemanticErrorKind::InvalidBinaryOp {
                            op: op.clone(),
                            left: typ1.clone(),
                            right: typ2.clone(),
                        }.at(span)),
                    },
                    BinOp::Sub | BinOp::Mul | BinOp::HighMul | BinOp::Div | BinOp::Mod => {
                        if typ1 == SemanticType::Single(Type::Int)
                            && typ2 == SemanticType::Single(Type::Int)
                        {
                            Ok(SemanticType::Single(Type::Int))
                        } else {
                            Err(SemanticErrorKind::InvalidBinaryOp {
                                op: op.clone(),
                                left: typ1,
                                right: typ2,
                            }.at(span))
                        }
                    }
                    BinOp::Lt | BinOp::Le | BinOp::Gt | BinOp::Ge => {
                        if typ1 == SemanticType::Single(Type::Int)
                            && typ2 == SemanticType::Single(Type::Int)
                        {
                            Ok(SemanticType::Single(Type::Bool))
                        } else {
                            Err(SemanticErrorKind::InvalidBinaryOp {
                                op: op.clone(),
                                left: typ1,
                                right: typ2,
                            }.at(span))
                        }
                    }
                    BinOp::Eq | BinOp::Ne => match (&typ1, &typ2) {
                        (t1, t2) if t1 == t2 => Ok(SemanticType::Single(Type::Bool)),

                        (
                            SemanticType::Single(Type::Array(_, _)),
                            SemanticType::EmptyArray,
                        ) => Ok(SemanticType::Single(Type::Bool)),
                        (
                            SemanticType::EmptyArray,
                            SemanticType::Single(Type::Array(_, _)),
                        ) => Ok(SemanticType::Single(Type::Bool)),

                        _ => Err(SemanticErrorKind::TypeMismatch {
                            expected: typ1,
                            found: typ2,
                        }.at(span)),
                    },
                    BinOp::Or | BinOp::And => {
                        if typ1 == SemanticType::Single(Type::Bool)
                            && typ2 == SemanticType::Single(Type::Bool)
                        {
                            Ok(SemanticType::Single(Type::Bool))
                        } else {
                            Err(SemanticErrorKind::InvalidBinaryOp {
                                op: op.clone(),
                                left: typ1,
                                right: typ2,
                            }.at(span))
                        }
                    }
                }
            }
            ExprKind::UnaryOp(op, inner) => {
                let typ = self.check_expr(inner)?;
                match op {
                    UnaryOp::Neg => {
                        if typ != SemanticType::Single(Type::Int) {
                            return Err(SemanticErrorKind::InvalidUnaryOp {
                                op: op.clone(),
                                exp: typ,
                            }.at(span));
                        }
                        Ok(SemanticType::Single(Type::Int))
                    }
                    UnaryOp::Not => {
                        if typ != SemanticType::Single(Type::Bool) {
                            return Err(SemanticErrorKind::InvalidUnaryOp {
                                op: op.clone(),
                                exp: typ,
                            }.at(span));
                        }
                        Ok(SemanticType::Single(Type::Bool))
                    }
                }
            }
            ExprKind::FuncCall(s, e_vec) => {
                let func = self.context.search(s).cloned();
                match func {
                    Some(SemanticType::Func { args, ret }) => {
                        match ret.as_ref() {
                            SemanticType::Tuple(types) if types.len() == 1 => {
                                if args.len() != e_vec.len() {
                                    return Err(SemanticErrorKind::ArgumentCountMismatch {
                                        expected: args.len(),
                                        found: e_vec.len(),
                                    }.at(span));
                                }
                                for (i, e) in e_vec.iter().enumerate() {
                                    let actual = self.check_expr(e)?;
                                    let expected = SemanticType::Single(args[i].clone());
                                    if !self.types_compatible(&actual, &expected) {
                                        return Err(SemanticErrorKind::ArgumentTypeMismatch {
                                            param_index: i,
                                            expected,
                                            found: actual,
                                        }.at(e.span));
                                    }
                                }
                                Ok(SemanticType::Single(types[0].clone()))
                            }
                            _ => Err(SemanticErrorKind::InvalidReturnType {
                                found: *ret,
                            }.at(span)),
                        }
                    }
                    Some(x) => Err(SemanticErrorKind::NotCallable { found: x }.at(span)),
                    None => Err(SemanticErrorKind::UndeclaredIdentifier {
                        name: s.clone(),
                    }.at(span)),
                }
            }

            ExprKind::Length(inner) => {
                let typ = self.check_expr(inner)?;
                match typ {
                    SemanticType::Single(Type::Array(_, _)) | SemanticType::EmptyArray => {
                        Ok(SemanticType::Single(Type::Int))
                    }
                    _ => Err(SemanticErrorKind::ExpectedArray { found: typ }.at(span)),
                }
            }
            ExprKind::Index(exp1, exp2) => {
                let typ1 = self.check_expr(exp1)?;
                let typ2 = self.check_expr(exp2)?;

                match typ1 {
                    SemanticType::Single(Type::Array(base, _)) => match typ2 {
                        SemanticType::Single(Type::Int) => Ok(SemanticType::Single(*base)),
                        _ => Err(SemanticErrorKind::TypeMismatch {
                            expected: SemanticType::Single(Type::Int),
                            found: typ2,
                        }.at(exp2.span)),
                    },
                    _ => Err(SemanticErrorKind::ExpectedArray { found: typ1 }.at(exp1.span)),
                }
            }

            ExprKind::ArrayConstructor(exp_lst) => {
                if exp_lst.is_empty() {
                    return Ok(SemanticType::EmptyArray);
                }

                let base_typ = self.check_expr(&exp_lst[0])?;

                for i in 1..exp_lst.len() {
                    let typ = self.check_expr(&exp_lst[i])?;

                    if typ != base_typ {
                        return Err(SemanticErrorKind::TypeMismatch {
                            expected: base_typ,
                            found: typ,
                        }.at(exp_lst[i].span));
                    }
                }

                match base_typ {
                    SemanticType::Single(t) => {
                        Ok(SemanticType::Single(Type::Array(Box::new(t), None)))
                    }
                    SemanticType::EmptyArray => Ok(SemanticType::EmptyArray),
                    _ => Err(SemanticErrorKind::InvalidBaseType { found: base_typ }
                        .at(span)),
                }
            }
        }
    }

    fn check_stmt(&mut self, stmt: &Spanned<StmtKind>) -> TypeResult {
        let span = stmt.span;
        match &stmt.node {
            StmtKind::If(expr, t_stmt, f_stmt) => {
                let initial_context = self.context.clone();
                let guard = self.check_expr(expr)?;

                if guard != SemanticType::Single(Type::Bool) {
                    return Err(SemanticErrorKind::ConditionNotBoolean { found: guard }
                        .at(expr.span));
                }

                let t_return = self.check_stmt(t_stmt)?;
                self.context = initial_context.clone();

                match f_stmt {
                    Some(s) => {
                        let f_return = self.check_stmt(s)?;
                        self.context = initial_context;

                        match (t_return, f_return) {
                            (SemanticType::Void, SemanticType::Void) => {
                                Ok(SemanticType::Void)
                            }
                            _ => Ok(SemanticType::Unit),
                        }
                    }
                    None => Ok(SemanticType::Unit),
                }
            }
            StmtKind::While(expr, s) => {
                let initial_context = self.context.clone();
                let guard = self.check_expr(expr)?;

                if guard != SemanticType::Single(Type::Bool) {
                    return Err(SemanticErrorKind::ConditionNotBoolean { found: guard }
                        .at(expr.span));
                }

                self.check_stmt(s)?;
                self.context = initial_context;

                Ok(SemanticType::Unit)
            }
            StmtKind::Block(Block { stmts, return_val }) => {
                let initial_context = self.context.clone();
                let mut result = SemanticType::Unit;

                for (i, s) in stmts.iter().enumerate() {
                    let s_type = self.check_stmt(s)?;

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
                            return Err(SemanticErrorKind::ArgumentCountMismatch {
                                expected: types.len(),
                                found: ret_exprs.len(),
                            }.at(span));
                        }
                        for (i, expr) in ret_exprs.iter().enumerate() {
                            let actual = self.check_expr(expr)?;
                            let expected = SemanticType::Single(types[i].clone());
                            if !self.types_compatible(&actual, &expected) {
                                return Err(SemanticErrorKind::TypeMismatch {
                                    expected,
                                    found: actual,
                                }.at(expr.span));
                            }
                        }
                    }
                    result = SemanticType::Void;
                }

                self.context = initial_context;
                Ok(result)
            }
            StmtKind::Assign(target_lst, expr_lst) => {
                // Case 1: Procedure call — targets=[], exprs=[FuncCall(...)]
                if target_lst.is_empty() && expr_lst.len() == 1 {
                    if let ExprKind::FuncCall(name, args) = &expr_lst[0].node {
                        let func = self.context.search(name).cloned();
                        match func {
                            Some(SemanticType::Func { args: param_types, ret }) => {
                                if args.len() != param_types.len() {
                                    return Err(SemanticErrorKind::ArgumentCountMismatch {
                                        expected: param_types.len(),
                                        found: args.len(),
                                    }.at(expr_lst[0].span));
                                }
                                for (i, arg) in args.iter().enumerate() {
                                    let actual = self.check_expr(arg)?;
                                    let expected =
                                        SemanticType::Single(param_types[i].clone());
                                    if !self.types_compatible(&actual, &expected) {
                                        return Err(
                                            SemanticErrorKind::ArgumentTypeMismatch {
                                                param_index: i,
                                                expected,
                                                found: actual,
                                            }.at(arg.span),
                                        );
                                    }
                                }
                                return Ok(SemanticType::Unit);
                            }
                            Some(x) => {
                                return Err(SemanticErrorKind::NotCallable { found: x }
                                    .at(expr_lst[0].span))
                            }
                            None => {
                                return Err(SemanticErrorKind::UndeclaredIdentifier {
                                    name: name.clone(),
                                }.at(expr_lst[0].span))
                            }
                        }
                    }
                }

                // Case 2: Declaration with no init — targets=[Decl(...)], exprs=[]
                if target_lst.len() == 1 && expr_lst.is_empty() {
                    if let AssignTarget::Decl(name, ty) = &target_lst[0] {
                        if self.context.search(name).is_some() {
                            return Err(SemanticErrorKind::DuplicateDeclaration {
                                name: name.clone(),
                            }.at(span));
                        }
                        self.context
                            .push(name, &SemanticType::Single(ty.clone()));
                        return Ok(SemanticType::Unit);
                    }
                }

                // Case 3: Multi-assign from function call
                if target_lst.len() > 1 && expr_lst.len() == 1 {
                    if let ExprKind::FuncCall(name, args) = &expr_lst[0].node {
                        let func = self.context.search(name).cloned();
                        match func {
                            Some(SemanticType::Func { args: param_types, ret }) => {
                                if args.len() != param_types.len() {
                                    return Err(SemanticErrorKind::ArgumentCountMismatch {
                                        expected: param_types.len(),
                                        found: args.len(),
                                    }.at(expr_lst[0].span));
                                }
                                for (i, arg) in args.iter().enumerate() {
                                    let actual = self.check_expr(arg)?;
                                    let expected =
                                        SemanticType::Single(param_types[i].clone());
                                    if !self.types_compatible(&actual, &expected) {
                                        return Err(
                                            SemanticErrorKind::ArgumentTypeMismatch {
                                                param_index: i,
                                                expected,
                                                found: actual,
                                            }.at(arg.span),
                                        );
                                    }
                                }
                                match ret.as_ref() {
                                    SemanticType::Tuple(ret_types) => {
                                        if target_lst.len() != ret_types.len() {
                                            return Err(
                                                SemanticErrorKind::ArgumentCountMismatch {
                                                    expected: ret_types.len(),
                                                    found: target_lst.len(),
                                                }.at(span),
                                            );
                                        }
                                        for (i, target) in target_lst.iter().enumerate() {
                                            let ret_type =
                                                SemanticType::Single(ret_types[i].clone());
                                            match target {
                                                AssignTarget::Discard => {}
                                                _ => {
                                                    let target_type =
                                                        self.check_assign_target(target, span)?;
                                                    if !self
                                                        .types_compatible(&ret_type, &target_type)
                                                    {
                                                        return Err(
                                                            SemanticErrorKind::TypeMismatch {
                                                                expected: target_type,
                                                                found: ret_type,
                                                            }.at(span),
                                                        );
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    _ => {
                                        return Err(SemanticErrorKind::InvalidReturnType {
                                            found: (*ret).clone(),
                                        }.at(expr_lst[0].span))
                                    }
                                }
                                return Ok(SemanticType::Unit);
                            }
                            Some(x) => {
                                return Err(SemanticErrorKind::NotCallable { found: x }
                                    .at(expr_lst[0].span))
                            }
                            None => {
                                return Err(SemanticErrorKind::UndeclaredIdentifier {
                                    name: name.clone(),
                                }.at(expr_lst[0].span))
                            }
                        }
                    }
                }

                // Case 4: General n-to-n assignment
                if target_lst.len() != expr_lst.len() {
                    return Err(SemanticErrorKind::ArgumentCountMismatch {
                        expected: target_lst.len(),
                        found: expr_lst.len(),
                    }.at(span));
                }

                for (target, expr) in target_lst.iter().zip(expr_lst.iter()) {
                    let expr_typ = self.check_expr(expr)?;
                    match target {
                        AssignTarget::Discard => {}
                        _ => {
                            let target_typ = self.check_assign_target(target, span)?;
                            if !self.types_compatible(&expr_typ, &target_typ) {
                                return Err(SemanticErrorKind::TypeMismatch {
                                    expected: target_typ,
                                    found: expr_typ,
                                }.at(expr.span));
                            }
                        }
                    }
                }

                Ok(SemanticType::Unit)
            }
        }
    }

    fn check_assign_target(
        &mut self,
        target: &AssignTarget,
        fallback_span: Span,
    ) -> TypeResult {
        match target {
            AssignTarget::Discard => Ok(SemanticType::Unit),
            AssignTarget::Var(var) => match self.context.search(var) {
                Some(typ) => Ok(typ.clone()),
                None => Err(SemanticErrorKind::UndeclaredIdentifier { name: var.clone() }
                    .at(fallback_span)),
            },
            AssignTarget::Decl(var, typ) => match self.context.search(var) {
                Some(_) => Err(SemanticErrorKind::DuplicateDeclaration { name: var.clone() }
                    .at(fallback_span)),
                None => {
                    self.context
                        .push(var, &SemanticType::Single(typ.clone()));
                    Ok(SemanticType::Single(typ.clone()))
                }
            },
            AssignTarget::ArrayIndex(var, e_lst) => {
                let var_typ = self.context.search(var).cloned();
                match var_typ {
                    None => Err(SemanticErrorKind::UndeclaredIdentifier {
                        name: var.clone(),
                    }.at(fallback_span)),
                    Some(mut current) => {
                        for expr in e_lst {
                            let idx_type = self.check_expr(expr)?;
                            if idx_type != SemanticType::Single(Type::Int) {
                                return Err(SemanticErrorKind::TypeMismatch {
                                    expected: SemanticType::Single(Type::Int),
                                    found: idx_type,
                                }.at(expr.span));
                            }
                            match current {
                                SemanticType::Single(Type::Array(base, _)) => {
                                    current = SemanticType::Single(*base);
                                }
                                _ => {
                                    return Err(SemanticErrorKind::ExpectedArray {
                                        found: current,
                                    }.at(expr.span));
                                }
                            }
                        }
                        Ok(current)
                    }
                }
            }
        }
    }
}
