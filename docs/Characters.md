# Characters and Emoji Reference

This document lists all supported character patterns and common emoji shortcodes for the `giv chars` command.

## Usage

```bash
giv chars <pattern1> <pattern2> ...
giv --json chars <pattern1> <pattern2> ...
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
| `--`    | —      | em dash |

**Example:**
```bash
$ giv chars "..." "--"
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

**Example:**
```bash
$ giv chars cent pound euro yen
¢ £ € ¥
```

## Math Symbols

| Pattern | Aliases | Result | Description |
|---------|---------|--------|-------------|
| `degree` | `deg` | °      | degree sign |
| `+-`    | -     | ±      | plus-minus sign |
| `x`     | `*`   | ×      | multiplication sign |
| `divide` | `div` | ÷      | division sign |
| `!=`    | `ne`  | ≠      | not equal to |
| `lte`   | -     | ≤      | less-than or equal to |
| `gte`   | -     | ≥      | greater-than or equal to |
| `~=`    | -     | ≈      | almost equal to |
| `infinity` | `inf` | ∞   | infinity |

**Example:**
```bash
$ giv chars degree "+-" x divide "!=" infinity
° ± × ÷ ≠ ∞
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

All conversions can be output as JSON with metadata:

```bash
$ giv --json chars alpha :smile: 1/4
[
  {
    "input": "alpha",
    "output": "α",
    "type": "pattern",
    "name": "greek small letter alpha"
  },
  {
    "input": ":smile:",
    "output": "😄",
    "type": "emoji",
    "name": "grinning face with smiling eyes"
  },
  {
    "input": "1/4",
    "output": "¼",
    "type": "pattern",
    "name": "fraction one quarter"
  }
]
```

## Error Handling

If an unknown pattern or emoji shortcode is provided, the command will exit with an error:

```bash
$ giv chars unknown
Error: Unknown character pattern or emoji shortcode: 'unknown'
```

## Finding More Emoji

The full list of supported emoji shortcodes can be found at:
- https://github.com/ikatyang/emoji-cheat-sheet
- https://www.webfx.com/tools/emoji-cheat-sheet/

All GitHub-compatible emoji shortcodes are supported.
