# nz

[![crates.io](https://img.shields.io/crates/v/nz.svg)](https://crates.io/crates/hyper)
[![docs](https://docs.rs/nz/badge.svg)](https://docs.rs/nz)
[![License:Zlib](https://img.shields.io/badge/License-Zlib-brightgreen.svg)](https://opensource.org/licenses/Zlib)
![Minimum Rust Version](https://img.shields.io/badge/Min%20Rust-1.47-green.svg)
[![Rust](https://github.com/noelhorvath/nz/workflows/rust/badge.svg)](https://github.com/noelhorvath/nz/actions?query=workflow%3rust)
![Unsafe-Zero-Percent](https://img.shields.io/badge/Unsafety-0%25-brightgreen.svg)

The `nz` crate provides a collection of user-friendly macros that simplify the creation
of new instances of non-zero numeric types found in the [`core::num`](https://doc.rust-lang.org/stable/core/num). With these macros,
you can effortlessly generate instances using numeric literals, constant values and
constant expressions, all at compile time.

## Features

* No unsafe code
* No dependencies
* `no_std` compatible
* Supports all numeric non-zero types from the [`core::num`](https://doc.rust-lang.org/stable/core/num) module
* Compile time evaluation
* Zero detection at compile time

## `NonZero` macros

| Type | Macro |
|------|-------|
| [`NonZeroI8`](https://doc.rust-lang.org/stable/core/num/struct.NonZeroI8.html) | [`nz::i8!`](https://docs.rs/nz/%2A/nz/macro.i8.html) |
| [`NonZeroI16`](https://doc.rust-lang.org/stable/core/num/struct.NonZeroI16.html) | [`nz::i16!`](https://docs.rs/nz/%2A/nz/macro.i16.html) |
| [`NonZeroI32`](https://doc.rust-lang.org/stable/core/num/struct.NonZeroI32.html) | [`nz::i32!`](https://docs.rs/nz/%2A/nz/macro.i32.html) |
| [`NonZeroI64`](https://doc.rust-lang.org/stable/core/num/struct.NonZeroI64.html) | [`nz::i64!`](https://docs.rs/nz/%2A/nz/macro.i64.html) |
| [`NonZeroI128`](https://doc.rust-lang.org/stable/core/num/struct.NonZeroI128.html) | [`nz::i128!`](https://docs.rs/nz/%2A/nz/macro.i128.html) |
| [`NonZeroIsize`](https://doc.rust-lang.org/stable/core/num/struct.NonZeroIsize.html) | [`nz::isize!`](https://docs.rs/nz/%2A/nz/macro.isize.html) |
| [`NonZeroU8`](https://doc.rust-lang.org/stable/core/num/struct.NonZeroU8.html) | [`nz::u8!`](https://docs.rs/nz/%2A/nz/macro.u8.html) |
| [`NonZeroU16`](https://doc.rust-lang.org/stable/core/num/struct.NonZeroU16.html) | [`nz::u16!`](https://docs.rs/nz/%2A/nz/macro.u16.html) |
| [`NonZeroU32`](https://doc.rust-lang.org/stable/core/num/struct.NonZeroU32.html) | [`nz::u32!`](https://docs.rs/nz/%2A/nz/macro.u32.html) |
| [`NonZeroU64`](https://doc.rust-lang.org/stable/core/num/struct.NonZeroU64.html) | [`nz::u64!`](https://docs.rs/nz/%2A/nz/macro.u64.html) |
| [`NonZeroU128`](https://doc.rust-lang.org/stable/core/num/struct.NonZeroU128.html) | [`nz::u128!`](https://docs.rs/nz/%2A/nz/macro.u128.html) |
| [`NonZeroUsize`](https://doc.rust-lang.org/stable/core/num/struct.NonZeroUsize.html) | [`nz::usize!`](https://docs.rs/nz/%2A/nz/macro.usize.html) |

## Basic usage

```rust
use core::num::NonZeroU8;

const NZ_U8_MIN: NonZeroU8 = nz::u8!(1); // with numeric literal
const NZ_U8_MAX: NonZeroU8 = nz::u8!(u8::MAX); // with constant value
let sum = nz::u8!(NZ_U8_MAX.get() & NZ_U8_MIN.get() + 7); // with constant expression
```

## Remarks

Non-zero macros cannot be used with constant function arguments as they
are not considered as constant values.

### Example

```rust, compile_fail
use core::num::NonZeroU64;

const fn wrapping_add_nz(a: u64, b: NonZeroU64) -> NonZeroU64 {
    // `a` and `b` is not constant
    nz::u64!(a.wrapping_add(b.get())) // <- cause of the compile error
}
let nz = wrapping_add_nz(2, nz::u64!(1));
```