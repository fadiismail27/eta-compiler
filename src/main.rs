use logos::{Logos, Lexer};

#[derive(Debug, Clone, PartialEq, Default)] 
pub enum LexerError {
    #[default]
    InvalidCharacter,    
    InvalidUnicode,   
    UnterminatedLiteral,    
    InvalidEscape,          
    InvalidHex,             
    EmptyCharacter,        
    MultiCharacterConstant, 
}

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(error = LexerError)]
pub enum Token {
		// AI SLOP 
    Id(String),
    Integer(String),
    CharLiteral(String),
    StringLiteral(String),
    Symbol(String),
    Keyword(String),
    Error(String),
    
    // ALT ORGANIZATION
    
    Ident(String),      // identifier name, e.g., "x"
    Int(i64),           // integer literal, e.g., 42
    Char(char),         // character literal, e.g., 'a'
    String(String),     // string literal, e.g., "hello"
    
    // These can directly be mapped to operator tokens (
    Mult,
    HighMult,
    Divide,
    Amp,
    Plus,
    Minus,
    
    LessThan,
    LessEq,
    GreaterThan,
    GreaterEqual,
    Equal,
    NotEqual,
    And,
    Or,
    Not,
    
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Semicolon,
    Colon,
}

/// Called by main lexer when it sees a single quote.
/// `source` is the full source chars, `pos` is the index of the opening quote.
/// Returns the token and the index to resume lexing from.
pub fn lex_char(lex: &mut logos::Lexer<Token>) -> Result<char, LexerError> {
    Ok('h')
}

pub fn lex_string(lex: &mut logos::Lexer<Token>) -> Result<String, LexerError> {
    let raw = &lex.slice();
    let inner = &raw[1..raw.len() - 1];

    let mut result = String::new();
    let mut chars = inner.chars();

    while let Some(c) = chars.next() {
        if c != '\\' {
            result.push(c);
            continue;
        }
        match chars.next() {
            Some('n') => result.push('n'),
            Some('\'') => result.push('\''),
            Some('\\') => result.push('\\'),
            Some('"') => result.push('"'),
            Some('x') => {
                if chars.next() != Some('{') {
                    return Err(LexerError::InvalidEscape);
                }
                let mut hex = String::new();

                while let Some(c) = chars.next() {
                    if c == '}' {break;}
                    hex.push(c);
                }
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
    Ok(result)
}

fn main() {
    println!("Hello, world!");
}
