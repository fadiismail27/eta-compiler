use clap::Parser;
mod cli;
mod lexer;
mod parser;

use cli::Args;
use cli::formatter;
use cli::io;
use lexer::{LexerExtras, Token};
use logos::Logos;
use parser::adapter::{self, AdapterResult};
use std::process;

fn main() {
    let args = Args::parse();

    if args.source_files.is_empty() && !args.lex && !args.parse {
        let _ = Args::try_parse_from(["etac", "--help"]);
        return;
    }

    if args.lex {
        run_lexer(&args);
    }

    if args.parse {
        run_parser(&args);
    }
}

fn run_lexer(args: &Args) {
    for source_file in &args.source_files {
        if let Err(e) = process_lex_file(source_file, args.output_dir.as_deref()) {
            eprintln!("Error processing {}: {}", source_file, e);
            process::exit(1);
        }
    }
}

fn run_parser(args: &Args) {
    for source_file in &args.source_files {
        if let Err(e) = process_parse_file(source_file, args.output_dir.as_deref()) {
            eprintln!("Error processing {}: {}", source_file, e);
            process::exit(1);
        }
    }
}

fn process_lex_file(source_path: &str, output_dir: Option<&str>) -> Result<(), String> {
    let source =
        io::read_source_file(source_path).map_err(|e| format!("Failed to read file: {}", e))?;

    let mut lex = Token::lexer_with_extras(&source, LexerExtras::new());
    let tokens = lexer::tokenize(&mut lex);

    let output = formatter::format_lexed_output(&tokens);

    let output_path = io::compute_output_path(source_path, output_dir, "lexed");

    io::write_output_file(&output_path, &output)
        .map_err(|e| format!("Failed to write output: {}", e))?;

    println!("Lexed {} -> {}", source_path, output_path.display());
    Ok(())
}

fn process_parse_file(source_path: &str, output_dir: Option<&str>) -> Result<(), String> {
    let source =
        io::read_source_file(source_path).map_err(|e| format!("Failed to read file: {}", e))?;

    // lex and convert tokens for the parser
    let tokens = match adapter::lex_for_parser(&source) {
        AdapterResult::Tokens(t) => t,
        AdapterResult::LexError { line, col, msg } => {
            let output = format!("{}:{} error:{}", line, col, msg);
            let output_path = io::compute_output_path(source_path, output_dir, "parsed");
            io::write_output_file(&output_path, &output)
                .map_err(|e| format!("Failed to write output: {}", e))?;
            return Ok(());
        }
    };

    // parse
    let parse_result =
        parser::eta::ProgramParser::new().parse(tokens.into_iter().map(Ok::<_, String>));

    let output = match parse_result {
        Ok(ast) => {
            // TODO: replace with s-expression printer
            format!("{:?}", ast)
        }
        Err(e) => format_parse_error(&source, e),
    };

    let output_path = io::compute_output_path(source_path, output_dir, "parsed");
    io::write_output_file(&output_path, &output)
        .map_err(|e| format!("Failed to write output: {}", e))?;

    println!("Parsed {} -> {}", source_path, output_path.display());
    Ok(())
}

fn format_parse_error(
    source: &str,
    error: lalrpop_util::ParseError<usize, parser::ast::Token, String>,
) -> String {
    use lalrpop_util::ParseError;

    match error {
        ParseError::InvalidToken { location } => {
            let (line, col) = byte_offset_to_line_col(source, location);
            format!("{}:{} error:Invalid token", line, col)
        }
        ParseError::UnrecognizedEof { location, .. } => {
            let (line, col) = byte_offset_to_line_col(source, location);
            format!("{}:{} error:Unexpected end of file", line, col)
        }
        ParseError::UnrecognizedToken {
            token: (start, _, _),
            ..
        } => {
            let (line, col) = byte_offset_to_line_col(source, start);
            format!("{}:{} error:Unexpected token", line, col)
        }
        ParseError::ExtraToken {
            token: (start, _, _),
        } => {
            let (line, col) = byte_offset_to_line_col(source, start);
            format!("{}:{} error:Extra token", line, col)
        }
        ParseError::User { error } => {
            format!("1:1 error:{}", error)
        }
    }
}

// convert a byte offset in source to (line, col), both 1-indexed
fn byte_offset_to_line_col(source: &str, offset: usize) -> (usize, usize) {
    let mut line = 1;
    let mut line_start = 0;
    for (i, ch) in source.char_indices() {
        if i >= offset {
            break;
        }
        if ch == '\n' {
            line += 1;
            line_start = i + 1;
        }
    }
    let col = offset - line_start + 1;
    (line, col)
}
