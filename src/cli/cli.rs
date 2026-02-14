use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "etac")]
#[command(about = "Eta compiler - CS4120/5120/5121 Spring 2026")]
pub struct Args {
    /// Generate lexer output
    #[arg(long = "lex")]
    pub lex: bool,

    /// Generate parser output
    #[arg(long = "parse")]
    pub parse: bool,

    /// Specify output directory for generated files
    #[arg(short = 'D', value_name = "PATH")]
    pub output_dir: Option<String>,

    /// Source files to process (.eta or .eti)
    #[arg(required = false)]
    pub source_files: Vec<String>,
}
