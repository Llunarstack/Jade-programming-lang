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

Jade is a **modern, memory-safe** programming language crafted for clean, expressive code—especially when you're working with data, algorithms, concurrency, or tools. It feels **concise and readable** while giving you powerful, low-noise abstractions:

- **Values flow naturally through pipelines:** `x |> map(_ * 2) |> filter(_ > 0) |> sum()`
- **cond** and **when** let a value travel through branches until one matches:  
  `cond(x) { |> x == 42 : "the answer" |> else : "keep looking" }`
- **either** is a pipeline-friendly ternary/Result handler that picks the first valid path
- **Deep pattern matching with guards:** `match point { (x,y) if x == y : "diagonal" }`
- **Broadcast calls:** `fn.(list, scalar)` applies a function element-wise or with broadcasting
- **borrow_split(list, i) |> left, right { parallel { left.map(...); right.map(...) } }** splits mutable borrows safely so you can work on both halves concurrently without locks
- **converge** loop runs until the body's result stabilizes (great for fixed-point computations, iterative solvers, cellular automata)
- **Loops are rich and varied:** indexed, reverse, step, zip, parallel, chunked, sweep, meet, while_nonzero, while_change, etc.
- **First-class support** for graphs (BFS, DFS, Dijkstra, topological sort), deques, priority queues, union-find, tries, ring buffers, sorted lists, sliding windows, and more
- **Security is first-class:** enc&lt;T&gt; for encrypted values, secret&lt;T&gt; for redacted secrets, **zeroize** for secure wiping
- **Memoization is trivial:** `memo(f)` in stdlib
- **algo** module provides lower_bound, upper_bound, prefix_sum, kadane, merge_sorted, sliding_window, two_pointers_sum, flood_fill, and other classic helpers

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

More options (portable zip, MSI): **[docs/INSTALL.md](docs/INSTALL.md)**.

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
| [jade-lang/installers/](jade-lang/installers/) | Windows (Inno, MSI, portable), Linux, macOS |
| [docs/](docs/README.md) | [INSTALL](docs/INSTALL.md) · [CONTRIBUTING](docs/CONTRIBUTING.md) · [Structure](docs/PROJECT_STRUCTURE.md) |
| [bootstrap/](bootstrap/) | Jade written in Jade: scripts that process Jade source |

---

## Documentation

- **Install & run:** [docs/INSTALL.md](docs/INSTALL.md)
- **Contributing:** [docs/CONTRIBUTING.md](docs/CONTRIBUTING.md) · [Code of Conduct](docs/CODE_OF_CONDUCT.md)
- **Changelog:** [CHANGELOG.md](CHANGELOG.md)

---

## Contributing

Contributions are welcome. Open an [issue](https://github.com/Llunarstack/j/issues) or see [docs/CONTRIBUTING.md](docs/CONTRIBUTING.md).

---

## License

MIT — see [LICENSE](LICENSE). The name and logo **Jade** are used for this project; to use them elsewhere, please [open an issue](https://github.com/Llunarstack/j/issues) to discuss.
