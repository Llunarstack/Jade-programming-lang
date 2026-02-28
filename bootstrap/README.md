# Bootstrap scripts

Jade scripts that run on the Rust-based interpreter and operate on Jade source or the repo. Part of the [bootstrapping plan](../BOOTSTRAP.md).

## Run from repo root

```bash
jade bootstrap/stage0_hello.jdl
jade bootstrap/stage0_stats.jdl
```

- **stage0_hello.jdl** — Prints a short “Jade bootstrapping” message.
- **stage0_stats.jdl** — Reads `bootstrap/stage0_hello.jdl` and prints line count and simple pattern counts.

## Adding scripts

1. Add a new `.jdl` file here (e.g. `stage0_foo.jdl`).
2. Update this README and the stages table in [BOOTSTRAP.md](../BOOTSTRAP.md).
