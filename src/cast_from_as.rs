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
    ( $into_type:ty; $( $doc:expr, $($from_type:ty),* );* ) => {
        impl CastFromAs for $into_type {
            paste!{
                #[doc = concat!("Converts ", stringify!($into_type), " to ", stringify!($into_type), " losslessly.")]
                #[inline]
                fn [<from_$into_type>](n: $into_type) -> Self {
                    n
                }
            }

            $(
                $(
                    paste!{
                        #[doc = concat!("Converts ", stringify!($from_type), " to ", stringify!($into_type), " ",$doc,".")]
                        #[inline]
                        fn [<from_$from_type>](n: $from_type) -> Self {
                            n as Self
                        }
                    }
                )*
            )*
        }
    }
}

cast_from_as_impls! { i8; "possible overflow", u8, i16, u16, i32, u32, i64, u64, isize, usize, i128, u128 }
cast_from_as_impls! { i16; "losslessly", i8, u8; "possible overflow", u16, i32, u32, i64, u64, isize, usize, i128, u128 }
cast_from_as_impls! { i32; "losslessly", i8, u8, i16, u16; "possible overflow", u32, i64, u64, isize, usize, i128, u128 }
cast_from_as_impls! { i64; "losslessly", i8, u8, i16, u16, i32, u32, isize; "possible overflow", u64, usize, i128, u128 }
cast_from_as_impls! { isize; "losslessly", i8, u8, i16, u16, i32, u32, i64; "possibe overflow", u64, usize, i128, u128 }
cast_from_as_impls! { i128; "losslessly", i8, u8, u16, i16, u32, i32, u64, i64, usize, isize; "possibly overflow", u128 }

cast_from_as_impls! { u8; "possible overflow", i8, u16, i16, i32, u32, i64, u64, isize, usize, i128, u128 }
cast_from_as_impls! { u16; "losslessly", u8; "possible overflow", i8, i16, i32, u32, i64, u64, isize, usize, i128, u128 }
cast_from_as_impls! { u32; "losslessly", u8, u16; "possible overflow", i8, i16, i32, i64, u64, isize, usize, i128, u128 }
cast_from_as_impls! { u64; "losslessly", u8, u16, u32, usize; "possible overflow", i8, i16, i32, i64, isize, i128, u128 }
cast_from_as_impls! { usize; "losslessly", u8, u16, u32, u64; "possible overflow", i8, i16, i32, i64, isize, i128, u128 }
cast_from_as_impls! { u128; "losslessly", u8, u16, u32, u64, usize; "possible overflow", i8, i16, i32, i64, isize, i128 }
