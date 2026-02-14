use super::*;

/// Helper: normalize whitespace in an S-expression string for comparison.
/// Collapses all runs of whitespace to a single space and trims.
fn normalize(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[test]
fn test_simple_program_no_uses() {
    // foo() { x:int = 2; }
    let p = Program {
        uses: vec![],
        items: vec![TopLevelItem::Func(FuncDef {
            name: "foo".into(),
            params: vec![],
            returns: vec![],
            body: Block {
                stmts: vec![Stmt::Assign(
                    vec![AssignTarget::Decl("x".into(), Type::Int)],
                    vec![Expr::IntLit(2)],
                )],
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
    // use io; main(args: int[][]) { }
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
    // 1 + 2 * 3  →  (+ 1 (* 2 3))
    let e = Expr::BinOp(
        BinOp::Add,
        Box::new(Expr::IntLit(1)),
        Box::new(Expr::BinOp(
            BinOp::Mul,
            Box::new(Expr::IntLit(2)),
            Box::new(Expr::IntLit(3)),
        )),
    );
    assert_eq!(render_flat(&sexp_expr(&e)), "(+ 1 (* 2 3))");
}

#[test]
fn test_unary_neg() {
    // -4  →  (- 4)
    let e = Expr::UnaryOp(UnaryOp::Neg, Box::new(Expr::IntLit(4)));
    assert_eq!(render_flat(&sexp_expr(&e)), "(- 4)");
}

#[test]
fn test_func_call() {
    // gcd(q1, q2)  →  (gcd q1 q2)
    let e = Expr::FuncCall(
        "gcd".into(),
        vec![Expr::Var("q1".into()), Expr::Var("q2".into())],
    );
    assert_eq!(render_flat(&sexp_expr(&e)), "(gcd q1 q2)");
}

#[test]
fn test_index_expr() {
    // a[j]  →  ([] a j)
    let e = Expr::Index(
        Box::new(Expr::Var("a".into())),
        Box::new(Expr::Var("j".into())),
    );
    assert_eq!(render_flat(&sexp_expr(&e)), "([] a j)");
}

#[test]
fn test_length_expr() {
    // length(a)  →  (length a)
    let e = Expr::Length(Box::new(Expr::Var("a".into())));
    assert_eq!(render_flat(&sexp_expr(&e)), "(length a)");
}

#[test]
fn test_array_constructor() {
    // {1, 2, 3}  →  (1 2 3)
    let e = Expr::ArrayConstructor(vec![
        Expr::IntLit(1),
        Expr::IntLit(2),
        Expr::IntLit(3),
    ]);
    assert_eq!(render_flat(&sexp_expr(&e)), "(1 2 3)");
}

#[test]
fn test_type_int() {
    assert_eq!(render_flat(&sexp_type(&Type::Int)), "int");
}

#[test]
fn test_type_array() {
    // int[][]  →  ([] ([] int))
    let t = Type::Array(Box::new(Type::Array(Box::new(Type::Int), None)), None);
    assert_eq!(render_flat(&sexp_type(&t)), "([] ([] int))");
}

#[test]
fn test_decl_only_stmt() {
    // z: int  →  (z int)
    let s = Stmt::Assign(
        vec![AssignTarget::Decl("z".into(), Type::Int)],
        vec![],
    );
    assert_eq!(render_flat(&sexp_stmt(&s)), "(z int)");
}

#[test]
fn test_assign_stmt() {
    // x = x + 1  →  (= x (+ x 1))
    let s = Stmt::Assign(
        vec![AssignTarget::Var("x".into())],
        vec![Expr::BinOp(
            BinOp::Add,
            Box::new(Expr::Var("x".into())),
            Box::new(Expr::IntLit(1)),
        )],
    );
    assert_eq!(render_flat(&sexp_stmt(&s)), "(= x (+ x 1))");
}

#[test]
fn test_decl_init_stmt() {
    // x: int = 2  →  (= (x int) 2)
    let s = Stmt::Assign(
        vec![AssignTarget::Decl("x".into(), Type::Int)],
        vec![Expr::IntLit(2)],
    );
    assert_eq!(render_flat(&sexp_stmt(&s)), "(= (x int) 2)");
}

#[test]
fn test_multi_decl_assign() {
    // b: bool, i:int = f(x)  →  (= ((b bool) (i int)) (f x))
    let s = Stmt::Assign(
        vec![
            AssignTarget::Decl("b".into(), Type::Bool),
            AssignTarget::Decl("i".into(), Type::Int),
        ],
        vec![Expr::FuncCall("f".into(), vec![Expr::Var("x".into())])],
    );
    assert_eq!(render_flat(&sexp_stmt(&s)), "(= ((b bool) (i int)) (f x))");
}

#[test]
fn test_discard_multi_assign() {
    // _, i: int = foo()  →  (= (_ (i int)) (foo))
    let s = Stmt::Assign(
        vec![
            AssignTarget::Discard,
            AssignTarget::Decl("i".into(), Type::Int),
        ],
        vec![Expr::FuncCall("foo".into(), vec![])],
    );
    assert_eq!(render_flat(&sexp_stmt(&s)), "(= (_ (i int)) (foo))");
}

#[test]
fn test_proc_call() {
    // print("Hello")  →  (print "Hello")
    let s = Stmt::Assign(
        vec![],
        vec![Expr::FuncCall(
            "print".into(),
            vec![Expr::StringLit("Hello".into())],
        )],
    );
    assert_eq!(render_flat(&sexp_stmt(&s)), "(print \"Hello\")");
}

#[test]
fn test_array_index_assign() {
    // a[n] = n  →  (= ([] a n) n)
    let s = Stmt::Assign(
        vec![AssignTarget::ArrayIndex(
            "a".into(),
            vec![Expr::Var("n".into())],
        )],
        vec![Expr::Var("n".into())],
    );
    assert_eq!(render_flat(&sexp_stmt(&s)), "(= ([] a n) n)");
}

#[test]
fn test_if_else() {
    // if (a < b) a = 1 else a = 2
    let s = Stmt::If(
        Expr::BinOp(
            BinOp::Lt,
            Box::new(Expr::Var("a".into())),
            Box::new(Expr::Var("b".into())),
        ),
        Box::new(Stmt::Assign(
            vec![AssignTarget::Var("a".into())],
            vec![Expr::IntLit(1)],
        )),
        Some(Box::new(Stmt::Assign(
            vec![AssignTarget::Var("a".into())],
            vec![Expr::IntLit(2)],
        ))),
    );
    assert_eq!(
        render_flat(&sexp_stmt(&s)),
        "(if (< a b) (= a 1) (= a 2))"
    );
}

#[test]
fn test_while_block() {
    // while (n > 0) { n = n - 1 }
    let s = Stmt::While(
        Expr::BinOp(
            BinOp::Gt,
            Box::new(Expr::Var("n".into())),
            Box::new(Expr::IntLit(0)),
        ),
        Box::new(Stmt::Block(Block {
            stmts: vec![Stmt::Assign(
                vec![AssignTarget::Var("n".into())],
                vec![Expr::BinOp(
                    BinOp::Sub,
                    Box::new(Expr::Var("n".into())),
                    Box::new(Expr::IntLit(1)),
                )],
            )],
            return_val: None,
        })),
    );
    assert_eq!(
        render_flat(&sexp_stmt(&s)),
        "(while (> n 0) ((= n (- n 1))))"
    );
}

#[test]
fn test_return_in_block() {
    // { return pred, expr }  →  ((return pred expr))
    let block = Block {
        stmts: vec![],
        return_val: Some(vec![
            Expr::Var("pred".into()),
            Expr::Var("expr".into()),
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
    // Interface format is (( methods... )) — two levels of parens
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
    // gcd(a:int, b:int):int { while (a != 0) { if (a<b) b=b-a else a=a-b } return b }
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
                stmts: vec![Stmt::While(
                    Expr::BinOp(
                        BinOp::Ne,
                        Box::new(Expr::Var("a".into())),
                        Box::new(Expr::IntLit(0)),
                    ),
                    Box::new(Stmt::Block(Block {
                        stmts: vec![Stmt::If(
                            Expr::BinOp(
                                BinOp::Lt,
                                Box::new(Expr::Var("a".into())),
                                Box::new(Expr::Var("b".into())),
                            ),
                            Box::new(Stmt::Assign(
                                vec![AssignTarget::Var("b".into())],
                                vec![Expr::BinOp(
                                    BinOp::Sub,
                                    Box::new(Expr::Var("b".into())),
                                    Box::new(Expr::Var("a".into())),
                                )],
                            )),
                            Some(Box::new(Stmt::Assign(
                                vec![AssignTarget::Var("a".into())],
                                vec![Expr::BinOp(
                                    BinOp::Sub,
                                    Box::new(Expr::Var("a".into())),
                                    Box::new(Expr::Var("b".into())),
                                )],
                            ))),
                        )],
                        return_val: None,
                    })),
                )],
                return_val: Some(vec![Expr::Var("b".into())]),
            },
        })],
    };
    let out = sexp_program(&p);
    let expected = "(() ((gcd ((a int) (b int)) (int) ((while (!= a 0) ((if (< a b) (= b (- b a)) (= a (- a b))))) (return b)))))";
    assert_eq!(normalize(&out), normalize(expected));
}
