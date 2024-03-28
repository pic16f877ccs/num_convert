use crate::{FromAs, IntoAs};
use core::ops::Rem;
use core::ops::Div;

/// A trait IntegerLen to determine the number of digits of integers.
///
/// # Usage
/// Basic use of the trait.
///
/// ```
/// use num_convert::IntegerLen;
///
/// assert_eq!(i8::MAX.len(), 3usize);
/// assert_eq!(u128::MAX.len(), 39usize);
/// ```
///
/// # Examples
/// ```
/// # use num_convert::IntegerLen;
/// assert_eq!(0_i8.len(), 1_usize);
/// assert_eq!(i128::MAX.len(), 39_usize);
/// ```
#[allow(clippy::len_without_is_empty)]
pub trait IntegerLen {
    /// Returns the number of digits for self value.
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

/// Trait for type info.
pub trait TypeInfo {
    /// Returns the type.
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

/// Trait value type info.
pub trait ValTypeInfo {
    /// Returns value type info.
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

impl<T> CheckRem for T
where
    T: Copy + Default + Rem<Output = T> + PartialEq,
{ }

/// Trait for checking the remainder.
pub trait CheckRem: Rem<Output = Self> + PartialEq
where
    Self: Copy + Default,
{
    /// Returns true if there is no remainder.
    ///
    /// # Arguments
    ///
    /// * `n` - The divisor with which the remainder is checked.
    ///
    /// # Examples
    ///
    /// ```
    /// use num_convert::CheckRem;
    ///
    /// assert_eq!(10.no_rem(2), true);
    /// assert_eq!(10.no_rem(3), false);
    /// ```
    #[inline]
    fn no_rem(&self, n: Self) -> bool {
        *self % n == Self::default()
    }

    /// Returns true if there is a remainder.
    ///
    /// # Arguments
    ///
    /// * `n` - The divisor with which the remainder is checked.
    ///
    /// # Examples
    ///
    /// ```
    /// use num_convert::CheckRem;
    ///
    /// assert_eq!(10.is_rem(2), false);
    /// assert_eq!(10.is_rem(3), true);
    /// ```
    #[inline]
    fn is_rem(&self, n: Self) -> bool {
        *self % n != Self::default()
    }
}
