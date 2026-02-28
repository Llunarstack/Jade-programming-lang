# Project structure

```
Jade/
├── README.md              # Main project readme
├── CHANGELOG.md           # Version history
├── LICENSE
├── docs/                  # Project documentation
│   ├── README.md          # Doc index
│   ├── INSTALL.md         # Install guide
│   ├── BOOTSTRAP.md       # Bootstrapping
│   ├── CONTRIBUTING.md    # Contribution guide
│   └── CODE_OF_CONDUCT.md
├── assets/                # Logos, icons (repo-level)
├── bootstrap/             # Bootstrap Jade scripts (stage0_hello.jdl, etc.)
├── jade-lang/             # Main language crate (Rust)
│   ├── src/               # Interpreter, compiler, REPL, Jolt
│   ├── tests/             # Integration tests (.rs) and fixtures
│   │   ├── fixtures/       # Sample .jdl files for testing
│   │   └── integration/   # Rust integration tests
│   └── installers/        # Platform installers and IDE support
│       ├── windows/       # Inno Setup, MSI, portable, icons
│       ├── linux/         # install.sh, .deb, .rpm, AppImage
│       ├── macos/         # install.sh, .pkg, .dmg
│       ├── android/       # Termux build
│       ├── ide/           # VS Code/Cursor extension (syntax, run from buffer)
│       └── mime/          # MIME type docs
└── .github/               # CI/CD, issue templates
```

Build outputs go to `jade-lang/target/` and `dist/` (installers); both are gitignored.
