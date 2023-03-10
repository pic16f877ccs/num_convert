use core::fmt;
use core::fmt::Display;
use crate::{IntStrError, TryFromIntStr, TryFromIntStrErr};
extern crate try_tup_to_arr_macro;
use try_tup_to_arr_macro::try_tup_to_arr_impl;

/// A structure to store the error and its position.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TryTupToArrErr {
    pub(crate) source: TryFromIntStrErr,
    posice: usize,
}

impl Display for TryTupToArrErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.source.multi_err() {
            IntStrError::ErrorStr(parse_int_error) => {
                write!(f, "{parse_int_error}")
            }
            IntStrError::ErrorInt(try_from_int_error) => {
                write!(f, "{try_from_int_error}")
            }
        }
    }
}

/// A trait to convert a [`tuple`] to an [`array`] of integers, a possible conversion error.
///
/// Usage:
///
/// ```
/// # use num_convert::TryTupToArr;
/// assert_eq!(TryTupToArr::<i32>::try_into_arr((45u8, 2023u16, -60i8,)),
/// Ok([45i32, 2023i32, -60i32]));
/// assert_eq!(TryTupToArr::<i16>::try_into_arr(("45", 2023u16, true,)),
/// Ok([45i16, 2023i16, 1i16]));
/// assert_eq!(TryTupToArr::try_into_arr((45u8, 2023u16, -53i8,))
/// .unwrap().iter().sum::<i32>(), 2015i32);
///assert!(TryTupToArr::<i16>::try_into_arr(("6032023", 2023u16, true,)).is_err());
/// ```
pub trait TryTupToArr<U> {
    ///Output type [`array`] of integers.
    type A;

    /// Returns an [`array`] of integers or a conversion error.
    fn try_into_arr(self) -> Result<Self::A, TryTupToArrErr>;
}

try_tup_to_arr_impl!();
