// Formatter for token output
// Depends on Person A's token types from lexer module

/// Formats a single token for .lexed output
/// Returns: "<line>:<col> <token_type> [<value>]"
///
/// Expected TokenInfo structure:
/// ```
/// pub struct TokenInfo {
///     pub token: Token,
///     pub pos: SourcePos,
/// }
///
/// pub struct SourcePos {
///     pub line: usize,
///     pub col: usize,
/// }
///
/// pub enum Token {
///     Keyword(String),
///     Id(String),
///     Integer(String),
///     CharLiteral(String),
///     StringLiteral(String),
///     Symbol(String),
///     Error(String),
/// }
/// ```
pub fn format_token(t: &crate::lexer::TokenInfo) -> String {
    let pos = format!("{}:{}", t.pos.line, t.pos.col);
    match &t.token {
        crate::lexer::Token::Id(name) => format!("{} id {}", pos, name),
        crate::lexer::Token::Integer(val) => format!("{} integer {}", pos, val),
        crate::lexer::Token::CharLiteral(val) => format!("{} character {}", pos, val),
        crate::lexer::Token::StringLiteral(val) => format!("{} string {}", pos, val),
        crate::lexer::Token::Symbol(s) => format!("{} {}", pos, s),
        crate::lexer::Token::Keyword(k) => format!("{} {}", pos, k),
        crate::lexer::Token::Error(desc) => format!("{} error:{}", pos, desc),
    }
}

/// Formats all tokens into the complete .lexed file content
pub fn format_lexed_output(tokens: &[crate::lexer::TokenInfo]) -> String {
    tokens
        .iter()
        .map(format_token)
        .collect::<Vec<_>>()
        .join("\n")
}
