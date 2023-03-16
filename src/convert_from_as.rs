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
pub trait FromAs<T> {
    /// Convert from value between integer types with overflow.
    fn from_as(n: T) -> Self;
}

macro_rules! from_as_impls {
    ( $($type:ty),*; $for_type:ty ) => {
        impl FromAs<$for_type> for $for_type {
            #[doc = concat!("Returns the same type ", stringify!($for_type), ".")]
            #[inline]
            fn from_as(num: $for_type) -> Self {
                num
            }
        }

        $(
            impl FromAs<$type> for $for_type {
                #[doc = concat!("Converts ", stringify!($type), " to ", stringify!($for_type), ".")]
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
