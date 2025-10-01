# giv

A CLI for generating useful values.

## Available Commands

- `giv bytes`: Generate random bytes with various encodings.
  - Base64 (default, no padding): `giv bytes`
  - Hex encoding: `giv bytes -e hex`
  - Base64 with padding: `giv bytes -e base64 --pad`
  - Rust array: `giv bytes -e rust 16`
  - JavaScript array: `giv bytes -e javascript 16`
  - TypeScript array: `giv bytes -e typescript 16`
  - Raw bytes: `giv bytes -e raw 100 | hexyl`
- `giv date`: Prints the date in various formats.
  - `giv now`: Prints the current time in various formats.
- `giv uuid`: Print a random UUID v7.
- `giv key`: Prints a strong random key with a 'key_' prefix.
- `giv pi`: Prints the requested number of digits of PI with rounding.
- `giv rng`: Generate cryptographically secure random numbers.
  - Dice notation: `giv rng 2d6`, `giv rng d20`
  - Integer ranges: `giv rng 1..100`
  - Float ranges: `giv rng 0.0..1.0`, `giv rng 0.000..1`
  - Multiple specs: `giv rng 2d6 1..100 0.0..1.0`

`giv --help` for basic help. `giv <COMMAND> --help` for help on a specific command.
