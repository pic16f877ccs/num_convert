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
                write!(f, "{parse_int_error}, position {pos}", pos = self.posice)
            }
            IntStrError::ErrorInt(try_from_int_error) => {
                write!(f, "{try_from_int_error}, position {pos}", pos = self.posice)
            }
        }
    }
}

/// A trait to convert a [`tuple`] of different types to an [`array`] of integers, handling possible conversion errors.
/// - Without using the heap.
/// - Returns an error if the value is out of range.
///
/// # Usage
/// Basic use of the trait.
///
/// ```
/// use num_convert::TryTupToArr;
///
/// assert_eq!(TryTupToArr::<i32>::try_into_arr((45_u8, 2023_u16, -60_i8,)),
/// Ok([45_i32, 2023_i32, -60_i32]));
///
/// let arr: [i16; 3] = ("45", 2023_u16, true,).try_into_arr().unwrap();
/// assert_eq!(arr, [45_i16, 2023_i16, 1_i16]);
/// ```
///
/// # Examples
///
/// ```
/// # use num_convert::TryTupToArr;
/// assert_eq!(TryTupToArr::<i16>::try_into_arr(("-12345", 9923_u16, false,)),
/// Ok([-12345_i16, 9923_i16, 0_i16]));
///
/// assert_eq!(TryTupToArr::try_into_arr((45_u8, 2023_u16, -53_i8,))
/// .unwrap().iter().sum::<i32>(), 2015i32);
///
/// assert!(TryTupToArr::<i16>::try_into_arr(("6032023", 2023_u16, true,)).is_err());
/// ```
pub trait TryTupToArr<U> {
    /// Output type [`array`] of integers.
    type A;

    /// Returns an [`array`] of integers or a conversion error.
    fn try_into_arr(self) -> Result<Self::A, TryTupToArrErr>;
}

try_tup_to_arr_impl!();
