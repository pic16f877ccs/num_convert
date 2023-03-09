use core::fmt;
use core::fmt::Display;
use core::num::ParseIntError;
use core::num::TryFromIntError;

#[cfg(feature = "try_from_int_str")]
use crate::TryFromIntStrErr;
#[cfg(any(feature = "try_tup_to_arr8", feature = "try_tup_to_arr16"))]
use crate::TryTupToArrErr;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MultiErrors{
    #[cfg(any(feature = "try_tup_to_arr8", feature = "try_tup_to_arr16"))]
    TryTupToArrError(TryTupToArrErr),
    #[cfg(feature = "try_from_int_str")]
    TryFromIntStrError(TryFromIntStrErr),
    TryFromIntErr(TryFromIntError),
    ParseIntErr(ParseIntError),
}

/// Structure for storing miscellaneous errors.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ConvertErrors {
    pub(crate) source: MultiErrors,
}

impl Display for ConvertErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.source {
            #[cfg(any(feature = "try_tup_to_arr8", feature = "try_tup_to_arr16"))]
            MultiErrors::TryTupToArrError(try_tup_to_arr_err) => {
                write!(f, "{try_tup_to_arr_err}")
            }
            #[cfg(feature = "try_from_int_str")]
            MultiErrors::TryFromIntStrError(try_from_int_str_err) => {
                write!(f, "{try_from_int_str_err}")
            }
            MultiErrors::TryFromIntErr(try_from_int_err) => {
                write!(f, "{try_from_int_err}")
            }
            MultiErrors::ParseIntErr(parse_int_err) => {
                write!(f, "{parse_int_err}")
            }
        }
    }
}

#[cfg(feature = "try_from_int_str")]
impl From<TryFromIntStrErr> for ConvertErrors {
    fn from(err: TryFromIntStrErr) -> Self {
            Self { source: MultiErrors::TryFromIntStrError(err), }
    }
}

#[cfg(any(feature = "try_tup_to_arr8", feature = "try_tup_to_arr16"))]
impl From<TryTupToArrErr> for ConvertErrors {
    fn from(err: TryTupToArrErr) -> Self {
            Self { source: MultiErrors::TryTupToArrError(err), }
    }
}

impl From<TryFromIntError> for ConvertErrors {
    fn from(err: TryFromIntError) -> Self {
            Self { source: MultiErrors::TryFromIntErr(err), }
    }
}

impl From<ParseIntError> for ConvertErrors {
    fn from(err: ParseIntError) -> Self {
            Self { source: MultiErrors::ParseIntErr(err), }
    }
}

