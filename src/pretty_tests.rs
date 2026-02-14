use super::*;

// ── expression tests ──────────────────────────────────────────────

#[test]
fn test_simple_expr() {
    let e = Expr::BinOp(
        BinOp::Add,
        Box::new(Expr::IntLit(1)),
        Box::new(Expr::IntLit(2)),
    );
    assert_eq!(pretty_expr(&e), "1 + 2");
}

#[test]
fn test_precedence_parens() {
    // 1 + 2 * 3  should NOT get parens
    let e = Expr::BinOp(
        BinOp::Add,
        Box::new(Expr::IntLit(1)),
        Box::new(Expr::BinOp(
            BinOp::Mul,
            Box::new(Expr::IntLit(2)),
            Box::new(Expr::IntLit(3)),
        )),
    );
    assert_eq!(pretty_expr(&e), "1 + 2 * 3");

    // (1 + 2) * 3
    let e2 = Expr::BinOp(
        BinOp::Mul,
        Box::new(Expr::BinOp(
            BinOp::Add,
            Box::new(Expr::IntLit(1)),
            Box::new(Expr::IntLit(2)),
        )),
        Box::new(Expr::IntLit(3)),
    );
    assert_eq!(pretty_expr(&e2), "(1 + 2) * 3");
}

#[test]
fn test_left_assoc() {
    // (a - b) - c  should print as  a - b - c
    let e = Expr::BinOp(
        BinOp::Sub,
        Box::new(Expr::BinOp(
            BinOp::Sub,
            Box::new(Expr::Var("a".into())),
            Box::new(Expr::Var("b".into())),
        )),
        Box::new(Expr::Var("c".into())),
    );
    assert_eq!(pretty_expr(&e), "a - b - c");

    // a - (b - c)  must print with parens
    let e2 = Expr::BinOp(
        BinOp::Sub,
        Box::new(Expr::Var("a".into())),
        Box::new(Expr::BinOp(
            BinOp::Sub,
            Box::new(Expr::Var("b".into())),
            Box::new(Expr::Var("c".into())),
        )),
    );
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
            return_val: Some(vec![Expr::BoolLit(true)]),
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
    let s = Stmt::If(
        Expr::BoolLit(true),
        Box::new(Stmt::Block(Block {
            stmts: vec![],
            return_val: None,
        })),
        None,
    );
    let out = pretty_stmt(&s);
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
    let s = Stmt::If(
        Expr::Var("x".into()),
        Box::new(Stmt::Block(Block {
            stmts: vec![],
            return_val: None,
        })),
        Some(Box::new(Stmt::Block(Block {
            stmts: vec![],
            return_val: None,
        }))),
    );
    let out = pretty_stmt(&s);
    assert!(out.contains("if x {"), "got: {}", out);
    assert!(out.contains("} else {"), "got: {}", out);
}

#[test]
fn test_while_no_parens_around_condition() {
    let s = Stmt::While(
        Expr::BoolLit(false),
        Box::new(Stmt::Block(Block {
            stmts: vec![],
            return_val: None,
        })),
    );
    let out = pretty_stmt(&s);
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
    let s = Stmt::Assign(
        vec![AssignTarget::Var("x".into())],
        vec![Expr::IntLit(42)],
    );
    let out = pretty_stmt(&s);
    assert!(
        !out.ends_with('\n'),
        "pretty_stmt should NOT end with newline, got: {:?}",
        out
    );
    assert_eq!(out, "x = 42;");
}

#[test]
fn test_pretty_stmt_if_no_trailing_newline() {
    let s = Stmt::If(
        Expr::BoolLit(true),
        Box::new(Stmt::Block(Block {
            stmts: vec![],
            return_val: None,
        })),
        None,
    );
    let out = pretty_stmt(&s);
    assert!(
        !out.ends_with('\n'),
        "pretty_stmt should NOT end with newline, got: {:?}",
        out
    );
}

// ── procedure call statement ──────────────────────────────────────

#[test]
fn test_proc_call_single_value() {
    let s = Stmt::Assign(
        vec![],
        vec![Expr::FuncCall("print".into(), vec![Expr::IntLit(1)])],
    );
    let out = pretty_stmt(&s);
    assert_eq!(out, "print(1);");
}

#[test]
#[should_panic(expected = "expected exactly 1 value in procedure-call statement")]
fn test_proc_call_multiple_values_panics_in_debug() {
    // The AST invariant is exactly one FuncCall when targets is empty.
    // In debug builds the debug_assert! fires; this test documents that.
    let values = vec![
        Expr::FuncCall("foo".into(), vec![]),
        Expr::FuncCall("bar".into(), vec![]),
    ];
    let mut buf = String::new();
    write_assign(&mut buf, &[], &values, 0);
}
