# Taplo

The core Rust library.

The parser is a mostly hand-written recursive descent parser, and the tokenization is done by [Logos](https://github.com/maciejhirsz/logos).
The parser then creates a [Rowan](https://github.com/rust-analyzer/rowan) syntax tree, every further functionality is based on this syntax tree, including both the DOM and the formatter.
