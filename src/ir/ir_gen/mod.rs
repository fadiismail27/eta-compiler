// ir/gen/mod.rs
mod context;
mod expr;
mod stmt;
mod mangle;
mod checker

pub use context::IrContext;

// the main entry point -- takes a type-checked program, returns IR
pub use stmt::translate_program;