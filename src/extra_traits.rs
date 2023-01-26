use crate::{FromAs, IntoAs};
use core::ops::Div;

/// A trait IntegerLen to determine the number of digits of integers.
pub trait IntegerLen {
    /// Returns the number of digits to self.
    fn len(self) -> usize;
}

impl<T> IntegerLen for T
where
    T: Eq + Copy + Div<Output = T> + IntoAs<T> + FromAs<u8>,
    u8: IntoAs<T>,
{
    #[inline]
    fn len(mut self) -> usize {
        let mut count = 0;
        let ten = 10.into_as();
        let zero = <T>::from_as(0);
        while self != zero {
            self = self / ten;
            count += 1;
        }
        if count == 0 {
            1
        } else {
            count
        }
    }
}

pub trait TypeInfo {
    fn info() -> &'static str;
}

macro_rules! type_info_impls {
    ($($type:ty, $type_str:expr);* ) => {
        $(
            impl TypeInfo for $type {
                #[inline]
                fn info() -> &'static str {
                    $type_str
                }
            }
        )*
    }
}

type_info_impls! { i8, "i8"; i16, "i16"; i32, "i32"; i64, "i64"; isize, "isize"; i128, "i128" }
type_info_impls! { u8, "u8"; u16, "u16"; u32, "u32"; u64, "u64"; usize, "usize"; u128, "u128" }

pub trait ValTypeInfo {
    fn info(self) -> &'static str;
}

macro_rules! val_type_info_impls {
    ($($type:ty, $type_str:expr);* ) => {
        $(
            impl ValTypeInfo for $type {
                #[inline]
                fn info(self) -> &'static str {
                    $type_str
                }
            }
        )*
    }
}

val_type_info_impls! { i8, "i8"; i16, "i16"; i32, "i32"; i64, "i64"; isize, "isize"; i128, "i128" }
val_type_info_impls! { u8, "u8"; u16, "u16"; u32, "u32"; u64, "u64"; usize, "usize"; u128, "u128" }
