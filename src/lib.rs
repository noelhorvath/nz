//! # nz
//!
//! [![github]](https://github.com/noelhorvath/zn)
//! [![crates.io]](https://crates.io/crates/nz)
//! [![docs.rs]](https://docs.rs/nz)
//! [![msrv]](https://releases.rs/docs/1.56.0/)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&logo=github
//! [crates.io]: https://img.shields.io/crates/v/nz?style=for-the-badge&logo=rust
//! [docs.rs]: https://img.shields.io/docsrs/nz/latest?style=for-the-badge&logo=docs.rs
//! [msrv]: https://img.shields.io/badge/MSRV-1.56.0-F21D1D?style=for-the-badge&logo=rust
//!
//!
//! The `nz` crate provides a collection of macros that simplify the creation
//! of non-zero integers implemented in [`core::num`]. With these macros, you
//! can easily generate constants of all the `NonZero` prefixed types using
//! literals, constant values or expressions at compile time.
//!
//! ## Changelog
//!
//! All changes to `nz` crate are documented in [changelog.md](https://github.com/noelhorvath/nz/blob/main/changelog.md).
//!
//! ## Features
//!
//! * No unsafe code
//! * No dependencies
//! * `no_std` compatible
//! * Supports all non-zero types in `core::num`
//! * Compile-time evaluation
//!
//! ## Macros
//!
//! | Type | Macro |
//! |------|-------|
//! | [`NonZeroI8`][`core::num::NonZeroI8`] | [`nz::i8`!][`crate::i8`] |
//! | [`NonZeroI16`][`core::num::NonZeroI16`] | [`nz::i16!`][`crate::i16`] |
//! | [`NonZeroI32`][`core::num::NonZeroI32`] | [`nz::i32!`][`crate::i32`] |
//! | [`NonZeroI64`][`core::num::NonZeroI64`] | [`nz::i64!`][`crate::i64`] |
//! | [`NonZeroI128`][`core::num::NonZeroI128`] | [`nz::i128!`][`crate::i128`] |
//! | [`NonZeroIsize`][`core::num::NonZeroIsize`] | [`nz::isize!`][`crate::isize`] |
//! | [`NonZeroU8`][`core::num::NonZeroU8`] | [`nz::u8!`][`crate::u8`] |
//! | [`NonZeroU16`][`core::num::NonZeroU16`] | [`nz::u16!`][`crate::u16`] |
//! | [`NonZeroU32`][`core::num::NonZeroU32`] | [`nz::u32!`][`crate::u32`] |
//! | [`NonZeroU64`][`core::num::NonZeroU64`] | [`nz::u64!`][`crate::u64`] |
//! | [`NonZeroU128`][`core::num::NonZeroU128`] | [`nz::u128!`][`crate::u128`] |
//! | [`NonZeroUsize`][`core::num::NonZeroUsize`] | [`nz::usize!`][`crate::usize`] |
//!
//! ## Usage
//!
//! ```rust
//! use core::num::NonZeroU8;
//!
//! // A `NonZero*` type can be constructed by different types of
//! // arguments when using the matching `nz` macro.
//! // Such argument can be an integer literal,
//! const NZ_MIN: NonZeroU8 = nz::u8!(1);
//! let nz_two = nz::u8!(2);
//! // a constant value,
//! const NZ_MAX: NonZeroU8 = nz::u8!(u8::MAX);
//! const SIX: u8 = 6;
//! let six = nz::u8!(SIX);
//! // or even a constant expression.
//! const RES: NonZeroU8 = nz::u8!({ 3 + 7 } - NZ_MIN.get());
//! let res = nz::u8!((NZ_MIN.get() & NZ_MAX.get()) + 7);
//! let five = nz::u8!({ const FIVE: u8 = 5; FIVE });
//! // However, a non-constant expression results in a compile-time error.
//! // const __ERR: NonZeroU8 = nz::u8!({ 3 + 7 } - nz_two.get());
//! # assert_eq!(1, NZ_MIN.get());
//! # assert_eq!(2, nz_two.get());
//! # assert_eq!(6, six.get());
//! # assert_eq!(5, five.get());
//! # assert_eq!(9, RES.get());
//! # assert_eq!(8, res.get());
//! ```
//!
//! ## Limitations
//!
//! ### Declarative macro hygiene
//!
//! [Declarative macro] is not [hygienic] when it comes to [items].
//! As a result, if the outermost [constant item] `_NZ_INTERNAL_NUM_VALUE_1_`
//! is referenced in the macro argument, a [cyclic dependency error] occurs as
//! shown in the below examples.
//!
//! #### Non-expanded
//!
//! ```rust, compile_fail
//! const __ERR: NonZeroI8 = nz::i8!(_NZ_INTERNAL_NUM_VALUE_1_ + 0x2C);
//! ```
//! #### Expanded
//!
//! ```rust, compile_fail
//! const _ERR: NonZeroI8 = {
//!     const _NZ_INTERNAL_NUM_VALUE_1_: i8 = _NZ_INTERNAL_NUM_VALUE_1_ + 0x2C;
//!     {
//!         /* rest of the expanded code */
//!     }
//! };
//! ```
//!
//! [Declarative macro]: https://doc.rust-lang.org/reference/macros-by-example.html
//! [items]: https://doc.rust-lang.org/reference/items.html
//! [hygienic]: https://danielkeep.github.io/tlborm/book/mbe-min-hygiene.html
//! [constant item]: https://doc.rust-lang.org/reference/items/constant-items.html
//! [cyclic dependency error]: https://doc.rust-lang.org/error_codes/E0391.html
#![no_std]
#![forbid(unsafe_code)]

/// Generates the non-zero macro for the specified non-zero integer type.
macro_rules! gen_non_zero_macros {
    (
        const $const_id:ident;
        $(#[$zero_error_attr:meta])*
        type ZeroError = $zero_error:ident;

        $(
            $nz_ty:ty => {
                type Integer = $int_ty:ident;
                type NonZero = $nz_ty_name:ident;

                $(#[$macro_attr:meta])*
                macro_rules! $macro_name:ident;
            }
        )*
    ) => {
        $(#[$zero_error_attr])*
        pub enum $zero_error {}
        $(
            #[doc = concat!("Creates a [`", stringify!($nz_ty_name), "`][`", stringify!($nz_ty), "`] from a")]
            /// literal, a constant value or expression that evaluates
            #[doc = concat!("to [`prim@", stringify!($int_ty), "`].")]
            ///
            /// # Compilation errors
            ///
            /// Compilation error occurs in the following cases:
            ///
            /// * The argument is either zero or non-constant.
            #[doc = concat!(" * The argument cannot be evaluated to a non-zero [`prim@", stringify!($int_ty), "`].")]
            #[doc = concat!(" * The argument references the macro constant `", stringify!($const_id), "`.")]
            ///     * For more information, see [`Limitations: Declarative macro hygiene`][crate#declarative-macro-hygiene].
            ///
            /// # Examples
            ///
            #[doc = concat!(" #### Creating `", stringify!($nz_ty_name), "` using integer literal")]
            /// ```rust
            #[doc = concat!(" # use ", stringify!($nz_ty), ";")]
            #[doc = concat!(" const NZ: ", stringify!($nz_ty_name), " = nz::", stringify!($int_ty), "!(0x10);")]
            #[doc = concat!(" let nz = nz::", stringify!($int_ty), "!(27);")]
            /// # assert_eq!(27, nz.get());
            /// # assert_eq!(0x10, NZ.get());
            /// ```
            ///
            #[doc = concat!(" #### Creating `", stringify!($nz_ty_name), "` using constant value")]
            /// ```rust
            #[doc = concat!(" # use ", stringify!($nz_ty), ";")]
            #[doc = concat!(" const NUM: ", stringify!($int_ty), " = 0b0111_1111;")]
            #[doc = concat!(" const NZ: ", stringify!($nz_ty_name), " = nz::", stringify!($int_ty),"!(NUM);")]
            #[doc = concat!(" let nz = nz::", stringify!($int_ty), "!(NZ.get());")]
            /// # assert_eq!(NUM, nz.get());
            /// # assert_eq!(nz, NZ);
            /// ```
            ///
            #[doc = concat!(" #### Creating `", stringify!($nz_ty_name), "` using constant expression")]
            /// ```rust
            #[doc = concat!(" # use ", stringify!($nz_ty), ";")]
            #[doc = concat!(" const NZ: ", stringify!($nz_ty_name), " = nz::", stringify!($int_ty), "!(0b1100 & 0b0110);")]
            #[doc = concat!(" let nz = nz::", stringify!($int_ty), "!(NZ.get() + 0x01);")]
            /// # assert_eq!(0b0100, NZ.get());
            /// # assert_eq!(0b0101, nz.get());
            /// ```
            ///
            /// #### Zero literal fails to compile
            /// ```rust, compile_fail
            #[doc = concat!(" let _ = nz::", stringify!($int_ty), "!(0);")]
            /// ```
            ///
            /// #### Non-constant expression fails to compile
            /// ```rust, compile_fail
            /// // compiles if `add` function has the `const` modifier
            #[doc = concat!(" fn add(a: ", stringify!($int_ty), ", b: ", stringify!($int_ty), ") -> ", stringify!($int_ty), " { a.wrapping_add(b) }")]
            #[doc = concat!(" let _ = nz::", stringify!($int_ty), "!(add(1, 1));")]
            /// ```
            ///
            /// #### Constant expression that evaluates to zero fails to compile
            /// ```rust, compile_fail
            #[doc = concat!(" let _ = nz::", stringify!($int_ty), "!(0x02 - 0b0010);")]
            /// ```
            ///
            #[doc = concat!(" #### Constant argument references `", stringify!($const_id), "` fails to compile")]
            /// ```rust, compile_fail
            #[doc = concat!(" const ", stringify!($const_id), ": ", stringify!($nz_ty_name), " = 11;")]
            #[doc = concat!(" let _ = nz::", stringify!($int_ty), "!(", stringify!($const_id), " + 10);")]
            /// ```
            $(#[$macro_attr])*
            macro_rules! $macro_name {
                ($int_expr:expr) => {{
                    const $const_id: $int_ty = $int_expr;
                    {
                        // compile-time error occurs if `$const_id` is `0` -> [_; 1] != [_; 0]
                        const ZERO_CHECK: [$crate::$zero_error; ($const_id == 0) as usize] = [];
                        const NZ: $nz_ty = match <$nz_ty>::new($const_id) {
                            Some(non_zero) => non_zero,
                            None => loop {}, // unreachable
                        };
                        NZ
                    }
                }};
            }
        )*
    };
}

gen_non_zero_macros! {
    // The outermost constant after an `nz` macro expansion that contains
    // the constant value of `$int_expr`.
    const _NZ_INTERNAL_NUM_VALUE_1_;

    /// The empty error that is only intended for showing in
    /// the compile-error message when the specified numeric
    /// macro argument fails the compile-time zero check.
    ///
    /// # Examples
    ///
    /// ```rust
    /// // never type cannot be instantiated
    /// use nz::ZeroIsInvalidForNonZero;
    /// ```
    #[allow(clippy::exhaustive_enums)]
    type ZeroError = ZeroIsInvalidForNonZero;

    ::core::num::NonZeroI8 => {
        type Integer = i8;
        type NonZero = NonZeroI8;

        #[macro_export]
        macro_rules! i8;
    }

    ::core::num::NonZeroI16 => {
        type Integer = i16;
        type NonZero = NonZeroI16;

        #[macro_export]
        macro_rules! i16;
    }

    ::core::num::NonZeroI32 => {
        type Integer = i32;
        type NonZero = NonZeroI32;

        #[macro_export]
        macro_rules! i32;
    }

    ::core::num::NonZeroI64 => {
        type Integer = i64;
        type NonZero = NonZeroI64;

        #[macro_export]
        macro_rules! i64;
    }

    ::core::num::NonZeroI128 => {
        type Integer = i128;
        type NonZero = NonZeroI128;

        #[macro_export]
        macro_rules! i128;
    }

    ::core::num::NonZeroIsize => {
        type Integer = isize;
        type NonZero = NonZeroIsize;

        #[macro_export]
        macro_rules! isize;
    }

    ::core::num::NonZeroU8 => {
        type Integer = u8;
        type NonZero = NonZeroU8;

        #[macro_export]
        macro_rules! u8;
    }

    ::core::num::NonZeroU16 => {
        type Integer = u16;
        type NonZero = NonZeroU16;

        #[macro_export]
        macro_rules! u16;
    }

    ::core::num::NonZeroU32 => {
        type Integer = u32;
        type NonZero = NonZeroU32;

        #[macro_export]
        macro_rules! u32;
    }

    ::core::num::NonZeroU64 => {
        type Integer = u64;
        type NonZero = NonZeroU64;

        #[macro_export]
        macro_rules! u64;
    }

    ::core::num::NonZeroU128 => {
        type Integer = u128;
        type NonZero = NonZeroU128;

        #[macro_export]
        macro_rules! u128;
    }

    ::core::num::NonZeroUsize => {
        type Integer = usize;
        type NonZero = NonZeroUsize;

        #[macro_export]
        macro_rules! usize;
    }
}
