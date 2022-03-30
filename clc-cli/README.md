# 🧮 clc - Command Line Calculator 

[![clc on crates.io](https://img.shields.io/crates/v/clc)](https://crates.io/crates/clc)
[![Documentation (latest release)](https://img.shields.io/badge/docs-latest-brightgreen.svg)](https://docs.rs/clc/)
[![Changelog](https://img.shields.io/badge/changelog-latest-blue)](https://github.com/ymgyt/clc/blob/main/CHANGELOG.md)
[![License](https://img.shields.io/badge/license-Apache-green.svg)](LICENSE-APACHE)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE-MIT)

Clc is a Command line calculator written in Rust 🦀
It eval given expression and print result.

---
# 📖 Table of Contents (Toc)
[Installation](#installation)  
[Usage](#usage)  
[License](#license)  
---

## 💻 Installation 
There are prebuilt x86-64 binaries for Linux, macOS and Windows [on the release page](https://github.com/ymgyt/clc/releases/tag/v0.1.4).  
You can install the latest release from source using cargo, or build directly from a source checkout.

### 📦 Via cargo

```shell
cargo install clc
```

### 🐧 Linux

```shell
curl -sSLf https://github.com/ymgyt/calculator/releases/download/0.1.4/clc-x86_64-unknown-linux-gnu.tar.gz | tar zxf - -C /usr/local/bin
```

### 🍎 Mac

```shell
curl -sSLf https://github.com/ymgyt/calculator/releases/download/0.1.4/clc-x86_64-apple-darwin.tar.gz | tar zxf - -C /usr/local/bin
```

### 🐳 Docker

To use clc with docker, execute the following command.

```shell
docker run --rm -it ghcr.io/ymgyt/clc:latest
```


## 👩‍💻 Usage
Execute the `clc` command to start a repl session. type the expression to evaluate and press Enter.  
To exit the session, type `quit` or press Ctrl + C.

```text
$ clc
Version: v0.1.4
To quit, press Ctrl+C or type quit
❯ sqrt(sqrt(16)) * (100 - 1) * (100 + 1) / 9
2222
❯ quit
bye
```

You can also retrieve results directly without using a repl session. pass the expression to evaluate with `--eval` flag

```shell
clc --eval 'sqrt(-2^2) - abs(2)'
```

### 🏃 Lambda expression
Some functions take lambda expression as argument.
A lambda expression is written like `|x| { x^2 }`.  
`{`,`}` are optional, so the above expression can also be written as follows `|x| x^2`.
```shell
❯ sig(1,10 |x| x^2)
385

❯ sig(1,10 |x| sig(1,10 |y| x*y))
3025
```

### 🍴 Supported Functions

| identifier        | description                                                                          |
|-------------------|--------------------------------------------------------------------------------------|
| `sqrt(n)`         | Returns the square root of a number <br />Return `NaN` if a negative number provided |
| `pow(n,m)`        | Raise n to the power of m. (= `n ^ m`)                                               |
| `abs(n)`          | Compute the absolute value of n                                                      |
 | `sig(n,m,lambda)` | Execute lambda with values from n to m and return the sum of the results             |            |                                                                                 |

### 🥣 Constants

| identifier | description              |
|------------|--------------------------|
| `pi`       | Archimedes’ constant (π) |
| `e`        | Euler’s number (e)       |

## 🪪 License

This project is available under the terms of either the [Apache 2.0 license](../LICENSE-APACHE) or the [MIT license](../LICENSE-MIT).
