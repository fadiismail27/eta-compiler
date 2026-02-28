use super::*;

fn e(kind: ExprKind) -> Expr { Spanned::dummy(kind) }
fn s(kind: StmtKind) -> Stmt { Spanned::dummy(kind) }

// ── expression tests ──────────────────────────────────────────────

#[test]
fn test_simple_expr() {
    let ex = e(ExprKind::BinOp(
        BinOp::Add,
        Box::new(e(ExprKind::IntLit(1))),
        Box::new(e(ExprKind::IntLit(2))),
    ));
    assert_eq!(pretty_expr(&ex), "1 + 2");
}

#[test]
fn test_precedence_parens() {
    let ex = e(ExprKind::BinOp(
        BinOp::Add,
        Box::new(e(ExprKind::IntLit(1))),
        Box::new(e(ExprKind::BinOp(
            BinOp::Mul,
            Box::new(e(ExprKind::IntLit(2))),
            Box::new(e(ExprKind::IntLit(3))),
        ))),
    ));
    assert_eq!(pretty_expr(&ex), "1 + 2 * 3");

    let e2 = e(ExprKind::BinOp(
        BinOp::Mul,
        Box::new(e(ExprKind::BinOp(
            BinOp::Add,
            Box::new(e(ExprKind::IntLit(1))),
            Box::new(e(ExprKind::IntLit(2))),
        ))),
        Box::new(e(ExprKind::IntLit(3))),
    ));
    assert_eq!(pretty_expr(&e2), "(1 + 2) * 3");
}

#[test]
fn test_left_assoc() {
    let ex = e(ExprKind::BinOp(
        BinOp::Sub,
        Box::new(e(ExprKind::BinOp(
            BinOp::Sub,
            Box::new(e(ExprKind::Var("a".into()))),
            Box::new(e(ExprKind::Var("b".into()))),
        ))),
        Box::new(e(ExprKind::Var("c".into()))),
    ));
    assert_eq!(pretty_expr(&ex), "a - b - c");

    let e2 = e(ExprKind::BinOp(
        BinOp::Sub,
        Box::new(e(ExprKind::Var("a".into()))),
        Box::new(e(ExprKind::BinOp(
            BinOp::Sub,
            Box::new(e(ExprKind::Var("b".into()))),
            Box::new(e(ExprKind::Var("c".into()))),
        ))),
    ));
    assert_eq!(pretty_expr(&e2), "a - (b - c)");
}

// ── type formatting tests ─────────────────────────────────────────

#[test]
fn test_type_format() {
    assert_eq!(format_type(&Type::Int), "int");
    assert_eq!(format_type(&Type::Bool), "bool");
    assert_eq!(
        format_type(&Type::Array(Box::new(Type::Array(Box::new(Type::Int), None)), None)),
        "int[][]"
    );
}

// ── program / function tests ──────────────────────────────────────

#[test]
fn test_func_def() {
    let f = FuncDef {
        name: "foo".into(),
        params: vec![Param {
            name: "x".into(),
            ty: Type::Int,
        }],
        returns: vec![Type::Bool],
        body: Block {
            stmts: vec![],
            return_val: Some(vec![e(ExprKind::BoolLit(true))]),
        },
    };
    let p = Program {
        uses: vec![],
        items: vec![TopLevelItem::Func(f)],
    };
    let out = pretty_program(&p);
    assert!(out.contains("foo(x: int): bool {"));
    assert!(out.contains("return true;"));
}

// ── if / while: no parentheses around conditions ──────────────────

#[test]
fn test_if_no_parens_around_condition() {
    let st = s(StmtKind::If(
        e(ExprKind::BoolLit(true)),
        Box::new(s(StmtKind::Block(Block {
            stmts: vec![],
            return_val: None,
        }))),
        None,
    ));
    let out = pretty_stmt(&st);
    assert!(
        out.contains("if true {"),
        "expected 'if true {{' without parens, got: {}",
        out
    );
    assert!(
        !out.contains("if (true)"),
        "condition should NOT be parenthesised, got: {}",
        out
    );
}

#[test]
fn test_if_else_no_parens() {
    let st = s(StmtKind::If(
        e(ExprKind::Var("x".into())),
        Box::new(s(StmtKind::Block(Block {
            stmts: vec![],
            return_val: None,
        }))),
        Some(Box::new(s(StmtKind::Block(Block {
            stmts: vec![],
            return_val: None,
        })))),
    ));
    let out = pretty_stmt(&st);
    assert!(out.contains("if x {"), "got: {}", out);
    assert!(out.contains("} else {"), "got: {}", out);
}

#[test]
fn test_while_no_parens_around_condition() {
    let st = s(StmtKind::While(
        e(ExprKind::BoolLit(false)),
        Box::new(s(StmtKind::Block(Block {
            stmts: vec![],
            return_val: None,
        }))),
    ));
    let out = pretty_stmt(&st);
    assert!(
        out.contains("while false {"),
        "expected 'while false {{' without parens, got: {}",
        out
    );
    assert!(
        !out.contains("while (false)"),
        "condition should NOT be parenthesised, got: {}",
        out
    );
}

// ── pretty_stmt: no trailing newline ──────────────────────────────

#[test]
fn test_pretty_stmt_no_trailing_newline() {
    let st = s(StmtKind::Assign(
        vec![AssignTarget::Var("x".into())],
        vec![e(ExprKind::IntLit(42))],
    ));
    let out = pretty_stmt(&st);
    assert!(
        !out.ends_with('\n'),
        "pretty_stmt should NOT end with newline, got: {:?}",
        out
    );
    assert_eq!(out, "x = 42;");
}

#[test]
fn test_pretty_stmt_if_no_trailing_newline() {
    let st = s(StmtKind::If(
        e(ExprKind::BoolLit(true)),
        Box::new(s(StmtKind::Block(Block {
            stmts: vec![],
            return_val: None,
        }))),
        None,
    ));
    let out = pretty_stmt(&st);
    assert!(
        !out.ends_with('\n'),
        "pretty_stmt should NOT end with newline, got: {:?}",
        out
    );
}

// ── procedure call statement ──────────────────────────────────────

#[test]
fn test_proc_call_single_value() {
    let st = s(StmtKind::Assign(
        vec![],
        vec![e(ExprKind::FuncCall("print".into(), vec![e(ExprKind::IntLit(1))]))],
    ));
    let out = pretty_stmt(&st);
    assert_eq!(out, "print(1);");
}

#[test]
#[should_panic(expected = "expected exactly 1 value in procedure-call statement")]
fn test_proc_call_multiple_values_panics_in_debug() {
    let values = vec![
        e(ExprKind::FuncCall("foo".into(), vec![])),
        e(ExprKind::FuncCall("bar".into(), vec![])),
    ];
    let mut buf = String::new();
    write_assign(&mut buf, &[], &values, 0);
}
