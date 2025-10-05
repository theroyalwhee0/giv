# Contributing to giv

Thank you for your interest in contributing to `giv`! This document provides guidelines for contributing to the project.

## Code of Conduct

This project adheres to the Contributor Covenant [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## How to Contribute

### Reporting Bugs

If you find a bug, please create an issue with:

- A clear, descriptive title
- Steps to reproduce the problem
- Expected vs actual behavior
- Your environment (OS, Rust version, `giv` version)
- Any relevant error messages or logs

### Suggesting Enhancements

Enhancement suggestions are welcome! Please create an issue describing:

- The motivation for the enhancement
- A clear description of the proposed functionality
- Any potential implementation considerations

### Pull Requests

1. **Fork and Clone**: Fork the repository and clone it locally
2. **Create a Branch**: Create a feature branch from `main`
3. **Make Changes**: Follow the project's coding standards (see below)
4. **Test**: Ensure all tests pass with `cargo test`
5. **Lint**: Run `cargo clippy` and address any warnings
6. **Commit**: Write clear, descriptive commit messages
7. **Push**: Push your branch to your fork
8. **Open a PR**: Submit a pull request to the `main` branch

## Development Setup

```bash
# Clone your fork
git clone https://github.com/YOUR-USERNAME/giv.git
cd giv

# Build the project
cargo build

# Run tests
cargo test

# Run clippy
cargo clippy

# Run the CLI locally
cargo run --features="bin" -- --help
```

## Coding Standards

This project maintains strict code quality standards:

### Required Practices

- **No unsafe code**: The codebase forbids `unsafe` code
- **Comprehensive documentation**: All items (public and private) must be documented
  - Include `# Errors`, `# Panics`, and `# Safety` sections where applicable
- **No direct output**: Use the `output` module instead of `println!` or `eprintln!`
- **One item per file**: Generally, place one public item (struct, enum, or trait) per file
  - File names should match the item name
  - Include a comment if violating this guideline

### Code Organization

- **Command modules**: Follow the existing pattern with feature flags
- **Output types**: Implement the `Output` trait for both plain text and JSON output
- **Error handling**: Use `Result` types with descriptive `GivError` variants
- **Testing**: Add tests for new functionality in `#[cfg(test)]` modules

### Whitelist .gitignore

This project uses a whitelist approach to version control. Only explicitly allowed files are tracked. When adding new files:

- Source code: Automatically included if in `src/**/*.rs`
- Documentation: Update `.gitignore` if adding new docs outside `docs/`
- Scripts: Update `.gitignore` if adding scripts outside `scripts/`

## Adding a New Command

To add a new command:

1. Create `src/commandname/mod.rs` with the command logic
2. Create `src/commandname/output.rs` implementing the `Output` trait
3. Add a feature flag to `Cargo.toml`
4. Add the command variant to `src/app/cli/commands.rs`
5. Add the command handler to `src/app/mod.rs`
6. Gate code with `#[cfg(feature = "commandname")]`
7. Update documentation (README.md, help text, etc.)
8. Add tests

## Testing

- Write unit tests for new functionality
- Test both plain text and JSON output modes
- Ensure tests pass with `cargo test`
- Document test panic expectations

## Documentation

- Update README.md for user-facing changes
- Add rustdoc comments for all public APIs
- Include examples in documentation where helpful
- Run `cargo doc --open` to preview documentation

## Dependencies

- Keep dependencies minimal and well-vetted
- Security is a priority (use cryptographic-strength randomness)
- Discuss new dependencies before adding them
- Use feature flags to keep dependencies optional when possible

## Questions?

If you have questions about contributing, feel free to:

- Open an issue for discussion
- Check existing issues and pull requests for context

Thank you for contributing to `giv`!
