pub mod ast;
pub mod adapter;
pub mod sexp;

// Include the LALRPOP-generated parser from OUT_DIR.
#[allow(clippy::all)]
#[allow(unused)]
pub mod eta {
    include!(concat!(env!("OUT_DIR"), "/parser/eta.rs"));
}
