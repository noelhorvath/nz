# nz

[![github]](https://github.com/noelhorvath/nz)
[![crates.io]](https://crates.io/crates/nz/0.4.1)
[![docs.rs]](https://docs.rs/nz/0.4.1/nz)
[![rust-ci]](https://github.com/noelhorvath/nz/actions?query=branch%3A0.4.1)
[![msrv]](https://releases.rs/docs/1.79.0/)
![unsafety]
[![license]](#license)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&logo=github
[crates.io]: https://img.shields.io/badge/crates.io-0.4.1-orange?style=for-the-badge&logo=rust
[docs.rs]: https://img.shields.io/docsrs/nz/0.4.1?style=for-the-badge&logo=docs.rs
[rust-ci]: https://img.shields.io/github/actions/workflow/status/noelhorvath/nz/check.yml?branch=0.4.1&style=for-the-badge&logo=github
[msrv]: https://img.shields.io/badge/MSRV-1.79.0-F21D1D?style=for-the-badge&logo=rust
[unsafety]: https://img.shields.io/badge/unsafe-forbidden-brightgreen?style=for-the-badge&logo=rust
[license]: https://img.shields.io/badge/License-MIT_OR_Zlib_OR_APACHE_2.0-blue?style=for-the-badge

## Table of contents

* [Description](#description)
* [Disclaimer](#disclaimer)
* [Changelog](#changelog)
* [Features](#features)
* [Macros](#macros)
* [Usage](#usage)
* [License](#license)

## Description

The `nz` crate provides a collection of macros that simplify the creation
of the [`NonZero`] type. With these macros, you can easily generate constants
of the generic type using literals, constant values or expressions at
compiletime.

[`NonZero`]: https://doc.rust-lang.org/1.79.0/core/num/struct.NonZero.html

## Changelog

All changes to `nz` crate are documented in [CHANGELOG.md](changelog.md).

## Features

* No unsafe code
* No dependencies
* `no_std` compatible
* Supports every type that implements [`ZeroablePrimitive`]
* Compile-time evaluation

[`ZeroablePrimitive`]: https://doc.rust-lang.org/1.79.0/core/num/trait.ZeroablePrimitive.html

## Macros

| Type | Macro |
|------|-------|
| [`NonZero<i8>`](https://doc.rust-lang.org/1.79.0/core/num/type.NonZeroI8.html) | [`nz::i8!`](https://docs.rs/nz/0.4.1/nz/macro.i8.html) |
| [`NonZero<i16>`](https://doc.rust-lang.org/1.79.0/core/num/type.NonZeroI16.html) | [`nz::i16!`](https://docs.rs/nz/0.4.1/nz/macro.i16.html) |
| [`NonZero<i32>`](https://doc.rust-lang.org/1.79.0/core/num/type.NonZeroI32.html) | [`nz::i32!`](https://docs.rs/nz/0.4.1/nz/macro.i32.html) |
| [`NonZero<i64>`](https://doc.rust-lang.org/1.79.0/core/num/type.NonZeroI16.html) | [`nz::i64!`](https://docs.rs/nz/0.4.1/nz/macro.i64.html) |
| [`NonZero<i128>`](https://doc.rust-lang.org/1.79.0/core/num/type.NonZeroI128.html) | [`nz::i128!`](https://docs.rs/nz/0.4.1/nz/macro.i128.html) |
| [`NonZero<isize>`](https://doc.rust-lang.org/1.79.0/core/num/type.NonZeroIsize.html) | [`nz::isize!`](https://docs.rs/nz/0.4.1/nz/macro.isize.html) |
| [`NonZero<u8>`](https://doc.rust-lang.org/1.79.0/core/num/type.NonZeroU8.html) | [`nz::u8!`](https://docs.rs/nz/0.4.1/nz/macro.u8.html) |
| [`NonZero<u16>`](https://doc.rust-lang.org/1.79.0/core/num/type.NonZeroU16.html) | [`nz::u16!`](https://docs.rs/nz/0.4.1/nz/macro.u16.html) |
| [`NonZero<u32>`](https://doc.rust-lang.org/1.79.0/core/num/type.NonZeroU32.html) | [`nz::u32!`](https://docs.rs/nz/0.4.1/nz/macro.u32.html) |
| [`NonZero<u64>`](https://doc.rust-lang.org/1.79.0/core/num/type.NonZeroU64.html) | [`nz::u64!`](https://docs.rs/nz/0.4.1/nz/macro.u64.html) |
| [`NonZero<u128>`](https://doc.rust-lang.org/1.79.0/core/num/type.NonZeroU128.html) | [`nz::u128!`](https://docs.rs/nz/0.4.1/nz/macro.u128.html) |
| [`NonZero<usize>`](https://doc.rust-lang.org/1.79.0/core/num/type.NonZeroUsize.html) | [`nz::usize!`](https://docs.rs/nz/0.4.1/nz/macro.usize.html) |

## Usage

```rust
use std::num::NonZero;

// A `NonZero<T>` type can be constructed from different types of
// arguments with the matching `nz` macro.
// Such argument can be an integer literal,
const NZ_MIN: NonZero<u8> = nz::u8!(1);
let nz_two = nz::u8!(2);
// a constant value,
const NZ_MAX: NonZero<u8> = nz::u8!(u8::MAX);
const SIX: u8 = 6;
let six = nz::u8!(SIX);
// or even a constant expression.
const RES: NonZero<u8> = nz::u8!({ 3 + 7 } - NZ_MIN.get());
let res = nz::u8!((NZ_MIN.get() & NZ_MAX.get()) + 7);
let five = nz::u8!({ const FIVE: u8 = 5; FIVE });
// However, a non-constant expression results in a compile-time error.
// const __ERR: NonZero<u8> = nz::u8!({ 3 + 7 } - nz_two.get());
```

## License

This library is distributed under the terms of either of the following licenses
at your option:

- [MIT License](http://opensource.org/licenses/MIT)
- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
- [Zlib License](https://www.zlib.net/zlib_license.html)