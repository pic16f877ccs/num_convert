/// The IntoAs trait for convert into value between integer types with possible overflow.
///
/// # Usage
/// Basic use of the trait.
///
/// ```
/// use num_convert::IntoAs;
///
/// assert_eq!(IntoAs::<u8>::into_as(256_u16), 0_u8);
/// assert_eq!(<u8 as IntoAs<i8>>::into_as(156_u8), -100_i8);
/// ```
///
/// # Examples
///
/// ```
/// # use num_convert::IntoAs;
/// let int: u8 = 612_u16.into_as();
/// assert_eq!(int, 100_u8);
/// assert_eq!(<u8 as IntoAs<i8>>::into_as(127_u8), 127_i8);
/// ```
pub trait IntoAs<T> {
    /// Convert value into between integer types with possible overflow.
    fn into_as(self) -> T;
}

macro_rules! into_as_impls {
    ( $($type:ty),*; $for_type:ty ) => {
        impl IntoAs<$for_type> for $for_type {
            #[doc = concat!("Returns the same type ", stringify!($for_type), ".")]
            #[inline]
            fn into_as(self) -> $for_type {
                self
            }
        }

        $(
            impl IntoAs<$type> for $for_type {
                #[doc = concat!("Converts ", stringify!($for_type), " to ", stringify!($type), ".")]
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
