use clap::Parser;
use compiler::cli::Args;

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
        println!("Would lex: {}", source_file);
        // If the user provided an output directory, print it. 
        // Unwrpas value and binds to dir 
        if let Some(ref dir) = args.output_dir {
            println!("  Output dir: {}", dir);
        }
    }
}
