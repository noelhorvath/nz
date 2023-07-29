//! # nz
//!
//! The `nz` crate provides a collection of user-friendly macros that simplify the creation
//! of new instances of non-zero numeric types found in the [`core::num`]. With these macros,
//! you can effortlessly generate instances using numeric literals, constant values and
//! constant expressions, all at compile time.
//!
//! ## Features
//! * No unsafe code
//! * No dependencies
//! * `no_std` compatible
//! * Supports all numeric non-zero types from the [`core::num`] module
//! * Compile time evaluation
//! * Zero detection at compile time
//!
//! ## `NonZero` macros
//!
//! | Type | Macro |
//! |------|-------|
//! | [`NonZeroI8`][`core::num::NonZeroI8`] | [`nz::i8!`][`crate::i8`] |
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
//! ## Basic usage
//!
//! ```rust
//! # use core::num::NonZeroU8;
//! const NZ_U8_MIN: NonZeroU8 = nz::u8!(1); // with numeric literal
//! const NZ_U8_MAX: NonZeroU8 = nz::u8!(u8::MAX); // with constant value
//! let sum = nz::u8!(NZ_U8_MAX.get() & NZ_U8_MIN.get() + 7); // with constant expression
//! # assert_eq!(0b1000, sum.get());
//! ```
//!
//! ## Remarks
//!
//! Non-zero macros cannot be used with constant function arguments as they
//! are not constant values.
//!
//! ### Example
//!
//! ```rust, compile_fail
//! # use core::num::NonZeroU64;
//! # use nz;
//! const fn wrapping_add_nz(a: u64, b: NonZeroU64) -> NonZeroU64 {
//!     // `a` and `b` is not constant
//!     nz::u64!(a.wrapping_add(b.get())) // <- cause of the compile error
//! }
//!
//! let nz = wrapping_add_nz(2, nz::u64!(1));
//! ```

#![no_std]
#![forbid(unsafe_code)]

#[doc(hidden)]
macro_rules! gen_nonzero_macros {
    (
        $(#[$zero_error_attr:meta])*
        type ZeroError = $zero_error:ident;
        $(
            $non_zero_type:ty => {
                type Numeric = $num_type:ty;

                $(#[$macro_attr:meta])*
                macro_rules! $macro_name:ident;
            }
        )*
    ) => {
        $(#[$zero_error_attr])*
        pub enum $zero_error {}
        $(
            $(#[$macro_attr])*
            macro_rules! $macro_name {
                ($num:expr) => {{
                    const __NUM__: $num_type = $num;
                    const __ZERO_CHECK__: [$crate::$zero_error; (__NUM__ == 0) as usize] = [];
                    const __NZ__: $non_zero_type = match <$non_zero_type>::new(__NUM__) {
                        Some(nz) => nz,
                        None => loop {}, // unreachable
                    };
                    __NZ__
                }};
            }
        )*
    };
}

gen_nonzero_macros! {
    /// The error that is shown when the macro argument fails the zero check.
    #[allow(clippy::exhaustive_enums)]
    type ZeroError = ZeroIsInvalidValueForNonZero;

    core::num::NonZeroI8 => {
        type Numeric = i8;

        /// Creates a [`NonZeroI8`][`core::num::NonZeroI8`] from an
        /// [`prim@i8`] literal, constant value or a constant expression
        /// that evaluates to [`prim@i8`].
        ///
        /// # Compilation errors
        ///
        /// Compilation error will occur in the following cases:
        /// * The argument is either zero or non-constant
        /// * The arugment cannot be evaluated to a non-zero [prim@i8]
        ///
        /// # Examples
        ///
        /// #### Creating `NonZeroI8` using a numeric literal
        /// ```rust
        /// # use core::num::NonZeroI8;
        /// let nz = nz::i8!(27);
        /// const NZ: NonZeroI8 = nz::i8!(0x10);
        /// # assert_eq!(27, nz.get());
        /// # assert_eq!(0x10, NZ.get());
        /// ```
        ///
        /// #### Creating `NonZeroI8` using a constant value
        /// ```rust
        /// # use core::num::NonZeroI8;
        /// const MIN: i8 = -128;
        /// const MAX: i8 = 127;
        /// let nz = nz::i8!(MAX);
        /// const NZ: NonZeroI8 = nz::i8!(MIN);
        /// # assert_eq!(MAX, nz.get());
        /// # assert_eq!(MIN, NZ.get());
        /// ```
        ///
        /// #### Creating `NonZeroI8` using a constant expression
        /// ```rust
        /// # use core::num::NonZeroI8;
        /// const NZ: NonZeroI8 = nz::i8!(0b1100 & 0b0110);
        /// let nz = nz::i8!(NZ.get() + 1);
        /// # assert_eq!(0b0100, NZ.get());
        /// # assert_eq!(5, nz.get());
        /// ```
        ///
        /// #### Zero literal fails to compile
        /// ```rust, compile_fail
        /// # use core::num::NonZeroI8;
        /// let _ = nz::i8!(0);
        /// ```
        ///
        /// #### Non-constant expression fails to compile
        /// ```rust, compile_fail
        /// # use core::num::NonZeroI8;
        /// fn add(a: i8, b: i8) -> i8 { a.wrapping_add(b) }
        /// let _ = nz::i8!(add(1, 1));
        /// ```
        ///
        /// #### Constant expression that evaluates to zero fails to compile
        /// ```rust, compile_fail
        /// # use core::num::NonZeroI8;
        /// let _ = nz::i8!(1 - 1);
        /// ```
        #[macro_export]
        macro_rules! i8;
    }

    core::num::NonZeroI16 => {
        type Numeric = i16;

        /// Creates a [`NonZeroI16`][`core::num::NonZeroI16`] from an
        /// [`prim@i16`] literal, constant value or a constant expression
        /// that evaluates to [`prim@i16`].
        ///
        /// # Compilation errors
        ///
        /// Compilation error will occur in the following cases:
        /// * The argument is either zero or non-constant
        /// * The arugment cannot be evaluated to a non-zero [prim@i16]
        ///
        /// # Examples
        ///
        /// #### Creating `NonZeroI16` using a numeric literal
        /// ```rust
        /// # use core::num::NonZeroI16;
        /// let nz = nz::i16!(0b0011_0001);
        /// const NZ: NonZeroI16 = nz::i16!(61);
        /// # assert_eq!(0b0011_0001, nz.get());
        /// # assert_eq!(61, NZ.get());
        /// ```
        ///
        /// #### Creating `NonZeroI16` using a constant value
        /// ```rust
        /// # use core::num::NonZeroI16;
        /// const POSITIVE: i16 = 1;
        /// const NEGATIVE: i16 = -8;
        /// let nz = nz::i16!(POSITIVE);
        /// const NZ: NonZeroI16 = nz::i16!(NEGATIVE);
        /// # assert_eq!(POSITIVE, nz.get());
        /// # assert_eq!(NEGATIVE, NZ.get());
        /// ```
        ///
        /// #### Creating `NonZeroI16` using a constant expression
        /// ```rust
        /// # use core::num::NonZeroI16;
        /// const NZ: NonZeroI16 = nz::i16!(2 * 1);
        /// let nz = nz::i16!(0 - 1);
        /// # assert_eq!(2, NZ.get());
        /// # assert_eq!(-1, nz.get());
        /// ```
        ///
        /// #### Zero literal fails to compile
        /// ```rust, compile_fail
        /// # use core::num::NonZeroI16;
        /// let _ = nz::i16!(0);
        /// ```
        ///
        /// #### Non-constant expression fails to compile
        /// ```rust, compile_fail
        /// # use core::num::NonZeroI16;
        /// fn add(a: i16, b: i16) -> i16 { a.wrapping_add(b) }
        /// let _ = nz::i16!(add(8, 8));
        /// ```
        ///
        /// #### Constant expression that evaluates to zero fails to compile
        /// ```rust, compile_fail
        /// # use core::num::NonZeroI16;
        /// let _ = nz::i16!(0x0101 - 0x0101);
        /// ```
        #[macro_export]
        macro_rules! i16;
    }

    core::num::NonZeroI32 => {
        type Numeric = i32;

        /// Creates a [`NonZeroI32`][`core::num::NonZeroI32`] from an
        /// [`prim@i32`] literal, constant value or a constant expression
        /// that evaluates to [`prim@i32`].
        ///
        /// # Compilation errors
        ///
        /// Compilation error will occur in the following cases:
        /// * The argument is either zero or non-constant
        /// * The arugment cannot be evaluated to a non-zero [prim@i32]
        ///
        /// # Examples
        ///
        /// #### Creating `NonZeroI32` using a numeric literal
        /// ```rust
        /// # use core::num::NonZeroI32;
        /// let nz = nz::i32!(99);
        /// const NZ: NonZeroI32 = nz::i32!(0o32);
        /// # assert_eq!(99, nz.get());
        /// # assert_eq!(0o32, NZ.get());
        /// ```
        ///
        /// #### Creating `NonZeroI32` using a constant value
        /// ```rust
        /// # use core::num::NonZeroI32;
        /// const POSITIVE: i32 = 8;
        /// const NEGATIVE: i32 = -1;
        /// let nz = nz::i32!(POSITIVE);
        /// const NZ: NonZeroI32 = nz::i32!(NEGATIVE);
        /// # assert_eq!(POSITIVE, nz.get());
        /// # assert_eq!(NEGATIVE, NZ.get());
        /// ```
        ///
        /// #### Creating `NonZeroI32` using a constant expression
        /// ```rust
        /// # use core::num::NonZeroI32;
        /// const NZ: NonZeroI32 = nz::i32!(0x0F & 0xFF);
        /// let nz = nz::i32!(-9 + 7);
        /// # assert_eq!(0x0F, NZ.get());
        /// # assert_eq!(-2, nz.get());
        /// ```
        ///
        /// #### Zero literal fails to compile
        /// ```rust, compile_fail
        /// # use core::num::NonZeroI32;
        /// let _ = nz::i32!(0);
        /// ```
        ///
        /// #### Non-constant expression fails to compile
        /// ```rust, compile_fail
        /// # use core::num::NonZeroI32;
        /// fn sub(a: i32, b: i32) -> i32 { a.wrapping_sub(b) }
        /// let _ = nz::i32!(sub(-3, 1));
        /// ```
        ///
        /// #### Constant expression that evaluates to zero fails to compile
        /// ```rust, compile_fail
        /// # use core::num::NonZeroI32;
        /// let _ = nz::i32!(0xFFFF * 0x00);
        /// ```
        #[macro_export]
        macro_rules! i32;
    }

    core::num::NonZeroI64 => {
        type Numeric = i64;

        /// Creates a [`NonZeroI64`][`core::num::NonZeroI64`] from an
        /// [`prim@i64`] literal, constant value or a constant expression
        /// that evaluates to [`prim@i64`].
        ///
        /// # Compilation errors
        ///
        /// Compilation error will occur in the following cases:
        /// * The argument is either zero or non-constant
        /// * The arugment cannot be evaluated to a non-zero [prim@i64]
        ///
        /// # Examples
        ///
        /// #### Creating `NonZeroI64` using a numeric literal
        /// ```rust
        /// # use core::num::NonZeroI64;
        /// let nz = nz::i64!(841);
        /// const NZ: NonZeroI64 = nz::i64!(0xFEFF);
        /// # assert_eq!(841, nz.get());
        /// # assert_eq!(0xFEFF, NZ.get());
        /// ```
        ///
        /// #### Creating `NonZeroI64` using a constant value
        /// ```rust
        /// # use core::num::NonZeroI64;
        /// const POSITIVE: i64 = 4;
        /// const NEGATIVE: i64 = -3;
        /// let nz = nz::i64!(POSITIVE);
        /// const NZ: NonZeroI64 = nz::i64!(NEGATIVE);
        /// # assert_eq!(POSITIVE, nz.get());
        /// # assert_eq!(NEGATIVE, NZ.get());
        /// ```
        ///
        /// #### Creating `NonZeroI64` using a constant expression
        /// ```rust
        /// # use core::num::NonZeroI64;
        /// const NZ: NonZeroI64 = nz::i64!(11 % 3);
        /// let nz = nz::i64!(0b01 ^ 0b10);
        /// # assert_eq!(2, NZ.get());
        /// # assert_eq!(0b11, nz.get());
        /// ```
        ///
        /// #### Zero literal fails to compile
        /// ```rust, compile_fail
        /// # use core::num::NonZeroI64;
        /// let _ = nz::i64!(0);
        /// ```
        ///
        /// #### Non-constant expression fails to compile
        /// ```rust, compile_fail
        /// # use core::num::NonZeroI64;
        /// fn sub(a: i64, b: i64) -> i64 { a.wrapping_sub(b) }
        /// let _ = nz::i64!(sub(0, 1));
        /// ```
        ///
        /// #### Constant expression that evaluates to zero fails to compile
        /// ```rust, compile_fail
        /// # use core::num::NonZeroI64;
        /// let _ = nz::i64!(0 | 0);
        /// ```
        #[macro_export]
        macro_rules! i64;
    }

    core::num::NonZeroI128 => {
        type Numeric = i128;

        /// Creates a [`NonZeroI128`][`core::num::NonZeroI128`] from an
        /// [`prim@i128`] literal, constant value or a constant expression
        /// that evaluates to [`prim@i128`].
        ///
        /// # Compilation errors
        ///
        /// Compilation error will occur in the following cases:
        /// * The argument is either zero or non-constant
        /// * The arugment cannot be evaluated to a non-zero [prim@i128]
        ///
        /// # Examples
        ///
        /// #### Creating `NonZeroI128` using a numeric literal
        /// ```rust
        /// # use core::num::NonZeroI128;
        /// let nz = nz::i128!(0b1111_1110);
        /// const NZ: NonZeroI128 = nz::i128!(72);
        /// # assert_eq!(0b1111_1110, nz.get());
        /// # assert_eq!(72, NZ.get());
        /// ```
        ///
        /// #### Creating `NonZeroI128` using a constant value
        /// ```rust
        /// # use core::num::NonZeroI128;
        /// const POSITIVE: i128 = 3;
        /// const NEGATIVE: i128 = -4;
        /// let nz = nz::i128!(POSITIVE);
        /// const NZ: NonZeroI128 = nz::i128!(NEGATIVE);
        /// # assert_eq!(POSITIVE, nz.get());
        /// # assert_eq!(NEGATIVE, NZ.get());
        /// ```
        ///
        /// #### Creating `NonZeroI128` using a constant expression
        /// ```rust
        /// # use core::num::NonZeroI128;
        /// const NZ: NonZeroI128 = nz::i128!(5 - 6);
        /// let nz = nz::i128!(0b01 << 1);
        /// # assert_eq!(-1, NZ.get());
        /// # assert_eq!(0b10, nz.get());
        /// ```
        ///
        /// #### Zero literal fails to compile
        /// ```rust, compile_fail
        /// # use core::num::NonZeroI128;
        /// let _ = nz::i128!(0);
        /// ```
        ///
        /// #### Non-constant expression fails to compile
        /// ```rust, compile_fail
        /// # use core::num::NonZeroI128;
        /// fn mul(a: i128, b: i128) -> i28 { a.wrapping_mul(b) }
        /// let _ = nz::i128!(mul(7, 12));
        /// ```
        ///
        /// #### Constant expression that evaluates to zero fails to compile
        /// ```rust, compile_fail
        /// # use core::num::NonZeroI128;
        /// let _ = nz::i128!(0b0000 << 4);
        /// ```
        #[macro_export]
        macro_rules! i128;
    }

    core::num::NonZeroIsize => {
        type Numeric = isize;

        /// Creates a [`NonZeroIsize`][`core::num::NonZeroIsize`] from an
        /// [`prim@isize`] literal, constant value or a constant expression
        /// that evaluates to [`prim@isize`].
        ///
        /// # Compilation errors
        ///
        /// Compilation error will occur in the following cases:
        /// * The argument is either zero or non-constant
        /// * The arugment cannot be evaluated to a non-zero [prim@isize]
        ///
        /// # Examples
        ///
        /// #### Creating `NonZeroIsize` using a numeric literal
        /// ```rust
        /// # use core::num::NonZeroIsize;
        /// let nz = nz::isize!(2023);
        /// const NZ: NonZeroIsize = nz::isize!(0b0001);
        /// # assert_eq!(2023, nz.get());
        /// # assert_eq!(1, NZ.get());
        /// ```
        ///
        /// #### Creating `NonZeroIsize` using a constant value
        /// ```rust
        /// # use core::num::NonZeroIsize;
        /// const POSITIVE: isize = 7;
        /// const NEGATIVE: isize = -4;
        /// let nz = nz::isize!(POSITIVE);
        /// const NZ: NonZeroIsize = nz::isize!(NEGATIVE);
        /// # assert_eq!(POSITIVE, nz.get());
        /// # assert_eq!(NEGATIVE, NZ.get());
        /// ```
        ///
        /// #### Creating `NonZeroIsize` using a constant expression
        /// ```rust
        /// # use core::num::NonZeroIsize;
        /// const NZ: NonZeroIsize = nz::isize!(0x10 << 8);
        /// let nz = nz::isize!(13 >> 2);
        /// # assert_eq!(0x1000, NZ.get());
        /// # assert_eq!(3, nz.get());
        /// ```
        ///
        /// #### Zero literal fails to compile
        /// ```rust, compile_fail
        /// # use core::num::NonZeroIsize;
        /// let _ = nz::isize!(0);
        /// ```
        ///
        /// #### Non-constant expression fails to compile
        /// ```rust, compile_fail
        /// # use core::num::NonZeroIsize;
        /// fn mul(a: isize, b: isize) -> isize { a.wrapping_mul(b) }
        /// let _ = nz::isize!(mul(32, 2));
        /// ```
        ///
        /// #### Constant expression that evaluates to zero fails to compile
        /// ```rust, compile_fail
        /// # use core::num::NonZeroIsize;
        /// let _ = nz::isize!(0 << 2);
        /// ```
        #[macro_export]
        macro_rules! isize;
    }

    core::num::NonZeroU8 => {
        type Numeric = u8;

        /// Creates a [`NonZeroU8`][`core::num::NonZeroU8`] from an
        /// [`prim@u8`] literal, constant value or a constant expression
        /// that evaluates to [`prim@u8`].
        ///
        /// # Compilation errors
        ///
        /// Compilation error will occur in the following cases:
        /// * The argument is either zero or non-constant
        /// * The arugment cannot be evaluated to a non-zero [prim@u8]
        ///
        /// # Examples
        ///
        /// #### Creating `NonZeroU8` using a numeric literal
        /// ```rust
        /// # use core::num::NonZeroU8;
        /// let nz = nz::u8!(0o17);
        /// const NZ: NonZeroU8 = nz::u8!(25);
        /// # assert_eq!(0o17, nz.get());
        /// # assert_eq!(25, NZ.get());
        /// ```
        ///
        /// #### Creating `NonZeroU8` using a constant value
        /// ```rust
        /// # use core::num::NonZeroU8;
        /// const ONE: u8 = 1;
        /// const LIMIT: u8 = 255;
        /// let nz = nz::u8!(ONE);
        /// const NZ: NonZeroU8 = nz::u8!(LIMIT);
        /// # assert_eq!(ONE, nz.get());
        /// # assert_eq!(LIMIT, NZ.get());
        /// ```
        ///
        /// #### Creating `NonZeroU8` using a constant expression
        /// ```rust
        /// # use core::num::NonZeroU8;
        /// const NZ: NonZeroU8 = nz::u8!(254 + 1);
        /// let nz = nz::u8!(0x10 & 0xF1);
        /// # assert_eq!(255, NZ.get());
        /// # assert_eq!(0x10, nz.get());
        /// ```
        ///
        /// #### Zero literal fails to compile
        /// ```rust, compile_fail
        /// # use core::num::NonZeroU8;
        /// let _ = nz::u8!(0);
        /// ```
        ///
        /// #### Non-constant expression fails to compile
        /// ```rust, compile_fail
        /// # use core::num::NonZeroU8;
        /// fn rem(a: u8, b: u8) -> u8 { a.wrapping_rem(b) }
        /// let _ = nz::u8!(rem(19, 2));
        /// ```
        ///
        /// #### Constant expression that evaluates to zero fails to compile
        /// ```rust, compile_fail
        /// # use core::num::NonZeroU8;
        /// let _ = nz::u8!(!0xFF);
        /// ```
        #[macro_export]
        macro_rules! u8;
    }

    core::num::NonZeroU16 => {
        type Numeric = u16;

        /// Creates a [`NonZeroU16`][`core::num::NonZeroU16`] from an
        /// [`prim@u16`] literal, constant value or a constant expression
        /// that evaluates to [`prim@u16`].
        ///
        /// # Compilation errors
        ///
        /// Compilation error will occur in the following cases:
        /// * The argument is either zero or non-constant
        /// * The arugment cannot be evaluated to a non-zero [prim@u16]
        ///
        /// # Examples
        ///
        /// #### Creating `NonZeroU16` using a numeric literal
        /// ```rust
        /// # use core::num::NonZeroU16;
        /// let nz = nz::u16!(283);
        /// const NZ: NonZeroU16 = nz::u16!(0b0001_1111);
        /// # assert_eq!(283, nz.get());
        /// # assert_eq!(0b0001_1111, NZ.get());
        /// ```
        ///
        /// #### Creating `NonZeroU16` using a constant value
        /// ```rust
        /// # use core::num::NonZeroU16;
        /// const ONE: u16 = 1;
        /// const LIMIT: u16 = 65535;
        /// let nz = nz::u16!(LIMIT);
        /// const NZ: NonZeroU16 = nz::u16!(ONE);
        /// # assert_eq!(LIMIT, nz.get());
        /// # assert_eq!(ONE, NZ.get());
        /// ```
        ///
        /// #### Creating `NonZeroU16` using a constant expression
        /// ```rust
        /// # use core::num::NonZeroU16;
        /// const NZ: NonZeroU16 = nz::u16!(0x04 | 0x02);
        /// let nz = nz::u16!(!0);
        /// # assert_eq!(6, NZ.get());
        /// # assert_eq!(65535, nz.get());
        /// ```
        ///
        /// #### Zero literal fails to compile
        /// ```rust, compile_fail
        /// # use core::num::NonZeroU16;
        /// let _ = nz::u16!(0);
        /// ```
        ///
        /// #### Non-constant expression fails to compile
        /// ```rust, compile_fail
        /// # use core::num::NonZeroU16;
        /// fn rem(a: u16, b: u16) -> u16 { a.wrapping_rem(b) }
        /// let _ = nz::u16!(rem(19, 2));
        /// ```
        ///
        /// #### Constant expression that evaluates to zero fails to compile
        /// ```rust, compile_fail
        /// # use core::num::NonZeroU16;
        /// let _ = nz::u16!((1 == 0) as u16);
        /// ```
        #[macro_export]
        macro_rules! u16;
    }

    core::num::NonZeroU32 => {
        type Numeric = u32;

        /// Creates a [`NonZeroU32`][`core::num::NonZeroU32`] from an
        /// [`prim@u32`] literal, constant value or a constant expression
        /// that evaluates to [`prim@u32`].
        ///
        /// # Compilation errors
        ///
        /// Compilation error will occur in the following cases:
        /// * The argument is either zero or non-constant
        /// * The arugment cannot be evaluated to a non-zero [prim@u32]
        ///
        /// # Examples
        ///
        /// #### Creating `NonZeroU32` using a numeric literal
        /// ```rust
        /// # use core::num::NonZeroU32;
        /// let nz = nz::u32!(0o713);
        /// const NZ: NonZeroU32 = nz::u32!(3);
        /// # assert_eq!(0o713, nz.get());
        /// # assert_eq!(3, NZ.get());
        /// ```
        ///
        /// #### Creating `NonZeroU32` using a constant value
        /// ```rust
        /// # use core::num::NonZeroU32;
        /// const ONE: u32 = 1;
        /// const LIMIT: u32 = 101;
        /// let nz = nz::u32!(ONE);
        /// const NZ: NonZeroU32 = nz::u32!(LIMIT);
        /// # assert_eq!(ONE, nz.get());
        /// # assert_eq!(LIMIT, NZ.get());
        /// ```
        ///
        /// #### Creating `NonZeroU32` using a constant expression
        /// ```rust
        /// # use core::num::NonZeroU32;
        /// const NZ: NonZeroU32 = nz::u32!(0o01 & 0o51);
        /// let nz = nz::u32!(255 % 7);
        /// # assert_eq!(1, NZ.get());
        /// # assert_eq!(3, nz.get());
        /// ```
        ///
        /// #### Zero literal fails to compile
        /// ```rust, compile_fail
        /// # use core::num::NonZeroU32;
        /// let _ = nz::u32!(0);
        /// ```
        ///
        /// #### Non-constant expression fails to compile
        /// ```rust, compile_fail
        /// # use core::num::NonZeroU32;
        /// fn xor(a: u32, b: u32) -> u32 { a ^ b }
        /// let _ = nz::u32!(xor(5, 4));
        /// ```
        ///
        /// #### Constant expression that evaluates to zero fails to compile
        /// ```rust, compile_fail
        /// # use core::num::NonZeroU32;
        /// let _ = nz::u32!(30 % 5);
        /// ```
        #[macro_export]
        macro_rules! u32;
    }

    core::num::NonZeroU64 => {
        type Numeric = u64;

        /// Creates a [`NonZeroU64`][`core::num::NonZeroU64`] from an
        /// [`prim@u64`] literal, constant value or a constant expression
        /// that evaluates to [`prim@u64`].
        ///
        /// # Compilation errors
        ///
        /// Compilation error will occur in the following cases:
        /// * The argument is either zero or non-constant
        /// * The arugment cannot be evaluated to a non-zero [prim@u64]
        ///
        /// # Examples
        ///
        /// #### Creating `NonZeroU64` using a numeric literal
        /// ```rust
        /// # use core::num::NonZeroU64;
        /// let nz = nz::u64!(40);
        /// const NZ: NonZeroU64 = nz::u64!(0xABF1);
        /// # assert_eq!(40, nz.get());
        /// # assert_eq!(0xABF1, NZ.get());
        /// ```
        ///
        /// #### Creating `NonZeroU64` using a constant value
        /// ```rust
        /// # use core::num::NonZeroU64;
        /// const ONE: u64 = 1;
        /// const LIMIT: u64 = 24;
        /// let nz = nz::u64!(ONE);
        /// const NZ: NonZeroU64 = nz::u64!(LIMIT);
        /// # assert_eq!(ONE, nz.get());
        /// # assert_eq!(LIMIT, NZ.get());
        /// ```
        ///
        /// #### Creating `NonZeroU64` using a constant expression
        /// ```rust
        /// # use core::num::NonZeroU64;
        /// const NZ: NonZeroU64 = nz::u64!(0x09 * 0x04);
        /// let nz = nz::u64!(3 ^ 1);
        /// # assert_eq!(0x24, NZ.get());
        /// # assert_eq!(2, nz.get());
        /// ```
        ///
        /// #### Zero literal fails to compile
        /// ```rust, compile_fail
        /// # use core::num::NonZeroU64;
        /// let _ = nz::u64!(0);
        /// ```
        ///
        /// #### Non-constant expression fails to compile
        /// ```rust, compile_fail
        /// # use core::num::NonZeroU32;
        /// fn xor(a: u32, b: u32) -> u32 { a ^ b }
        /// let _ = nz::u32!(xor(2, 0b1011));
        /// ```
        ///
        /// #### Constant expression that evaluates to zero fails to compile
        /// ```rust, compile_fail
        /// # use core::num::NonZeroU64;
        /// let _ = nz::u64!(-0x01 + 0x01);
        /// ```
        #[macro_export]
        macro_rules! u64;
    }

    core::num::NonZeroU128 => {
        type Numeric = u128;

        /// Creates a [`NonZeroU128`][`core::num::NonZeroU128`] from an
        /// [`prim@u128`] literal, constant value or a constant expression
        /// that evaluates to [`prim@u128`].
        ///
        /// # Compilation errors
        ///
        /// Compilation error will occur in the following cases:
        /// * The argument is either zero or non-constant
        /// * The arugment cannot be evaluated to a non-zero [prim@u128]
        ///
        /// # Examples
        ///
        /// #### Creating `NonZeroU128` using a numeric literal
        /// ```rust
        /// # use core::num::NonZeroU128;
        /// let nz = nz::u128!(0o200);
        /// const NZ: NonZeroU128 = nz::u128!(80);
        /// # assert_eq!(0o200, nz.get());
        /// # assert_eq!(80, NZ.get());
        /// ```
        ///
        /// #### Creating `NonZeroU128` using a constant value
        /// ```rust
        /// # use core::num::NonZeroU128;
        /// const ONE: u128 = 1;
        /// const LIMIT: u128 = 128;
        /// let nz = nz::u128!(LIMIT);
        /// const NZ: NonZeroU128 = nz::u128!(ONE);
        /// # assert_eq!(LIMIT, nz.get());
        /// # assert_eq!(ONE, NZ.get());
        /// ```
        ///
        /// #### Creating `NonZeroU128` using a constant expression
        /// ```rust
        /// # use core::num::NonZeroU128;
        /// const NZ: NonZeroU128 = nz::u128!(44 + 26);
        /// let nz = nz::u128!(0b0011 >> 1);
        /// # assert_eq!(70, NZ.get());
        /// # assert_eq!(0b0001, nz.get());
        /// ```
        ///
        /// #### Zero literal fails to compile
        /// ```rust, compile_fail
        /// # use core::num::NonZeroU128;
        /// let _ = nz::u128!(0);
        /// ```
        ///
        /// #### Non-constant expression fails to compile
        /// ```rust, compile_fail
        /// # use core::num::NonZeroU128;
        /// fn and(a: u128, b: u128) -> u128 { a & b }
        /// let _ = nz::u128!(and(11, 3));
        /// ```
        ///
        /// #### Constant expression that evaluates to zero fails to compile
        /// ```rust, compile_fail
        /// # use core::num::NonZeroU128;
        /// let _ = nz::u128!(0 + 0);
        /// ```
        #[macro_export]
        macro_rules! u128;
    }

    core::num::NonZeroUsize => {
        type Numeric = usize;

        /// Creates a [`NonZeroUsize`][`core::num::NonZeroUsize`] from an
        /// [`prim@usize`] literal, constant value or a constant expression
        /// that evaluates to [`prim@usize`].
        ///
        /// # Compilation errors
        ///
        /// Compilation error will occur in the following cases:
        /// * The argument is either zero or non-constant
        /// * The arugment cannot be evaluated to a non-zero [prim@usize]
        ///
        /// # Examples
        ///
        /// #### Creating `NonZeroUsize` using a numeric literal
        /// ```rust
        /// # use core::num::NonZeroUsize;
        /// let nz = nz::usize!(2);
        /// const NZ: NonZeroUsize = nz::usize!(0x10FF);
        /// # assert_eq!(2, nz.get());
        /// # assert_eq!(0x10FF, NZ.get());
        /// ```
        ///
        /// #### Creating `NonZeroUsize` using a constant value
        /// ```rust
        /// # use core::num::NonZeroUsize;
        /// const ONE: usize = 1;
        /// const LIMIT: usize = 36;
        /// let nz = nz::usize!(ONE);
        /// const NZ: NonZeroUsize = nz::usize!(LIMIT);
        /// # assert_eq!(ONE, nz.get());
        /// # assert_eq!(LIMIT, NZ.get());
        /// ```
        ///
        /// #### Creating `NonZeroUsize` using a constant expression
        /// ```rust
        /// # use core::num::NonZeroUsize;
        /// const NZ: NonZeroUsize = nz::usize!(0o2 * 0o4);
        /// let nz = nz::usize!(4 / 3);
        /// # assert_eq!(0o10, NZ.get());
        /// # assert_eq!(1, nz.get());
        /// ```
        ///
        /// #### Zero literal fails to compile
        /// ```rust, compile_fail
        /// # use core::num::NonZeroUsize;
        /// let _ = nz::usize!(0);
        /// ```
        ///
        /// #### Non-constant expression fails to compile
        /// ```rust, compile_fail
        /// # use core::num::NonZeroUsize;
        /// fn and(a: usize, b: usize) -> usize { a & b }
        /// let _ = nz::usize!(and(2, 66));
        /// ```
        ///
        /// #### Constant expression that evaluates to zero fails to compile
        /// ```rust, compile_fail
        /// # use core::num::NonZeroUsize;
        /// let _ = nz::usize!(0b0100 ^ 0b0100);
        /// ```
        #[macro_export]
        macro_rules! usize;
    }
}
