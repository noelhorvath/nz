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
//! of non-zero integers implemented in [`core::num`](https://doc.rust-lang.org/core/num/index.html).
//! With these macros, you can easily generate constants of all the `NonZero`
//! types using literals, constant values or expressions at compile time.
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
//! * Supports all `core::num::NonZero{Integer}` types
//! * Compile-time evaluation and zero detection
//!
//! ## `NonZero` macros
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
//! // A `NonZeroU8` type can be constructed by different types
//! // of arguments when using the matching macro.
//! // such argument can be a integer literal
//! const NZ_MIN: NonZeroU8 = nz::u8!(1);
//! let nz_two = nz::u8!(2);
//! // or a constant value
//! const NZ_MAX: NonZeroU8 = nz::u8!(u8::MAX);
//! const SIX: u8 = 6;
//! let six = nz::u8!(SIX);
//! // or even a constant expression
//! const RES: NonZeroU8 = nz::u8!({ 3 + 7 } - NZ_MIN.get());
//! // non-constant expression results in a compile-time error
//! // which is caused by `nz_two` in this case
//! // const OUTPUT: NonZeroU8 = nz::u8!({ 3 + 7 } - nz_two.get());
//! let res = nz::u8!((NZ_MIN.get() & NZ_MAX.get()) + 7);
//! let five = nz::u8!({ const FIVE: u8 = 5; FIVE });
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
//! ### const fn
//!
//! Declarative macros, such as all the `nz` macros, cannot be used with
//! constant function arguments since they are not considered constant
//! values, as demonstrated in the code below.
//!
//! ```rust, compile_fail
//! use core::num::NonZeroU64;
//!
//! const fn wrapping_add_nz(a: u64, b: NonZeroU64) -> NonZeroU64 {
//!     // `a` and `b` is not constant which results
//!     // in a compile-time error when passed to
//!     // `nz::u64!` in an expression
//!     nz::u64!(a.wrapping_add(b.get()))
//! }
//! let nz = wrapping_add_nz(2, nz::u64!(1));
//! ```
//!
//! ### const hygiene
//!
//! When constants are used in a declarative macro, specifically in the
//! most outer scope where a constant can be declared, there is a possibility
//! of cyclic dependency when an expression is expected as an argument and an
//! outer constant is used within that expression. This *collision* can occur
//! if any of the inner constants share the same identifier as the outer constant
//! after the macro is expanded at compile-time. The code snippet below demonstrates
//! this scenario.
//!
//! ```rust, compile_fail
//! use core::num::NonZeroU16;
//!
//! const NZ: NonZeroU16 = nz::u16!(0xA3FE);
//! const CHECK_ZERO: NonZeroU16 = nz::u16!(777);
//! // although `CHECK_ZERO` is used in `nz::u16!` macro, it will not result in
//! // an error when a constant with the same name is passed as part
//! // of a constant expression, because this inner macro constant is not
//! // declared in the most outer scope
//! const OK: NonZeroU16 = nz::u16!(CHECK_ZERO.get());
//! // using `NZ` is fine for the same reason
//! const _NZ_INTERNAL_NUM_VALUE_1_: u16
//!     = nz::u16!(NZ.get()).get();
//! // using `_NZ_INTERNAL_NUM_VALUE_1_` constant as the argument
//! // causes compile-time error in the code line below, because the
//! // internal macro constant has the same identifier as the constant
//! // specified in the macro argument
//! const _: NonZeroU16 = nz::u16!(_NZ_INTERNAL_NUM_VALUE_1_);
//! ```
//!
//! More concisely, the problem is:
//!
//! ```rust, compile_fail
//! const X: u8 = X;
//! ```
//!
//! This *collision* between the outer and inner constants results in a compile-time
//! error[^error], because the inner macro constant depends on itself, creating
//! a cyclic dependency.
//!
//! [^error]: [`[E0391]`](https://doc.rust-lang.org/error_codes/E0391.html),
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
            /// Compilation error will occur in the following cases:
            ///
            /// * The argument is either zero or non-constant.
            #[doc = concat!("The argument cannot be evaluated to a non-zero [prim@", stringify!($int_ty), "].")]
            #[doc = concat!("The argument contains the `", stringify!($zero_error), "` identifier.")]
            ///     * For more information, see [`Limitations: const hygene`][crate#const-hygiene].
            ///
            /// # Examples
            ///
            #[doc = concat!(" #### Creating `", stringify!($nz_ty_name), "` using integer literal")]
            /// ```rust
            #[doc = concat!(" # use ", stringify!($nz_ty), ";")]
            #[doc = concat!(" const NZ: ", stringify!($nz_ty_name), " = nz::", stringify!($int_ty), "!(0x10);")]
            #[doc = concat!(" let nz = nz::", stringify!($int_ty), "!(27);")]
            /// let nz = nz::i8!(27);
            /// # assert_eq!(27, nz.get());
            /// # assert_eq!(0x10, NZ.get());
            /// ```
            ///
            #[doc = concat!(" #### Creating `", stringify!($nz_ty_name), "` using constant value")]
            /// ```rust
            #[doc = concat!(" # use ", stringify!($nz_ty), ";")]
            #[doc = concat!(" const NUM: ", stringify!($int_ty), "= 0b0111_1111;")]
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
            /// // compiles if `add` function has `const` modifier
            #[doc = concat!(" fn add(a: ", stringify!($int_ty), ", b: ", stringify!($int_ty), ") -> ", stringify!($int_ty), " { a.wrapping_add(b) }")]
            #[doc = concat!(" let _ = nz::", stringify!($int_ty), "!(add(1, 1));")]
            /// ```
            ///
            /// #### Constant expression that evaluates to zero fails to compile
            /// ```rust, compile_fail
            #[doc = concat!(" let _ = nz::", stringify!($int_ty), "!(0x02 - 0x02);")]
            /// ```
            ///
            #[doc = concat!(" #### Constant argument that contains the `", stringify!($const_id), "` identifier fails to compile")]
            /// ```rust, compile_fail
            #[doc = concat!(" const ", stringify!($const_id), ": ", stringify!($nz_ty_name), " = 11;")]
            #[doc = concat!(" let _ = nz::", stringify!($int_ty), "!(", stringify!($const_id), " + 10);")]
            /// ```
            $(#[$macro_attr])*
            macro_rules! $macro_name {
                ($int_expr:expr) => {{
                    const $const_id: $int_ty = $int_expr;
                    {
                        // the below check generates a compile-time error if `$const_id` is zero,
                        // because the constant will have a type of `[$crate::$zero_error; 1]`
                        // instead of `[$crate::$zero_error; 0]`, which cannot be assigned to an
                        // empty array (`[]`).
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
    // The most outer constant after an `nz` macro expansion.
    const _NZ_INTERNAL_NUM_VALUE_1_;

    /// The empty error that is only intended for showing in
    /// the compile-error message when the specified numeric
    /// macro argument fails the compile-time zero check.
    ///
    /// # Examples
    ///
    /// ```rust
    /// // this never type cannot be instantiated
    /// use nz::ZeroIsInvalidForNonZero;
    /// ```
    #[allow(clippy::exhaustive_enums)]
    type ZeroError = ZeroIsInvalidForNonZero;

    core::num::NonZeroI8 => {
        type Integer = i8;
        type NonZero = NonZeroI8;

        #[macro_export]
        macro_rules! i8;
    }

    core::num::NonZeroI16 => {
        type Integer = i16;
        type NonZero = NonZeroI16;

        #[macro_export]
        macro_rules! i16;
    }

    core::num::NonZeroI32 => {
        type Integer = i32;
        type NonZero = NonZeroI32;

        #[macro_export]
        macro_rules! i32;
    }

    core::num::NonZeroI64 => {
        type Integer = i64;
        type NonZero = NonZeroI64;

        #[macro_export]
        macro_rules! i64;
    }

    core::num::NonZeroI128 => {
        type Integer = i128;
        type NonZero = NonZeroI128;

        #[macro_export]
        macro_rules! i128;
    }

    core::num::NonZeroIsize => {
        type Integer = isize;
        type NonZero = NonZeroIsize;

        #[macro_export]
        macro_rules! isize;
    }

    core::num::NonZeroU8 => {
        type Integer = u8;
        type NonZero = NonZeroU8;

        #[macro_export]
        macro_rules! u8;
    }

    core::num::NonZeroU16 => {
        type Integer = u16;
        type NonZero = NonZeroU16;

        #[macro_export]
        macro_rules! u16;
    }

    core::num::NonZeroU32 => {
        type Integer = u32;
        type NonZero = NonZeroU32;

        #[macro_export]
        macro_rules! u32;
    }

    core::num::NonZeroU64 => {
        type Integer = u64;
        type NonZero = NonZeroU64;

        #[macro_export]
        macro_rules! u64;
    }

    core::num::NonZeroU128 => {
        type Integer = u128;
        type NonZero = NonZeroU128;

        #[macro_export]
        macro_rules! u128;
    }

    core::num::NonZeroUsize => {
        type Integer = usize;
        type NonZero = NonZeroUsize;

        #[macro_export]
        macro_rules! usize;
    }
}
