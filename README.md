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
# Run the lexer on a specific .eta file
./etac --lex <filename>.eta

# Specify output directory
./etac --lex -D <output_dir> <filename>.eta
```

## Testing

```bash
# Run the lexer on all PA1 test files
for f in eth/tests/pa1/*.eta; do ./etac --lex "$f"; done

# Run all PA1 tests and check against solutions
for f in eth/tests/pa1/*.eta; do ./etac --lex "$f"; done && \
for f in eth/tests/pa1/*.eta; do \
  base="${f%.eta}"; \
  if [ -f "$base.lexedsol" ]; then \
    diff "$base.lexed" "$base.lexedsol" > /dev/null || echo "FAIL: $f"; \
  fi; \
done && echo "Done"
```

## Project Structure

```
src/
├── main.rs        # Entry point
├── lib.rs         # Module declarations
├── cli.rs         # Command-line argument parsing
├── token.rs       # Token enum, LexerExtras, LexResult, LexerError
├── callbacks.rs   # Lexer callbacks for strings, chars, newlines, comments
├── formatter.rs   # Output formatting for .lexed files
└── io.rs          # File I/O operations
```

## Documentation

See `overview.md` for full design documentation.
