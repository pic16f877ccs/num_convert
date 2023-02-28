extern crate try_from_tup_macro;
use core::fmt;
use crate::{TryFromIntStr, TryFromIntStrErr, IntStrError};
use try_from_tup_macro::{tup_from_trait, tup_from_impl};
use core::fmt::Display;

#[doc = "Conversion errors."]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TryFromTupErr {
    source: TryFromIntStrErr,
    posice: usize,
}

impl Display for TryFromTupErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.source.int_str_error {
            IntStrError::ErrorStr(parse_int_error) => {
                write!(f, "{parse_int_error}")
            }
            IntStrError::ErrorInt(try_from_int_error) => {
                write!(f, "{try_from_int_error}")
            }
        }
    }
}

impl From<TryFromIntStrErr> for TryFromTupErr {
    fn from(err: TryFromIntStrErr) -> Self {
        Self { source: err, posice: 0 }
    }
}

/// Trait to convert tuple to array, possible conversion error.
///
/// Usage:
///
/// ```
/// # use num_convert::TryFromTup;
/// assert_eq!(<i32>::try_from_3((45u8, 2023u16, -60i8,)), Ok([45i32, 2023i32, -60i32]));
/// assert_eq!(<i32>::try_from_3((45u8, 2023u16, -53i8,)).unwrap().iter().sum::<i32>(), 2015i32);
/// ```
pub trait TryFromTup {
    tup_from_trait!();
}

macro_rules! tuple_from_impls {
    ( $($for_type:ty),+ ) => {
        $(
            impl TryFromTup for $for_type {
                tup_from_impl!();
            }
        )+
    }
}

tuple_from_impls! { i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, i128, u128, bool }

