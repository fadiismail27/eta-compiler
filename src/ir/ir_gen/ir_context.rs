use crate::checker::context::Context;
use crate::parser::ast::{
    AssignTarget, BinOp, Block, Expr, ExprKind, FuncDef, Interface, Program, Span, Spanned,
    StmtKind, TopLevelItem, Type, UnaryOp,
};

#[derive(Clone, Debug)]
pub struct Label {
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct Temp {
    pub name: String,
}

// #[derive(Clone, Debug)]
// pub struct IrContext {
//     register_count: usize,
//     temp_count: usize,
// }

pub struct IrContext {
    // temp == used registers
    temp_count: usize,
    label_count: usize,
    // maps source-level name -> Temp, so the same variable always gets the same temp
    pub locals: std::collections::HashMap<String, Temp>,
    // set of names that are globals, so Var(x) knows to emit Mem(Name(_Vx))
    pub globals: std::collections::HashSet<String>,
}

impl IrContext {
    pub fn new(globals: std::collections::HashSet<String>) -> Self {
        IrContext {
            temp_count: 0,
            label_count: 0,
            locals: std::collections::HashMap::new(),
            globals,
        }
    }

    // fresh unnamed intermediate temp
    pub fn fresh_temp(&mut self) -> Temp {
        let n = self.temp_count;
        self.temp_count += 1;
        Temp { name: format!("t{}", n) }
    }

    // get or create the temp for a named local variable
    pub fn local_temp(&mut self, name: &str) -> Temp {
        if let Some(t) = self.locals.get(name) {
            return t.clone();
        }
        let t = self.fresh_temp();
        self.locals.insert(name.to_string(), t.clone());
        t
    }

    // fresh synthetic control-flow label
    pub fn fresh_label(&mut self) -> Label {
        let n = self.label_count;
        self.label_count += 1;
        Label { name: format!("_L{}", n) }
    }

    // ABI-mangled function entry label
   pub fn function_label(func: &FuncDef) -> Label {
        let encoded_name = encode_name(&func.name);
        let ret_enc = if func.returns.is_empty() {
            "p".to_string()
        } else if func.returns.len() == 1 {
            encode_type(&func.returns[0])
        } else {
            format!("t{}{}", func.returns.len(), func.returns.iter().map(encode_type).collect::<String>())
        };
        let args_enc: String = func.params.iter().map(|p| encode_type(&p.ty)).collect();
        Label { name: format!("_I{}_{}{}", encoded_name, ret_enc, args_enc) }
    }

    // ABI-mangled global variable label
    pub fn global_label(name: &str) -> Label {
        Label { name: format!("_V{}", encode_name(name)) }
    }

    // dedicated arg temps per ABI (_ARG1, _ARG2, ...)
    pub fn arg_temp(index: usize) -> Temp {
        Temp { name: format!("_ARG{}", index + 1) }
    }

    // dedicated return value temps per ABI (_RV1, _RV2, ...)
    pub fn rv_temp(index: usize) -> Temp {
        Temp { name: format!("_RV{}", index + 1) }
    }

    pub fn is_global(&self, name: &str) -> bool {
        self.globals.contains(name)
    }

    // reset locals when entering a new function
    pub fn enter_function(&mut self) {
        self.locals.clear();
    }
}

fn encode_name(name: &str) -> String {
    name.replace('\'', "$1").replace('_', "__")
}

fn encode_type(ty: &Type) -> String {
    match ty {
        Type::Int => "i".to_string(),
        Type::Bool => "b".to_string(),
        Type::Array(inner, _) => format!("a{}", encode_type(inner)),
    }
}

