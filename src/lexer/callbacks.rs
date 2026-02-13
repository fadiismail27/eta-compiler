use logos::{Lexer, Skip};
use crate::lexer::token::{Token, LexerError};

pub fn newline_callback(lex: &mut Lexer<Token>) -> Skip {
    lex.extras.line += 1;
    lex.extras.line_start = lex.span().end;
    lex.extras.has_token = false;
    lex.extras.saw_comment = false;
    Skip
}

pub fn comment_callback(lex: &mut Lexer<Token>) -> Skip {
    lex.extras.saw_comment = true;
    Skip
}

/// Called by main lexer when it sees a single quote.
/// `source` is the full source chars, `pos` is the index of the opening quote.
/// Returns the token and the index to resume lexing from.
pub fn lex_char(lex: &mut Lexer<Token>) -> Result<char, LexerError> {
    let remainder = lex.remainder();
    let mut consumed = 0;
    let mut chars = remainder.chars();
    
    // Parse the content until closing quote
    let result = match chars.next() {
        None | Some('\n') => return Err(LexerError::UnterminatedLiteral(consumed)),
        Some('\'') => {
            consumed += 1;
            lex.bump(consumed);
            return Err(LexerError::EmptyCharacter(consumed));
        }
        Some('\\') => {
            consumed += 1;
            match chars.next() {
                None | Some('\n') => return Err(LexerError::UnterminatedLiteral(consumed)),
                Some('n') => { consumed += 1; '\n' }
                Some('t') => { consumed += 1; '\t' }
                Some('\'') => { consumed += 1; '\'' }
                Some('\\') => { consumed += 1; '\\' }
                Some('"') => { consumed += 1; '"' }
                Some('x') => {
                    consumed += 1;
                    if chars.next() != Some('{') {
                        return Err(LexerError::InvalidEscape(consumed));
                    }
                    consumed += 1;
                    let mut hex = String::new();
                    loop {
                        match chars.next() {
                            None | Some('\n') => return Err(LexerError::UnterminatedLiteral(consumed)),
                            Some('}') => { consumed += 1; break; }
                            Some(c) => { consumed += c.len_utf8(); hex.push(c); }
                        }
                    }
                    let code = u32::from_str_radix(&hex, 16).map_err(|_| LexerError::InvalidHex(consumed))?;
                    char::from_u32(code).ok_or(LexerError::InvalidHex(consumed))?
                }
                _ => return Err(LexerError::InvalidEscape(consumed)),
            }
        }
        Some(c) => {
            consumed += c.len_utf8();
            c
        }
    };
    
    // Expect closing quote
    match chars.next() {
        Some('\'') => {
            consumed += 1;
            lex.bump(consumed);
            Ok(result)
        }
        Some('\n') | None => Err(LexerError::UnterminatedLiteral(consumed)),
        Some(_) => Err(LexerError::MultiCharacterConstant(consumed)),
    }
}

pub fn lex_string(lex: &mut Lexer<Token>) -> Result<String, LexerError> {
    let remainder = lex.remainder();
    let mut consumed = 0;
    let mut result = String::new();
    let mut chars = remainder.chars();

    loop {
        match chars.next() {
            None | Some('\n') => return Err(LexerError::UnterminatedLiteral(consumed)),
            Some('"') => {
                consumed += 1;
                lex.bump(consumed);
                return Ok(result);
            }
            Some('\\') => {
                consumed += 1;
                match chars.next() {
                    None | Some('\n') => return Err(LexerError::UnterminatedLiteral(consumed)),
                    Some('n') => { consumed += 1; result.push('\n'); }
                    Some('t') => { consumed += 1; result.push('\t'); }
                    Some('\'') => { consumed += 1; result.push('\''); }
                    Some('\\') => { consumed += 1; result.push('\\'); }
                    Some('"') => { consumed += 1; result.push('"'); }
                    Some('x') => {
                        consumed += 1;
                        if chars.next() != Some('{') {
                            return Err(LexerError::InvalidEscape(consumed));
                        }
                        consumed += 1;
                        let mut hex = String::new();
                        loop {
                            match chars.next() {
                                None | Some('\n') => return Err(LexerError::UnterminatedLiteral(consumed)),
                                Some('}') => { consumed += 1; break; }
                                Some(c) => { consumed += c.len_utf8(); hex.push(c); }
                            }
                        }
                        let code = u32::from_str_radix(&hex, 16).map_err(|_| LexerError::InvalidHex(consumed))?;
                        let uchar = char::from_u32(code).ok_or(LexerError::InvalidHex(consumed))?;
                        result.push(uchar);
                    }
                    _ => return Err(LexerError::InvalidEscape(consumed)),
                }
            }
            Some(c) => {
                consumed += c.len_utf8();
                result.push(c);
            }
        }
    }
}
