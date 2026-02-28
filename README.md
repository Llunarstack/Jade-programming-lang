<p align="center">
  <img src="assets/jade.png" width="120" alt="Jade logo">
</p>

<h1 align="center">Jade</h1>

<p align="center">
  <strong>An expressive language for data and algorithms.</strong>
</p>

<p align="center">
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License: MIT"></a>
  <a href="https://github.com/Llunarstack/j/actions/workflows/release.yml"><img src="https://github.com/Llunarstack/j/actions/workflows/release.yml/badge.svg" alt="CI"></a>
  <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/rust-1.70%2B-orange.svg" alt="Rust 1.70+"></a>
  <a href="CHANGELOG.md"><img src="https://img.shields.io/badge/version-1.0.0-green.svg" alt="Version 1.0.0"></a>
</p>

---

## What is Jade?

Jade is a **memory-safe**, **readable** programming language and toolchain built for data-heavy and algorithmic code. One binary: interpreter, REPL, package manager, and VS Code support.

```jdl
fn | greet ( str | name ) > {
  out("Hello, " + name)
}

greet("Jade")
int: total = 0
for x in [1, 2, 3] { total = total + x }
out("Sum: " + total)
```

---

## Features

| | |
|---|---|
| **Readable** | Pipelines (`\|>`), pattern matching, `cond`/`when`, lambdas. Typed variables and classes without boilerplate. |
| **Algorithm-friendly** | Loops that match how you think: `sweep`, `meet`, binary-search, `while_nonzero`. Rich stdlib: graphs, deques, priority queues, matrices, `gcd`/`lcm`, `bfs`/`dfs`. |
| **One toolchain** | Run scripts (`jade file.jdl`), REPL (`jade repl`), or structured projects with the Jolt package manager. |
| **Editor support** | VS Code & Cursor: syntax, run-from-buffer (no save), debounced autosave. Optional .jdl file association and icon. |

---

## Install

### Windows

Download the installer from [Releases](https://github.com/Llunarstack/j/releases) or build locally:

```powershell
cd jade-lang
cargo build --release
.\installers\windows\build-exe.ps1
# → dist\installers\windows\jade-1.0.0-windows-x86_64-setup.exe
```

### macOS / Linux

```bash
git clone https://github.com/Llunarstack/j.git
cd j/jade-lang
cargo build --release
./installers/linux/install.sh    # Linux
# or
./installers/macos/install.sh   # macOS
```

### From source (any OS)

```bash
cd jade-lang
cargo build --release
# Binary: target/release/jade
# Add to PATH or: cargo install --path .
```

Full install options (portable zip, MSI, IDE setup): **[docs/INSTALL.md](docs/INSTALL.md)**.

---

## Quick start

```bash
# Create a file
echo 'out("Hi from Jade!")' > hello.jdl

# Run it
jade hello.jdl
# → Hi from Jade!

# REPL
jade repl
```

---

## Project layout

| Path | What |
|------|------|
| [jade-lang/](jade-lang/) | Rust crate: interpreter, compiler, REPL, Jolt |
| [jade-lang/installers/](jade-lang/installers/) | Windows (Inno, MSI, portable), Linux, macOS, IDE extension |
| [docs/](docs/README.md) | [INSTALL](docs/INSTALL.md) · [BOOTSTRAP](docs/BOOTSTRAP.md) · [CONTRIBUTING](docs/CONTRIBUTING.md) · [Structure](docs/PROJECT_STRUCTURE.md) |
| [bootstrap/](bootstrap/) | Jade scripts that process Jade source (bootstrapping) |

---

## Documentation

- **Install & run:** [docs/INSTALL.md](docs/INSTALL.md)
- **Bootstrapping:** [docs/BOOTSTRAP.md](docs/BOOTSTRAP.md)
- **Contributing:** [docs/CONTRIBUTING.md](docs/CONTRIBUTING.md) · [Code of Conduct](docs/CODE_OF_CONDUCT.md)
- **Changelog:** [CHANGELOG.md](CHANGELOG.md)

---

## Contributing

Contributions are welcome. Open an [issue](https://github.com/Llunarstack/j/issues) or see [docs/CONTRIBUTING.md](docs/CONTRIBUTING.md).

---

## License

MIT — see [LICENSE](LICENSE). The name and logo **Jade** are used for this project; to use them elsewhere, please [open an issue](https://github.com/Llunarstack/j/issues) to discuss.
