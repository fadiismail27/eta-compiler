use logos::{Lexer, Skip};
use crate::token::{Token, LexerError};

pub fn newline_callback(lex: &mut Lexer<Token>) -> Skip {
    lex.extras.0 += 1;
    lex.extras.1 = lex.span().end;
    Skip
}

/// Called by main lexer when it sees a single quote.
/// `source` is the full source chars, `pos` is the index of the opening quote.
/// Returns the token and the index to resume lexing from.
pub fn lex_char(lex: &mut Lexer<Token>) -> Result<char, LexerError> {
    let raw = lex.slice();
    if raw.len() < 2 {
        return Err(LexerError::UnterminatedLiteral);
    }
    let inner = &raw[1..raw.len() - 1];
    
    let mut chars = inner.chars();
    match chars.next() {
        None => Err(LexerError::EmptyCharacter),
        Some('\\') => {
            match chars.next() {
                Some('n') => {
                    if chars.next().is_some() { return Err(LexerError::MultiCharacterConstant); }
                    Ok('\n')
                }
                Some('\'') => {
                    if chars.next().is_some() { return Err(LexerError::MultiCharacterConstant); }
                    Ok('\'')
                }
                Some('\\') => {
                    if chars.next().is_some() { return Err(LexerError::MultiCharacterConstant); }
                    Ok('\\')
                }
                Some('\"') => {
                    if chars.next().is_some() { return Err(LexerError::MultiCharacterConstant); }
                    Ok('\"')
                }
                Some('x') => {
                    if chars.next() != Some('{') {
                        return Err(LexerError::InvalidEscape);
                    }
                    let mut hex = String::new();
                    while let Some(c) = chars.next() {
                        if c == '}' { break; }
                        hex.push(c);
                    }
                    if chars.next().is_some() { return Err(LexerError::MultiCharacterConstant); }
                    let code = u32::from_str_radix(&hex, 16).map_err(|_| LexerError::InvalidUnicode)?;
                    char::from_u32(code).ok_or(LexerError::InvalidUnicode)
                }
                _ => Err(LexerError::InvalidEscape),
            }
        }
        Some(c) => {
            if chars.next().is_some() {
                return Err(LexerError::MultiCharacterConstant);
            }
            Ok(c)
        }
    }
}

pub fn lex_string(lex: &mut Lexer<Token>) -> Result<String, LexerError> {
    let raw = lex.slice();
    if raw.len() < 2 {
        return Err(LexerError::UnterminatedLiteral);
    }
    let inner = &raw[1..raw.len() - 1];

    let mut result = String::new();
    let mut chars = inner.chars();

    while let Some(c) = chars.next() {
        if c != '\\' {
            result.push(c);
            continue;
        }
        match chars.next() {
            Some('n') => result.push('\n'),
            Some('\'') => result.push('\''),
            Some('\\') => result.push('\\'),
            Some('\"') => result.push('\"'),
            Some('x') => {
                if chars.next() != Some('{') {
                    return Err(LexerError::InvalidEscape);
                }
                let mut hex = String::new();

                while let Some(c) = chars.next() {
                    if c == '}' {
                        break;
                    }
                    hex.push(c);
                }
                let code = u32::from_str_radix(&hex, 16).map_err(|_| LexerError::InvalidUnicode)?;
                let uchar = char::from_u32(code).ok_or(LexerError::InvalidUnicode)?;
                result.push(uchar);
            }
            _ => return Err(LexerError::InvalidEscape),
        }
    }
    Ok(result)
}
