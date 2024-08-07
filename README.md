# str-escape

Escapes stdin input according to [these rules](https://doc.rust-lang.org/std/primitive.char.html#method.escape_default) and outputs it to stdout.

## Installation

This tool requires the [Rust](https://www.rust-lang.org) compiler toolchain. Install it via [rustup](https://rustup.rs).

Run in this repository:
```sh
cargo install --path .
```

## Usage

```sh
some-cli-tool | str-escape | some-other-cli-tool
```

## [License](LICENSE.md)
