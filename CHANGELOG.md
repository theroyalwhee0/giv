# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2025-10-02

### Added

- Library API for programmatic access to all generation functions (#19, #21)
  - Public functions in each command module that return structured output types
  - All functions return `Result<OutputType, GivError>`
  - Feature flags control which modules are included (default: all)
  - Exported constants like `DEFAULT_KEY_SIZE` and `PI_DEFAULT_PLACES`
- Reproducible builds with SOURCE_DATE_EPOCH support (#23, #25)
  - Build process now respects SOURCE_DATE_EPOCH environment variable
  - Enables reproducible binary builds for package managers

### Changed

- Version output now includes repository, crates.io, and docs.rs links (#18)
- README now includes crates.io badge (#20)

### Fixed

- Code cleanup: Removed unused code and fixed compiler warnings (#22, #24)

## [0.1.0] - 2025-10-01

### Added

- `chars` command for emoji shortcode and character pattern conversion
  - Emoji support with `:shortcode:` syntax
  - Fractions (1/2, 1/4, 3/4)
  - Symbols ((c), (r), (tm))
  - Arrows (->, <-, =>)
  - Punctuation (..., --)
  - Greek letters (alpha, beta, gamma, etc.)
- `bytes` command for random byte generation
  - Multiple encoding formats: base64, hex, Rust, JavaScript, TypeScript, raw
  - Optional padding for base64
- Dice modifier support in `rng` command
  - Support for modifiers like `2d6+3` or `d20-1`
  - Overflow protection
- `rng` command for cryptographically secure random number generation
  - Dice notation (2d6, d20, etc.)
  - Integer ranges (1..100)
  - Float ranges (0.0..1.0)
  - Multiple specifications in a single command
- `uuid` command for UUID v7 generation
- `key` command for cryptographic key generation with `key_` prefix
- `pi` command for π digit generation with configurable precision
- `date` and `now` commands for date/time formatting
- JSON output support for all commands via `--json` flag
- Structured output system with `Output` trait
- Comprehensive documentation and usage guides

[unreleased]: https://github.com/theroyalwhee0/giv/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/theroyalwhee0/giv/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/theroyalwhee0/giv/releases/tag/v0.1.0
