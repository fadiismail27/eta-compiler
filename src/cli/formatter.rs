use crate::lexer::{LexResult, LexResultKind, Token, LexerError};

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LexerError::UnterminatedLiteral(_) => write!(f, "Unterminated String Literal"),
            LexerError::InvalidEscape(_) => write!(f, "Invalid Escape"),
            LexerError::InvalidHex(_) => write!(f, "Invalid Hex Escape"),
            LexerError::EmptyCharacter(_) => write!(f, "Empty Character Literal"),
            LexerError::MultiCharacterConstant(_) => write!(f, "Multi-Character Literal"),
            LexerError::InvalidCharacter(_) => write!(f, "Invalid Character"),
        }
    }
}

pub fn format_result(r: &LexResult) -> String {
    let pos = format!("{}:{}", r.line, r.col);
    match &r.result {
        LexResultKind::Token(token) => format_token(&pos, token),
        LexResultKind::Error(err) => format!("{} error:{}", pos, err),
    }
}

fn format_token(pos: &str, token: &Token) -> String {
    match token {
        Token::Identifier(name) => format!("{} id {}", pos, name),
        Token::Integer(val) => format!("{} integer {}", pos, val),
        Token::Char(val) => format!("{} character {}", pos, escape_char(*val)),
        Token::String(val) => format!("{} string {}", pos, escape_string(val)),
        Token::RBrace => format!("{} }}", pos),
        other => format!("{} {}", pos, other),  // uses Display from strum
    }
}

fn escape_char(c: char) -> String {
    match c {
        '\n' => "\\n".to_string(),
        '\t' => "\\t".to_string(),
        '\r' => "\\r".to_string(),
        '\\' => "\\\\".to_string(),
        c if !c.is_control() => c.to_string(),
        c => format!("\\x{{{:X}}}", c as u32),
    }
}

fn escape_string(s: &str) -> String {
    s.chars().map(escape_char).collect()
}

/// Formats all tokens into the complete .lexed file content
pub fn format_lexed_output(results: &[LexResult]) -> String {
    if results.is_empty() {
        return String::new();
    }
    let mut output = results
        .iter()
        .map(format_result)
        .collect::<Vec<_>>()
        .join("\n");
    output.push('\n');
    output
}
