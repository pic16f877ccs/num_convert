use crate::IntoAs;
use core::ops::Div;

/// A trait IntegerLen to determine the number of digits of integers.
pub trait IntegerLen {
    /// Returns the number of digits to self.
    fn len(self) -> usize;
}

impl<T> IntegerLen for T
where
    T: Eq + Copy + Div<Output = T> + IntoAs<T>,
{
    #[inline]
    fn len(mut self) -> usize {
        let mut count = 0;
        let ten = 10.into_as();
        let zero = <T>::from_u8(0);
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

trait IsType {
    fn is_type() -> &'static str;
}
macro_rules! is_type_impls {
    ($($type:ty, $type_str:expr);* ) => {
        $(
            impl IsType for $type {
                #[inline]
                fn is_type() -> &'static str {
                    $type_str
                }
            }
        )*
    }
}

is_type_impls! { i8, "i8"; i16, "i16"; i32, "i32"; i64, "i64"; isize, "isize"; i128, "i128" }
is_type_impls! { u8, "u8"; u16, "u16"; u32, "u32"; u64, "u64"; usize, "usize"; u128, "u128" }

trait IsValueType {
    fn is_value_type(self) -> &'static str;
}

macro_rules! is_val_type_impls {
    ($($type:ty, $type_str:expr);* ) => {
        $(
            impl IsValueType for $type {
                #[inline]
                fn is_value_type(self) -> &'static str {
                    $type_str
                }
            }
        )*
    }
}

is_val_type_impls! { i8, "i8"; i16, "i16"; i32, "i32"; i64, "i64"; isize, "isize"; i128, "i128" }
is_val_type_impls! { u8, "u8"; u16, "u16"; u32, "u32"; u64, "u64"; usize, "usize"; u128, "u128" }
