# Usage Guide

This document provides detailed usage examples for all commands, flags, and arguments available in the `giv` CLI tool.

## Global Options

### JSON Output

- **Flag**: `-j, --json`
- **Description**: Format the output as JSON
- **Default**: `false`
- **Available**: When compiled with `json` feature

```bash
# Standard output
giv uuid
# Output: 01234567-89ab-cdef-0123-456789abcdef

# JSON output
giv --json uuid
# Output: "01234567-89ab-cdef-0123-456789abcdef"
```

---

## Commands

### `key` - Generate Random Key

Generate a random key in the format `key_<alphanumeric[size]>`

**Usage**: `giv key [SIZE]`

**Arguments**:

- `SIZE` (optional): Size of the output key in characters (default: 36)

**Examples**:

```bash
# Generate key with default size (36 characters)
giv key
# Output: key_a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6q7r8

# Generate key with custom size
giv key 10
# Output: key_a1b2c3d4e5

# Generate key with JSON output
giv --json key 8
# Output: "key_a1b2c3d4"
```

---

### `uuid` - Generate UUID v7

Generate and display a UUID version 7

**Usage**: `giv uuid`

**Arguments**: None

**Examples**:

```bash
# Generate UUID v7
giv uuid
# Output: 01234567-89ab-cdef-0123-456789abcdef

# Generate UUID v7 with JSON output
giv --json uuid
# Output: "01234567-89ab-cdef-0123-456789abcdef"
```

---

### `pi` - Calculate Pi

Display Pi with the specified number of decimal places

**Usage**: `giv pi [OPTIONS] [PLACES]`

**Arguments**:

- `PLACES` (optional): Number of decimal places to display (default: 15)

**Options**:

- `--round`: Round the result (default: true)
- `--no-round`: Don't round the result (hidden flag, negation of --round)

**Examples**:

```bash
# Pi with default precision (15 decimal places)
giv pi
# Output: 3.141592653589793

# Pi with custom precision
giv pi 10
# Output: 3.1415926536

# Pi with no rounding
giv pi 10 --no-round
# Output: 3.1415926535

# Pi with JSON output
giv --json pi 5
# Output: "3.14159"
```

---

### `date` - Generate Date

Generate a date in various formats

**Usage**: `giv date [OPTIONS] <KIND>`

**Arguments**:

- `KIND`: Type of date to generate
  - `now`: Current date and time (default format: RFC 3339)
  - `timestamp`: Current date and time (default format: Unix timestamp)
  - `today`: Current date only (default format: RFC 3339 date)
  - `yesterday`: Yesterday's date (default format: RFC 3339 date)
  - `tomorrow`: Tomorrow's date (default format: RFC 3339 date)

**Options**:

- `-f, --format <FORMAT>`: Output format
  - `rfc3339`: RFC 3339 format with milliseconds (e.g., `2025-04-17T15:14:12.748Z`)
  - `rfc3339-date`: RFC 3339 date only (e.g., `2025-04-17`)
  - `rfc3339-time`: RFC 3339 time only (e.g., `15:14:12.748Z`)
  - `timestamp`: Unix timestamp (e.g., `1744902865`)
  - `timestamp-ms`: Unix timestamp in milliseconds (e.g., `1744902874772`)
  - `rfc2882`: RFC 2822 format (e.g., `Fri, 17 Apr 2025 15:14:12 +0000`)

**Examples**:

```bash
# Current date and time (RFC 3339 format)
giv date now
# Output: 2025-09-02T14:30:15.123Z

# Current date and time as timestamp
giv date timestamp
# Output: 1725283815

# Today's date only
giv date today
# Output: 2025-09-02

# Yesterday's date
giv date yesterday
# Output: 2025-09-01

# Tomorrow's date
giv date tomorrow
# Output: 2025-09-03

# Custom format - current time as timestamp
giv date now --format timestamp
# Output: 1725283815

# Custom format - today as RFC 3339 time
giv date today --format rfc3339-time
# Output: 14:30:15.123Z

# JSON output
giv --json date now
# Output: "2025-09-02T14:30:15.123Z"
```

---

### `now` - Current Time Alias

Display the current UTC time. This is an alias for `date now`.

**Usage**: `giv now [OPTIONS]`

**Options**:

- `-f, --format <FORMAT>`: Output format (same options as `date` command)

**Examples**:

```bash
# Current time (default RFC 3339 format)
giv now
# Output: 2025-09-02T14:30:15.123Z

# Current time as timestamp
giv now --format timestamp
# Output: 1725283815

# Current time as RFC 2822
giv now --format rfc2882
# Output: Mon, 02 Sep 2025 14:30:15 +0000

# JSON output
giv --json now
# Output: "2025-09-02T14:30:15.123Z"
```

---

### `rng` - Random Number Generation

Generate random numbers using dice notation, integer ranges, or float ranges.

**Usage**: `giv rng <SPEC>...`

**Arguments**:

- `SPEC`: One or more specifications for random number generation
  - Dice notation: `XdY` or `dY` (e.g., `3d6`, `d20`)
    - Optional modifiers: `+N` or `-N` (e.g., `3d6+2`, `1d20-1`)
  - Integer ranges: `X..Y` (e.g., `1..100`)
  - Float ranges: `X.Y..A.B` (e.g., `0.0..1.0`, `1.5..10.75`)

**Examples**:

```bash
# Roll a single six-sided die
giv rng d6
# Output: 4

# Roll three six-sided dice
giv rng 3d6
# Output: 11

# Roll dice with positive modifier
giv rng 3d6+2
# Output: 15

# Roll dice with negative modifier
giv rng 1d20-1
# Output: 14

# Generate integer in range
giv rng 1..100
# Output: 42

# Generate float in range
giv rng 0.0..1.0
# Output: 0.7

# Multiple specifications
giv rng 2d6 1d20+5 1..100
# Output:
# 8
# 19
# 67

# JSON output
giv --json rng 3d6+2
# Output: {"rng":[{"type":"dice","notation":"3d6+2","value":15,"modifier":2,"source":[5,6,2]}]}
```

**Notes**:

- Dice modifiers can result in negative values (e.g., `1d4-10` might return `-6`)
- Arithmetic operations use overflow checking for safety
- Float precision is determined by decimal places in the specification

---

## Feature Compilation

The `giv` tool supports conditional compilation of features. Commands are only available when their corresponding features are enabled:

- `key` command: Requires `key` feature
- `uuid` command: Requires `uuid` feature
- `pi` command: Requires `pi` feature
- `date` and `now` commands: Require `date` feature
- `rng` command: Requires `rng` feature
- `--json` flag: Requires `json` feature

## Help and Version

```bash
# Show help information
giv --help

# Show version information
giv --version

# Show help for specific command
giv key --help
giv date --help
```
