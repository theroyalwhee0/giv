# giv

[![Crates.io](https://img.shields.io/crates/v/giv.svg)](https://crates.io/crates/giv)
[![Documentation](https://docs.rs/giv/badge.svg)](https://docs.rs/giv)
[![Downloads](https://img.shields.io/crates/d/giv.svg)](https://crates.io/crates/giv)
[![License](https://img.shields.io/crates/l/giv.svg)](https://github.com/theroyalwhee0/giv)

A CLI for generating useful values.

## Global Options

- `--clip` / `-c`: Copy output to clipboard (still prints to stdout)
- `--json` / `-j`: Format output as JSON

Examples:

```bash
giv --clip uuid           # Copy UUID to clipboard + print
giv -c now                # Copy timestamp to clipboard + print
giv --json uuid           # Output UUID as JSON
giv --clip --json key 32  # Copy JSON to clipboard + print
```

## Available Commands

- `giv bytes`: Generate random bytes with various encodings.
  - Base64 (default, no padding): `giv bytes`
  - Hex encoding: `giv bytes -e hex`
  - Base64 with padding: `giv bytes -e base64 --pad`
  - Rust array: `giv bytes -e rust 16`
  - JavaScript array: `giv bytes -e javascript 16`
  - TypeScript array: `giv bytes -e typescript 16`
  - Raw bytes: `giv bytes -e raw 100 | hexyl`
- `giv chars`: Convert emoji shortcodes and character patterns to Unicode.
  - Emoji: `giv chars :smile: :rocket: :thumbsup:`
  - HTML entities: `giv chars "&nbsp;" "&copy;" "&lt;"`
  - Fractions: `giv chars 1/4 1/2 3/4`
  - Symbols: `giv chars "(c)" "(r)" "(tm)"`
  - Arrows: `giv chars -- "->" "<-" "=>"`
  - Punctuation: `giv chars "..." em` (em-dash: —)
  - Greek: `giv chars alpha beta gamma delta lambda pi omega`
  - Multiple: `giv chars 1/4 :smile: "(c)" lambda` → `¼ 😄 © λ`
  - **Note**: Patterns starting with `-` (like `--` or `->`) require quoting or using `--` separator: `giv chars -- "--" "->"` or `giv chars em` for em-dash
- `giv date`: Prints the date in various formats.
  - `giv now`: Prints the current time in various formats.
- `giv uuid`: Generate UUIDs with multiple versions and formats.
  - UUID v7 (default): `giv uuid`
  - UUID v4: `giv uuid --version v4`
  - Simple format: `giv uuid --format simple`
  - Braced format: `giv uuid --format braced`
  - URN format: `giv uuid --format urn`
  - Hex format: `giv uuid --format hex`
  - Uppercase: `giv uuid --uppercase`
- `giv key`: Prints a strong random key with a 'key_' prefix.
- `giv lorem`: Generate lorem ipsum placeholder text.
  - Words (default): `giv lorem 50`
  - Sentences: `giv lorem -s 3`
  - Paragraphs: `giv lorem -p 2`
- `giv pi`: Prints the requested number of digits of PI with rounding.
- `giv rng`: Generate cryptographically secure random numbers.
  - Dice notation: `giv rng 2d6`, `giv rng d20`
  - Integer ranges: `giv rng 1..100`
  - Float ranges: `giv rng 0.0..1.0`, `giv rng 0.000..1`
  - Multiple specs: `giv rng 2d6 1..100 0.0..1.0`

`giv --help` for basic help. `giv <COMMAND> --help` for help on a specific command.

## Development Notes

- This project uses a whitelist approach to the `.gitignore`.

## Security

For security vulnerabilities and reporting guidelines, see [SECURITY.md](SECURITY.md).

## License

Copyright © 2025 Adam Mill

Licensed under the Apache License, Version 2.0. See [LICENSE.txt](LICENSE.txt) for details.
