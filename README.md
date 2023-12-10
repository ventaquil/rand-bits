# rand-bits

[![crates.io](https://img.shields.io/crates/v/rand-bits?style=flat-square&logo=rust "crates.io")](https://crates.io/crates/rand-bits)
[![docs.rs](https://img.shields.io/docsrs/rand-bits?style=flat-square&logo=docsdotrs "docs.rs")](https://docs.rs/rand-bits)
[![MSRV](https://img.shields.io/badge/MSRV-1.61.0-informational?style=flat-square "MSRV")](https://github.com/ventaquil/rand-bits/blob/master/Cargo.toml)
[![deps.rs](https://deps.rs/crate/rand-bits/0.1.0/status.svg?style=flat-square "deps.rs")](https://deps.rs/crate/rand-bits/0.1.0)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg?style=flat-square "unsafe forbidden")](https://github.com/rust-secure-code/safety-dance)
[![LICENSE](https://img.shields.io/github/license/ventaquil/rand-bits?style=flat-square "LICENSE")](https://github.com/ventaquil/rand-bits/blob/master/LICENSE)

Random number generators with a fixed number of set bits (ones).

## Setup

Add the following entry to the `dependencies` section of your `Cargo.toml` file:

```toml
[dependencies]
rand-bits = "0.1.0"
```

Alternatively, you can use the [`cargo add`](https://doc.rust-lang.org/cargo/commands/cargo-add.html) subcommand:

```shell
cargo add rand-bits
```

## Usage

```rust
use rand::thread_rng;
use rand_bits::RngBits;

let mut rng = thread_rng();
let x: u8 = rng.gen_bits(4); // generates a u8 with 4 set bits
assert_eq!(x.count_ones(), 4);
let y: u16 = rng.gen_bits(15); // generates a u16 with 15 set bits
assert_eq!(y.count_ones(), 15);
let z: u64 = rng.gen_bits(1); // generates a u64 with 1 set bits
assert_eq!(z.count_ones(), 1);
```

For more usage examples, refer to the documentation available at [docs.rs](https://docs.rs/rand-bits).

## License

This crate is licensed under the MIT License.
