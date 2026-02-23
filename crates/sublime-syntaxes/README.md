# sublime-syntaxes

[![Crates.io](https://img.shields.io/crates/v/sublime-syntaxes.svg)](https://crates.io/crates/sublime-syntaxes)
[![docs.rs](https://docs.rs/sublime-syntaxes/badge.svg)](https://docs.rs/sublime-syntaxes)
[![CI](https://github.com/macalinao/mad/actions/workflows/ci.yml/badge.svg)](https://github.com/macalinao/mad/actions/workflows/ci.yml)
[![License](https://img.shields.io/crates/l/sublime-syntaxes.svg)](https://github.com/macalinao/mad/blob/master/LICENSE)

Precompiled Sublime Text syntax definitions for languages not in syntect's defaults

Provides precompiled Sublime Text `.sublime-syntax` definitions for languages not included in [syntect](https://github.com/trishume/syntect)'s default set. Sourced from [bat](https://github.com/sharkdp/bat)'s syntax collection.

Syntax files in the `syntaxes/` directory are compiled to a binary blob at build time and deserialized at runtime via `syntect::dumps`, avoiding the cost of parsing YAML definitions on every startup.

## Usage

```rust
let extra_syntaxes = sublime_syntaxes::extra_syntax_set();
// Merge with syntect defaults or use standalone
```

## License

Apache-2.0
