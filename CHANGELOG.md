# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[unreleased]: https://github.com/theroyalwhee0/giv/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/theroyalwhee0/giv/releases/tag/v0.1.0
