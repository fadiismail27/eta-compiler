/// Called by main lexer when it sees a single quote.
/// `source` is the full source chars, `pos` is the index of the opening quote.
/// Returns the token and the index to resume lexing from.
pub fn lex_char(lex: &mut logos::Lexer<Token>) -> Result<char, LexerError> {
    let raw = &lex.slice();
    let inner = &raw
}

pub fn lex_string(lex: &mut logos::Lexer<Token>) -> Result<String, LexerError> {
    lex.bump(1);
    let mut result = String::new();
    let mut chars = lex.remainder().chars();

    loop {
        match chars.next() {
            None => return Err(LexerError::UnterminatedLiteral),
            Some('\n') => return Err(LexerError::UnterminatedLiteral),
            Some('"') => {lex.bump(1); break;}
            Some('\\') => {
                lex.bump(1);
                match chars.next() {
                    Some('n') => {result.push('\n'); lex.bump(1);}
                    Some('\'') => {result.push('\''); lex.bump(1);}
                    Some('\\') => {result.push('\\'); lex.bump(1);}
                    Some('"') => {result.push('"'); lex.bump(1);}
                    Some('x') => {
                        lex.bump(1);
                        if chars.next() != Some('{') {
                            return Err(LexerError::InvalidEscape);
                        }
                        lex.bump(1);
                        let mut hex = String::new();

                        while let Some(c) = chars.next() {
                            if c == '\n' || c == '\'' {
                                return Err(LexerError::UnterminatedLiteral);
                            }
                            if c == '}' {break;}
                            hex.push(c);
                        }
                        lex.bump(1);
                        
                        let code = match u32::from_str_radix(&hex,16) {
                            Ok(c) => c,
                            Err(_) => return Err(LexerError::InvalidUnicode),
                        };

                        let uchar = match char::from_u32(code) {
                            Some(c) => c,
                            None => return Err(LexerError::InvalidUnicode),
                        };
                        result.push(uchar);
                    }
                    _ => return Err(LexerError::InvalidEscape),
                }
            }
            Some(c) => {
                lex.bump(c.len_utf8());
                result.push(uchar);
            }
        }

    }
    Ok(result)
}