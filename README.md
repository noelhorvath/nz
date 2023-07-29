# nz

[![crates.io](https://img.shields.io/crates/v/nz?style=for-the-badge)](https://crates.io/crates/nz)
[![docs](https://img.shields.io/docsrs/nz/latest?style=for-the-badge)](https://docs.rs/nz)
[![license](https://img.shields.io/badge/License-MIT_OR_Zlib_OR_APACHE_2.0-blue?style=for-the-badge)](#license)
[![rust](https://img.shields.io/github/actions/workflow/status/noelhorvath/nz/rust.yml?event=push&style=for-the-badge)](https://github.com/noelhorvath/nz/actions/workflows/rust.yml)
[![msrv](https://img.shields.io/badge/MSRV-1.47.0-F21D1D?style=for-the-badge)](https://releases.rs/docs/1.47.0/)
![safety](https://img.shields.io/badge/Safety-100%25-brightgreen?style=for-the-badge)

The `nz` crate provides a collection of macros that simplify the creation of
new instances of non-zero numeric types implemented in [`core::num`](https://doc.rust-lang.org/core/num/index.html). With these macros, you can easily generate constants of such numeric types using
literals, constant values or expressions, all at compile time.

## Features

* No unsafe code
* No dependencies
* `no_std` compatible
* Supports all `core::num::NonZero{Integer}` types
* Compile time evaluation
* Zero detection at compile time

## Macros

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
// A NonZero type can be constructed by different
// types of arguments when using the matching macro
//
// such argument can be a numeric literal
const NZ_MIN: NonZeroU8 = nz::u8!(1);
let nz_two = nz::u8!(2);
// or a constant value
const NZ_MAX: NonZeroU8 = nz::u8!(u8::MAX);
let five = nz::u8!({ const FIVE: u8 = 5; FIVE });
// or even a constant expression
const RES: NonZeroU8 = nz::u8!({ 3 + 7 } - NZ_MIN.get());
// non-constant expression leads to compile-time error
// const OUTPUT: NonZeroU8 = nz::u8!({ 3 + 7 } - nz_two.get()); // casued by `mz_two.get()`
let result_as_nz = nz::u8!((NZ_MIN.get() & NZ_MAX.get()) + 7);
```

## Limitations

### const fn

Declarative macros (such as all the `nz` macros) cannot be used with
constant function arguments since they are not currently recognized
as constant values, as demonstrated in the code below.

```rust, compile_fail
use core::num::NonZeroU64;

const fn wrapping_add_nz(a: u64, b: NonZeroU64) -> NonZeroU64 {
    // `a` and `b` is not constant
    // the line below causes compile error
    nz::u64!(a.wrapping_add(b.get()))
}
let nz = wrapping_add_nz(2, nz::u64!(1));
```

### const hygiene

When constants are used in a declarative macro, specifically in the
most outer scope where a constant can be declared, there is a possibility
of cyclic reference when an expression is expected as an argument and an
outer constant is used within that expression. This "collision" can occur
if any of the inner constants share the same identifier as the outer constant
after the macro is expanded compile-time. The code snippet below demonstrates
this scenario.

```rust, compile_fail
use core::num::NonZeroU16;

const NZ: NonZeroU16 = nz::u16!(0xA3FE);
const CHECK_ZERO: NonZeroU16 = nz::u16!(777);
// although `CHECK_ZERO` is used in the macro
// it won't collide when passing it in a constant
// expression, because it is not in the most outer
// scope where a constant is declared
const OK: NonZeroU16 = nz::u16!(CHECK_ZERO.get());
// using NUM.get() is fine
const ___NZ___INTERNAL___NUM___1___: u16
    = nz::u16!(NZ.get()).get();
// using `___NZ___INTERNAL___NUM___1___` constant as the argument
// causes compile-time error in the code line below, because the
// internal macro constant has the same identifier
const FAILS: NonZeroU16 = nz::u16!(
    ___NZ___INTERNAL___NUM___1___ // <-- error
);
```

This "collision" between the outer and inner constants leads to a compile-time
error, specifically error [`[E0391]`](https://doc.rust-lang.org/error_codes/E0391.html),
because the inner macro constant tries to use itself, creating a cyclic dependency
during the evaluation of the macro at compile-time. Essentially, the code above has
the same error as this single line:
```rust, compile_fail
const X: u8 = X;
```

## License

This library is distributed under the terms of either of the following licenses at your option:

- [MIT License](http://opensource.org/licenses/MIT)
- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
- [Zlib License](http://www.apache.org/licenses/LICENSE-2.0)
