#[derive(Debug, Clone, PartialEq)]
pub struct LexerError {
    #[default]
    InvalidCharacter,       // For things like '@'
    UnterminatedLiteral,    // Hit newline or EOF
    InvalidEscape,          // e.g., \q
    InvalidHex,             // e.g., \x{GG}
    EmptyCharacter         // ''
    MultiCharacterConstant // 'ab'
}

/// Called by main lexer when it sees a single quote.
/// `source` is the full source chars, `pos` is the index of the opening quote.
/// Returns the token and the index to resume lexing from.
pub fn lex_char(source: &[char]) -> Result<char,LexerError> {
    let mut raw = &lex 
}

/// Called by main lexer when it sees a double quote.
/// Same pattern.
pub fn lex_string(source: &[char]) -> Result<String,LexerError>