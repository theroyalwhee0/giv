# Characters and Emoji Reference

This document lists commonly used character patterns and emoji shortcodes for the `giv chars` command.

The `giv chars` command uses the [penmanship](https://github.com/theroyalwhee0/penmanship) crate for Unicode character lookup, which provides comprehensive support for:

- **Character patterns**: Punctuation, math symbols, Greek letters, fractions, currency, and more
- **HTML entities**: 2200+ named character references (e.g., `&nbsp;`, `&copy;`, `&alpha;`)
- **Emoji shortcodes**: 1800+ GitHub-compatible emoji (e.g., `:smile:`, `:rocket:`)

For a complete list of all supported patterns, see:

- [penmanship character mappings](https://github.com/theroyalwhee0/penmanship/blob/main/docs/mappings.md)
- [penmanship HTML entities](https://github.com/theroyalwhee0/penmanship/blob/main/docs/html-entities.md)

## Usage

```bash
giv chars <pattern1> <pattern2> ...
giv --json chars <pattern1> <pattern2> ...
```

**Important Note**: Patterns starting with `-` (like `--` or `->`) require quoting or using the `--` separator to prevent them being interpreted as command-line flags:

```bash
# Using quotes
giv chars "--" "->"

# Using -- separator
giv chars -- "--" "->"

# Or use the "em" alias for em-dash
giv chars em
```

## Fractions

| Pattern | Result | Description |
|---------|--------|-------------|
| `1/4`   | ¼      | fraction one quarter |
| `1/2`   | ½      | fraction one half |
| `3/4`   | ¾      | fraction three quarters |
| `1/3`   | ⅓      | fraction one third |
| `2/3`   | ⅔      | fraction two thirds |
| `1/8`   | ⅛      | fraction one eighth |
| `3/8`   | ⅜      | fraction three eighths |
| `5/8`   | ⅝      | fraction five eighths |
| `7/8`   | ⅞      | fraction seven eighths |

**Example:**

```bash
$ giv chars 1/4 1/2 3/4
¼ ½ ¾
```

## Symbols

| Pattern | Result | Description |
|---------|--------|-------------|
| `(c)`, `(C)` | © | copyright sign |
| `(r)`, `(R)` | ® | registered sign |
| `(tm)`, `(TM)`, `(t)`, `(T)` | ™ | trade mark sign |
| `(p)`, `(P)` | ℗ | sound recording copyright |

**Example:**

```bash
$ giv chars "(c)" "(r)" "(tm)"
© ® ™
```

## Punctuation

| Pattern | Result | Description |
|---------|--------|-------------|
| `...`   | …      | horizontal ellipsis |
| `--`, `em` | —   | em dash |

**Example:**

```bash
$ giv chars "..." em
… —

# Or use the pattern (requires -- separator or quotes)
$ giv chars -- "..." "--"
… —
```

## Arrows

| Pattern | Result | Description |
|---------|--------|-------------|
| `->`    | →      | rightwards arrow |
| `<-`    | ←      | leftwards arrow |
| `=>`    | ⇒      | rightwards double arrow |
| `<=`    | ⇐      | leftwards double arrow |
| `<->`   | ↔      | left right arrow |
| `<=>`   | ⇔      | left right double arrow |
| `^^`, `up` | ↑   | upwards arrow |
| `vv`, `down` | ↓ | downwards arrow |

**Example:**

```bash
$ giv chars -- "->" "<-" "=>" "<="
→ ← ⇒ ⇐
```

Note: Use `--` before arrow patterns to prevent them being interpreted as flags.

## Currency

| Pattern | Result | Description |
|---------|--------|-------------|
| `cent`  | ¢      | cent sign |
| `pound` | £      | pound sign |
| `euro`  | €      | euro sign |
| `yen`   | ¥      | yen sign |
| `rupee` | ₹      | rupee sign |
| `won`   | ₩      | won sign |
| `bitcoin`, `btc` | ₿ | bitcoin sign |

**Example:**

```bash
$ giv chars cent pound euro yen rupee won bitcoin
¢ £ € ¥ ₹ ₩ ₿
```

## Math Symbols

| Pattern | Result | Description |
|---------|--------|-------------|
| `degree`, `deg` | °      | degree sign |
| `+-`    | ±      | plus-minus sign |
| `x`, `*`   | ×      | multiplication sign |
| `divide`, `div` | ÷      | division sign |
| `!=`, `ne`  | ≠      | not equal to |
| `lte`   | ≤      | less-than or equal to |
| `gte`   | ≥      | greater-than or equal to |
| `~=`    | ≈      | almost equal to |
| `infinity`, `inf` | ∞   | infinity |
| `sqrt`  | √      | square root |
| `sum`   | ∑      | n-ary summation |
| `prod`, `product` | ∏ | n-ary product |
| `int`   | ∫      | integral |
| `partial` | ∂    | partial differential |
| `nabla` | ∇      | nabla |
| `in`    | ∈      | element of |
| `notin` | ∉      | not an element of |
| `subset` | ⊂     | subset of |
| `superset` | ⊃   | superset of |
| `union` | ∪      | union |
| `intersect` | ∩  | intersection |
| `forall` | ∀     | for all |
| `exists` | ∃     | there exists |
| `emptyset` | ∅   | empty set |
| `propto` | ∝     | proportional to |

**Example:**

```bash
$ giv chars sqrt sum int partial nabla in subset union forall exists
√ ∑ ∫ ∂ ∇ ∈ ⊂ ∪ ∀ ∃
```

## Greek Letters (Lowercase)

| Pattern | Result | Description |
|---------|--------|-------------|
| `alpha`   | α    | greek small letter alpha |
| `beta`    | β    | greek small letter beta |
| `gamma`   | γ    | greek small letter gamma |
| `delta`   | δ    | greek small letter delta |
| `epsilon` | ε    | greek small letter epsilon |
| `zeta`    | ζ    | greek small letter zeta |
| `eta`     | η    | greek small letter eta |
| `theta`   | θ    | greek small letter theta |
| `iota`    | ι    | greek small letter iota |
| `kappa`   | κ    | greek small letter kappa |
| `lambda`, `lamda` | λ | greek small letter lambda |
| `mu`      | μ    | greek small letter mu |
| `nu`      | ν    | greek small letter nu |
| `xi`      | ξ    | greek small letter xi |
| `omicron` | ο    | greek small letter omicron |
| `pi`      | π    | greek small letter pi |
| `rho`     | ρ    | greek small letter rho |
| `sigma`   | σ    | greek small letter sigma |
| `tau`     | τ    | greek small letter tau |
| `upsilon` | υ    | greek small letter upsilon |
| `phi`     | φ    | greek small letter phi |
| `chi`     | χ    | greek small letter chi |
| `psi`     | ψ    | greek small letter psi |
| `omega`   | ω    | greek small letter omega |

**Example:**

```bash
$ giv chars alpha beta gamma delta lambda mu pi omega
α β γ δ λ μ π ω
```

## Greek Letters (Uppercase)

| Pattern | Result | Description |
|---------|--------|-------------|
| `Alpha`   | Α    | greek capital letter alpha |
| `Beta`    | Β    | greek capital letter beta |
| `Gamma`   | Γ    | greek capital letter gamma |
| `Delta`   | Δ    | greek capital letter delta |
| `Theta`   | Θ    | greek capital letter theta |
| `Lambda`, `Lamda` | Λ | greek capital letter lambda |
| `Pi`      | Π    | greek capital letter pi |
| `Sigma`   | Σ    | greek capital letter sigma |
| `Phi`     | Φ    | greek capital letter phi |
| `Psi`     | Ψ    | greek capital letter psi |
| `Omega`   | Ω    | greek capital letter omega |

**Example:**

```bash
$ giv chars Alpha Beta Gamma Delta Sigma Omega
Α Β Γ Δ Σ Ω
```

## Punctuation and Symbols

| Pattern | Result | Description |
|---------|--------|-------------|
| `section`, `sect` | § | section sign |
| `para`, `paragraph` | ¶ | pilcrow sign |
| `dag`, `dagger` | † | dagger |
| `ddag` | ‡ | double dagger |
| `bullet` | • | bullet |
| `middot` | · | middle dot |

**Example:**

```bash
$ giv chars section para dag ddag bullet middot
§ ¶ † ‡ • ·
```

## Superscripts

| Pattern | Result | Description |
|---------|--------|-------------|
| `^0` | ⁰ | superscript zero |
| `^1` | ¹ | superscript one |
| `^2` | ² | superscript two |
| `^3` | ³ | superscript three |
| `^4` | ⁴ | superscript four |
| `^5` | ⁵ | superscript five |
| `^6` | ⁶ | superscript six |
| `^7` | ⁷ | superscript seven |
| `^8` | ⁸ | superscript eight |
| `^9` | ⁹ | superscript nine |
| `^n` | ⁿ | superscript n |

**Example:**

```bash
$ giv chars "^2" "^3" "^n"
² ³ ⁿ
```

## Subscripts

| Pattern | Result | Description |
|---------|--------|-------------|
| `_0` | ₀ | subscript zero |
| `_1` | ₁ | subscript one |
| `_2` | ₂ | subscript two |
| `_3` | ₃ | subscript three |
| `_4` | ₄ | subscript four |
| `_5` | ₅ | subscript five |
| `_6` | ₆ | subscript six |
| `_7` | ₇ | subscript seven |
| `_8` | ₈ | subscript eight |
| `_9` | ₉ | subscript nine |

**Example:**

```bash
$ giv chars "_0" "_1" "_2"
₀ ₁ ₂
```

## Miscellaneous

| Pattern | Result | Description |
|---------|--------|-------------|
| `star` | ★ | black star |

**Example:**

```bash
$ giv chars star
★
```

## HTML Entities

The `chars` command supports 2200+ HTML named character references. Here are some commonly used ones:

| Pattern | Result | Description |
|---------|--------|-------------|
| `&nbsp;` | (non-breaking space) | html named character reference |
| `&lt;` | < | html named character reference |
| `&gt;` | > | html named character reference |
| `&amp;` | & | html named character reference |
| `&copy;` | © | html named character reference |
| `&reg;` | ® | html named character reference |
| `&alpha;` | α | html named character reference |
| `&beta;` | β | html named character reference |
| `&rarr;` | → | html named character reference |

**Example:**

```bash
$ giv chars "&lt;" "&gt;" "&amp;" "&copy;"
< > & ©
```

For a complete list of HTML entities, see:

- [penmanship HTML entities documentation](https://github.com/theroyalwhee0/penmanship/blob/main/docs/html-entities.md)

## Common Emoji Shortcodes

The `chars` command supports all emoji shortcodes from the GitHub emoji set. Here are some commonly used ones:

### Smileys & Emotion

| Shortcode | Result | Name |
|-----------|--------|------|
| `:smile:` | 😄 | grinning face with smiling eyes |
| `:grin:` | 😁 | beaming face with smiling eyes |
| `:joy:` | 😂 | face with tears of joy |
| `:heart_eyes:` | 😍 | smiling face with heart-eyes |
| `:thinking:` | 🤔 | thinking face |
| `:thumbsup:` | 👍 | thumbs up |
| `:thumbsdown:` | 👎 | thumbs down |
| `:clap:` | 👏 | clapping hands |
| `:wave:` | 👋 | waving hand |
| `:heart:` | ❤️ | red heart |
| `:fire:` | 🔥 | fire |
| `:star:` | ⭐ | star |

### Common Symbols

| Shortcode | Result | Name |
|-----------|--------|------|
| `:rocket:` | 🚀 | rocket |
| `:tada:` | 🎉 | party popper |
| `:sparkles:` | ✨ | sparkles |
| `:zap:` | ⚡ | high voltage |
| `:boom:` | 💥 | collision |
| `:bulb:` | 💡 | light bulb |
| `:warning:` | ⚠️ | warning |
| `:check:` | ✅ | check mark button |
| `:x:` | ❌ | cross mark |
| `:question:` | ❓ | red question mark |

### Technology

| Shortcode | Result | Name |
|-----------|--------|------|
| `:computer:` | 💻 | laptop |
| `:keyboard:` | ⌨️ | keyboard |
| `:phone:` | 📱 | mobile phone |
| `:email:` | 📧 | e-mail |
| `:gear:` | ⚙️ | gear |
| `:wrench:` | 🔧 | wrench |
| `:hammer:` | 🔨 | hammer |
| `:lock:` | 🔒 | locked |
| `:unlock:` | 🔓 | unlocked |
| `:key:` | 🔑 | key |

**Example:**

```bash
$ giv chars :smile: :rocket: :thumbsup: :fire:
😄 🚀 👍 🔥
```

## Mixed Examples

You can combine emoji and character patterns in a single command:

```bash
$ giv chars 1/4 :smile: "(c)" alpha lambda
¼ 😄 © α λ
```

```bash
$ giv chars :rocket: "->" :star: "+" :sparkles:
🚀 → ⭐ + ✨
```

## JSON Output

All conversions can be output as JSON with metadata. The `type` field indicates whether the input was a pattern, HTML entity, or emoji:

```bash
$ giv --json chars alpha "&nbsp;" :smile:
[
  {
    "input": "alpha",
    "output": "α",
    "type": "pattern",
    "name": "greek small letter alpha"
  },
  {
    "input": "&nbsp;",
    "output": " ",
    "type": "html",
    "name": "html named character reference"
  },
  {
    "input": ":smile:",
    "output": "😄",
    "type": "emoji",
    "name": "grinning face with smiling eyes"
  }
]
```

## Error Handling

If an unknown pattern or emoji shortcode is provided, the command will exit with an error:

```bash
$ giv chars unknown
Error: Unknown character pattern or emoji shortcode: 'unknown'
```

## Finding More Patterns

### Character Patterns

For the complete list of supported character patterns (punctuation, math symbols, Greek letters, fractions, currency, etc.), see:

- [penmanship character mappings](https://github.com/theroyalwhee0/penmanship/blob/main/docs/mappings.md)

### HTML Entities

For the complete list of 2200+ HTML named character references, see:

- [penmanship HTML entities](https://github.com/theroyalwhee0/penmanship/blob/main/docs/html-entities.md)

### Emoji Shortcodes

GitHub-compatible emoji shortcodes are supported. The full list of supported emoji shortcodes can be found at:

- <https://github.com/ikatyang/emoji-cheat-sheet>

## Implementation Details

Character lookup is provided by:

- [penmanship](https://github.com/theroyalwhee0/penmanship) - Unicode character lookup library

Emoji support is provided by:

- <https://github.com/rossmacarthur/emojis>
