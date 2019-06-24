A drop-in global allocator wrapper around the [mimalloc](https://github.com/microsoft/mimalloc) allocator.
Mimalloc is a general purpose, performance oriented allocator built by Microsoft.

## Usage
```rust
use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
```

## Usage without secure mode
By default this library builds mimalloc in safe-mode. This means that
heap allocations are encrypted, but this results in a 3% increase in overhead.

In `Cargo.toml`:
```rust
[dependencies]
mimalloc = { version = "*", features = ["no_secure"] }
```
