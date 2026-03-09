use clap::Parser;
mod checker;
mod cli;
mod lexer;
mod parser;
mod ir;
// mod typecheck;

use cli::formatter;
use cli::io;
use cli::Args;
use lexer::{LexerExtras, Token};
use logos::Logos;
use parser::adapter::{self, AdapterResult};
use std::process;

fn normalize_args() -> Vec<String> {
    std::env::args()
        .map(|arg| match arg.as_str() {
            "-sourcepath" => "--sourcepath".to_string(),
            "-libpath" => "--libpath".to_string(),
            _ => arg,
        })
        .collect()
}

fn main() {
    let args = Args::parse_from(normalize_args());

    if args.source_files.is_empty() && !args.lex && !args.parse && !args.typecheck {
        let _ = Args::try_parse_from(["etac", "--help"]);
        return;
    }

    if args.lex {
        run_lexer(&args);
    }

    if args.parse {
        run_parser(&args);
    }

    if args.typecheck {
        run_typecheck(&args);
    }
}

fn run_lexer(args: &Args) {
    for source_file in &args.source_files {
        if let Err(e) = process_lex_file(
            source_file,
            args.source_path.as_deref(),
            args.output_dir.as_deref(),
        ) {
            eprintln!("Error processing {}: {}", source_file, e);
            process::exit(1);
        }
    }
}

fn run_parser(args: &Args) {
    for source_file in &args.source_files {
        if let Err(e) = process_parse_file(
            source_file,
            args.source_path.as_deref(),
            args.output_dir.as_deref(),
        ) {
            eprintln!("Error processing {}: {}", source_file, e);
            process::exit(1);
        }
    }
}

fn run_typecheck(args: &Args) {
    for source_file in &args.source_files {
        if let Err(e) = process_typecheck_file(
            source_file,
            args.source_path.as_deref(),
            args.lib_path.as_deref(),
            args.output_dir.as_deref(),
        ) {
            eprintln!("Error processing {}: {}", source_file, e);
            process::exit(1);
        }
    }
}

/// Resolve the actual file path on disk: if `-sourcepath` is given, prepend it.
fn resolve_input_path(file: &str, source_dir: Option<&str>) -> String {
    match source_dir {
        Some(dir) => {
            let p = std::path::Path::new(dir).join(file);
            p.to_string_lossy().into_owned()
        }
        None => file.to_string(),
    }
}

fn process_lex_file(
    source_file: &str,
    source_dir: Option<&str>,
    output_dir: Option<&str>,
) -> Result<(), String> {
    let input_path = resolve_input_path(source_file, source_dir);
    let source =
        io::read_source_file(&input_path).map_err(|e| format!("Failed to read file: {}", e))?;

    let mut lex = Token::lexer_with_extras(&source, LexerExtras::new());
    let tokens = lexer::tokenize(&mut lex);

    let output = formatter::format_lexed_output(&tokens);

    // Output path is based on the original source_file, ignoring -sourcepath
    let output_path = io::compute_output_path(source_file, output_dir, "lexed");

    io::write_output_file(&output_path, &output)
        .map_err(|e| format!("Failed to write output: {}", e))?;

    println!("Lexed {} -> {}", source_file, output_path.display());
    Ok(())
}

fn process_parse_file(
    source_file: &str,
    source_dir: Option<&str>,
    output_dir: Option<&str>,
) -> Result<(), String> {
    let input_path = resolve_input_path(source_file, source_dir);
    let source =
        io::read_source_file(&input_path).map_err(|e| format!("Failed to read file: {}", e))?;

    // lex and convert tokens for the parser
    let tokens = match adapter::lex_for_parser(&source) {
        AdapterResult::Tokens(t) => t,
        AdapterResult::LexError { line, col, msg } => {
            let output = format!("{}:{} error:{}", line, col, msg);
            let output_path = io::compute_output_path(source_file, output_dir, "parsed");
            io::write_output_file(&output_path, &output)
                .map_err(|e| format!("Failed to write output: {}", e))?;
            return Ok(());
        }
    };

    let is_interface = source_file.ends_with(".eti");

    let output = if is_interface {
        // Parse as interface file (.eti)
        let parse_result =
            parser::eta::InterfaceParser::new().parse(tokens.into_iter().map(Ok::<_, String>));
        match parse_result {
            Ok(ast) => parser::sexp::sexp_interface(&ast),
            Err(e) => format_parse_error(&source, e),
        }
    } else {
        // Parse as program file (.eta)
        let parse_result =
            parser::eta::ProgramParser::new().parse(tokens.into_iter().map(Ok::<_, String>));
        match parse_result {
            Ok(ast) => parser::sexp::sexp_program(&ast),
            Err(e) => format_parse_error(&source, e),
        }
    };

    // Output path is based on the original source_file, ignoring -sourcepath
    let output_path = io::compute_output_path(source_file, output_dir, "parsed");
    io::write_output_file(&output_path, &output)
        .map_err(|e| format!("Failed to write output: {}", e))?;

    println!("Parsed {} -> {}", source_file, output_path.display());
    Ok(())
}

fn process_typecheck_file(
    source_file: &str,
    source_dir: Option<&str>,
    lib_dir: Option<&str>,
    output_dir: Option<&str>,
) -> Result<(), String> {
    let input_path = resolve_input_path(source_file, source_dir);
    let source =
        io::read_source_file(&input_path).map_err(|e| format!("Failed to read file: {}", e))?;

    // Lex
    let tokens = match adapter::lex_for_parser(&source) {
        AdapterResult::Tokens(t) => t,
        AdapterResult::LexError { line, col, msg } => {
            let output = format!("{}:{} error:{}", line, col, msg);
            let output_path = io::compute_output_path(source_file, output_dir, "typed");
            io::write_output_file(&output_path, &output)
                .map_err(|e| format!("Failed to write output: {}", e))?;
            return Ok(());
        }
    };

    let is_interface = source_file.ends_with(".eti");

    let output = if is_interface {
        // Parse and typecheck interface file
        let parse_result =
            parser::eta::InterfaceParser::new().parse(tokens.into_iter().map(Ok::<_, String>));
        match parse_result {
            Ok(_ast) => {
                // TODO: interface type checking
                "Valid Eta Program".to_string()
            }
            Err(e) => format_parse_error(&source, e),
        }
    } else {
        // Parse program file
        let parse_result =
            parser::eta::ProgramParser::new().parse(tokens.into_iter().map(Ok::<_, String>));
        match parse_result {
            Ok(ast) => {
                let interfaces = load_interfaces(&ast.uses, source_dir, lib_dir)?;
                let mut tc = checker::check::TypeChecker::new();
                for (_name, iface) in &interfaces {
                    tc.register_interface(iface);
                }
                match tc.check_program(&ast) {
                    Ok(()) => "Valid Eta Program".to_string(),
                    Err(e) => {
                        let (line, col) = byte_offset_to_line_col(&source, e.span.start);
                        format!("{}:{} error:{:?}", line, col, e.kind)
                    }
                }
            }
            Err(e) => format_parse_error(&source, e),
        }
    };

    let output_path = io::compute_output_path(source_file, output_dir, "typed");
    io::write_output_file(&output_path, &output)
        .map_err(|e| format!("Failed to write output: {}", e))?;

    println!("Typechecked {} -> {}", source_file, output_path.display());
    Ok(())
}

/// Load and parse .eti interface files for the given `use` names.
/// Searches in `lib_dir` first, then falls back to `source_dir`, then current directory.
fn load_interfaces(
    uses: &[String],
    source_dir: Option<&str>,
    lib_dir: Option<&str>,
) -> Result<Vec<(String, parser::ast::Interface)>, String> {
    let mut interfaces = Vec::new();

    for name in uses {
        let eti_filename = format!("{}.eti", name);

        // Try libpath first, then sourcepath, then current directory
        let eti_path = if let Some(lib) = lib_dir {
            let p = std::path::Path::new(lib).join(&eti_filename);
            if p.exists() {
                p.to_string_lossy().into_owned()
            } else {
                resolve_input_path(&eti_filename, source_dir)
            }
        } else {
            resolve_input_path(&eti_filename, source_dir)
        };

        let eti_source = io::read_source_file(&eti_path)
            .map_err(|e| format!("Failed to read interface file {}: {}", eti_path, e))?;

        let tokens = match adapter::lex_for_parser(&eti_source) {
            AdapterResult::Tokens(t) => t,
            AdapterResult::LexError { line, col, msg } => {
                return Err(format!(
                    "Lexical error in interface file {}: {}:{} {}",
                    eti_filename, line, col, msg
                ));
            }
        };

        let iface = parser::eta::InterfaceParser::new()
            .parse(tokens.into_iter().map(Ok::<_, String>))
            .map_err(|e| format!("Parse error in interface file {}: {:?}", eti_filename, e))?;

        interfaces.push((name.clone(), iface));
    }

    Ok(interfaces)
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
            token: (start, tok, _),
            ..
        } => {
            let (line, col) = byte_offset_to_line_col(source, start);
            format!(
                "{}:{} error:Unexpected token {}",
                line,
                col,
                token_to_string(&tok)
            )
        }
        ParseError::ExtraToken {
            token: (start, tok, _),
        } => {
            let (line, col) = byte_offset_to_line_col(source, start);
            format!(
                "{}:{} error:Unexpected token {}",
                line,
                col,
                token_to_string(&tok)
            )
        }
        ParseError::User { error } => {
            format!("1:1 error:{}", error)
        }
    }
}

/// Convert a Token to a human-readable string for error messages.
fn token_to_string(tok: &parser::ast::Token) -> String {
    use parser::ast::Token;
    match tok {
        Token::Use => "use".into(),
        Token::If => "if".into(),
        Token::Else => "else".into(),
        Token::While => "while".into(),
        Token::Return => "return".into(),
        Token::Length => "length".into(),
        Token::IntType => "int".into(),
        Token::BoolType => "bool".into(),
        Token::True => "true".into(),
        Token::False => "false".into(),
        Token::LParen => "(".into(),
        Token::RParen => ")".into(),
        Token::LBracket => "[".into(),
        Token::RBracket => "]".into(),
        Token::LBrace => "{".into(),
        Token::RBrace => "}".into(),
        Token::Colon => ":".into(),
        Token::Semicolon => ";".into(),
        Token::Comma => ",".into(),
        Token::Assign => "=".into(),
        Token::Underscore => "_".into(),
        Token::Plus => "+".into(),
        Token::Minus => "-".into(),
        Token::Mul => "*".into(),
        Token::HighMul => "*>>".into(),
        Token::Div => "/".into(),
        Token::Mod => "%".into(),
        Token::Not => "!".into(),
        Token::Lt => "<".into(),
        Token::Le => "<=".into(),
        Token::Gt => ">".into(),
        Token::Ge => ">=".into(),
        Token::Eq => "==".into(),
        Token::Ne => "!=".into(),
        Token::And => "&".into(),
        Token::Or => "|".into(),
        Token::Identifier(s) => s.clone(),
        Token::IntLiteral(n) => n.to_string(),
        Token::CharLiteral(c) => format!("'{}'", char::from_u32(*c as u32).unwrap_or('?')),
        Token::StringLiteral(s) => format!("\"{}\"", s),
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
