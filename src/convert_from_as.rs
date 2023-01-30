use paste::paste;

/// The FromAs generic trait for convert from value between integer types with possible overflow.
///
/// # Examples
/// Usage:
///
/// ```
/// # use num_convert::FromAs;
/// assert_eq!(<u8>::from_as(258u16), 2u8);
/// assert_eq!(<u8 as FromAs<u16>>::from_as(255u16), 255u8);
/// ```
pub trait FromAs<T>: CastInto {
    /// Convert from value between integer types with overflow.
    fn from_as(n: T) -> Self;
}

macro_rules! from_as_impls {
    ( $($type:ty),*; $for_type:ty ) => {
        impl FromAs<$for_type> for $for_type {
            #[inline]
            fn from_as(num: $for_type) -> Self {
                num
            }
        }

        $(
            impl FromAs<$type> for $for_type {
                #[inline]
                fn from_as(num: $type) -> Self {
                    num as Self
                }
            }
        )*
    }
}

from_as_impls! { u8, i16, u16, i32, u32, i64, u64, isize, usize, i128, u128; i8 }
from_as_impls! { i8, i16, u16, i32, u32, i64, u64, isize, usize, i128, u128; u8 }
from_as_impls! { i8, u8, u16, i32, u32, i64, u64, isize, usize, i128, u128; i16 }
from_as_impls! { i8, u8, i16, i32, u32, i64, u64, isize, usize, i128, u128; u16 }
from_as_impls! { i8, u8, i16, u16, u32, i64, u64, isize, usize, i128, u128; i32 }
from_as_impls! { i8, u8, i16, u16, i32, i64, u64, isize, usize, i128, u128; u32 }
from_as_impls! { i8, u8, i16, u16, i32, u32, u64, isize, usize, i128, u128; i64 }
from_as_impls! { i8, u8, i16, u16, i32, u32, i64, isize, usize, i128, u128; u64 }
from_as_impls! { i8, u8, i16, u16, i32, u32, i64, u64, usize, i128, u128; isize }
from_as_impls! { i8, u8, i16, u16, i32, u32, i64, u64, isize, i128, u128; usize }
from_as_impls! { i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, u128; i128 }
from_as_impls! { i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, i128; u128 }

/// The CastInto trait for simple convert self value between integer types with possible overflow.
///
/// # Examples
/// Usage:
///
/// ```
/// # use num_convert::CastInto;
/// assert_eq!(255u16.into_u8(), 255u8);
/// assert_eq!(258u16.into_u8(), 2u8);
/// ```
pub trait CastInto {
    /// Converts the value of self to i8. Overflow possible during conversion.
    fn into_i8(self) -> i8;

    /// Converts the value of self to i16. Overflow possible during conversion.
    fn into_i16(self) -> i16;

    /// Converts the value of self to i32. Overflow possible during conversion.
    fn into_i32(self) -> i32;

    /// Converts the value of self to i64. Overflow possible during conversion.
    fn into_i64(self) -> i64;

    /// Converts the value of self to isize. Overflow possible during conversion.
    fn into_isize(self) -> isize;

    /// Converts the value of self to i128. Overflow possible during conversion.
    fn into_i128(self) -> i128;

    /// Converts the value of self to u8. Overflow possible during conversion.
    fn into_u8(self) -> u8;

    /// Converts the value of self to u16. Overflow possible during conversion.
    fn into_u16(self) -> u16;

    /// Converts the value of self to u32. Overflow possible during conversion.
    fn into_u32(self) -> u32;

    /// Converts the value of self to u64. Overflow possible during conversion.
    fn into_u64(self) -> u64;

    /// Converts the value of self to usize. Overflow possible during conversion.
    fn into_usize(self) -> usize;

    /// Converts the value of self to u128.
    fn into_u128(self) -> u128;
}

macro_rules! cast_into_impls {
    ($($from_type:ty),+; $type:ty ) => {
        impl CastInto for $type {
            paste!{
                #[inline]
                fn [<into_$type>](self) -> $type {
                    self
                }
            }
            $(
                paste!{
                    #[inline]
                    fn [<into_$from_type>](self) -> $from_type {
                        self as $from_type
                    }
                }
            )*
        }
    }
}

cast_into_impls! { i16, i32, i64, isize, i128, u8, u16, u32, u64, usize, u128; i8 }
cast_into_impls! { i8, i32, i64, isize, i128, u8, u16, u32, u64, usize, u128; i16 }
cast_into_impls! { i8, i16, i64, isize, i128, u8, u16, u32, u64, usize, u128; i32 }
cast_into_impls! { i8, i16, i32, isize, i128, u8, u16, u32, u64, usize, u128; i64 }
cast_into_impls! { i8, i16, i32, i64, i128, u8, u16, u32, u64, usize, u128; isize }
cast_into_impls! { i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, u128; i128 }
cast_into_impls! { i8, i16, i32, i64, isize, i128, u16, u32, u64, usize, u128; u8 }
cast_into_impls! { i8, i16, i32, i64, isize, i128, u8, u32, u64, usize, u128; u16 }
cast_into_impls! { i8, i16, i32, i64, isize, i128, u8, u16, u64, usize, u128; u32 }
cast_into_impls! { i8, i16, i32, i64, isize, i128, u8, u16, u32, usize, u128; u64 }
cast_into_impls! { i8, i16, i32, i64, isize, i128, u8, u16, u32, u64, u128; usize }
cast_into_impls! { i8, i16, i32, i64, isize, i128, u8, u16, u32, u64, usize; u128 }
