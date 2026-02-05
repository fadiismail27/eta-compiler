use crate::token::{LexResult, LexResultKind, Token, LexerError};

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LexerError::InvalidCharacter => write!(f, "Invalid character"),
            LexerError::UnterminatedLiteral => write!(f, "Unterminated literal"),
            LexerError::InvalidEscape => write!(f, "Invalid escape"),
            LexerError::InvalidHex => write!(f, "Invalid hex"),
            LexerError::EmptyCharacter => write!(f, "Empty character"),
            LexerError::MultiCharacterConstant => write!(f, "Multi-character constant"),
        }
    }
}

pub fn format_result(r: &LexResult) -> String {
    let pos = format!("{}:{}", r.line, r.col);
    match &r.kind {
        LexResultKind::Token(token) => format_token(&pos, token),
        LexResultKind::Error(err) => format!("{} error:{}", pos, err),
    }
}

fn format_token(pos: &str, token: &Token) -> String {
    match token {
        Token::Id(name) => format!("{} id {}", pos, name),
        Token::Integer(val) => format!("{} integer {}", pos, val),
        Token::CharLiteral(val) => format!("{} character {}", pos, val),
        Token::StringLiteral(val) => format!("{} string {}", pos, val),
        Token::Symbol(s) => format!("{} {}", pos, s),
        Token::Keyword(k) => format!("{} {}", pos, k),
    }
}

/// Formats all tokens into the complete .lexed file content
pub fn format_lexed_output(results: &[LexResult]) -> String {
    results
        .iter()
        .map(format_result)
        .collect::<Vec<_>>()
        .join("\n")
}
