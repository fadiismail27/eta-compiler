use super::ir_context::IrContext;
use super::super::ir_expr::Expr as IrExpr;
use super::super::ir_stmt::Stmt as IrStmt;
use super::super::parser::ast::{Expr as AstExpr, ExprKind as AstExprKind, BinOp as AstBinOp, UnaryOp as AstUnOp};
use super::super::ir_op::{BinOp as IrBinOp, RelOp, UnOp as IrUnOp};
use super::super::symbol::Label;

const WORD_SIZE: i64 = 8;

/// Lower one AST expression node into IR expression form.
pub fn lower_expr(ctx: &mut IrContext, e: &AstExpr) -> IrExpr {
    match &e.node {
        AstExprKind::IntLit(n) => IrExpr::Const(*n),
        AstExprKind::StringLit(s) => IrExpr::Name(ctx.label_for_string_literal(s)),
        AstExprKind::CharLit(c) => IrExpr::Const(*c),
        AstExprKind::BoolLit(b) => IrExpr::Const(if *b { 1 } else { 0 }),
        AstExprKind::Var(name) => {
            if ctx.is_global(name) {
                IrExpr::Mem(Box::new(IrExpr::Name(IrContext::global_label(name))))
            } else {
                IrExpr::Temp(ctx.local_temp(name))
            }
        },
        AstExprKind::UnaryOp(op, inner) => {
            let inner_ir = lower_expr(ctx, inner);

            match op {
                AstUnOp::Not => IrExpr::UnOp {
                    op: IrUnOp::Not,
                    expr: Box::new(inner_ir),
                },
                AstUnOp::Neg => IrExpr::BinOp {
                    op: IrBinOp::Sub,
                    left: Box::new(IrExpr::Const(0)),
                    right: Box::new(inner_ir),
                },
            }
        },
        AstExprKind::BinOp(op, left, right) => {
            let left_ir = lower_expr(ctx, left);
            let right_ir = lower_expr(ctx, right);

            if let Some(ir_op) = ast_binop_to_ir_binop(op) {
                IrExpr::BinOp {
                    op: ir_op,
                    left: Box::new(left_ir),
                    right: Box::new(right_ir),
                }
            } else if let Some(rel_op) = ast_binop_to_relop(op) {
                let result_temp = ctx.fresh_temp();
                let true_label = ctx.fresh_label();
                let false_label = ctx.fresh_label();
                let end_label = ctx.fresh_label();

                IrExpr::ESeq {
                    stmt: Box::new(IrStmt::Seq(vec![
                        IrStmt::CJump {
                            op: rel_op,
                            left: Box::new(left_ir),
                            right: Box::new(right_ir),
                            t: true_label.clone(),
                            f: false_label.clone(),
                        },
                        IrStmt::Label(true_label),
                        IrStmt::Move {
                            dst: Box::new(IrExpr::Temp(result_temp.clone())),
                            src: Box::new(IrExpr::Const(1)),
                        },
                        IrStmt::Jump(end_label.clone()),
                        IrStmt::Label(false_label),
                        IrStmt::Move {
                            dst: Box::new(IrExpr::Temp(result_temp.clone())),
                            src: Box::new(IrExpr::Const(0)),
                        },
                        IrStmt::Label(end_label),
                    ])),
                    expr: Box::new(IrExpr::Temp(result_temp)),
                }
            } else {
                unreachable!("unmapped AstBinOp variant")
            }
        },
        AstExprKind::FuncCall(func, params) => {
            let target = IrExpr::Name(
                ctx.function_label_by_name(func)
                    .unwrap_or_else(|| {
                        panic!(
                            "function label missing in IrContext for `{}`; register functions before lowering calls",
                            func
                        )
                    }),
            );
            let mut args = Vec::new();
            
            for p in params {
                args.push(lower_expr(ctx, p));
            }

            IrExpr::Call {
                target: Box::new(target),
                args,
            }
        },
        AstExprKind::Index(arr, idx) => {
            let base = lower_expr(ctx, arr);
            let index = lower_expr(ctx, idx);
            let scaled = IrExpr::BinOp {
                op: IrBinOp::Mul,
                left: Box::new(index),
                right: Box::new(IrExpr::Const(WORD_SIZE)),
            };
            let addr = IrExpr::BinOp {
                op: IrBinOp::Add,
                left: Box::new(base),
                right: Box::new(scaled),
            };
            IrExpr::Mem(Box::new(addr))
        },
        AstExprKind::Length(inner) => {
            let base = lower_expr(ctx, inner);
            let header_addr = IrExpr::BinOp {
                op: IrBinOp::Sub,
                left: Box::new(base),
                right: Box::new(IrExpr::Const(WORD_SIZE)),
            };
            IrExpr::Mem(Box::new(header_addr))
        },
        AstExprKind::ArrayConstructor(elems) => {
            let len = elems.len() as i64;
            let total_bytes = (len + 1) * WORD_SIZE;
            let raw_base = ctx.fresh_temp();
            let arr_base = ctx.fresh_temp();

            let alloc_target = Label::new("_eta_alloc");

            let mut stmts = Vec::new();

            // raw_base <- CALL _eta_alloc(total_bytes)
            stmts.push(IrStmt::Move {
                dst: Box::new(IrExpr::Temp(raw_base.clone())),
                src: Box::new(IrExpr::Call {
                    target: Box::new(IrExpr::Name(alloc_target)),
                    args: vec![IrExpr::Const(total_bytes)],
                }),
            });

            // MEM(raw_base) <- len   // header stores array length
            stmts.push(IrStmt::Move {
                dst: Box::new(IrExpr::Mem(Box::new(IrExpr::Temp(raw_base.clone())))),
                src: Box::new(IrExpr::Const(len)),
            });

            // arr_base <- raw_base + 8  // first element pointer
            stmts.push(IrStmt::Move {
                dst: Box::new(IrExpr::Temp(arr_base.clone())),
                src: Box::new(IrExpr::BinOp {
                    op: IrBinOp::Add,
                    left: Box::new(IrExpr::Temp(raw_base)),
                    right: Box::new(IrExpr::Const(WORD_SIZE)),
                }),
            });

            // Store elements at arr_base + i*8
            for (i, elem) in elems.iter().enumerate() {
                let value_ir = lower_expr(ctx, elem);
                let offset = (i as i64) * WORD_SIZE;
                let elem_addr = IrExpr::BinOp {
                    op: IrBinOp::Add,
                    left: Box::new(IrExpr::Temp(arr_base.clone())),
                    right: Box::new(IrExpr::Const(offset)),
                };

                stmts.push(IrStmt::Move {
                    dst: Box::new(IrExpr::Mem(Box::new(elem_addr))),
                    src: Box::new(value_ir),
                });
            }

            IrExpr::ESeq {
                stmt: Box::new(IrStmt::Seq(stmts)),
                expr: Box::new(IrExpr::Temp(arr_base)),
            }
        },

    }
}

// Maps AST Binary Operations to IR Binary Operations
pub fn ast_binop_to_ir_binop(op: &AstBinOp) -> Option<IrBinOp> {
    match op {
        AstBinOp::Add => Some(IrBinOp::Add),
        AstBinOp::Sub => Some(IrBinOp::Sub),
        AstBinOp::Mul => Some(IrBinOp::Mul),
        AstBinOp::HighMul => Some(IrBinOp::HMul),
        AstBinOp::Div => Some(IrBinOp::Div),
        AstBinOp::Mod => Some(IrBinOp::Mod),
        AstBinOp::And => Some(IrBinOp::And),
        AstBinOp::Or => Some(IrBinOp::Or),

        // Comparisons are handled via RelOp helper.
        AstBinOp::Lt
        | AstBinOp::Le
        | AstBinOp::Gt
        | AstBinOp::Ge
        | AstBinOp::Eq
        | AstBinOp::Ne => None,
    }
}

// Maps AST Binary Operations to IR Relational Operations
pub fn ast_binop_to_relop(op: &AstBinOp) -> Option<RelOp> {
    match op {
        AstBinOp::Eq => Some(RelOp::Eq),
        AstBinOp::Ne => Some(RelOp::Neq),
        AstBinOp::Lt => Some(RelOp::Lt),
        AstBinOp::Le => Some(RelOp::Leq),
        AstBinOp::Gt => Some(RelOp::Gt),
        AstBinOp::Ge => Some(RelOp::Geq),
        _ => None,
    }
}
