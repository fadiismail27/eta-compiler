use crate::token::{Token, TokenInfo};

/// Formats a single token for .lexed output
/// Returns: "<line>:<col> <token_type> [<value>]"
pub fn format_token(t: &TokenInfo) -> String {
    let pos = format!("{}:{}", t.pos.line, t.pos.col);
    match &t.token {
        Token::Id(name) => format!("{} id {}", pos, name),
        Token::Integer(val) => format!("{} integer {}", pos, val),
        Token::CharLiteral(val) => format!("{} character {}", pos, val),
        Token::StringLiteral(val) => format!("{} string {}", pos, val),
        Token::Symbol(s) => format!("{} {}", pos, s),
        Token::Keyword(k) => format!("{} {}", pos, k),
        Token::Error(desc) => format!("{} error:{}", pos, desc),
    }
}

/// Formats all tokens into the complete .lexed file content
pub fn format_lexed_output(tokens: &[TokenInfo]) -> String {
    tokens
        .iter()
        .map(format_token)
        .collect::<Vec<_>>()
        .join("\n")
}
