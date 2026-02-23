# mad

[![Crates.io](https://img.shields.io/crates/v/mad.svg)](https://crates.io/crates/mad)
[![docs.rs](https://docs.rs/mad/badge.svg)](https://docs.rs/mad)
[![CI](https://github.com/macalinao/mad/actions/workflows/ci.yml/badge.svg)](https://github.com/macalinao/mad/actions/workflows/ci.yml)
[![License](https://img.shields.io/crates/l/mad.svg)](https://github.com/macalinao/mad/blob/master/LICENSE)

A fast Markdown terminal renderer with syntax highlighting

Renders Markdown files as beautifully formatted ANSI terminal output with syntax-highlighted code blocks, clickable hyperlinks, tables, and intelligent text wrapping.

## Installation

### Cargo

```bash
cargo install mad
```

### Nix

```bash
nix run github:macalinao/mad -- README.md
nix profile install github:macalinao/mad
```

## Usage

```bash
# Render a Markdown file
mad README.md

# Pipe from stdin
cat README.md | mad

# Disable syntax highlighting
mad --no-highlight README.md

# Set a custom width
mad --width 100 README.md
```

### Options

| Flag | Short | Description |
| --- | --- | --- |
| \`--no-highlight\` | \`-n\` | Disable syntax highlighting for code blocks |
| \`--width <COLS>\` | \`-w\` | Wrap text to specified width (defaults to terminal width) |
| \`--version\` | | Print version |
| \`--help\` | \`-h\` | Print help |

## License

Apache-2.0
