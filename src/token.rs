use logos::Logos;
use regex::Regex;

#[derive(Debug, Logos)]
#[logos(extras = )]
#[logos(skip(r"\n", newline_callback))]
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
       #[token("//")]
       DQuote,

       // Infinite sets - carry data
       #[regex(r"[a-zA-Z][a-zA-Z0-9_']*")]
       Identifier(String), // String starts with " so we can ignore precedence - any char followed by non-special chars
       #[regex(r"[0-9]+", |lex| (lex.slice().parse().ok()?))]
       Integer(u64),
       #[regex(r#""([^"\n\\]|\\.)*""#)] // Double quote followed by any sequence of chars (escape chars necessary for " and '), followed by a double quote
       String(String),
}

pub struct TokenInfo {
    pub token: Token,
    pub row: usize,
    pub col: usize,
}

// TODO: Add appropriate token / regex attributed for errors
pub enum LexerError {
    InvalidCharacter,       // For things like '@'
    UnterminatedLiteral,    // Hit newline or EOF
    InvalidEscape,          // e.g., \q
    InvalidHex,             // e.g., \x{GG}
    EmptyCharacter,         // ''
    MultiCharacterConstant, // 'ab'
}

// Maybe change name lolz
pub enum LexerWrapper {
    Information(TokenInfo)
    Error(LexerError)
}

// Parse the list, when you match, push the TokenInfo to the vector, or return error
pub fn tokenize(lex: &mut Lexer<Token>) -> Vector<LexerWrapper> {
    let mut vec: Vector<LexerWrapper> = Vec::new(); // Find information necessary to know <line> and <col>
    let (line, col) = lex.extras; // Grab position from extras

    for result in lex {
        match result {
            Ok(token) => vec.push(LexerWrapper::Information(
                TokenInfo {
                token: token,
                line: line,
                col: col
            }))
            Err(e) => vec.push(LexerError::Error(e)) // match e with LexerError, push error to vector
        }
    }
}
