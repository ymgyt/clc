# 🧮 {{crate}} - Command Line Calculator 

[![clc on crates.io](https://img.shields.io/crates/v/clc)](https://crates.io/crates/clc)
[![Documentation (latest release)](https://img.shields.io/badge/docs-latest-brightgreen.svg)](https://docs.rs/clc/)
[![Changelog](https://img.shields.io/badge/changelog-latest-blue)](https://github.com/ymgyt/clc/blob/main/CHANGELOG.md)
[![License](https://img.shields.io/badge/license-Apache-green.svg)](LICENSE-APACHE)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE-MIT)

{{readme}}

---
# 📖 Table of Contents (Toc)
💻 [Installation](#-installation)  
👩‍💻 [Usage](#-usage)  
🦀 [Using Clc in Rust Codes](#-using-clc-in-rust)  
🕸 [Access clc via GraphQL](#-access-clc-via-graphql)  
🪪 [License](#-license)  
---

## 💻 Installation 
There are prebuilt x86-64 binaries for Linux, macOS and Windows [on the release page](https://github.com/ymgyt/clc/releases/tag/v{{version}}).  
You can install the latest release from source using cargo, or build directly from a source checkout.

### 📦 Via cargo

```shell
cargo install clc
```

### 🐧 Linux

```shell
curl -sSLf https://github.com/ymgyt/calculator/releases/download/{{version}}/clc-x86_64-unknown-linux-gnu.tar.gz | tar zxf - -C /usr/local/bin
```

### 🍎 Mac

```shell
curl -sSLf https://github.com/ymgyt/calculator/releases/download/{{version}}/clc-x86_64-apple-darwin.tar.gz | tar zxf - -C /usr/local/bin
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
Version: v{{version}}
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

## 🦀 Using Clc in Rust

Clc is also available in Rust codes with [`clc-engine`]

```rust
use clc_engine::Calculator;

fn main() {
    let clc = Calculator::new();
    let eval = clc.calculate_line("sqrt(sqrt(16)) * (4 + 2)");

    assert_eq!(eval, Ok(12.));
}
```

## 🕸 Access clc via GraphQL

[`clc-gql`]exposes clc with graphql api.  
To run [`clc-gql`] with docker ,execute the following command.
```shell
docker run --rm --publish 9696:9696 ghcr.io/ymgyt/clc-gql:latest
```

then access 
```shell
curl \
  -X POST \
  -d '{"query": "{ eval(expression: \"100 - 1\") }"}' \
  http://localhost:9696/graphql

{"data":{"eval":99.0}}
```

playground can be accessed from `localhost:9696/graphsql/playground`.

## 🪪 License

This project is available under the terms of either the [Apache 2.0 license](../LICENSE-APACHE) or the [MIT license](../LICENSE-MIT).

[`clc-engine`]: https://github.com/ymgyt/clc/clc-engine
[`clc-gql`]: https://github.com/ymgyt/clc/clc-gql
