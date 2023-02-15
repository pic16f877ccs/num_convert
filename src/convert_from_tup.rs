/// Train for conversion tuple to array.
/// The FromTuple trait is intended for perfect conversions.
///
/// Usage:
///
/// ```
/// # use num_convert::FromTuple;
/// assert_eq!(<i32 as FromTuple>::from_3((45u8, 2023u16, -60i8,)), [45i32, 2023i32, -60i32]);
/// assert_eq!(<i32>::from_3((45u8, 2023u16, -53i8,)).iter().sum::<i32>(), 2015i32);
/// ```
///
extern crate from_tup_macro;
use from_tup_macro::{ tup_from_impl, tup_from_trait };

#[doc = "Trait for conversion tuple to array."]
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

tuple_from_impls! { i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, i128, u128 }
