# giv

A CLI for generating useful values.

## Available Commands

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
