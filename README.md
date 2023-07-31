# nz

[![github]](https://github.com/noelhorvath/zn)
[![crates.io]](https://crates.io/crates/nz)
[![docs.rs]](https://docs.rs/nz)
[![rust-ci]](https://github.com/noelhorvath/nz/actions/workflows/rust.yml)
[![msrv]](https://releases.rs/docs/1.56.0/)
![unsafety]
[![license]](#license)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&logo=github
[crates.io]: https://img.shields.io/crates/v/nz?style=for-the-badge&logo=rust
[docs.rs]: https://img.shields.io/docsrs/nz/latest?style=for-the-badge&logo=docs.rs
[rust-ci]: https://img.shields.io/github/actions/workflow/status/noelhorvath/nz/rust.yml&style=for-the-badge
[msrv]: https://img.shields.io/badge/MSRV-1.56.0-F21D1D?style=for-the-badge&logo=rust
[unsafety]: https://img.shields.io/badge/unsafe-forbidden-brightgreen?style=for-the-badge&logo=rust
[license]: https://img.shields.io/badge/License-MIT_OR_Zlib_OR_APACHE_2.0-blue?style=for-the-badge

The `nz` crate provides a collection of macros that simplify the creation
of non-zero numerics implemented in [`core::num`](https://doc.rust-lang.org/core/num/index.html).
With these macros, you can easily generate constants of all the `NonZero`
types using literals, constant values or expressions at compile time.

## Features

* No unsafe code
* No dependencies
* `no_std` compatible
* Supports all `core::num::NonZero{Integer}` types
* Compile-time evaluation and zero detection

## `NonZero` macros

| Type | Macro |
|------|-------|
| [`NonZeroI8`](https://doc.rust-lang.org/stable/core/num/struct.NonZeroI8.html) | [`nz::i8!`](https://docs.rs/nz/latest/nz/macro.i8.html) |
| [`NonZeroI16`](https://doc.rust-lang.org/stable/core/num/struct.NonZeroI16.html) | [`nz::i16!`](https://docs.rs/nz/latest/nz/macro.i16.html) |
| [`NonZeroI32`](https://doc.rust-lang.org/stable/core/num/struct.NonZeroI32.html) | [`nz::i32!`](https://docs.rs/nz/latest/nz/macro.i32.html) |
| [`NonZeroI64`](https://doc.rust-lang.org/stable/core/num/struct.NonZeroI64.html) | [`nz::i64!`](https://docs.rs/nz/latest/nz/macro.i64.html) |
| [`NonZeroI128`](https://doc.rust-lang.org/stable/core/num/struct.NonZeroI128.html) | [`nz::i128!`](https://docs.rs/nz/latest/nz/macro.i128.html) |
| [`NonZeroIsize`](https://doc.rust-lang.org/stable/core/num/struct.NonZeroIsize.html) | [`nz::isize!`](https://docs.rs/nz/latest/nz/macro.isize.html) |
| [`NonZeroU8`](https://doc.rust-lang.org/stable/core/num/struct.NonZeroU8.html) | [`nz::u8!`](https://docs.rs/nz/latest/nz/macro.u8.html) |
| [`NonZeroU16`](https://doc.rust-lang.org/stable/core/num/struct.NonZeroU16.html) | [`nz::u16!`](https://docs.rs/nz/latest/nz/macro.u16.html) |
| [`NonZeroU32`](https://doc.rust-lang.org/stable/core/num/struct.NonZeroU32.html) | [`nz::u32!`](https://docs.rs/nz/latest/nz/macro.u32.html) |
| [`NonZeroU64`](https://doc.rust-lang.org/stable/core/num/struct.NonZeroU64.html) | [`nz::u64!`](https://docs.rs/nz/latest/nz/macro.u64.html) |
| [`NonZeroU128`](https://doc.rust-lang.org/stable/core/num/struct.NonZeroU128.html) | [`nz::u128!`](https://docs.rs/nz/latest/nz/macro.u128.html) |
| [`NonZeroUsize`](https://doc.rust-lang.org/stable/core/num/struct.NonZeroUsize.html) | [`nz::usize!`](https://docs.rs/nz/latest/nz/macro.usize.html) |

## Basic usage

```rust
use core::num::NonZeroU8;
// A `NonZeroU8` type can be constructed by different types
// of arguments when using the matching macro.
// such argument can be a numeric literal
const NZ_MIN: NonZeroU8 = nz::u8!(1);
let nz_two = nz::u8!(2);
// or a constant value
const NZ_MAX: NonZeroU8 = nz::u8!(u8::MAX);
const SIX: u8 = 6;
let six = nz::u8!(SIX);
// or even a constant expression
const RES: NonZeroU8 = nz::u8!({ 3 + 7 } - NZ_MIN.get());
// non-constant expression results in a compile-time error
// which is caused by `nz_two` in this case
// const OUTPUT: NonZeroU8 = nz::u8!({ 3 + 7 } - nz_two.get());
let res = nz::u8!((NZ_MIN.get() & NZ_MAX.get()) + 7);
let five = nz::u8!({ const FIVE: u8 = 5; FIVE });
# assert_eq!(1, NZ_MIN.get());
# assert_eq!(2, nz_two.get());
# assert_eq!(6, six.get());
# assert_eq!(5, five.get());
# assert_eq!(9, RES.get());
# assert_eq!(8, res.get());
```

## Limitations

### const fn

Declarative macros, such as all the `nz` macros, cannot be used with
constant function arguments since they are not considered constant
values, as demonstrated in the code below.

```rust, compile_fail
use core::num::NonZeroU64;

const fn wrapping_add_nz(a: u64, b: NonZeroU64) -> NonZeroU64 {
    // `a` and `b` is not constant which results
    // in a compile-time error when passed to
    // `nz::u64!` in an expression
    nz::u64!(a.wrapping_add(b.get()))
}
let nz = wrapping_add_nz(2, nz::u64!(1));
```

### const hygiene

When constants are used in a declarative macro, specifically in the
most outer scope where a constant can be declared, there is a possibility
of cyclic dependency when an expression is expected as an argument and an
outer constant is used within that expression. This *collision* can occur
if any of the inner constants share the same identifier as the outer constant
after the macro is expanded at compile-time. The code snippet below demonstrates
this scenario.

```rust, compile_fail
use core::num::NonZeroU16;

const NZ: NonZeroU16 = nz::u16!(0xA3FE);
const CHECK_ZERO: NonZeroU16 = nz::u16!(777);
// although `CHECK_ZERO` is used in `nz::u16!` macro, it will not result in
// an error when a constant with the same name is passed as part
// of a constant expression, because this inner macro constant is not
// declared in the most outer scope
const OK: NonZeroU16 = nz::u16!(CHECK_ZERO.get());
// using `NZ` is fine for the same reason
const ___NZ___INTERNAL___NUM___1___: u16
    = nz::u16!(NZ.get()).get();
// using `___NZ___INTERNAL___NUM___1___` constant as the argument
// causes compile-time error in the code line below, because the
// internal macro constant has the same identifier as the constant
// specified in the macro argument
const _: NonZeroU16 = nz::u16!(___NZ___INTERNAL___NUM___1___);
```

More concisely, the problem is:

```rust, compile_fail
const X: u8 = X;
```

This *collision* between the outer and inner constants results in a compile-time
error[^cd_error], because the inner macro constant depends on itself, creating
a cyclic dependency.

[^cd_error]: [`[E0391]`](https://doc.rust-lang.org/error_codes/E0391.html),

## License

This library is distributed under the terms of either of the following licenses at your option:

- [MIT License](http://opensource.org/licenses/MIT)
- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
- [Zlib License](http://www.apache.org/licenses/LICENSE-2.0)
