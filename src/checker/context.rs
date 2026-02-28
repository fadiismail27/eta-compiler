use crate::checker::check::{SemanticError, SemanticType};
use crate::parser::ast::{BinOp, Expr, Stmt, Type};

#[derive(Clone)]
pub struct Context;

impl Context {
    // THIS IS A STUB
    pub fn search(&self, name: &String) -> Option<&SemanticType> {
        None
    }
    pub fn push(&self, name: &String, typ: &SemanticType) -> bool {
        false
    }
    pub fn remove(&self, name: &String) -> bool {
        false
    }
}
