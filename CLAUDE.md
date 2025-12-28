# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

This is a Cargo workspace. Install cargo-make (`cargo install cargo-make`), then run:

```bash
cargo make              # Build entire workspace
cargo build             # Standard build
cargo build --release   # Optimized release build
cargo build -p patterns # Build specific package
```

## Testing

```bash
cargo test                    # Run all tests
cargo test -p patterns        # Test specific package
cargo test -- --nocapture     # Show println! output during tests
```

## Running Applications

```bash
# Design patterns demo
cargo run -p cli

# grep-like search utility
cargo run -p grrs -- "pattern" path/to/file
# Example: cargo run -p grrs -- "Lorem" cli-apps/grrs/data/lorem_ipsum.txt
```

## Architecture

**Workspace structure with three crates:**

- `design-patterns/patterns` - Core library implementing design patterns (Strategy, Newtype, Command, Interpreter)
- `design-patterns/cli` - Executable that demonstrates all patterns from the library
- `cli-apps/grrs` - Standalone grep-like CLI tool

**Dependency flow:**
- `cli` depends on `patterns` (local path dependency)
- `grrs` is standalone (uses clap + anyhow)
- `patterns` has no external dependencies

**Key patterns demonstrated:**
- **Strategy**: `LogFormatter` trait with `SimpleFormatter`/`RedactedFormatter` implementations
- **Command**: `Action` trait with `ActionBuffer` for queuing/executing commands
- **Interpreter**: State machine parser that converts strings like "Run, Jump, Shoot" into Action objects
- **Newtype**: Type-safe `Milliseconds(u32)` wrapper

Tests are inline using `#[cfg(test)]` modules at the bottom of each pattern file.
