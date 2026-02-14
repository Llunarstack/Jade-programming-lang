# The J Programming Language

This is the main source code repository for J. It contains the compiler, standard library, and documentation.

## Why J?

**Built for Competitive Programming and Data Problems:** Native support for Counters, Grids with neighbor queries, Intervals, Deques, Priority Queues, Graphs, and Trees. Solve algorithmic problems with less boilerplate.

**Unique Type System:** First-class support for specialized types including Emoji, Money (with currency), Hex literals, Date/Time, and mathematical Infinity. Express domain concepts directly in code.

**Advanced Grid Operations:** Built-in grid.neighbors(), grid.neighbors8(), grid.find_all(), grid.row(), and grid.col() methods. Perfect for pathfinding, game development, and spatial algorithms.

**Powerful Iteration:** Window operations, scan operations (scan_max, scan_sum), broadcast operations, and pipeline operators. Transform data with expressive syntax.

**Counter Arithmetic:** Add, subtract, and compare frequency counters directly. Built-in most_common() and total() methods for statistical operations.

**Productivity Features:** Defer statements for cleanup, converge loops for fixed-point iteration, pattern matching, and comprehensive error handling.

## Quick Start

Read the [Language Guide](j-lang/README.md) to learn J.

## Installing from Source

**Note:** If you wish to contribute or modify J, building from source is recommended.

### Prerequisites
- Rust 1.70 or higher
- Cargo

### Build and Install
```bash
git clone https://github.com/Llunarstack/j.git
cd j/j-lang
cargo build --release
cargo install --path .
```

### Platform-Specific Installers
Pre-built installers are available in [j-lang/installers/](j-lang/installers/) for Windows, macOS, and Linux.

## Getting Help

See the [examples directory](j-lang/examples/) for sample programs and tutorials.

For questions and discussions, open an issue on [GitHub Issues](https://github.com/Llunarstack/j/issues).

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for contribution guidelines.

## License

J is distributed under the terms of the MIT license.

See [LICENSE](LICENSE) for details.

## Trademark

The J language name and logo are trademarks. If you want to use these names or brands, please open an issue to discuss.
