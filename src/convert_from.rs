use paste::paste;

/// A generic trait for converting from possible types.
///
/// # Examples
/// Usage:
///
/// ```
/// # use num_convert::FromByAdd;
/// // -128_i8 -> 0_u32
/// assert_eq!(<u8>::MIN as u32, <u32 as FromByAdd>::from_i8(<i8>::MIN));
/// // 127_i8 -> 255_u64
/// assert_eq!(<u8>::MAX as u64, <u64 as FromByAdd>::from_i8(<i8>::MAX));
/// ```

pub trait FromByAdd {
    /// Converts the value from `i8` to `self`.
    fn from_i8(n: i8) -> Self;

    /// Converts the value from `u8` to `self`.
    fn from_u8(n: u8) -> Self;

    /// Converts the value from `i16` to `self`.
    fn from_i16(n: i16) -> Self;

    /// Converts the value from `u16` to `self`.
    fn from_u16(n: u16) -> Self;

    /// Converts the value from `i32` to `self`.
    fn from_i32(n: i32) -> Self;

    /// Converts the value from `u32` to `self`.
    fn from_u32(n: u32) -> Self;

    /// Converts the value from `i64` to `self`.
    fn from_i64(n: i64) -> Self;

    /// Converts the value from `u64` to `self`.
    fn from_u64(n: u64) -> Self;

    /// Converts the value from `isize` to `self`.
    fn from_isize(n: isize) -> Self;

    /// Converts the value from `usize` to `self`.
    fn from_usize(n: usize) -> Self;

    /// Converts the value from `i128` to `self`.
    fn from_i128(n: i128) -> Self;

    /// Converts the value from `u128` to `self`.
    fn from_u128(n: u128) -> Self;
}

macro_rules! fns {
    //equal type
    //from signed = to signed
    //from unsigned = to unsigned
    ( equ $from_type:ty; $to_type:ty ) => {
        paste!{
            #[inline]
            fn [<from_$from_type>](n: $from_type) -> $to_type {
                n
            }
        }
    };

    //as equal type
    //from signed < to signed
    //from unsigned < to unsigned
    ( as equ $($from_type:ty),+; $to_type:ty ) => {
        $(
        paste!{
            #[inline]
            fn [<from_$from_type>](n: $from_type) -> $to_type {
                n as $to_type
            }
        }
        )*
    };

    //from signed > to signed
    //from unsigned > to unsigned
    ( $($from_type:ty),+; $to_type:ty ) => {
        $(
        paste!{
            #[inline]
            fn [<from_$from_type>](_n: $from_type) -> $to_type {
                unimplemented!();
            }
        }
        )*
    };


    //from unsigned > to unsigned
    ( $($as_type:ty = $from_type:ty),+; $to_type:ty ) => {
        $(
        paste!{
            #[inline]
            fn [<from_$from_type>](n: $from_type) -> $to_type {
                ((n as $as_type).wrapping_add($as_type::MAX)).wrapping_add(1) as $to_type
            }
        }
        )*
    };

    //from unsigned > to unsigned
    ( sig $($from_type:ty),+; $to_type:ty ) => {
        $(
        paste!{
            #[inline]
            fn [<from_$from_type>](n: $from_type) -> $to_type {
                ((n as $to_type).wrapping_add($to_type::MAX)).wrapping_add(1)
            }
        }
        )*
    };

    //from signed to unsigned
    ( $($add_value:expr, $from_type:ty),+; $to_type:ty ) => {
        $(
        paste!{
            #[inline]
            fn [<from_$from_type>](n: $from_type) -> $to_type {
                (n as $to_type).wrapping_add($add_value)
            }
        }
        )*
    };
}

macro_rules! from_impls {
    ( $value_i32:expr; $value_i64:expr; $value_isize:expr ) => {
        impl FromByAdd for i8 {
            fns! {equ i8; i8}
            fns! {sig u8; i8}
            fns! {i16, i32, i64, isize, i128, u16, u32, u64, usize, u128; i8}
        }

        impl FromByAdd for i16 {
            fns! {i8 = u8; i16}
            fns! {sig u16; i16}
            fns! {as equ i8; i16}
            fns! {equ i16; i16}
            fns! {i32, i64, isize, i128, u32, u64, usize, u128; i16}
        }

        // For adding 32 bit arch.
        #[cfg(target_pointer_width = "32")]
        impl FromByAdd for i32 {
            fns! {i8 = u8, i16 = u16, isize = usize; i32}
            fns! {sig u32; i32}
            fns! {as equ i8, i16, isize; i32}
            fns! {equ i32; i32}
            fns! {i64, i128, u64, u128; i32}
        }

        // For adding 64 bit arch.
        #[cfg(target_pointer_width = "64")]
        impl FromByAdd for i32 {
            fns! {i8 = u8, i16 = u16; i32}
            fns! {sig u32; i32}
            fns! {as equ i8, i16; i32}
            fns! {equ i32; i32}
            fns! {i64, isize, i128, u64, usize, u128; i32}
        }

        // For adding 32 bit arch.
        #[cfg(target_pointer_width = "32")]
        impl FromByAdd for i64 {
            fns! {i8 = u8, i16 = u16, i32 = u32, isize = usize; i64}
            fns! {sig u64; i64}
            fns! {as equ i8, i16, i32, isize; i64}
            fns! {equ i64; i64}
            fns! {i128, u128; i64}
        }

        // For adding 64 bit arch.
        #[cfg(target_pointer_width = "64")]
        impl FromByAdd for i64 {
            fns! {i8 = u8, i16 = u16, i32 = u32; i64}
            fns! {sig u64, usize; i64}
            fns! {as equ i8, i16, i32, isize; i64}
            fns! {equ i64; i64}
            fns! {i128, u128; i64}
        }

        // For adding 32 bit arch.
        #[cfg(target_pointer_width = "32")]
        impl FromByAdd for isize {
            fns! {i8 = u8, i16 = u16, i32 = u32; isize}
            fns! {sig usize; isize}
            fns! {as equ i8, i16, i32; isize}
            fns! {equ isize; isize}
            fns! {i64, i128, u64, u128; isize}
        }

        // For adding 64 bit arch.
        #[cfg(target_pointer_width = "64")]
        impl FromByAdd for isize {
            fns! {i8 = u8, i16 = u16, i32 = u32; isize}
            fns! {sig u64, usize; isize}
            fns! {as equ i8, i16, i32, i64; isize}
            fns! {equ isize; isize}
            fns! {i128, u128; isize}
        }

        impl FromByAdd for i128 {
            fns! {i8 = u8, i16 = u16, i32 = u32, i64 = u64, isize = usize; i128}
            fns! {sig u128; i128}
            fns! {as equ i8, i16, i32, i64, isize; i128}
            fns! {equ i128; i128}
        }

        impl FromByAdd for u8 {
            fns! {equ u8; u8}
            fns! {128, i8; u8}
            fns! {i16, i32, i64, isize, i128, u16, u32, u64, usize, u128; u8}
        }

        impl FromByAdd for u16 {
            fns! {128, i8, 32_768, i16; u16}
            fns! {as equ u8; u16}
            fns! {equ u16; u16}
            fns! {i32, i64, isize, i128, u32, u64, usize, u128; u16}
        }

        // For adding 32 bit arch.
        #[cfg(target_pointer_width = "32")]
        impl FromByAdd for u32 {
            fns! {128, i8, 32_768, i16, $value_i32, i32, $value_isize, isize; u32}
            fns! {as equ u8, u16, usize; u32}
            fns! {equ u32; u32}
            fns! {i64, i128, u64, u128; u32}
        }

        // For adding 64 bit arch.
        #[cfg(target_pointer_width = "64")]
        impl FromByAdd for u32 {
            fns! {128, i8, 32_768, i16, $value_i32, i32; u32}
            fns! {as equ u8, u16; u32}
            fns! {equ u32; u32}
            fns! {i64, isize, i128, u64, usize, u128; u32}
        }

        impl FromByAdd for u64 {
            fns! {128, i8, 32_768, i16, $value_i32, i32, $value_i64, i64, $value_isize, isize; u64}
            fns! {as equ u8, u16, u32, usize; u64}
            fns! {equ u64; u64}
            fns! {i128, u128; u64}
        }

        // For adding 32 bit arch.
        #[cfg(target_pointer_width = "32")]
        impl FromByAdd for usize {
            fns! {128, i8, 32_768, i16, $value_i32, i32, $value_isize, isize; usize}
            fns! {as equ u8, u16, u32; usize}
            fns! {equ usize; usize}
            fns! {i64, i128, u64, u128; usize}
        }

        // For adding 64 bit arch.
        #[cfg(target_pointer_width = "64")]
        impl FromByAdd for usize {
            fns! {128, i8, 32_768, i16, $value_i32, i32, $value_i64, i64, $value_isize, isize; usize}
            fns! {as equ u8, u16, u32, u64; usize}
            fns! {equ usize; usize}
            fns! {i128, u128; usize}
        }

        impl FromByAdd for u128 {
            fns! {128, i8, 32_768, i16, $value_i32, i32, $value_i64, i64, $value_isize, isize; u128}
            fns! {as equ u8, u16, u32, u64, usize; u128}
            fns! {equ u128; u128}
            fns! {170_141_183_460_469_231_731_687_303_715_884_105_728, i128; u128}
        }
    }
}

// For adding 32 bit arch.
#[cfg(target_pointer_width = "32")]
from_impls!(2_147_483_648; 9_223_372_036_854_775_808; 2_147_483_648);

// For adding 64 bit arch.
#[cfg(target_pointer_width = "64")]
from_impls!(2_147_483_648; 9_223_372_036_854_775_808; 9_223_372_036_854_775_808);
