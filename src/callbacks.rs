use logos::{Lexer, Filter};
use crate::token::{Token, LexerError};

pub fn newline_callback(lex: &mut Lexer<Token>) -> Filter<()> {
    lex.extras.line += 1;
    lex.extras.line_start = lex.span().end;
    lex.extras.has_token = false;
    lex.extras.saw_comment = false;
    Filter::Skip
}

pub fn comment_callback(lex: &mut Lexer<Token>) -> Filter<()> {
    lex.extras.saw_comment = true;
    Filter::Skip
}

/// Called by main lexer when it sees a single quote.
/// `source` is the full source chars, `pos` is the index of the opening quote.
/// Returns the token and the index to resume lexing from.
pub fn lex_char(lex: &mut Lexer<Token>) -> Filter<char> {
    let remainder = lex.remainder();
    let mut consumed = 0;
    let mut chars = remainder.chars();

    // Parse the content until closing quote
    let result = match chars.next() {
        None | Some('\n') => {
            lex.extras.error = Some(LexerError::UnterminatedLiteral);
            return Filter::Emit('\0');
        }
        Some('\'') => {
            consumed += 1;
            lex.bump(consumed);
            lex.extras.error = Some(LexerError::EmptyCharacter);
            return Filter::Emit('\0');
        }
        Some('\\') => {
            consumed += 1;
            match chars.next() {
                None | Some('\n') => {
                    lex.extras.error = Some(LexerError::UnterminatedLiteral);
                    return Filter::Emit('\0');
                }
                Some('n') => { consumed += 1; '\n' }
                Some('\'') => { consumed += 1; '\'' }
                Some('\\') => { consumed += 1; '\\' }
                Some('"') => { consumed += 1; '"' }
                Some('x') => {
                    consumed += 1;
                    if chars.next() != Some('{') {
                        lex.extras.error = Some(LexerError::InvalidEscape);
                        return Filter::Emit('\0');
                    }
                    consumed += 1;
                    let mut hex = String::new();
                    loop {
                        match chars.next() {
                            None | Some('\n') => {
                                lex.extras.error = Some(LexerError::UnterminatedLiteral);
                                return Filter::Emit('\0');
                            }
                            Some('}') => { consumed += 1; break; }
                            Some(c) => { consumed += c.len_utf8(); hex.push(c); }
                        }
                    }
                    match u32::from_str_radix(&hex, 16) {
                        Ok(code) => {
                            match char::from_u32(code) {
                                Some(c) => c,
                                None => {
                                    lex.extras.error = Some(LexerError::InvalidHex);
                                    return Filter::Emit('\0');
                                }
                            }
                        }
                        Err(_) => {
                            lex.extras.error = Some(LexerError::InvalidHex);
                            return Filter::Emit('\0');
                        }
                    }
                }
                _ => {
                    lex.extras.error = Some(LexerError::InvalidEscape);
                    return Filter::Emit('\0');
                }
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
            Filter::Emit(result)
        }
        Some('\n') | None => {
            lex.extras.error = Some(LexerError::UnterminatedLiteral);
            Filter::Emit('\0')
        }
        Some(_) => {
            lex.extras.error = Some(LexerError::MultiCharacterConstant);
            Filter::Emit('\0')
        }
    }
}

pub fn lex_string(lex: &mut Lexer<Token>) -> Filter<String> {
    let remainder = lex.remainder();
    let mut consumed = 0;
    let mut result = String::new();
    let mut chars = remainder.chars();

    loop {
        match chars.next() {
            None | Some('\n') => {
                lex.extras.error = Some(LexerError::UnterminatedLiteral);
                return Filter::Emit(String::new());
            }
            Some('"') => {
                consumed += 1;
                lex.bump(consumed);
                return Filter::Emit(result);
            }
            Some('\\') => {
                consumed += 1;
                match chars.next() {
                    None | Some('\n') => {
                        lex.extras.error = Some(LexerError::UnterminatedLiteral);
                        return Filter::Emit(String::new());
                    }
                    Some('n') => { consumed += 1; result.push('\n'); }
                    Some('\'') => { consumed += 1; result.push('\''); }
                    Some('\\') => { consumed += 1; result.push('\\'); }
                    Some('"') => { consumed += 1; result.push('"'); }
                    Some('x') => {
                        consumed += 1;
                        if chars.next() != Some('{') {
                            lex.extras.error = Some(LexerError::InvalidEscape);
                            return Filter::Emit(String::new());
                        }
                        consumed += 1;
                        let mut hex = String::new();
                        loop {
                            match chars.next() {
                                None | Some('\n') => {
                                    lex.extras.error = Some(LexerError::UnterminatedLiteral);
                                    return Filter::Emit(String::new());
                                }
                                Some('}') => { consumed += 1; break; }
                                Some(c) => { consumed += c.len_utf8(); hex.push(c); }
                            }
                        }
                        match u32::from_str_radix(&hex, 16) {
                            Ok(code) => {
                                match char::from_u32(code) {
                                    Some(uchar) => result.push(uchar),
                                    None => {
                                        lex.extras.error = Some(LexerError::InvalidHex);
                                        return Filter::Emit(String::new());
                                    }
                                }
                            }
                            Err(_) => {
                                lex.extras.error = Some(LexerError::InvalidHex);
                                return Filter::Emit(String::new());
                            }
                        }
                    }
                    _ => {
                        lex.extras.error = Some(LexerError::InvalidEscape);
                        return Filter::Emit(String::new());
                    }
                }
            }
            Some(c) => {
                consumed += c.len_utf8();
                result.push(c);
            }
        }
    }
}
