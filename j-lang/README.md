# J Programming Language

A modern, expressive programming language with powerful features for rapid development.

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)]()
[![License](https://img.shields.io/badge/license-MIT-blue)]()
[![Version](https://img.shields.io/badge/version-0.1.0-orange)]()

## Features

- 🚀 **Fast Execution** - Optimized interpreter with JIT compilation support
- 🎯 **Expressive Syntax** - Clean, readable code with powerful operators
- 🔧 **Rich Standard Library** - 300+ built-in functions for common tasks
- 🎨 **Modern Type System** - Static typing with type inference
- 🔄 **Pattern Matching** - Powerful destructuring and matching capabilities
- ⚡ **Async/Await** - First-class support for asynchronous programming
- 📦 **Module System** - Clean imports and exports
- 🎭 **OOP Support** - Classes, inheritance, and traits

## Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/j-lang.git
cd j-lang/j-lang

# Build from source
cargo build --release

# Install (optional)
cargo install --path .
```

### Hello World

```j
out("Hello, World!")
```

### Basic Example

```j
# Variables with type inference
int | x -> 42
str | name -> "Alice"
list | numbers -> [1, 2, 3, 4, 5]

# Functions
fn greet(name: str) -> str {
    "Hello, " + name + "!"
}

# Loops and control flow
for num in numbers {
    if num % 2 == 0 {
        out("Even: " + str(num))
    }
}

# Pattern matching
match x {
    42: out("The answer!")
    n: out("Got: " + str(n))
    _: out("Unknown")
}
```

## Usage

### Run a Program

```bash
j run examples/hello.j
```

### Interactive REPL

```bash
j repl
```

### Compile to Binary

```bash
j build myprogram.j -o myprogram
```

### Run Tests

```bash
cargo test
```

## Project Structure

```
j-lang/
├── src/              # Core implementation
│   ├── lexer.rs      # Tokenization
│   ├── parser.rs     # AST generation
│   ├── interpreter.rs # Runtime execution
│   ├── compiler.rs   # AOT compilation
│   ├── jit.rs        # JIT compilation
│   └── main.rs       # CLI entry point
├── examples/         # Example programs
├── tests/            # Test suite
├── lib/              # Standard library
├── docs/             # Documentation
├── vscode-extension/ # VS Code support
└── installers/       # Platform installers

```

## Documentation

- [Language Features](docs/FEATURES.md) - Complete feature list
- [Standard Library](docs/BUILTIN_ALGORITHMS.md) - Built-in functions
- [Advanced Loops](docs/ADVANCED_LOOPS.md) - Loop constructs
- [Error Handling](docs/ERROR_HANDLING.md) - Error handling guide
- [Crypto Features](docs/CRYPTO_FEATURES.md) - Cryptography support
- [Matrix Operations](docs/MATRIX_FEATURES.md) - Linear algebra

## Development

### Prerequisites

- Rust 1.70+ 
- Cargo

### Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run with optimizations
cargo run --release -- run examples/hello.j
```

### Code Quality

```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Run all tests
cargo test

# Run specific test
cargo test test_name
```

### Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Editor Support

### VS Code

Install the J Language extension from `vscode-extension/`:

```bash
cd vscode-extension
npm install
npm run compile
```

Features:
- Syntax highlighting
- Code snippets
- IntelliSense
- Error diagnostics

## Performance

J is designed for performance:

- Optimized interpreter with minimal overhead
- JIT compilation for hot code paths
- Efficient memory management
- Zero-cost abstractions where possible

## Roadmap

- [x] Core language features
- [x] Standard library (300+ functions)
- [x] Pattern matching
- [x] Async/await
- [x] Module system
- [ ] Package manager
- [ ] LSP server
- [ ] Debugger
- [ ] WASM target
- [ ] Self-hosting compiler

## License

This project is licensed under the MIT License - see the [LICENSE](../LICENSE) file for details.

## Acknowledgments

Inspired by modern languages like Rust, Python, and Swift, J aims to combine the best features of each into a cohesive, productive language.

## Community

- GitHub Issues: Bug reports and feature requests
- Discussions: Questions and community chat

---

**Note:** J is under active development. APIs may change between versions.
