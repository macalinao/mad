# ansi-term-styles

[![Crates.io](https://img.shields.io/crates/v/ansi-term-styles.svg)](https://crates.io/crates/ansi-term-styles)
[![docs.rs](https://docs.rs/ansi-term-styles/badge.svg)](https://docs.rs/ansi-term-styles)
[![CI](https://github.com/macalinao/mad/actions/workflows/ci.yml/badge.svg)](https://github.com/macalinao/mad/actions/workflows/ci.yml)
[![License](https://img.shields.io/crates/l/ansi-term-styles.svg)](https://github.com/macalinao/mad/blob/master/LICENSE)

ANSI terminal escape code constants for styling text

A `no_std` crate providing constants for common ANSI escape sequences. No allocations, no dependencies.

## Available Constants

**Text attributes:** `BOLD`, `DIM`, `ITALIC`, `UNDERLINE`, `RESET`

**Foreground colors:** `RED`, `GREEN`, `YELLOW`, `BLUE`, `MAGENTA`, `CYAN`

**Bold colors:** `BOLD_RED`, `BOLD_GREEN`

**OSC 8 hyperlinks:** `LINK_START`, `LINK_MID`, `LINK_END`

## Usage

```rust
use ansi_term_styles::{BOLD, RESET, BLUE, LINK_START, LINK_MID, LINK_END, UNDERLINE};

// Bold text
print!("{BOLD}hello{RESET}");

// Colored text
print!("{BLUE}info{RESET}");

// Clickable terminal hyperlink (OSC 8)
print!("{LINK_START}https://example.com{LINK_MID}{UNDERLINE}click here{RESET}{LINK_END}");
```

## License

Apache-2.0
