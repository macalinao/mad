# mad

**Markdown in your terminal, beautifully rendered.**

`mad` is a fast Markdown terminal renderer written in Rust. It takes Markdown files and renders them as richly formatted ANSI output with syntax-highlighted code blocks, clickable hyperlinks, tables, and intelligent text wrapping.

## Features

- **Syntax-highlighted code blocks** powered by [syntect](https://github.com/trishume/syntect) with the base16-ocean.dark theme and 50+ extra language syntaxes
- **Clickable hyperlinks** via [OSC 8](https://gist.github.com/egmontkob/eb114294efbcd5adb1944c9f3cb5feda) terminal sequences
- **Tables** with Unicode box-drawing borders, alignment, and bold headers
- **Smart text wrapping** respects terminal width and preserves ANSI escape sequences
- **Headings, bold, italic, lists** all rendered with proper ANSI formatting
- **Reads from files or stdin** for easy integration with pipes and scripts
- **Fast** no runtime overhead from syntax parsing; extra syntaxes are precompiled at build time

## Installation

### From source

```bash
cargo install mad
```

### With Nix

```bash
# Run directly
nix run github:macalinao/mad -- README.md

# Install into your profile
nix profile install github:macalinao/mad
```

### As a Nix flake input

```nix
{
  inputs.mad.url = "github:macalinao/mad";

  outputs = { mad, ... }: {
    # Use mad.packages.${system}.default
  };
}
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
| `--no-highlight` | `-n` | Disable syntax highlighting for code blocks |
| `--width <COLS>` | `-w` | Wrap text to specified width (defaults to terminal width) |
| `--version` | | Print version |
| `--help` | `-h` | Print help |

## Crates

| Crate | Description |
| --- | --- |
| [`mad`](crates/mad) | CLI binary |
| [`markdown-to-ansi`](crates/markdown-to-ansi) | Library: render Markdown to ANSI terminal text |
| [`ansi-term-styles`](crates/ansi-term-styles) | `no_std` ANSI escape code constants |
| [`sublime-syntaxes`](crates/sublime-syntaxes) | Precompiled extra syntax definitions for syntect |

## Library Usage

The rendering engine is available as a standalone library:

```rust
use markdown_to_ansi::{render, Options};

let opts = Options {
    syntax_highlight: true,
    width: Some(80),
};

let output = render("# Hello\n\nThis is **bold**.", &opts);
println!("{output}");
```

## License

Apache-2.0
