//! # nz
//!
//! The `nz` crate provides a collection of macros that simplify the creation of
//! new instances of non-zero numeric types implemented in [`core::num`]. With
//! these macros, you can easily generate constants of such core types using numeric
//! literals, constant values or expressions, all at compile time.
//!
//! ## Features
//! * No unsafe code
//! * No dependencies
//! * `no_std` compatible
//! * Supports all `core::num::NonZero{Integer}` types
//! * Compile time evaluation
//! * Zero detection at compile time
//!
//! ## Macros
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
//! use core::num::NonZeroU8;
//! // A NonZero type can be constructed by different
//! // types of arguments when using the matching macro
//! //
//! // such argument can be a numeric literal
//! const NZ_MIN: NonZeroU8 = nz::u8!(1);
//! let nz_two = nz::u8!(2);
//! # assert_eq!(2, nz_two.get());
//! // or a constant value
//! const NZ_MAX: NonZeroU8 = nz::u8!(u8::MAX);
//! let five = nz::u8!({ const FIVE: u8 = 5; FIVE });
//! # assert_eq!(5, five.get());
//! // or even a constant expression
//! const RES: NonZeroU8 = nz::u8!({ 3 + 7 } - NZ_MIN.get());
//! // non-constant expression leads to compile-time error
//! // const OUTPUT: NonZeroU8 = nz::u8!({ 3 + 7 } - nz_two.get()); // casued by `mz_two.get()`
//! let result_as_nz = nz::u8!((NZ_MIN.get() & NZ_MAX.get()) + 7);
//! # assert_eq!(0b1000, result_as_nz.get());
//! ```
//!
//! ## Limitations
//!
//! ### const fn
//!
//! Declarative macros (such as all the `nz` macros) cannot be used with
//! constant function arguments since they are not currently recognized
//! as constant values, as demonstrated in the code below.
//!
//! ```rust, compile_fail
//! # use core::num::NonZeroU64;
//! const fn wrapping_add_nz(a: u64, b: NonZeroU64) -> NonZeroU64 {
//!     // `a` and `b` is not constant
//!     // the line below causes compile error
//!     nz::u64!(a.wrapping_add(b.get()))
//! }
//!
//! let nz = wrapping_add_nz(2, nz::u64!(1));
//! ```
//!
//! ### const hygiene
//!
//! When constants are used in a declarative macro, specifically in the
//! most outer scope where a constant can be declared, there is a possibility
//! of "name collision" when an expression is expected as an argument and an
//! outer constant is used within that expression. This "collision" can occur
//! if  any of the inner constants share the same name as the outer constant.
//! The code snippet below demonstrates this scenario.
//!
//! This collision between the outer and inner constants leads to a compile-time
//! error, specifically [`[E0391]`](<https://doc.rust-lang.org/error_codes/E0391.html>),
//! because the inner macro constant tries to reference itself, creating a cyclic
//! dependency during the evaluation of the macro at compile-time.
//!
//! ```rust, compile_fail
//! # use core::num::NonZeroU16;
//! const NZ: NonZeroU16 = nz::u16!(0xA3FE);
//! const CHECK_ZERO: NonZeroU16 = nz::u16!(777);
//! // although `CHECK_ZERO` is used in the macro
//! // it won't collide when passing it in a constant
//! // expression, because it is not in the most outer
//! // scope where a constant is declared
//! const OK: NonZeroU16 = nz::u16!(CHECK_ZERO.get());
//! // using NUM.get() is fine
//! const ___NZ___INTERNAL___NUM___1___: u16
//!     = nz::u16!(NZ.get()).get();
//! // using `___NZ___INTERNAL___NUM___1___` constant as the argument
//! // causes compile-time error in the code line below, because the
//! // internal macro constant has the same identifier
//! const FAILS: NonZeroU16 = nz::u16!(
//!     ___NZ___INTERNAL___NUM___1___ // <-- error
//! );
//! ```
//!
//! Essentially, the code above has the same error as this
//! single line:
//! ```rust, compile_fail
//! const X: u8 = X;
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
                type Numeric = $num_type:ident;

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
                    const ___NZ___INTERNAL___NUM___1___: $num_type = $num;
                    {
                        const ZERO_CHECK: [$crate::$zero_error; (___NZ___INTERNAL___NUM___1___ == 0) as usize] = [];
                        const NZ: $non_zero_type = match <$non_zero_type>::new(___NZ___INTERNAL___NUM___1___) {
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
        /// const NZ: NonZeroI8 = nz::i8!(0x10);
        /// let nz = nz::i8!(27);
        /// # assert_eq!(27, nz.get());
        /// # assert_eq!(0x10, NZ.get());
        /// ```
        ///
        /// #### Creating `NonZeroI8` using a constant value
        /// ```rust
        /// # use core::num::NonZeroI8;
        /// const MAX: i8 = 127;
        /// const NZ: NonZeroI8 = nz::i8!(MAX);
        /// let nz = nz::i8!(NZ.get());
        /// # assert_eq!(MAX, nz.get());
        /// # assert_eq!(nz, NZ);
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
        /// const NZ: NonZeroI16 = nz::i16!(61);
        /// let nz = nz::i16!(0b0011_0001);
        /// # assert_eq!(0b0011_0001, nz.get());
        /// # assert_eq!(61, NZ.get());
        /// ```
        ///
        /// #### Creating `NonZeroI16` using a constant value
        /// ```rust
        /// # use core::num::NonZeroI16;
        /// const POSITIVE: i16 = 1;
        /// const NEGATIVE: i16 = -8;
        /// const NZ: NonZeroI16 = nz::i16!(NEGATIVE);
        /// let nz = nz::i16!(POSITIVE);
        /// # assert_eq!(NEGATIVE, NZ.get());
        /// # assert_eq!(POSITIVE, nz.get());
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
        /// const NZ: NonZeroI32 = nz::i32!(0o32);
        /// let nz = nz::i32!(99);
        /// # assert_eq!(99, nz.get());
        /// # assert_eq!(0o32, NZ.get());
        /// ```
        ///
        /// #### Creating `NonZeroI32` using a constant value
        /// ```rust
        /// # use core::num::NonZeroI32;
        /// const POSITIVE: i32 = 8;
        /// const NEGATIVE: i32 = -1;
        /// const NZ: NonZeroI32 = nz::i32!(NEGATIVE);
        /// let nz = nz::i32!(POSITIVE);
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
        /// const NZ: NonZeroI64 = nz::i64!(0xFEFF);
        /// let nz = nz::i64!(841);
        /// # assert_eq!(841, nz.get());
        /// # assert_eq!(0xFEFF, NZ.get());
        /// ```
        ///
        /// #### Creating `NonZeroI64` using a constant value
        /// ```rust
        /// # use core::num::NonZeroI64;
        /// const POSITIVE: i64 = 4;
        /// const NEGATIVE: i64 = -3;
        /// const NZ: NonZeroI64 = nz::i64!(NEGATIVE);
        /// let nz = nz::i64!(POSITIVE);
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
        /// const NZ: NonZeroI128 = nz::i128!(72);
        /// let nz = nz::i128!(0b1111_1110);
        /// # assert_eq!(0b1111_1110, nz.get());
        /// # assert_eq!(72, NZ.get());
        /// ```
        ///
        /// #### Creating `NonZeroI128` using a constant value
        /// ```rust
        /// # use core::num::NonZeroI128;
        /// const POSITIVE: i128 = 3;
        /// const NEGATIVE: i128 = -4;
        /// const NZ: NonZeroI128 = nz::i128!(NEGATIVE);
        /// let nz = nz::i128!(POSITIVE);
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
        /// const NZ: NonZeroIsize = nz::isize!(0b0001);
        /// let nz = nz::isize!(2023);
        /// # assert_eq!(2023, nz.get());
        /// # assert_eq!(1, NZ.get());
        /// ```
        ///
        /// #### Creating `NonZeroIsize` using a constant value
        /// ```rust
        /// # use core::num::NonZeroIsize;
        /// const POSITIVE: isize = 7;
        /// const NEGATIVE: isize = -4;
        /// const NZ: NonZeroIsize = nz::isize!(NEGATIVE);
        /// let nz = nz::isize!(POSITIVE);
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
        /// const NZ: NonZeroU8 = nz::u8!(25);
        /// let nz = nz::u8!(0o17);
        /// # assert_eq!(0o17, nz.get());
        /// # assert_eq!(25, NZ.get());
        /// ```
        ///
        /// #### Creating `NonZeroU8` using a constant value
        /// ```rust
        /// # use core::num::NonZeroU8;
        /// const ONE: u8 = 1;
        /// const LIMIT: u8 = 255;
        /// const NZ: NonZeroU8 = nz::u8!(LIMIT);
        /// let nz = nz::u8!(ONE);
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
        /// const NZ: NonZeroU16 = nz::u16!(0b0001_1111);
        /// let nz = nz::u16!(283);
        /// # assert_eq!(283, nz.get());
        /// # assert_eq!(0b0001_1111, NZ.get());
        /// ```
        ///
        /// #### Creating `NonZeroU16` using a constant value
        /// ```rust
        /// # use core::num::NonZeroU16;
        /// const ONE: u16 = 1;
        /// const LIMIT: u16 = 65535;
        /// const NZ: NonZeroU16 = nz::u16!(ONE);
        /// let nz = nz::u16!(LIMIT);
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
        /// const NZ: NonZeroU32 = nz::u32!(3);
        /// let nz = nz::u32!(0o713);
        /// # assert_eq!(0o713, nz.get());
        /// # assert_eq!(3, NZ.get());
        /// ```
        ///
        /// #### Creating `NonZeroU32` using a constant value
        /// ```rust
        /// # use core::num::NonZeroU32;
        /// const ONE: u32 = 1;
        /// const LIMIT: u32 = 101;
        /// const NZ: NonZeroU32 = nz::u32!(LIMIT);
        /// let nz = nz::u32!(ONE);
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
        /// const NZ: NonZeroU64 = nz::u64!(0xABF1);
        /// let nz = nz::u64!(40);
        /// # assert_eq!(40, nz.get());
        /// # assert_eq!(0xABF1, NZ.get());
        /// ```
        ///
        /// #### Creating `NonZeroU64` using a constant value
        /// ```rust
        /// # use core::num::NonZeroU64;
        /// const ONE: u64 = 1;
        /// const LIMIT: u64 = 24;
        /// const NZ: NonZeroU64 = nz::u64!(LIMIT);
        /// let nz = nz::u64!(ONE);
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
        /// const NZ: NonZeroU128 = nz::u128!(80);
        /// let nz = nz::u128!(0o200);
        /// # assert_eq!(0o200, nz.get());
        /// # assert_eq!(80, NZ.get());
        /// ```
        ///
        /// #### Creating `NonZeroU128` using a constant value
        /// ```rust
        /// # use core::num::NonZeroU128;
        /// const ONE: u128 = 1;
        /// const LIMIT: u128 = 128;
        /// const NZ: NonZeroU128 = nz::u128!(ONE);
        /// let nz = nz::u128!(LIMIT);
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
        /// const NZ: NonZeroUsize = nz::usize!(0x10FF);
        /// let nz = nz::usize!(2);
        /// # assert_eq!(2, nz.get());
        /// # assert_eq!(0x10FF, NZ.get());
        /// ```
        ///
        /// #### Creating `NonZeroUsize` using a constant value
        /// ```rust
        /// # use core::num::NonZeroUsize;
        /// const ONE: usize = 1;
        /// const LIMIT: usize = 36;
        /// const NZ: NonZeroUsize = nz::usize!(LIMIT);
        /// let nz = nz::usize!(ONE);
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
