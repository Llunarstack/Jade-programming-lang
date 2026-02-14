# J Language Implementation

Core implementation of the J programming language interpreter, compiler, and runtime.

## Architecture

- **Lexer** (`src/lexer.rs`) - Tokenization
- **Parser** (`src/parser.rs`) - AST generation
- **Interpreter** (`src/interpreter.rs`) - Runtime execution
- **Compiler** (`src/compiler.rs`) - AOT compilation
- **JIT** (`src/jit.rs`) - Just-in-time compilation
- **Runtime** (`src/runtime.rs`) - Runtime support

## Building

```bash
cargo build --release
```

## Running

```bash
# Run a file
cargo run -- run examples/basic/hello.j

# Start REPL
cargo run -- repl

# Compile to binary
cargo run -- build examples/basic/hello.j -o hello
```

## Examples

See the `examples/` directory for code samples:
- `basic/` - Basic language features
- `advanced/` - Advanced features
- `modules/` - Module system examples
- `tests/` - Test files

## Development

```bash
# Run tests
cargo test

# Check code
cargo clippy

# Format code
cargo fmt
```

## Features Implemented

✅ Variables and types
✅ Functions and closures
✅ Classes and OOP
✅ Module system
✅ Trait system
✅ Async/await
✅ Pattern matching
✅ Generators
✅ Rich standard library

## VS Code Extension

A VS Code extension is available in `vscode-extension/` with:
- Syntax highlighting
- Code snippets
- Language configuration
- Themes

## License

MIT License - see [LICENSE](../LICENSE)
