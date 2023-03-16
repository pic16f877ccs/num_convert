extern crate from_tup_macro;
use from_tup_macro::{tup_from_impl, tup_from_trait};

/// A trait to convert a tuple of different types to an array of integers.
///
/// - The FromTuple trait is intended for perfect conversions.
///
/// # Usage
/// Basic use of the trait.
///
/// ```
/// use num_convert::FromTuple;
///
/// assert_eq!(<u16 as FromTuple>::from_3((true, false, 255_u8)), [1_u16, 0_u16, 255_u16]);
/// assert_eq!(<i64>::from_2((-19_500_000_i32, 230_u8)), [-19_500_000_i64, 230_i64]);
/// ```
///
/// # Examples
/// ```
/// # use num_convert::FromTuple;
/// assert_eq!(<i32>::from_2((true, 10101_u16)), [1_i32, 10101_i32]);
/// assert_eq!(<i32>::from_3((45_u8, 2023_u16, -53_i8)).iter().sum::<i32>(), 2015_i32);
/// ```
pub trait FromTuple {
    tup_from_trait!();
}

macro_rules! tuple_from_impls {
    ( $($for_type:ty),+ ) => {
        $(
            impl FromTuple for $for_type {
                tup_from_impl!();
            }
        )+
    }
}

tuple_from_impls! { i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, i128, u128, bool }
