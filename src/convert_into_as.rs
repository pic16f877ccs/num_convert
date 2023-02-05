use paste::paste;

/// The IntoAs trait for convert into value between integer types with overflow.
///
/// Usage:
///
/// ```
/// # use num_convert::IntoAs;
/// # use std::ops::Div;
/// fn primitive_type_len<T>(mut num: T) -> usize
/// where
///     T: Eq + Copy + Div<Output = T> + IntoAs<T>,
///     u8: IntoAs<T>,
/// {
///     let mut count = 0;
///     // There will never be a conversion error here.
///     let ten = 10u8.into_as();
///     // There will never be a conversion error here.
///     let zero = 0u8.into_as();
///
///     while num != zero {
///         num = num / ten;
///         count += 1;
///     }
///
///     if count == 0 {
///         1
///     } else {
///         count
///     }
/// }
/// ```
pub trait IntoAs<T>: CastFrom {
    /// Convert value into between integer types with overflow.
    fn into_as(self) -> T;
}

macro_rules! into_as_impls {
    ( $($type:ty),*; $for_type:ty ) => {
        impl IntoAs<$for_type> for $for_type {
            #[inline]
            fn into_as(self) -> $for_type {
                self
            }
        }

        $(
            impl IntoAs<$type> for $for_type {
                #[inline]
                fn into_as(self) -> $type {
                    self as $type
                }
            }
        )*
    }
}

into_as_impls! { u8, i16, u16, i32, u32, i64, u64, isize, usize, i128, u128; i8 }
into_as_impls! { i8, i16, u16, i32, u32, i64, u64, isize, usize, i128, u128; u8 }
into_as_impls! { i8, u8, u16, i32, u32, i64, u64, isize, usize, i128, u128; i16 }
into_as_impls! { i8, u8, i16, i32, u32, i64, u64, isize, usize, i128, u128; u16 }
into_as_impls! { i8, u8, i16, u16, u32, i64, u64, isize, usize, i128, u128; i32 }
into_as_impls! { i8, u8, i16, u16, i32, i64, u64, isize, usize, i128, u128; u32 }
into_as_impls! { i8, u8, i16, u16, i32, u32, u64, isize, usize, i128, u128; i64 }
into_as_impls! { i8, u8, i16, u16, i32, u32, i64, isize, usize, i128, u128; u64 }
into_as_impls! { i8, u8, i16, u16, i32, u32, i64, u64, usize, i128, u128; isize }
into_as_impls! { i8, u8, i16, u16, i32, u32, i64, u64, isize, i128, u128; usize }
into_as_impls! { i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, u128; i128 }
into_as_impls! { i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, i128; u128 }

/// The CastFrom trait for simple convert from value between integer types with possible overflow.
///
/// # Examples
/// Usage:
///
/// ```
/// # use num_convert::CastFrom;
/// assert_eq!(<i8>::from_u8(127u8), 127i8);
/// assert_eq!(<i8>::from_u8(255u8), -1i8);
/// ```
pub trait CastFrom {
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

macro_rules! cast_from_impl {
    ($($from_type:ty),+; $type:ty ) => {
        impl CastFrom for $type {
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

cast_from_impl! { i16, i32, i64, isize, i128, u8, u16, u32, u64, usize, u128; i8 }
cast_from_impl! { i8, i32, i64, isize, i128, u8, u16, u32, u64, usize, u128; i16 }
cast_from_impl! { i8, i16, i64, isize, i128, u8, u16, u32, u64, usize, u128; i32 }
cast_from_impl! { i8, i16, i32, isize, i128, u8, u16, u32, u64, usize, u128; i64 }
cast_from_impl! { i8, i16, i32, i64, i128, u8, u16, u32, u64, usize, u128; isize }
cast_from_impl! { i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, u128; i128 }
cast_from_impl! { i8, i16, i32, i64, isize, i128, u16, u32, u64, usize, u128; u8 }
cast_from_impl! { i8, i16, i32, i64, isize, i128, u8, u32, u64, usize, u128; u16 }
cast_from_impl! { i8, i16, i32, i64, isize, i128, u8, u16, u64, usize, u128; u32 }
cast_from_impl! { i8, i16, i32, i64, isize, i128, u8, u16, u32, usize, u128; u64 }
cast_from_impl! { i8, i16, i32, i64, isize, i128, u8, u16, u32, u64, u128; usize }
cast_from_impl! { i8, i16, i32, i64, isize, i128, u8, u16, u32, u64, usize; u128 }
