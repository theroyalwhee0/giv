# JSON Output Format

This document describes the JSON output format for all commands in the `giv` CLI tool. JSON output is enabled using the `--json` or `-j` flag.

## Overview

All commands return structured JSON objects with descriptive property names and relevant metadata. This provides:

- **Consistency**: Predictable structure across all commands
- **Metadata**: Additional context (version, precision, rounding flags, source data)
- **Parseability**: Easy integration with other tools and scripts
- **Type information**: Clear distinction between formatted values and raw data

### Common Patterns

- **Descriptive properties**: Use command-specific names (e.g., `"key"`, `"pi"`, `"uuid"`)
- **Metadata fields**: Include contextual information where relevant
- **Source data**: Raw or unformatted values stored in `source` arrays
- **Formatted values**: Display-ready values in `value` fields

---

## Commands

### `key` - Generate Random Key

**Output Structure**:

```json
{
  "key": "key_<alphanumeric>"
}
```

**Properties**:

- `key` (string): The generated key with `key_` prefix

**Examples**:

```bash
# Default size (36 characters)
giv --json key
```

```json
{
  "key": "key_a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6q7r8"
}
```

```bash
# Custom size (10 characters)
giv --json key 10
```

```json
{
  "key": "key_a1b2c3d4e5"
}
```

---

### `uuid` - Generate UUID v7

**Output Structure**:

```json
{
  "uuid": "<uuid-string>",
  "version": "v7"
}
```

**Properties**:

- `uuid` (string): The generated UUID in standard format
- `version` (string): The UUID version (always `"v7"`)

**Examples**:

```bash
giv --json uuid
```

```json
{
  "uuid": "01234567-89ab-cdef-0123-456789abcdef",
  "version": "v7"
}
```

---

### `pi` - Calculate Pi

**Output Structure**:

```json
{
  "pi": "<value>",
  "rounded": true|false
}
```

**Properties**:

- `pi` (string): Pi value formatted to the specified decimal places
- `rounded` (boolean): Whether rounding was applied to the last digit

**Examples**:

```bash
# Default precision (15 decimal places, rounded)
giv --json pi
```

```json
{
  "pi": "3.141592653589793",
  "rounded": true
}
```

```bash
# Custom precision (10 decimal places, rounded)
giv --json pi 10
```

```json
{
  "pi": "3.1415926536",
  "rounded": true
}
```

```bash
# No rounding
giv --json pi 10 --no-round
```

```json
{
  "pi": "3.1415926535",
  "rounded": false
}
```

---

### `date` - Generate Date

**Output Structure**:

```json
{
  "date": "<formatted-date>"
}
```

**Properties**:

- `date` (string): The formatted date/time string

**Examples**:

```bash
# Current date and time (RFC 3339)
giv --json date now
```

```json
{
  "date": "2025-09-02T14:30:15.123Z"
}
```

```bash
# Current timestamp
giv --json date timestamp
```

```json
{
  "date": "1725283815"
}
```

```bash
# Today's date
giv --json date today
```

```json
{
  "date": "2025-09-02"
}
```

```bash
# Custom format
giv --json date now --format timestamp-ms
```

```json
{
  "date": "1725283815123"
}
```

---

### `now` - Current Time Alias

**Output Structure**:

```json
{
  "date": "<formatted-date>"
}
```

**Properties**:

- `date` (string): The current UTC time in the specified format

**Examples**:

```bash
# Current time (default RFC 3339)
giv --json now
```

```json
{
  "date": "2025-09-02T14:30:15.123Z"
}
```

```bash
# Current time as timestamp
giv --json now --format timestamp
```

```json
{
  "date": "1725283815"
}
```

---

### `rng` - Random Number Generation

**Output Structure**:

```json
{
  "rng": [
    {
      "type": "dice|range_int|range_float",
      "notation": "<spec>",
      "value": <formatted-value>,
      "precision": <number>,
      "source": [<raw-values>]
    }
  ]
}
```

**Properties**:

- `rng` (array): Array of results, one per specification
  - `type` (string): The type of random generation
    - `"dice"`: Dice roll (e.g., `3d6`)
    - `"range_int"`: Integer range (e.g., `1..100`)
    - `"range_float"`: Float range (e.g., `1.0..10.0`)
  - `notation` (string): The parsed specification notation
  - `value` (number|string): The final result
    - For `dice`: Sum of all rolls plus modifier (number, can be negative)
    - For `range_int`: The generated integer (number)
    - For `range_float`: Formatted string with precision
  - `modifier` (number): The modifier applied to dice rolls (dice only, can be negative)
  - `precision` (number): Decimal places (float ranges only)
  - `source` (array): Raw data
    - For `dice`: Individual roll results (before modifier)
    - For `range_float`: Full-precision floating point value(s)
    - Not present for `range_int`

**Examples**:

```bash
# Dice roll without modifier
giv --json rng 3d6
```

```json
{
  "rng": [
    {
      "type": "dice",
      "notation": "3d6",
      "value": 13,
      "modifier": 0,
      "source": [6, 4, 3]
    }
  ]
}
```

```bash
# Dice roll with positive modifier
giv --json rng 3d6+2
```

```json
{
  "rng": [
    {
      "type": "dice",
      "notation": "3d6+2",
      "value": 15,
      "modifier": 2,
      "source": [5, 6, 2]
    }
  ]
}
```

```bash
# Dice roll with negative modifier
giv --json rng 1d20-1
```

```json
{
  "rng": [
    {
      "type": "dice",
      "notation": "d20-1",
      "value": 14,
      "modifier": -1,
      "source": [15]
    }
  ]
}
```

```bash
# Integer range
giv --json rng 1..100
```

```json
{
  "rng": [
    {
      "type": "range_int",
      "notation": "1..100",
      "value": 42
    }
  ]
}
```

```bash
# Float range
giv --json rng 1.0..10.0
```

```json
{
  "rng": [
    {
      "type": "range_float",
      "notation": "1.0..10.0",
      "value": "2.5",
      "precision": 1,
      "source": [2.510738494148031]
    }
  ]
}
```

```bash
# Multiple specifications
giv --json rng 2d6 1..100 0.0..1.0
```

```json
{
  "rng": [
    {
      "type": "dice",
      "notation": "2d6",
      "value": 8,
      "modifier": 0,
      "source": [5, 3]
    },
    {
      "type": "range_int",
      "notation": "1..100",
      "value": 67
    },
    {
      "type": "range_float",
      "notation": "0.0..1.0",
      "value": "0.7",
      "precision": 1,
      "source": [0.7234567890123456]
    }
  ]
}
```

---

## Usage Tips

### Parsing JSON Output

Use `jq` or similar tools to parse and extract values:

```bash
# Extract just the UUID value
giv --json uuid | jq -r '.uuid'
# Output: 01234567-89ab-cdef-0123-456789abcdef

# Extract pi value
giv --json pi 10 | jq -r '.pi'
# Output: 3.1415926536

# Check if rounding was applied
giv --json pi 10 | jq '.rounded'
# Output: true

# Extract all dice roll results
giv --json rng 3d6 | jq '.rng[0].source'
# Output: [6, 4, 3]

# Get float source value for higher precision
giv --json rng 1.0..10.0 | jq '.rng[0].source[0]'
# Output: 2.510738494148031
```

### Scripting Integration

JSON output is designed for integration with scripts and automation:

```bash
# Generate multiple keys and process with jq
for i in {1..5}; do
  giv --json key 8 | jq -r '.key'
done

# Get current timestamp for logging
timestamp=$(giv --json now --format timestamp | jq -r '.date')
echo "Event at: $timestamp"

# Generate random number for scripting
random_value=$(giv --json rng 1..100 | jq '.rng[0].value')
if [ "$random_value" -gt 50 ]; then
  echo "High roll: $random_value"
fi
```

### Type Safety

JSON output provides type information making it easier to handle values correctly:

- Strings are always quoted: `"3.14"`, `"key_abc"`
- Numbers are unquoted: `42`, `3.14159`
- Booleans are lowercase: `true`, `false`
- Arrays provide access to multiple values: `[6, 4, 3]`

---

## Availability

JSON output is only available when `giv` is compiled with the `json` feature enabled. This is the default configuration.

To check if JSON support is available:

```bash
giv --help | grep -q "\-\-json" && echo "JSON supported" || echo "JSON not available"
```
