use logos::Logos;
use regex::Regex;

#[derive(Logos)]
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
       False

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
       #[regex("", func)]
       Identifier(String), // String starts with " so we can ignore precedence - any char followed by non-special chars
       #[regex("[0-9]+")]
       Integer(i64),
       #[regex("")] // double quote followed by any sequence of chars (escape chars necessary for " and '), followed by a double quote
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

// Maybe change name lol
pub enum LexerWrapper {
    Information(TokenInfo)
    Error(LexerError)
}

// parse the list, when you match, push the TokenInfo to the vector, or return error...
pub fn tokenize(lex: &mut Lexer<Token>) -> Vector<LexerWrapper> {


}

/*
 * In the main loop:
 *  1. Create lexer from given input - different functions borrow it sequentially
 *  2. Call tokenize with this lexer, and store the result Vector
 *  3. Call some parsing function to push the final output to the specified file
 */
