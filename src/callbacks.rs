use logos::Lexer;
use crate::Token;
use crate::LexerError;

fn newline_callback(lex: &mut Lexer<Token>) -> Skip {
    lex.extras.0 += 1;
    lex.extras.1 = lex.span().end;
    Skip
}
