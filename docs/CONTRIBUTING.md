# Contributing to Jade

Thank you for contributing to the Jade programming language. This guide will help you get started.

## Table of contents

- [Code of conduct](#code-of-conduct)
- [Getting started](#getting-started)
- [Development setup](#development-setup)
- [Making changes](#making-changes)
- [Submitting changes](#submitting-changes)
- [Reporting issues](#reporting-issues)

## Code of conduct

This project adheres to the [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you agree to uphold it.

## Getting started

1. **Fork** the repository on GitHub.
2. **Clone** your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/j.git
   cd j
   ```
   (Replace `j` with the actual repo name if different.)
3. **Create a branch:**
   ```bash
   git checkout -b feature/your-feature-name
   ```
   Use a short prefix: `feature/`, `fix/`, `docs/`, `refactor/`, `test/`.

## Development setup

### Prerequisites

- **Rust** 1.70 or higher  
  Install from [rustup.rs](https://rustup.rs).
- **Cargo** (included with Rust)
- **Git**

### Build and test

All commands below are run from the **`jade-lang`** directory (the main crate).

```bash
cd jade-lang
cargo build
cargo test
```

### Code quality

```bash
cargo fmt
cargo clippy
```

Run `cargo fmt --check`, `cargo clippy`, and `cargo test` before submitting.

### Project structure

| Purpose     | Path                    |
|------------|-------------------------|
| Source     | `jade-lang/src/`        |
| Installers | `jade-lang/installers/`  |
| Tests      | `jade-lang/tests/`      |

## Making changes

### Commit messages

Use [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation only
- `style:` Formatting, no logic change
- `refactor:` Code change that is not a fix or feature
- `test:` Adding or updating tests
- `chore:` Build, tooling, or maintenance

Example: `feat: add pipeline operator for list comprehensions`

### Code guidelines

- **Rust:** Follow standard naming and style. Add doc comments for public APIs. Prefer small, focused functions.
- **Jade code (examples/tests):** Use clear names and comments; keep examples runnable.

## Submitting changes

1. Ensure tests pass and the code is formatted:
   ```bash
   cd jade-lang && cargo fmt && cargo clippy && cargo test
   ```
2. Update **CHANGELOG.md** (at repo root) for user-visible changes.
3. Push your branch and open a **Pull Request** against `main` (or `master`).
4. Describe your changes and link any related issues.

Maintainers will review and may request changes. Once approved, your PR can be merged.

## Reporting issues

- **Bugs:** Use the [Bug report](https://github.com/Llunarstack/j/issues/new?template=bug_report.md) template.
- **Features:** Use the [Feature request](https://github.com/Llunarstack/j/issues/new?template=feature_request.md) template.
- **Security:** Report vulnerabilities privately (e.g. via maintainer contact); do not post them in public issues.

Include version, OS, and minimal steps or code to reproduce where relevant.

## Questions

- Search [existing issues](https://github.com/Llunarstack/j/issues) first.
- Open a new issue for questions or discussions.
- Be respectful and constructive.

---

By contributing, you agree that your contributions will be licensed under the [MIT License](LICENSE).
