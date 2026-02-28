use super::*;

fn e(kind: ExprKind) -> Expr { Spanned::dummy(kind) }
fn s(kind: StmtKind) -> Stmt { Spanned::dummy(kind) }

/// Helper: normalize whitespace in an S-expression string for comparison.
fn normalize(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[test]
fn test_simple_program_no_uses() {
    let p = Program {
        uses: vec![],
        items: vec![TopLevelItem::Func(FuncDef {
            name: "foo".into(),
            params: vec![],
            returns: vec![],
            body: Block {
                stmts: vec![s(StmtKind::Assign(
                    vec![AssignTarget::Decl("x".into(), Type::Int)],
                    vec![e(ExprKind::IntLit(2))],
                ))],
                return_val: None,
            },
        })],
    };
    let out = sexp_program(&p);
    let expected = "(() ((foo () () ((= (x int) 2)))))";
    assert_eq!(normalize(&out), normalize(expected));
}

#[test]
fn test_program_with_use() {
    let p = Program {
        uses: vec!["io".into()],
        items: vec![TopLevelItem::Func(FuncDef {
            name: "main".into(),
            params: vec![Param {
                name: "args".into(),
                ty: Type::Array(Box::new(Type::Array(Box::new(Type::Int), None)), None),
            }],
            returns: vec![],
            body: Block {
                stmts: vec![],
                return_val: None,
            },
        })],
    };
    let out = sexp_program(&p);
    let expected = "(((use io)) ((main ((args ([] ([] int)))) () ())))";
    assert_eq!(normalize(&out), normalize(expected));
}

#[test]
fn test_binop_expr() {
    let ex = e(ExprKind::BinOp(
        BinOp::Add,
        Box::new(e(ExprKind::IntLit(1))),
        Box::new(e(ExprKind::BinOp(
            BinOp::Mul,
            Box::new(e(ExprKind::IntLit(2))),
            Box::new(e(ExprKind::IntLit(3))),
        ))),
    ));
    assert_eq!(render_flat(&sexp_expr(&ex)), "(+ 1 (* 2 3))");
}

#[test]
fn test_unary_neg() {
    let ex = e(ExprKind::UnaryOp(UnaryOp::Neg, Box::new(e(ExprKind::IntLit(4)))));
    assert_eq!(render_flat(&sexp_expr(&ex)), "(- 4)");
}

#[test]
fn test_func_call() {
    let ex = e(ExprKind::FuncCall(
        "gcd".into(),
        vec![e(ExprKind::Var("q1".into())), e(ExprKind::Var("q2".into()))],
    ));
    assert_eq!(render_flat(&sexp_expr(&ex)), "(gcd q1 q2)");
}

#[test]
fn test_index_expr() {
    let ex = e(ExprKind::Index(
        Box::new(e(ExprKind::Var("a".into()))),
        Box::new(e(ExprKind::Var("j".into()))),
    ));
    assert_eq!(render_flat(&sexp_expr(&ex)), "([] a j)");
}

#[test]
fn test_length_expr() {
    let ex = e(ExprKind::Length(Box::new(e(ExprKind::Var("a".into())))));
    assert_eq!(render_flat(&sexp_expr(&ex)), "(length a)");
}

#[test]
fn test_array_constructor() {
    let ex = e(ExprKind::ArrayConstructor(vec![
        e(ExprKind::IntLit(1)),
        e(ExprKind::IntLit(2)),
        e(ExprKind::IntLit(3)),
    ]));
    assert_eq!(render_flat(&sexp_expr(&ex)), "(1 2 3)");
}

#[test]
fn test_type_int() {
    assert_eq!(render_flat(&sexp_type(&Type::Int)), "int");
}

#[test]
fn test_type_array() {
    let t = Type::Array(Box::new(Type::Array(Box::new(Type::Int), None)), None);
    assert_eq!(render_flat(&sexp_type(&t)), "([] ([] int))");
}

#[test]
fn test_decl_only_stmt() {
    let st = s(StmtKind::Assign(
        vec![AssignTarget::Decl("z".into(), Type::Int)],
        vec![],
    ));
    assert_eq!(render_flat(&sexp_stmt(&st)), "(z int)");
}

#[test]
fn test_assign_stmt() {
    let st = s(StmtKind::Assign(
        vec![AssignTarget::Var("x".into())],
        vec![e(ExprKind::BinOp(
            BinOp::Add,
            Box::new(e(ExprKind::Var("x".into()))),
            Box::new(e(ExprKind::IntLit(1))),
        ))],
    ));
    assert_eq!(render_flat(&sexp_stmt(&st)), "(= x (+ x 1))");
}

#[test]
fn test_decl_init_stmt() {
    let st = s(StmtKind::Assign(
        vec![AssignTarget::Decl("x".into(), Type::Int)],
        vec![e(ExprKind::IntLit(2))],
    ));
    assert_eq!(render_flat(&sexp_stmt(&st)), "(= (x int) 2)");
}

#[test]
fn test_multi_decl_assign() {
    let st = s(StmtKind::Assign(
        vec![
            AssignTarget::Decl("b".into(), Type::Bool),
            AssignTarget::Decl("i".into(), Type::Int),
        ],
        vec![e(ExprKind::FuncCall("f".into(), vec![e(ExprKind::Var("x".into()))]))],
    ));
    assert_eq!(render_flat(&sexp_stmt(&st)), "(= ((b bool) (i int)) (f x))");
}

#[test]
fn test_discard_multi_assign() {
    let st = s(StmtKind::Assign(
        vec![
            AssignTarget::Discard,
            AssignTarget::Decl("i".into(), Type::Int),
        ],
        vec![e(ExprKind::FuncCall("foo".into(), vec![]))],
    ));
    assert_eq!(render_flat(&sexp_stmt(&st)), "(= (_ (i int)) (foo))");
}

#[test]
fn test_proc_call() {
    let st = s(StmtKind::Assign(
        vec![],
        vec![e(ExprKind::FuncCall(
            "print".into(),
            vec![e(ExprKind::StringLit("Hello".into()))],
        ))],
    ));
    assert_eq!(render_flat(&sexp_stmt(&st)), "(print \"Hello\")");
}

#[test]
fn test_array_index_assign() {
    let st = s(StmtKind::Assign(
        vec![AssignTarget::ArrayIndex(
            "a".into(),
            vec![e(ExprKind::Var("n".into()))],
        )],
        vec![e(ExprKind::Var("n".into()))],
    ));
    assert_eq!(render_flat(&sexp_stmt(&st)), "(= ([] a n) n)");
}

#[test]
fn test_if_else() {
    let st = s(StmtKind::If(
        e(ExprKind::BinOp(
            BinOp::Lt,
            Box::new(e(ExprKind::Var("a".into()))),
            Box::new(e(ExprKind::Var("b".into()))),
        )),
        Box::new(s(StmtKind::Assign(
            vec![AssignTarget::Var("a".into())],
            vec![e(ExprKind::IntLit(1))],
        ))),
        Some(Box::new(s(StmtKind::Assign(
            vec![AssignTarget::Var("a".into())],
            vec![e(ExprKind::IntLit(2))],
        )))),
    ));
    assert_eq!(
        render_flat(&sexp_stmt(&st)),
        "(if (< a b) (= a 1) (= a 2))"
    );
}

#[test]
fn test_while_block() {
    let st = s(StmtKind::While(
        e(ExprKind::BinOp(
            BinOp::Gt,
            Box::new(e(ExprKind::Var("n".into()))),
            Box::new(e(ExprKind::IntLit(0))),
        )),
        Box::new(s(StmtKind::Block(Block {
            stmts: vec![s(StmtKind::Assign(
                vec![AssignTarget::Var("n".into())],
                vec![e(ExprKind::BinOp(
                    BinOp::Sub,
                    Box::new(e(ExprKind::Var("n".into()))),
                    Box::new(e(ExprKind::IntLit(1))),
                ))],
            ))],
            return_val: None,
        }))),
    ));
    assert_eq!(
        render_flat(&sexp_stmt(&st)),
        "(while (> n 0) ((= n (- n 1))))"
    );
}

#[test]
fn test_return_in_block() {
    let block = Block {
        stmts: vec![],
        return_val: Some(vec![
            e(ExprKind::Var("pred".into())),
            e(ExprKind::Var("expr".into())),
        ]),
    };
    assert_eq!(render_flat(&sexp_block(&block)), "((return pred expr))");
}

#[test]
fn test_interface() {
    let iface = Interface {
        decls: vec![
            InterfaceDecl {
                name: "print".into(),
                params: vec![Param {
                    name: "str".into(),
                    ty: Type::Array(Box::new(Type::Int), None),
                }],
                returns: vec![],
            },
            InterfaceDecl {
                name: "readln".into(),
                params: vec![],
                returns: vec![Type::Array(Box::new(Type::Int), None)],
            },
        ],
    };
    let out = sexp_interface(&iface);
    let expected = "((print ((str ([] int))) ()) (readln () (([] int))))";
    assert_eq!(normalize(&out), normalize(&format!("({})", expected)));
}

#[test]
fn test_string_escaping() {
    assert_eq!(format_string_lit("Hello, World!\n"), "\"Hello, World!\\n\"");
}

#[test]
fn test_char_lit() {
    assert_eq!(format_char_lit(b'x' as i64), "'x'");
    assert_eq!(format_char_lit(b'\n' as i64), "'\\n'");
}

#[test]
fn test_gcd_program() {
    let p = Program {
        uses: vec![],
        items: vec![TopLevelItem::Func(FuncDef {
            name: "gcd".into(),
            params: vec![
                Param { name: "a".into(), ty: Type::Int },
                Param { name: "b".into(), ty: Type::Int },
            ],
            returns: vec![Type::Int],
            body: Block {
                stmts: vec![s(StmtKind::While(
                    e(ExprKind::BinOp(
                        BinOp::Ne,
                        Box::new(e(ExprKind::Var("a".into()))),
                        Box::new(e(ExprKind::IntLit(0))),
                    )),
                    Box::new(s(StmtKind::Block(Block {
                        stmts: vec![s(StmtKind::If(
                            e(ExprKind::BinOp(
                                BinOp::Lt,
                                Box::new(e(ExprKind::Var("a".into()))),
                                Box::new(e(ExprKind::Var("b".into()))),
                            )),
                            Box::new(s(StmtKind::Assign(
                                vec![AssignTarget::Var("b".into())],
                                vec![e(ExprKind::BinOp(
                                    BinOp::Sub,
                                    Box::new(e(ExprKind::Var("b".into()))),
                                    Box::new(e(ExprKind::Var("a".into()))),
                                ))],
                            ))),
                            Some(Box::new(s(StmtKind::Assign(
                                vec![AssignTarget::Var("a".into())],
                                vec![e(ExprKind::BinOp(
                                    BinOp::Sub,
                                    Box::new(e(ExprKind::Var("a".into()))),
                                    Box::new(e(ExprKind::Var("b".into()))),
                                ))],
                            )))),
                        ))],
                        return_val: None,
                    }))),
                ))],
                return_val: Some(vec![e(ExprKind::Var("b".into()))]),
            },
        })],
    };
    let out = sexp_program(&p);
    let expected = "(() ((gcd ((a int) (b int)) (int) ((while (!= a 0) ((if (< a b) (= b (- b a)) (= a (- a b))))) (return b)))))";
    assert_eq!(normalize(&out), normalize(expected));
}
