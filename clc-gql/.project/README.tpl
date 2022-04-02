# 🕸 {{crate}} - Clc GraphQL

[![clc on crates.io](https://img.shields.io/crates/v/clc-gql)](https://crates.io/crates/clc)
[![Documentation (latest release)](https://img.shields.io/badge/docs-latest-brightgreen.svg)](https://docs.rs/clc-gql/)
[![Changelog](https://img.shields.io/badge/changelog-latest-blue)](https://github.com/ymgyt/clc/blob/main/clc-gql/CHANGELOG.md)
[![License](https://img.shields.io/badge/license-Apache-green.svg)](LICENSE-APACHE)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE-MIT)

{{readme}}

## Features

* `cache`
  * If enabled, cache the result to the redis specified by `CLCGQL_REDIS_ENDPOINT`

## Environment variables

| Key                     | Description             | Example                       | 
|-------------------------|-------------------------|-------------------------------|
| `CLCGQL_LOG`            | Rust logging directive. | `info`                        | 
| `CLCGQL_LOG_COLOR`      | Control logging color   | `false`                       | 
| `CLCGQL_REDIS_ENDPOINT` | Redis connection info`  | `redis://cache.ymgyt.io:6376` |

## 🪪 License

This project is available under the terms of either the [Apache 2.0 license](../LICENSE-APACHE) or the [MIT license](../LICENSE-MIT).

