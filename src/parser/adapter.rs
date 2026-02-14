use logos::Logos;
use crate::lexer::token::{Token as LexToken, LexerExtras, LexerError};
use crate::parser::ast::Token as ParseToken;

// result of lexing for the parser: either a positioned token or an error with location
pub enum AdapterResult {
    Tokens(Vec<(usize, ParseToken, usize)>),
    LexError { line: usize, col: usize, msg: String },
}

// lex source and convert to LALRPOP token triples
// iterates the logos lexer directly to get byte offsets
pub fn lex_for_parser(source: &str) -> AdapterResult {
    let mut lex = LexToken::lexer_with_extras(source, LexerExtras::new());
    let mut tokens = Vec::new();

    while let Some(result) = lex.next() {
        let span = lex.span();

        match result {
            Ok(tok) => {
                match convert(tok) {
                    Ok(pt) => tokens.push((span.start, pt, span.end)),
                    Err(msg) => {
                        let line = lex.extras.line;
                        let col = span.start - lex.extras.line_start + 1;
                        return AdapterResult::LexError { line, col, msg };
                    }
                }
            }
            Err(e) => {
                let line = lex.extras.line;
                let mut byte_pos = span.start;
                match &e {
                    LexerError::InvalidCharacter(offset) => byte_pos += offset,
                    LexerError::UnterminatedLiteral(offset) => byte_pos += offset,
                    LexerError::InvalidEscape(offset) => byte_pos += offset,
                    LexerError::InvalidHex(offset) => byte_pos += offset,
                    LexerError::EmptyCharacter(offset) => byte_pos += offset,
                    LexerError::MultiCharacterConstant(offset) => byte_pos += offset,
                }
                let col = byte_pos - lex.extras.line_start + 1;
                return AdapterResult::LexError { line, col, msg: format_lex_error(&e) };
            }
        }
    }

    AdapterResult::Tokens(tokens)
}

fn format_lex_error(e: &LexerError) -> String {
    match e {
        LexerError::InvalidCharacter(_) => "Invalid character".to_string(),
        LexerError::UnterminatedLiteral(_) => "Unterminated string literal".to_string(),
        LexerError::InvalidEscape(_) => "Invalid escape sequence".to_string(),
        LexerError::InvalidHex(_) => "Invalid hex escape".to_string(),
        LexerError::EmptyCharacter(_) => "Empty character literal".to_string(),
        LexerError::MultiCharacterConstant(_) => "Multi-character constant".to_string(),
    }
}

fn convert(tok: LexToken) -> Result<ParseToken, String> {
    let pt = match tok {
        // keywords
        LexToken::Use       => ParseToken::Use,
        LexToken::If        => ParseToken::If,
        LexToken::Else      => ParseToken::Else,
        LexToken::While     => ParseToken::While,
        LexToken::Return    => ParseToken::Return,
        LexToken::Length    => ParseToken::Length,
        LexToken::Int       => ParseToken::IntType,
        LexToken::Bool      => ParseToken::BoolType,
        LexToken::True      => ParseToken::True,
        LexToken::False     => ParseToken::False,

        // delimiters
        LexToken::LParen    => ParseToken::LParen,
        LexToken::RParen    => ParseToken::RParen,
        LexToken::LBracket  => ParseToken::LBracket,
        LexToken::RBracket  => ParseToken::RBracket,
        LexToken::LBrace    => ParseToken::LBrace,
        LexToken::RBrace    => ParseToken::RBrace,

        // punctuation
        LexToken::Colon     => ParseToken::Colon,
        LexToken::Semicolon => ParseToken::Semicolon,
        LexToken::Comma     => ParseToken::Comma,
        LexToken::Assign    => ParseToken::Assign,
        LexToken::Underscore => ParseToken::Underscore,

        // operators (renamed ones)
        LexToken::Plus      => ParseToken::Plus,
        LexToken::Minus     => ParseToken::Minus,
        LexToken::Mul       => ParseToken::Mul,
        LexToken::HighMul   => ParseToken::HighMul,
        LexToken::Division  => ParseToken::Div,
        LexToken::Remainder => ParseToken::Mod,
        LexToken::Negation  => ParseToken::Not,
        LexToken::LThan     => ParseToken::Lt,
        LexToken::Leq       => ParseToken::Le,
        LexToken::GThan     => ParseToken::Gt,
        LexToken::Geq       => ParseToken::Ge,
        LexToken::Equal     => ParseToken::Eq,
        LexToken::Neq       => ParseToken::Ne,
        LexToken::And       => ParseToken::And,
        LexToken::Or        => ParseToken::Or,

        // data-carrying
        LexToken::Identifier(s)  => ParseToken::Identifier(s),
        LexToken::Integer(n)     => ParseToken::IntLiteral(n as i64),
        LexToken::Char(c)        => ParseToken::CharLiteral(c as i64),
        LexToken::String(s)      => ParseToken::StringLiteral(s),

        // tokens not in the grammar
        LexToken::Break => return Err("Unexpected token: break".to_string()),
        LexToken::For   => return Err("Unexpected token: for".to_string()),
        LexToken::Let   => return Err("Unexpected token: let".to_string()),
    };
    Ok(pt)
}
