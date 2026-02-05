use clap::Parser;
use compiler::cli::Args;
use compiler::formatter;
use compiler::io;
use std::process;
use compiler::token::{self, Token, LexerExtras};
use logos::Logos;

fn main() {
    let args = Args::parse();

    // If no source files and no action flags, print help
    if args.source_files.is_empty() && !args.lex {
        // Re-parse with --help to show usage
        let _ = Args::try_parse_from(["etac", "--help"]);
        return;
    }

    if args.lex {
        run_lexer(&args);
    }
}

fn run_lexer(args: &Args) {
    for source_file in &args.source_files {
        if let Err(e) = process_file(source_file, args.output_dir.as_deref()) {
            eprintln!("Error processing {}: {}", source_file, e);
            process::exit(1);
        }
    }
}

fn process_file(source_path: &str, output_dir: Option<&str>) -> Result<(), String> {
    // 1. Read source file
    let source = io::read_source_file(source_path)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    // 2. Call lexer
    let mut lex = Token::lexer_with_extras(&source, LexerExtras::new());
    let tokens = token::tokenize(&mut lex);

    // 3. Format tokens
    let output = formatter::format_lexed_output(&tokens);

    // 4. Compute output path
    let output_path = io::compute_output_path(source_path, output_dir);

    // 5. Write .lexed file
    io::write_lexed_file(&output_path, &output)
        .map_err(|e| format!("Failed to write output: {}", e))?;

    println!("Lexed {} -> {}", source_path, output_path.display());
    Ok(())
}
