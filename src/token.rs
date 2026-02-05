use logos::Logos;
use regex::Regex;
use crate::callbacks::{newline_callback, parse_string, parse_char};

#[derive(Debug, Logos)]
#[logos(extras = (usize, usize))]
#[logos(skip r"[ \t\r]+")] // whitespace (no newlines)
#[logos(skip(r"[\n]+[\\]", newline_callback))] // Add double slash to skip, whitespace
#[logos(skip r"//[^\n]*")]  // ignore comments
pub enum Token {
       // Keywords - each gets its own variant
       #[token("if")]
       If,
       #[token("while")]
       While,
       #[token("for")]
       For,
       #[token("let")]
       Let,
       #[token("int")]
       Int,
       #[token("true")]
       True,
       #[token("false")]
       False,

       // Symbols - each gets its own variant - think about grouping some symbols later...
       #[token("+")]
       Plus,
       #[token("-")]
       Minus,
       #[token("(")]
       LParen,
       #[token(")")]
       RParen,
       #[token("[")]
       LBracket,
       #[token("]")]
       RBracket,
       #[token("{")]
       LBrace,
       #[token("}")]
       RBrace,
       #[token(";")]
       Semicolon,
       #[token(":")]
       Colon,
       #[token("!")]
       Negation,
       #[token("*")]
       Mul,
       #[token("*>>")]
       HighMul,
       #[token("/")]
       Division,
       #[token("%")]
       Remainder,
       #[token("<")]
       LThan,
       #[token("<=")]
       Leq,
       #[token(">=")]
       Geq,
       #[token(">")]
       GThan,
       #[token("==")]
       Equal,
       #[token("=")]
       Assign,
       #[token("!=")]
       Neq,
       #[token("&")]
       And,
       #[token("|")]
       Or,

       // Infinite sets - carry data
       #[regex(r"[a-zA-Z][a-zA-Z0-9_']*")]
       Identifier(String), // String starts with " so we can ignore precedence - any char followed by non-special chars
       #[regex(r"[0-9]+", |lex| format!(lex.slice().parse().ok()?))]
       Integer(u64),
       #[token("\"", parse_string)]
       String(String),
       #[token('\'', parse_char)]
       Char(char)
}

pub struct LexResult {
    pub result: LexResultKind,
    pub row: usize,
    pub col: usize,
}

pub enum LexResultKind {
    Token(Token)
    Error(LexerError)
}

// TODO: Add appropriate token / regex attributed for errors
pub enum LexerError {
    InvalidCharacter,       // For things like '@'
    UnterminatedLiteral,    // Hit newline or EOF
    InvalidEscape,          // e.g., \q
    InvalidHex,             // e.g., \x{GG}
}

// Parse the list, when you match, push the TokenInfo to the vector, or return error
pub fn tokenize(lex: &mut Lexer<Token>) -> Vector<LexResult> {
    let mut vec: Vector<LexerWrapper> = Vec::new(); // Find information necessary to know <line> and <col>

    for result in lex {
        let (line, col) = lex.extras; // Grab position from extras

        match result {
            Ok(token) => vec.push(
                LexResult {
                    LexResultKind::Token(token),
                    line,
                    col,
            }),
            Err(e) => {
                vec.push(
                    LexResult {
                        LexResultKind::Error(e),
                        line,
                        col,
                }),
                return vec;
            }
        }
    }
    vec
}
