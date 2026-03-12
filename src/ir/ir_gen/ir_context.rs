use std::collections::{HashMap, HashSet};

use crate::ir::symbol::{Label, Temp};
use crate::parser::ast::{FuncDef, Program, TopLevelItem, Type};

pub struct IrContext {
    // temp == used registers
    temp_count: usize,
    label_count: usize,
    // maps source-level name -> Temp, so the same variable always gets the same temp
    pub locals: HashMap<String, Temp>,
    // set of names that are globals, so Var(x) knows to emit Mem(Name(_Vx))
    pub globals: HashSet<String>,
    // function name -> canonical ABI-mangled label
    function_labels: HashMap<String, Label>,
    // string literal contents -> canonical label
    string_labels: HashMap<String, Label>,
    // emitted string data table (label, value), useful for backend emission
    string_data: Vec<(Label, String)>,
}

impl IrContext {
    /// Create a fresh IR lowering context from a known set of global variable names.
    pub fn new(globals: HashSet<String>) -> Self {
        IrContext {
            temp_count: 0,
            label_count: 0,
            locals: HashMap::new(),
            globals,
            function_labels: HashMap::new(),
            string_labels: HashMap::new(),
            string_data: Vec::new(),
        }
    }

    /// Build context directly from a parsed program by collecting globals
    /// and registering canonical function labels.
    pub fn from_program(program: &Program) -> Self {
        let globals = collect_globals(program);
        let mut ctx = IrContext::new(globals);
        ctx.register_functions_from_program(program);
        ctx
    }

    /// Allocate a fresh unnamed temporary (t0, t1, ...).
    pub fn fresh_temp(&mut self) -> Temp {
        let n = self.temp_count;
        self.temp_count += 1;
        Temp::new(format!("t{}", n))
    }

    /// Get (or create) the canonical temp bound to a local source variable name.
    pub fn local_temp(&mut self, name: &str) -> Temp {
        if let Some(t) = self.locals.get(name) {
            return t.clone();
        }
        let t = self.fresh_temp();
        self.locals.insert(name.to_string(), t.clone());
        t
    }

    /// Allocate a fresh synthetic control-flow label (_L0, _L1, ...).
    pub fn fresh_label(&mut self) -> Label {
        let n = self.label_count;
        self.label_count += 1;
        Label::new(format!("_L{}", n))
    }

    /// Compute ABI-mangled function entry label for a function definition.
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
        Label::new(format!("_I{}_{}{}", encoded_name, ret_enc, args_enc))
    }

    /// Register one function's canonical label in the context lookup table.
    pub fn register_function(&mut self, func: &FuncDef) {
        self.function_labels
            .insert(func.name.clone(), Self::function_label(func));
    }

    /// Register canonical labels for all top-level functions in a program.
    pub fn register_functions_from_program(&mut self, program: &Program) {
        for item in &program.items {
            if let TopLevelItem::Func(func) = item {
                self.register_function(func);
            }
        }
    }

    /// Lookup previously registered function label by source function name.
    pub fn function_label_by_name(&self, name: &str) -> Option<Label> {
        self.function_labels.get(name).cloned()
    }

    /// Register and reuse a label for a string literal.
    /// If the same literal appears again, returns the existing label.
    pub fn label_for_string_literal(&mut self, literal: &str) -> Label {
        if let Some(existing) = self.string_labels.get(literal) {
            return existing.clone();
        }

        let label = Label::new(format!("_S{}", self.string_labels.len()));
        self.string_labels
            .insert(literal.to_string(), label.clone());
        self.string_data.push((label.clone(), literal.to_string()));
        label
    }

    /// Expose deduplicated string table for data-section emission.
    pub fn string_literals(&self) -> &[(Label, String)] {
        self.string_data.as_slice()
    }

    /// Compute ABI-mangled global variable label (_V...).
    pub fn global_label(name: &str) -> Label {
        Label::new(format!("_V{}", encode_name(name)))
    }

    /// Return ABI argument temp name for positional index (_ARG1, _ARG2, ...).
    pub fn arg_temp(index: usize) -> Temp {
        Temp::new(format!("_ARG{}", index + 1))
    }

    /// Return ABI return-value temp name for positional index (_RV1, _RV2, ...).
    pub fn rv_temp(index: usize) -> Temp {
        Temp::new(format!("_RV{}", index + 1))
    }

    /// True if source name is a global binding.
    pub fn is_global(&self, name: &str) -> bool {
        self.globals.contains(name)
    }

    /// Clear local temp bindings when entering a new function body.
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

fn collect_globals(program: &Program) -> HashSet<String> {
    program
        .items
        .iter()
        .filter_map(|item| match item {
            TopLevelItem::Global(g) => Some(g.name.clone()),
            _ => None,
        })
        .collect()
}

