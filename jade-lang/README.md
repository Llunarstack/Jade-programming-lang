# Jade Programming Language

Jade is an experimental programming language and toolchain focused on expressive syntax and practical tooling. This repository contains the compiler/interpreter, standard library, examples, and editor support.

## Status

Version: 1.0.0. See [CHANGELOG](../CHANGELOG.md) for updates.

## Features (Implemented or In Progress)

- Interpreter with a CLI entry point
- Module system and standard library
- Pattern matching and structured control flow
- Async/await support (feature-gated in Rust build)
- VS Code extension for syntax highlighting

## Quick Start

### Build From Source

```bash
cd jade-lang
cargo build --release
```

### Run a file

```bash
jade main.jdl
# or
cargo run -- run main.jdl
```

### Install locally

```bash
cargo install --path .
```

### REPL

```bash
jade
# or
cargo run -- repl
```

## Project structure

```
jade-lang/
├── src/               # Core (lexer, parser, interpreter, JIT, jolt)
├── installers/        # Windows (MSI, exe), Linux, macOS
├── tests/             # Integration tests
└── Cargo.toml
```

## Documentation

- **`jade --help`** — CLI usage
- **`jade jolt --help`** — Package manager
- See repo root [README](../README.md) and [INSTALL.md](../INSTALL.md) for more.

## Development

### Prerequisites

- Rust 1.70+
- Cargo

### Code Quality

```bash
cargo fmt
cargo clippy
cargo test
```

## Contributing

See `../CONTRIBUTING.md` for contribution guidelines.

## License

MIT. See `../LICENSE`.
