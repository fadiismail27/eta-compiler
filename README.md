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
# Parse a .eta or .eti file
./etac --parse <filename>.eta
./etac --parse <filename>.eti

# Parse with output directory and source path
./etac --parse -D <outdir> --sourcepath <srcdir> <files...>

# Lex a file (PA1)
./etac --lex <filename>.eta

# Specify output directory
./etac --lex -D <output_dir> <filename>.eta
```

## Testing

```bash
# Run all PA2 tests and check against expected output
./etac-build && for f in eth/tests/pa2/*.eta eth/tests/pa2/*.eti; do ./etac --parse "$f" 2>/dev/null; done && PASS=0 && FAIL=0 && for f in eth/tests/pa2/*.parsedsol; do base="${f%.parsedsol}"; if diff -q "$base.parsed" "$f" > /dev/null 2>&1; then PASS=$((PASS+1)); else echo "FAIL: $base"; FAIL=$((FAIL+1)); fi; done && echo "$PASS passed, $FAIL failed" && rm -f eth/tests/pa2/*.parsed

# Run all PA1 tests
for f in eth/tests/pa1/*.eta; do ./etac --lex "$f"; done && \
for f in eth/tests/pa1/*.eta; do \
  base="${f%.eta}"; \
  if [ -f "$base.lexedsol" ]; then \
    diff "$base.lexed" "$base.lexedsol" > /dev/null || echo "FAIL: $f"; \
  fi; \
done && echo "Done"

# Run unit tests
cargo test
```

## Project Structure

```
src/
├── main.rs              # Entry point, CLI dispatch
├── lib.rs               # Module declarations
├── cli/
│   ├── cli.rs           # Command-line argument parsing (clap)
│   ├── io.rs            # File I/O operations
│   └── formatter.rs     # Lexer output formatting (.lexed files)
├── lexer/
│   ├── token.rs         # Logos token definitions, tokenize(), LexerExtras
│   └── callbacks.rs     # String/char literal parsing callbacks
├── parser/
│   ├── ast.rs           # AST node types, parser Token enum
│   ├── eta.lalrpop      # LALRPOP grammar for Eta programs and interfaces
│   └── adapter.rs       # Logos-to-LALRPOP token conversion
├── sexp.rs              # S-expression printer for AST output
├── sexp_tests.rs        # S-expression printer unit tests
├── pretty.rs            # Source-code pretty printer
└── pretty_tests.rs      # Pretty printer unit tests
```

## Documentation

See `overview.md` for full design documentation.
