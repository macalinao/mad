# Welcome to mad

**`cat`, but for Markdown.** Render any Markdown file beautifully in your terminal.

## Features

- **Syntax highlighting** for 50+ languages
- **Clickable hyperlinks** via OSC 8
- **Unicode tables** with box-drawing borders
- Smart **text wrapping** that respects terminal width

## Quick Start

```bash
# Install with cargo
cargo install mad

# Render a file
mad README.md

# Pipe from stdin
curl -s https://example.com/doc.md | mad
```

## Code Highlighting

```rust
fn main() {
    let greeting = "Hello, world!";
    println!("{greeting}");

    for i in 0..5 {
        println!("  count: {i}");
    }
}
```

```python
def fibonacci(n: int) -> list[int]:
    """Generate the first n Fibonacci numbers."""
    seq = [0, 1]
    for _ in range(n - 2):
        seq.append(seq[-1] + seq[-2])
    return seq[:n]

print(fibonacci(10))
```

## Options

| Flag             | Short | Description                 |
| ---------------- | ----- | --------------------------- |
| `--no-highlight` | `-n`  | Disable syntax highlighting |
| `--width`        | `-w`  | Set custom wrap width       |
| `--help`         | `-h`  | Print help                  |
