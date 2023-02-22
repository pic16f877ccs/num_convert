/// The IntoAs trait for convert into value between integer types with possible overflow.
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
pub trait IntoAs<T> {
    /// Convert value into between integer types with possible overflow.
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
