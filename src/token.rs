use logos::{Logos, Lexer};
use strum_macros::Display;
use crate::callbacks::{newline_callback, comment_callback, lex_string, lex_char};

#[derive(Debug, Logos, PartialEq, Display)]
#[logos(extras = LexerExtras)]
pub enum Token {
    // Error token for logos 0.12
    #[error]
    Error,

    // Skip whitespace (not newlines)
    #[regex(r"[ \t\r]+", logos::skip)]
    Whitespace,

    // Newlines - need to track line numbers
    #[regex(r"\n", newline_callback)]
    Newline,

    // Comments
    #[regex(r"//[^\n]*", comment_callback)]
    Comment,

    // Keywords
    #[strum(serialize = "if")]
    #[token("if")]
    If,

    #[strum(serialize = "while")]
    #[token("while")]
    While,

    #[strum(serialize = "for")]
    #[token("for")]
    For,

    #[strum(serialize = "let")]
    #[token("let")]
    Let,

    #[strum(serialize = "int")]
    #[token("int")]
    Int,

    #[strum(serialize = "bool")]
    #[token("bool")]
    Bool,

    #[strum(serialize = "true")]
    #[token("true")]
    True,

    #[strum(serialize = "false")]
    #[token("false")]
    False,

    #[strum(serialize = "use")]
    #[token("use")]
    Use,

    #[strum(serialize = "else")]
    #[token("else")]
    Else,

    #[strum(serialize = "return")]
    #[token("return")]
    Return,

    #[strum(serialize = "length")]
    #[token("length")]
    Length,

    // Symbols
    #[strum(serialize = "+")]
    #[token("+")]
    Plus,

    #[strum(serialize = "-")]
    #[token("-")]
    Minus,

    #[strum(serialize = "(")]
    #[token("(")]
    LParen,

    #[strum(serialize = ")")]
    #[token(")")]
    RParen,

    #[strum(serialize = "[")]
    #[token("[")]
    LBracket,

    #[strum(serialize = "]")]
    #[token("]")]
    RBracket,

    #[strum(serialize = "{")]
    #[token("{")]
    LBrace,

    #[strum(serialize = "}}")]
    #[token("}")]
    RBrace,

    #[strum(serialize = ";")]
    #[token(";")]
    Semicolon,

    #[strum(serialize = ":")]
    #[token(":")]
    Colon,

    #[strum(serialize = "!")]
    #[token("!")]
    Negation,

    #[strum(serialize = "*")]
    #[token("*")]
    Mul,

    #[strum(serialize = "*>>")]
    #[token("*>>")]
    HighMul,

    #[strum(serialize = "/")]
    #[token("/")]
    Division,

    #[strum(serialize = "%")]
    #[token("%")]
    Remainder,

    #[strum(serialize = "<")]
    #[token("<")]
    LThan,

    #[strum(serialize = "<=")]
    #[token("<=")]
    Leq,

    #[strum(serialize = ">=")]
    #[token(">=")]
    Geq,

    #[strum(serialize = ">")]
    #[token(">")]
    GThan,

    #[strum(serialize = "==")]
    #[token("==")]
    Equal,

    #[strum(serialize = "=")]
    #[token("=")]
    Assign,

    #[strum(serialize = "!=")]
    #[token("!=")]
    Neq,

    #[strum(serialize = "&")]
    #[token("&")]
    And,

    #[strum(serialize = "|")]
    #[token("|")]
    Or,

    #[strum(serialize = ",")]
    #[token(",")]
    Comma,

    #[strum(serialize = "_")]
    #[token("_")]
    Underscore,

    // Data-carrying variants (handled separately in format_token)
    #[strum(serialize = "id")]
    #[regex(r"[a-zA-Z][a-zA-Z0-9_']*", |lex| lex.slice().to_string())]
    Identifier(String),

    #[strum(serialize = "integer")]
    #[regex(r"[0-9]+", |lex| lex.slice().parse().ok())]
    Integer(u64),

    #[strum(serialize = "string")]
    #[token("\"", lex_string)]
    String(String),

    #[strum(serialize = "character")]
    #[token("'", lex_char)]
    Char(char),
}

#[derive(Default)]
pub struct LexerExtras {
    pub line: usize,
    pub line_start: usize,
    pub has_token: bool,
    pub saw_comment: bool,
    pub error: Option<LexerError>,
}

impl LexerExtras {
    pub fn new() -> Self {
        Self { line: 1, line_start: 0, has_token: false, saw_comment: false, error: None }
    }
}

pub struct LexResult {
    pub result: LexResultKind,
    pub line: usize,
    pub col: usize,
}

pub enum LexResultKind {
    Token(Token),
    Error(LexerError)
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum LexerError {
    #[default]
    InvalidCharacter,
    UnterminatedLiteral,    // Hit newline or EOF
    InvalidEscape,          // e.g., \q
    InvalidHex,             // e.g., \x{GG}
    EmptyCharacter,
    MultiCharacterConstant,
}

// lex the list, when you match, push the TokenInfo to the vector, or return error
pub fn tokenize(lex: &mut Lexer<Token>) -> Vec<LexResult> {
    let mut vec: Vec<LexResult> = Vec::new();

    while let Some(result) = lex.next() {
        let line = lex.extras.line;
        let col = lex.span().start - lex.extras.line_start + 1;

        // Check if callback stored an error
        if let Some(err) = lex.extras.error.take() {
            vec.push(LexResult {
                result: LexResultKind::Error(err),
                line,
                col,
            });
            return vec;
        }

        match result {
            Token::Error => {
                vec.push(LexResult {
                    result: LexResultKind::Error(LexerError::InvalidCharacter),
                    line,
                    col,
                });
                return vec;
            }
            Token::Newline | Token::Comment => {
                // Skip these, they're handled by callbacks for tracking
                continue;
            }
            token => {
                lex.extras.has_token = true;
                vec.push(LexResult {
                    result: LexResultKind::Token(token),
                    line,
                    col,
                });
            }
        }
    }
    vec
}
