# Mimalloc Rust

[![Build Status](https://travis-ci.org/purpleprotocol/mimalloc_rust.svg?branch=master)](https://travis-ci.org/purpleprotocol/mimalloc_rust) [![Latest Version]][crates.io] [![Documentation]][docs.rs]

A drop-in global allocator wrapper around the [mimalloc](https://github.com/microsoft/mimalloc) allocator.
Mimalloc is a general purpose, performance oriented allocator built by Microsoft.

## Usage

```rust
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
```

## Requirements

[CMake](https://cmake.org/) and a __C__ compiler are required for building 
[mimalloc](https://github.com/microsoft/mimalloc) with cargo.

## Usage without secure mode

By default this library builds mimalloc in secure mode. This add guard pages, 
randomized allocation, encrypted free lists, etc. The performance penalty is usually
around 10% according to [mimalloc](https://github.com/microsoft/mimalloc)
own benchmarks.

To disable secure mode, put in `Cargo.toml`:
```rust
[dependencies]
mimalloc = { version = "*", default-features = false }
```

[crates.io]: https://crates.io/crates/mimalloc
[Latest Version]: https://img.shields.io/crates/v/mimalloc.svg
[Documentation]: https://docs.rs/mimalloc/badge.svg
[docs.rs]: https://docs.rs/mimalloc
