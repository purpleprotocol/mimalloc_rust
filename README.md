# Mimalloc Rust

[![Latest Version]][crates.io] [![MSRV]][crates.io] [![Documentation]][docs.rs]

A drop-in global allocator wrapper around the [mimalloc][gh-mimalloc] allocator.
Mimalloc is a general purpose, performance oriented allocator built by Microsoft.

## Usage

```rust
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
```

## Requirements

A __C__ compiler is required for building [mimalloc][gh-mimalloc] with cargo.

## Usage with secure mode

Using secure mode adds guard pages,
randomized allocation, encrypted free lists, etc. The performance penalty is usually
around 10% according to [mimalloc][gh-mimalloc]
own benchmarks.

To enable secure mode, put in `Cargo.toml`:

```toml
[dependencies]
mimalloc = { version = "*", features = ["secure"] }
```

## Usage with v2

By default this library uses mimalloc `v3`.
To use MiMalloc `v2`, write in `Cargo.toml`:

```toml
[dependencies]
mimalloc = { version = "*", features = ["v2"] }
```

[crates.io]: https://crates.io/crates/mimalloc
[Latest Version]: https://img.shields.io/crates/v/mimalloc.svg
[Documentation]: https://docs.rs/mimalloc/badge.svg
[docs.rs]: https://docs.rs/mimalloc
[MSRV]: https://img.shields.io/crates/msrv/mimalloc.svg

[gh-mimalloc]: https://github.com/microsoft/mimalloc