# clc

[![clc on crates.io](https://img.shields.io/crates/v/clc)](https://crates.io/crates/clc)
[![Documentation (latest release)](https://img.shields.io/badge/docs-latest-brightgreen.svg)](https://docs.rs/clc/)
[![Changelog](https://img.shields.io/badge/changelog-latest-blue)](https://github.com/ymgyt/clc/blob/main/CHANGELOG.md)
[![License](https://img.shields.io/badge/license-Apache-green.svg)](LICENSE-APACHE)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE-MIT)

Clc is a Command line calculator written in Rust 🦀

## Usage
```
$ clc
Version: v0.1.2
To quit, press Ctrl+C or type quit
❯ sqrt(sqrt(16)) * (100 - 1) * (100 + 1) / 9
2222
❯ quit
bye
```

### Supported Functions

| identifier | description                                                                          |
|------------|--------------------------------------------------------------------------------------|
| `sqrt()`   | Returns the square root of a number <br />Return `NaN` if a negative number provided |

## Install

### Cargo

```shell
cargo install clc
```

### Linux

```shell
curl -sSLf https://github.com/ymgyt/calculator/releases/download/0.1.2/clc-x86_64-unknown-linux-gnu.tar.gz | tar zxf - -C /usr/local/bin
```

### Mac

```shell
curl -sSLf https://github.com/ymgyt/calculator/releases/download/0.1.2/clc-x86_64-apple-darwin.tar.gz | tar zxf - -C /usr/local/bin
```

## License

This project is available under the terms of either the [Apache 2.0 license](../LICENSE-APACHE) or the [MIT license](../LICENSE-MIT).
