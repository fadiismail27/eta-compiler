pub mod error;

pub use error::SemanticError;

use crate::parser::ast::{Interface, Program};

/// Type-check a program. Returns Ok(()) if valid, Err(SemanticError) on first error.
/// `interfaces` contains the parsed .eti files imported via `use` statements.
pub fn typecheck_program(
    _program: &Program,
    _interfaces: &[(String, Interface)],
) -> Result<(), SemanticError> {
    // TODO: teammates implement this
    Ok(())
}

/// Type-check an interface file for well-formedness.
pub fn typecheck_interface(_interface: &Interface) -> Result<(), SemanticError> {
    // TODO: teammates implement this
    Ok(())
}
