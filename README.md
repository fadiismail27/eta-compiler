# Compiler

CS 4120 Spring 2026

## Team

- Fadi Ismail (fmi4)
- Yoseph Endawoke (ye38)
- Charles Liggins (cll258)
- Temi Adebowale (ta375)

## Building

```bash
./etac-build
```

## Usage

```bash
# Type check a .eta file (PA3)
./etac --typecheck <filename>.eta

# Type check with library path for interface files
./etac --typecheck -libpath <libdir> <filename>.eta

# Type check with all options
./etac --typecheck -libpath <libdir> -D <outdir> --sourcepath <srcdir> <files...>

# Parse a .eta or .eti file (PA2)
./etac --parse <filename>.eta
./etac --parse <filename>.eti

# Lex a file (PA1)
./etac --lex <filename>.eta

# Specify output directory for any mode
./etac --lex -D <output_dir> <filename>.eta
```

## Testing

```bash
# Run unit tests (context, s-expression, pretty printer)
cargo test

# Run PA3 typecheck tests via eth (inside Docker)
cd ~/eth/tests/pa3 && eth -compilerpath ~/shared ethScript

# Run PA2 parse tests via eth
cd ~/eth/tests/pa2 && eth -compilerpath ~/shared ethScript

# Run PA1 lex tests via eth
cd ~/eth/tests/pa1 && eth -compilerpath ~/shared ethScript
```

## Project Structure

```
src/
├── main.rs                # Entry point, CLI dispatch, typecheck pipeline
├── cli/
│   ├── cli.rs             # Command-line argument parsing (clap)
│   ├── io.rs              # File I/O operations
│   └── formatter.rs       # Lexer output formatting (.lexed files)
├── lexer/
│   ├── token.rs           # Logos token definitions, tokenize(), LexerExtras
│   └── callbacks.rs       # String/char literal parsing callbacks
├── parser/
│   ├── ast.rs             # AST types, Span/Spanned<T>, Token enum
│   ├── eta.lalrpop        # LALRPOP grammar with @L/@R span captures
│   └── adapter.rs         # Logos-to-LALRPOP token conversion
├── checker/
│   ├── check.rs           # Type checker (expressions, statements, functions)
│   ├── context.rs         # Scoped symbol table and Context
│   └── context_tests.rs   # Context unit tests
├── sexp.rs                # S-expression printer for AST output
├── sexp_tests.rs          # S-expression printer unit tests
├── pretty.rs              # Source-code pretty printer
└── pretty_tests.rs        # Pretty printer unit tests
```

## Documentation

See `overview.md` for full PA3 design documentation.
