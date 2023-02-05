use paste::paste;

/// The CastFromAs trait for simple convert from value between integer types with possible overflow.
///
/// # Examples
/// Usage:
///
/// ```
/// # use num_convert::CastFromAs;
/// assert_eq!(<i8>::from_u8(127u8), 127i8);
/// assert_eq!(<i8>::from_u8(255u8), -1i8);
/// ```
pub trait CastFromAs {
    /// Converts the value from i8 to Self. Overflow possible during conversion.
    fn from_i8(n: i8) -> Self;

    /// Converts the value from i16 to Self. Overflow possible during conversion.
    fn from_i16(n: i16) -> Self;

    /// Converts the value from i32 to Self. Overflow possible during conversion.
    fn from_i32(n: i32) -> Self;

    /// Converts the value from i64 to Self. Overflow possible during conversion.
    fn from_i64(n: i64) -> Self;

    /// Converts the value from isize to Self. Overflow possible during conversion.
    fn from_isize(n: isize) -> Self;

    /// Converts the value from i128 to Self. Overflow possible during conversion.
    fn from_i128(n: i128) -> Self;

    /// Converts the value from u8 to Self. Overflow possible during conversion.
    fn from_u8(n: u8) -> Self;

    /// Converts the value from u16 to Self. Overflow possible during conversion.
    fn from_u16(n: u16) -> Self;

    /// Converts the value from u32 to Self. Overflow possible during conversion.
    fn from_u32(n: u32) -> Self;

    /// Converts the value from u64 to Self. Overflow possible during conversion.
    fn from_u64(n: u64) -> Self;

    /// Converts the value from usize to Self. Overflow possible during conversion.
    fn from_usize(n: usize) -> Self;

    /// Converts the value from u128 to Self. Overflow possible during conversion.
    fn from_u128(n: u128) -> Self;
}

macro_rules! cast_from_as_impls {
    ($($from_type:ty),+; $type:ty ) => {
        impl CastFromAs for $type {
            paste!{
                #[inline]
                fn [<from_$type>](n: $type) -> Self {
                    n
                }
            }

            $(
            paste!{
                #[inline]
                fn [<from_$from_type>](n: $from_type) -> Self {
                    n as Self
                }
            }
            )*
        }
    }
}

cast_from_as_impls! { i16, i32, i64, isize, i128, u8, u16, u32, u64, usize, u128; i8 }
cast_from_as_impls! { i8, i32, i64, isize, i128, u8, u16, u32, u64, usize, u128; i16 }
cast_from_as_impls! { i8, i16, i64, isize, i128, u8, u16, u32, u64, usize, u128; i32 }
cast_from_as_impls! { i8, i16, i32, isize, i128, u8, u16, u32, u64, usize, u128; i64 }
cast_from_as_impls! { i8, i16, i32, i64, i128, u8, u16, u32, u64, usize, u128; isize }
cast_from_as_impls! { i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, u128; i128 }
cast_from_as_impls! { i8, i16, i32, i64, isize, i128, u16, u32, u64, usize, u128; u8 }
cast_from_as_impls! { i8, i16, i32, i64, isize, i128, u8, u32, u64, usize, u128; u16 }
cast_from_as_impls! { i8, i16, i32, i64, isize, i128, u8, u16, u64, usize, u128; u32 }
cast_from_as_impls! { i8, i16, i32, i64, isize, i128, u8, u16, u32, usize, u128; u64 }
cast_from_as_impls! { i8, i16, i32, i64, isize, i128, u8, u16, u32, u64, u128; usize }
cast_from_as_impls! { i8, i16, i32, i64, isize, i128, u8, u16, u32, u64, usize; u128 }
