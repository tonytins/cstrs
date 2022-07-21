# CST.rs

Caret-Separated Text (or CST) is a key-value pair format represented by digits or words as keys and the value as text enclosed between carets. (e.g. ``<key> ^<text>^``) Any text which is not enclosed with carets is considered a comment and ignored. Neither strings nor comments may use the caret character. CST.NET is a library for parsing the CST format.

## Requirements

- Rust 2021 Edition or newer
- IDEs or Editors
  - [Visual Studio Code](https://code.visualstudio.com/)
  - [InteliJ IDEA](https://www.jetbrains.com/idea/) or [CLion](https://www.jetbrains.com/clion/)

## Usage

See [usage.md](./usage.md).

## Note

For the most part, this is to be a architecturally similar port of [CST.NET](https://github.com/tonytins/cstdotnet). With some changes to make it more idiomatic in Rust.

## License

I license this project under the BSD-3-Clause license - see [LICENSE](LICENSE) for details.
