//! # nz
//!
//! [![github]](https://github.com/noelhorvath/zn)
//! [![crates.io]](https://crates.io/crates/nz/0.4.0-beta.2)
//! [![docs.rs]](https://docs.rs/nz/0.4.0-beta.2/nz)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&logo=github
//! [crates.io]: https://img.shields.io/badge/crates.io-0.4.0--beta.2-orange?style=for-the-badge&logo=rust
//! [docs.rs]: https://img.shields.io/docsrs/nz/0.4.0-beta.2?style=for-the-badge&logo=docs.rs
//!
//! The `nz` crate provides a collection of macros that simplify the creation
//! of the [`NonZero`][`core::num::NonZero`] type. With these macros, you
//! can easily generate constants of the generic type using literals, constant
//! values or expressions at compile time.
//!
//! ## Disclaimer
//!
//! This beta version of `nz` uses the [`inline_const`] and [`generic_nonzero`]
//! features that require the `nightly` toolchain until they are both included
//! in a future stable release.
//!
//! [`inline_const`]: https://doc.rust-lang.org/unstable-book/language-features/inline-const.html?highlight=inline#inline_const
//! [`generic_nonzero`]: https://doc.rust-lang.org/stable/unstable-book/library-features/generic-nonzero.html
//!
//! ## Changelog
//!
//! All changes to `nz` crate are documented in [CHANGELOG.md](https://github.com/noelhorvath/nz/blob/main/changelog.md).
//!
//! ## Features
//!
//! * No unsafe code
//! * No dependencies
//! * `no_std` compatible
//! * Supports every type that implements the [`ZeroablePrimitive`][`core::num::ZeroablePrimitive`] marker trait
//! * Compile-time evaluation
//!
//! ## Macros
//!
//! | Type | Macro |
//! |------|-------|
//! | [`NonZero<i8>`][`core::num::NonZeroI8`] | [`nz::i8!`][`crate::i8`] |
//! | [`NonZero<i16>`][`core::num::NonZeroI16`] | [`nz::i16!`][`crate::i16`] |
//! | [`NonZero<i32>`][`core::num::NonZeroI32`] | [`nz::i32!`][`crate::i32`] |
//! | [`NonZero<i64>`][`core::num::NonZeroI64`] | [`nz::i64!`][`crate::i64`] |
//! | [`NonZero<i128>`][`core::num::NonZeroI128`] | [`nz::i128!`][`crate::i128`] |
//! | [`NonZero<isize>`][`core::num::NonZeroIsize`] | [`nz::isize!`][`crate::isize`] |
//! | [`NonZero<u8>`][`core::num::NonZeroU8`] | [`nz::u8!`][`crate::u8`] |
//! | [`NonZero<u16>`][`core::num::NonZeroU16`] | [`nz::u16!`][`crate::u16`] |
//! | [`NonZero<u32>`][`core::num::NonZeroU32`] | [`nz::u32!`][`crate::u32`] |
//! | [`NonZero<u64>`][`core::num::NonZeroU64`] | [`nz::u64!`][`crate::u64`] |
//! | [`NonZero<u128>`][`core::num::NonZeroU128`] | [`nz::u128!`][`crate::u128`] |
//! | [`NonZero<usize>`][`core::num::NonZeroUsize`] | [`nz::usize!`][`crate::usize`] |
//!
//! ## Usage
//!
//! ```rust
//! use std::num::NonZero;
//!
//! // A `NonZero<T>` type can be constructed from different types of
//! // arguments with the matching `nz` macro.
//! // Such argument can be an integer literal,
//! const NZ_MIN: NonZero<u8> = nz::u8!(1);
//! let nz_two = nz::u8!(2);
//! // a constant value,
//! const NZ_MAX: NonZero<u8> = nz::u8!(u8::MAX);
//! const SIX: u8 = 6;
//! let six = nz::u8!(SIX);
//! // or even a constant expression.
//! const RES: NonZero<u8> = nz::u8!({ 3 + 7 } - NZ_MIN.get());
//! let res = nz::u8!((NZ_MIN.get() & NZ_MAX.get()) + 7);
//! let five = nz::u8!({ const FIVE: u8 = 5; FIVE });
//! // However, a non-constant expression results in a compile-time error.
//! // const __ERR: NonZero<u8> = nz::u8!({ 3 + 7 } - nz_two.get());
//! # assert_eq!(1, NZ_MIN.get());
//! # assert_eq!(2, nz_two.get());
//! # assert_eq!(6, six.get());
//! # assert_eq!(5, five.get());
//! # assert_eq!(9, RES.get());
//! # assert_eq!(8, res.get());
//! ```
#![no_std]
#![forbid(unsafe_code)]

/// Generates a non-zero macro for the specified integer type.
macro_rules! gen_nz_macro {
    ($int_type:ident) => {
        #[doc = concat!("Creates a [`NonZero<", stringify!($int_type), ">`][`core::num::NonZero`] from a")]
        #[doc = r"literal, a constant value or expression that evaluates"]
        #[doc = concat!("to [`prim@", stringify!($int_type), "`].")]
        #[doc = r""]
        #[doc = concat!("If the argument cannot be evaluated to a [`prim@", stringify!($int_type), "`],")]
        /// a will occur [`panic`] at compile time.
        ///
        /// # Examples
        ///
        /// #### From integer literal
        /// ```rust
        /// # use std::num::NonZero;
        #[doc = concat!(" const NZ: NonZero<", stringify!($int_type), "> = nz::", stringify!($int_type), "!(0x10);")]
        #[doc = concat!(" let nz = nz::", stringify!($int_type), "!(27);")]
        /// let nz = nz::i8!(27);
        /// # assert_eq!(27, nz.get());
        /// # assert_eq!(0x10, NZ.get());
        /// ```
        ///
        /// #### From constant value
        /// ```rust
        /// # use std::num::NonZero;
        #[doc = concat!(" const NUM: ", stringify!($int_type), " = 0b0111_1111;")]
        #[doc = concat!(" const NZ: NonZero<", stringify!($int_type), "> = nz::", stringify!($int_type),"!(NUM);")]
        #[doc = concat!(" let nz = nz::", stringify!($int_type), "!(NZ.get());")]
        /// # assert_eq!(NUM, nz.get());
        /// # assert_eq!(nz, NZ);
        /// ```
        ///
        /// #### From constant expression
        /// ```rust
        /// # use std::num::NonZero;
        #[doc = concat!(" const NZ: NonZero<", stringify!($int_type), "> = nz::", stringify!($int_type), "!(0b1100 & 0b0110);")]
        #[doc = concat!(" let nz = nz::", stringify!($int_type), "!(NZ.get() + 0x01);")]
        /// # assert_eq!(0b0100, NZ.get());
        /// # assert_eq!(0b0101, nz.get());
        /// ```
        ///
        /// #### Zero literal fails to compile
        /// ```rust, compile_fail
        #[doc = concat!(" let _ = nz::", stringify!($int_type), "!(0);")]
        /// ```
        ///
        /// #### Non-constant expression fails to compile
        /// ```rust, compile_fail
        /// // compiles if `add` function has the `const` modifier
        #[doc = concat!(
            " fn add(a: ", stringify!($int_type), ", b: ", stringify!($int_type), ") -> ", stringify!($int_type),
            " { a.wrapping_add(b) }")]
        #[doc = concat!(" let _ = nz::", stringify!($int_type), "!(add(1, 1));")]
        /// ```
        ///
        /// #### Constant expression that evaluates to zero fails to compile
        /// ```rust, compile_fail
        #[doc = concat!(" let _ = nz::", stringify!($int_type), "!(0x02 - 0b0010);")]
        /// ```
        #[macro_export]
        macro_rules! $int_type {
            ($int_expr:expr) => {{
                const {{
                    match core::num::NonZero::<$int_type>::new($int_expr) {
                        Some(non_zero) => non_zero,
                        None => panic!("expected non-zero value"),
                    }
                }}
            }};
        }
    };
}

/// Generates a non-zero macro from each identifier.
macro_rules! gen_nz_macros {
    ($($int_type:ident), *) => {
        $(gen_nz_macro!($int_type);)*
    };
}

gen_nz_macros!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
