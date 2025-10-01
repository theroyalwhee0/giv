# giv - Project Context for Claude

## Project Overview

`giv` is a Rust CLI tool for generating useful values (dates, UUIDs, keys, pi digits). It emphasizes strict code quality, comprehensive documentation, and safe coding practices.

## Key Project Characteristics

### Strict Linting and Quality Standards

- **Forbidden unsafe code**: `unsafe_code = "forbid"`
- **Required documentation**: All items (public and private) must be documented
  - Missing docs are denied (both code and rustdoc)
  - All functions must include `# Errors`, `# Panics`, and `# Safety` docs where applicable
- **No direct stdout/stderr**: Use the `output` module instead
  - `print_stdout` and `print_stderr` are denied in clippy
  - All output goes through the structured output system

### Architecture Patterns

- **Command modules**: Named with `c_` prefix (e.g., `c_date`, `c_key`, `c_uuid`, `c_pi`)
- **Feature flags**: Each command is a Cargo feature that can be independently enabled/disabled
- **Output system**: Centralized in `output.rs`, supports both plain text and JSON output
- **Error handling**: Custom error types in `error.rs` using `thiserror`

### Code Structure

```text
src/
├── main.rs           # Entry point with command routing
├── cli.rs            # Clap-based CLI definitions
├── output.rs         # Centralized output handling
├── error.rs          # Error types
├── c_date/           # Date/time generation
├── c_key/            # Random key generation
├── c_uuid/           # UUID v7 generation
└── c_pi/             # Pi digit generation
    ├── mod.rs
    └── decimals.rs   # Pre-calculated pi digits
```

### Development Workflow

- Build/install script: `./bin/install.sh`
- Test with: `cargo test`
- Lint with: `cargo clippy`
- Documentation: `cargo doc --open`

### Git Configuration

**Whitelist .gitignore**: This project uses a whitelist approach to version control. By default, all files are ignored (`*`), and only specific file types and paths are explicitly allowed:
- Documentation: `*.md` files, `docs/**/*.md`
- Rust files: `Cargo.toml`, `Cargo.lock`, `clippy.toml`, `src/**/*.rs`
- Scripts: `bin/*.sh`
- Project config: `CLAUDE.md`, `.gitignore`

This ensures only intentional source files are committed, preventing accidental inclusion of build artifacts, local config, or temporary files.

### Important Conventions

1. **Documentation First**: Write docs before implementation
2. **Module Privacy**: Document all private items too
3. **Error Propagation**: Use `Result` types with descriptive errors
4. **Output Abstraction**: Never use `println!` or `eprintln!` directly (except in main.rs error handler)
5. **Feature Gates**: Use `#[cfg(feature = "...")]` for optional functionality

### Common Tasks

- **Adding a new command**:
  1. Create `c_commandname/mod.rs` module
  2. Add feature to `Cargo.toml` with dependencies
  3. Add command variant to `cli.rs`
  4. Add command handler to `main.rs`
  5. Gate with `#[cfg(feature = "commandname")]`

- **Output formatting**: Use `output::outputln(options, value)` for all program output

### Dependencies Philosophy

- Minimal, well-vetted dependencies
- Feature-gated to keep binary size down
- Security-focused (e.g., `rand` for cryptographic randomness)

### Testing Notes

- Use `#[cfg(test)]` modules
- Document test panics expectations
- Test both plain and JSON output modes
