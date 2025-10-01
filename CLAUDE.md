# giv - Project Context for Claude

## Project Overview

`giv` is a Rust CLI tool for generating useful values (dates, UUIDs, keys, pi digits, random numbers). It emphasizes strict code quality, comprehensive documentation, and safe coding practices.

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

- **Command modules**: Named with `c_` prefix (e.g., `c_date`, `c_key`, `c_uuid`, `c_pi`, `c_rng`)
- **Feature flags**: Each command is a Cargo feature that can be independently enabled/disabled
- **Output system**:
  - Core output trait in `app/output/output_trait.rs`
  - Each command has its own `output.rs` with a structured output type
  - Supports both plain text and JSON output via the `Output` trait
  - JSON output uses descriptive object properties (e.g., `{"key":"..."}`, `{"pi":"...","rounded":true}`)
  - Includes metadata fields where relevant (version, precision, rounding flags, source values)
- **Error handling**: Custom error types in `error.rs` using `thiserror`

### File Organization (One Item Per File)

- **Guideline**: Place one public item (struct, enum, or trait) per file as a general rule.
  - File names should match the item name (e.g., `Schema` struct goes in `schema.rs`)
  - Each file contains the item and all its implementations (Display, Default, methods, etc.)
  - Type aliases are exempt from this rule and can be grouped logically
  - When violating this guideline, include a comment explaining why:

    ```rust
    // Multiple error types grouped together for cohesion.
    pub enum AtsError { ... }
    pub enum LexError { ... }
    ```

- **Benefits of this pattern**:
  - Clear file-to-type mapping for navigation
  - Focused context when editing specific types
  - Precise git history (changes to `Field` only touch `field.rs`)
  - Reduced merge conflicts when working on different types
  - Tests can be colocated with their specific type

### Code Structure

```text
src/
├── main.rs           # Entry point with command routing
├── error.rs          # Error types
├── app/              # Application infrastructure
│   ├── cli/          # Clap-based CLI definitions
│   ├── output/       # Output trait and formatting
│   └── context.rs    # Command execution context
├── c_date/           # Date/time generation
│   ├── mod.rs
│   ├── date_format.rs
│   ├── date_kind.rs
│   └── output.rs     # DateOutput struct
├── c_key/            # Random key generation
│   ├── mod.rs
│   └── output.rs     # KeyOutput struct
├── c_uuid/           # UUID v7 generation
│   ├── mod.rs
│   └── output.rs     # UuidOutput struct
├── c_pi/             # Pi digit generation
│   ├── mod.rs
│   ├── decimals.rs   # Pre-calculated pi digits
│   └── output.rs     # PiOutput struct
└── c_rng/            # Random number generation
    ├── mod.rs
    ├── spec.rs       # Specification parsing
    ├── execute.rs    # Execution logic
    ├── generator.rs  # RNG functions
    ├── result.rs     # Result types
    └── output.rs     # RngOutput struct
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
6. **Structured Output**: All commands must provide structured JSON output with descriptive properties and metadata

### Output Pattern

Each command follows a consistent output pattern:

1. **Output struct**: Define in `output.rs` with `#[derive(Debug, Serialize)]`
2. **Implement `Output` trait**:
   - `to_plain()`: Returns simple string output for CLI users
   - `to_json()`: Returns structured JSON with descriptive properties
3. **JSON structure guidelines**:
   - Use object properties, not bare strings (e.g., `{"pi":"..."}` not `"..."`)
   - Include relevant metadata (version, precision, rounding flags, source data)
   - Use consistent naming: `source` for raw/unformatted data, `value` for formatted results
4. **Backward compatibility**: Plain text output should remain simple and unchanged

Example output structures:
- `{"key":"key_xxx"}` - Simple value
- `{"pi":"3.14","rounded":true}` - Value with metadata
- `{"uuid":"xxx","version":"v7"}` - Value with version info
- `{"rng":[{"value":"2.5","source":[2.51...],...}]}` - Array with formatted and raw data

### Common Tasks

- **Adding a new command**:
  1. Create `c_commandname/mod.rs` module with command logic
  2. Create `c_commandname/output.rs` implementing the `Output` trait
     - Define a struct for the command output (e.g., `CommandOutput`)
     - Implement `to_plain()` for plain text output
     - Implement `to_json()` for JSON output with descriptive properties
     - Include metadata fields where relevant (version, precision, flags, etc.)
  3. Add feature to `Cargo.toml` with dependencies
  4. Add command variant to `app/cli/commands.rs`
  5. Add command handler to `app/mod.rs`
  6. Gate with `#[cfg(feature = "commandname")]`

- **Output formatting**:
  - Create a structured output type in `output.rs` that implements the `Output` trait
  - Use `ctx.output().output(&output)` to send output
  - JSON output should use descriptive object properties, not bare strings
  - Include contextual metadata (e.g., `{"uuid":"...","version":"v7"}`)
  - Keep plain text output simple and backward compatible

### Dependencies Philosophy

- Minimal, well-vetted dependencies
- Feature-gated to keep binary size down
- Security-focused (e.g., `rand` for cryptographic randomness)

### Testing Notes

- Use `#[cfg(test)]` modules
- Document test panics expectations
- Test both plain and JSON output modes
