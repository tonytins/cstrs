# CST.rs

[![GitHub license](https://img.shields.io/github/license/tonytins/cst.rs)](https://github.com/tonytins/cst.rs/blob/main/LICENSE) ![GitHub Workflow Status (branch)](https://img.shields.io/github/workflow/status/tonytins/cst.rs/Rust/main) ![GitHub commit activity](https://img.shields.io/github/commit-activity/w/tonytins/cst.rs)  [![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-v2.0%20adopted-ff69b4.svg)](code_of_conduct.md)

Caret-Separated Text (or CST) is a key-value pair format represented by digits or words as keys and the value as text enclosed between carets. (e.g. ``<key> ^<text>^``) Any text which is not enclosed with carets is considered a comment and ignored. Neither strings nor comments may use the caret character. CST.NET is a library for parsing the CST format.

## Requirements

- Rust 2021 Edition or newer
- IDEs or Editors
  - [Visual Studio Code](https://code.visualstudio.com/)
  - [InteliJ IDEA](https://www.jetbrains.com/idea/) or [CLion](https://www.jetbrains.com/clion/)

## Usage

### Basic Parsing

If you want to create your own internal parsing framework for CST files, you can use the ``get_entry()`` function.

```rust
use cst::get_entry;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let example = "1 ^The quick brown fox jumps over the lazy dog.^";
    let entry = get_entry(example, 1);
    println!("{}", entry);
}
```

### In Production

Based on FreeSO's APIs, the UIText struct takes care of the heavy lifting of locating and parsing CST files.

```rust
use cst::UIText;

 // english.dir
let ui_text = UIText::new("english");

// entry 1 of _101_[name].cst, where [name] is whatever 
// you want to name the file.
let entry = ui_text.get_entry(101, 1);

println!("{}", entry);
```

In The Sims Online, it was required translation were prefixed with numbers enclosed in underscores, known as the ID. The IDs were used to locate the right file without having to remember it's name. Meanwhile, each translation was split into their respective ``uitext/<languae>.dir`` directories:

- ``uitext/english.dir/_154_miscstrings.cst``
- ``uitext/swedish.dir/_154_miscstrings.cst``

Note that that ``UIText`` class uses the above mentioned ``CST.Parse()`` method to parse the files.

### Examples

More complex stuff can be found in the [examples](./examples) directory.

## Note

For the most part, this is to be a architecturally similar port of [CST.NET](https://github.com/tonytins/cst.rs). With some changes to make it more idiomatic in Rust.

## License

This project is dual-licensed under the [BSD-3-Clause](COPYING) or the [UNLICENSE](UNLICENSE).
