# nz

[![github]](https://github.com/noelhorvath/nz)
[![crates.io]](https://crates.io/crates/nz)
[![docs.rs]](https://docs.rs/nz)
[![rust-ci]](https://github.com/noelhorvath/nz/actions/workflows/check.yml)
[![msrv]](https://releases.rs/docs/1.56.0/)
![unsafety]
[![license]](#license)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&logo=github
[crates.io]: https://img.shields.io/crates/v/nz?style=for-the-badge&logo=rust
[docs.rs]: https://img.shields.io/docsrs/nz/latest?style=for-the-badge&logo=docs.rs
[rust-ci]: https://img.shields.io/github/actions/workflow/status/noelhorvath/nz/check.yml?style=for-the-badge&logo=github
[msrv]: https://img.shields.io/badge/MSRV-1.56.0-F21D1D?style=for-the-badge&logo=rust
[unsafety]: https://img.shields.io/badge/unsafe-forbidden-brightgreen?style=for-the-badge&logo=rust
[license]: https://img.shields.io/badge/License-MIT_OR_Zlib_OR_APACHE_2.0-blue?style=for-the-badge

The `nz` crate provides a collection of macros that simplify the creation
of non-zero integers implemented in [`core::num`]. With these macros, you can easily generate constants of all the `NonZero-` prefixed types using literals, constant values or expressions at compile time.

[`core::num`]: https://doc.rust-lang.org/core/num/index.html

## Changelog

All changes to `nz` crate are documented in [changelog.md](changelog.md).

## Features

* No unsafe code
* No dependencies
* `no_std` compatible
* Supports all non-zero types in `core::num`
* Compile-time evaluation

## Macros

| Type | Macro |
|------|-------|
| [`NonZeroI8`](https://doc.rust-lang.org/1.56.0/core/num/struct.NonZeroI8.html) | [`nz::i8!`](https://docs.rs/nz/latest/nz/macro.i8.html) |
| [`NonZeroI16`](https://doc.rust-lang.org/1.56.0/core/num/struct.NonZeroI16.html) | [`nz::i16!`](https://docs.rs/nz/latest/nz/macro.i16.html) |
| [`NonZeroI32`](https://doc.rust-lang.org/1.56.0/core/num/struct.NonZeroI32.html) | [`nz::i32!`](https://docs.rs/nz/latest/nz/macro.i32.html) |
| [`NonZeroI64`](https://doc.rust-lang.org/1.56.0/core/num/struct.NonZeroI64.html) | [`nz::i64!`](https://docs.rs/nz/latest/nz/macro.i64.html) |
| [`NonZeroI128`](https://doc.rust-lang.org/1.56.0/core/num/struct.NonZeroI128.html) | [`nz::i128!`](https://docs.rs/nz/latest/nz/macro.i128.html) |
| [`NonZeroIsize`](https://doc.rust-lang.org/1.56.0/core/num/struct.NonZeroIsize.html) | [`nz::isize!`](https://docs.rs/nz/latest/nz/macro.isize.html) |
| [`NonZeroU8`](https://doc.rust-lang.org/1.56.0/core/num/struct.NonZeroU8.html) | [`nz::u8!`](https://docs.rs/nz/latest/nz/macro.u8.html) |
| [`NonZeroU16`](https://doc.rust-lang.org/1.56.0/core/num/struct.NonZeroU16.html) | [`nz::u16!`](https://docs.rs/nz/latest/nz/macro.u16.html) |
| [`NonZeroU32`](https://doc.rust-lang.org/1.56.0/core/num/struct.NonZeroU32.html) | [`nz::u32!`](https://docs.rs/nz/latest/nz/macro.u32.html) |
| [`NonZeroU64`](https://doc.rust-lang.org/1.56.0/core/num/struct.NonZeroU64.html) | [`nz::u64!`](https://docs.rs/nz/latest/nz/macro.u64.html) |
| [`NonZeroU128`](https://doc.rust-lang.org/1.56.0/core/num/struct.NonZeroU128.html) | [`nz::u128!`](https://docs.rs/nz/latest/nz/macro.u128.html) |
| [`NonZeroUsize`](https://doc.rust-lang.org/1.56.0/core/num/struct.NonZeroUsize.html) | [`nz::usize!`](https://docs.rs/nz/latest/nz/macro.usize.html) |

## Usage

```rust
use core::num::NonZeroU8;

// A `NonZero*` type can be constructed by different types of
// arguments when using the matching `nz` macro.
// Such argument can be an integer literal,
const NZ_MIN: NonZeroU8 = nz::u8!(1);
let nz_two = nz::u8!(2);
// a constant value,
const NZ_MAX: NonZeroU8 = nz::u8!(u8::MAX);
const SIX: u8 = 6;
let six = nz::u8!(SIX);
// or even a constant expression.
const RES: NonZeroU8 = nz::u8!({ 3 + 7 } - NZ_MIN.get());
let res = nz::u8!((NZ_MIN.get() & NZ_MAX.get()) + 7);
let five = nz::u8!({ const FIVE: u8 = 5; FIVE });
// However, a non-constant expression results in a compile-time error.
// const __ERR: NonZeroU8 = nz::u8!({ 3 + 7 } - nz_two.get());
```
## Limitations

### Declarative macro hygiene

[Declarative macro] is not [hygienic] when it comes to [items].
As a result, if the outermost [constant item] `_NZ_INTERNAL_NUM_VALUE_1_`
is referenced in the macro argument, a [cyclic dependency error] occurs as
shown in the below examples.

#### Non-expanded

```rust
const __ERR: NonZeroI8 = nz::i8!(_NZ_INTERNAL_NUM_VALUE_1_ + 0x2C);
```
#### Expanded

```rust
const _ERR: NonZeroI8 = {
    const _NZ_INTERNAL_NUM_VALUE_1_: i8 = _NZ_INTERNAL_NUM_VALUE_1_ + 0x2C;
    {
        /* rest of the expanded code */
    }
};
```

[Declarative macro]: https://doc.rust-lang.org/reference/macros-by-example.html
[items]: https://doc.rust-lang.org/reference/items.html
[hygienic]: https://danielkeep.github.io/tlborm/book/mbe-min-hygiene.html
[constant item]: https://doc.rust-lang.org/reference/items/constant-items.html
[cyclic dependency error]: https://doc.rust-lang.org/error_codes/E0391.html

## License

This library is distributed under the terms of either of the following licenses at your option:

- [MIT License](http://opensource.org/licenses/MIT)
- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
- [Zlib License](https://www.zlib.net/zlib_license.html)
