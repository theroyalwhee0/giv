# giv

A CLI for generating useful values.

## Available Commands

- `giv date`: Prints the date in various formats.
  - `giv now`: Prints the current time in various formats.
- `giv uuid`: Print a random UUID v7.
- `giv key`: Prints a strong random key with a 'key_' prefix.
- `giv pi`: Prints the requested number of digits of PI with rounding.

`giv --help` for basic help. `giv <COMMAND> --help` for help on a specific command.

## Installation

A shell script `./bin/install.sh` is provided to build and install the project using `cargo install` for local development and testing.
