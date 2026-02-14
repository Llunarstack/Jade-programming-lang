# J Programming Language

A modern, expressive programming language with powerful features for rapid development.

## Features

- **Type-Safe**: Strong typing with type inference
- **Module System**: Organize code with imports and exports
- **Async/Await**: Built-in concurrency support
- **Traits**: Interface-based polymorphism
- **Rich Type System**: Vectors, matrices, grids, counters, and more
- **Pattern Matching**: Powerful control flow
- **Generators**: Lazy evaluation support

## Quick Start

```bash
# Install
cargo install --path j-lang

# Run a program
j run examples/hello.j

# Start REPL
j repl
```

## Example

```j
// Variables with type annotations
str | name -> "World"
int | count -> 42

// Functions
fn | greet(str | name) > {
    out("Hello, " + name + "!")
}

// Call function
greet(name)
```

## Installation

### From Source
```bash
git clone https://github.com/Llunarstack/j.git
cd j/j-lang
cargo build --release
```

### Using Installers
Pre-built installers available in `j-lang/installers/` for:
- Windows (MSI, EXE)
- macOS (PKG)
- Linux (DEB, RPM)

## Documentation

- [Language Guide](j-lang/README.md)
- [Examples](j-lang/examples/)
- [VS Code Extension](j-lang/vscode-extension/)

## Building

```bash
cd j-lang
cargo build --release
```

## Testing

```bash
cargo test
```

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) first.

## License

MIT License - see [LICENSE](LICENSE) for details.

## Links

- [GitHub Repository](https://github.com/Llunarstack/j)
- [Issue Tracker](https://github.com/Llunarstack/j/issues)
