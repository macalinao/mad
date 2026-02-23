# markdown-to-ansi

[![Crates.io](https://img.shields.io/crates/v/markdown-to-ansi.svg)](https://crates.io/crates/markdown-to-ansi)
[![docs.rs](https://docs.rs/markdown-to-ansi/badge.svg)](https://docs.rs/markdown-to-ansi)
[![CI](https://github.com/macalinao/mad/actions/workflows/ci.yml/badge.svg)](https://github.com/macalinao/mad/actions/workflows/ci.yml)
[![License](https://img.shields.io/crates/l/markdown-to-ansi.svg)](https://github.com/macalinao/mad/blob/master/LICENSE)

Render Markdown as ANSI-formatted terminal text

Converts `CommonMark` Markdown into richly formatted ANSI terminal output. Powered by [pulldown-cmark](https://github.com/pulldown-cmark/pulldown-cmark) for parsing and [syntect](https://github.com/trishume/syntect) for syntax highlighting (base16-ocean.dark theme).

### Supported elements

- **Headings** (bold)
- **Bold**, _italic_, and inline `code` (colored)
- **Links** rendered as clickable [OSC 8](https://gist.github.com/egmontkob/eb114294efbcd5adb1944c9f3cb5feda) terminal hyperlinks
- **Fenced code blocks** with syntax highlighting and background padding
- **Tables** with Unicode box-drawing borders, alignment, and bold headers
- **Ordered and unordered lists** with nested indentation
- **Text wrapping** that respects terminal width and preserves ANSI escapes

### API

- `render(text, opts)` -- full block-level rendering (paragraphs, headings, code blocks, lists, tables)
- `render_inline(text, opts)` -- inline-only rendering (bold, italic, code spans, links)
- `has_syntax(lang)` -- check if a language token has a syntax definition

## Usage

```rust
use markdown_to_ansi::{render, render_inline, Options};

let opts = Options {
    syntax_highlight: true,
    width: Some(80),
    code_bg: true,
};

// Full document rendering
let output = render("# Hello

This is **bold**.", &opts);
println!("{output}");

// Inline-only rendering (no paragraph wrappers)
let inline = render_inline("Use `foo` for **bar**", &opts);
println!("{inline}");
```

## License

Apache-2.0
