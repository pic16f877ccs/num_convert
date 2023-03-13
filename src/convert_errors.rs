use core::fmt;
use core::fmt::Display;
use core::num::ParseIntError;
use core::num::TryFromIntError;
#[cfg(feature = "try_from_int_str")]
use crate::TryFromIntStrErr;
#[cfg(any(feature = "try_tup_to_arr8", feature = "try_tup_to_arr16"))]
use crate::TryTupToArrErr;

/// Enumeration variants for the crate.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MultiErrors{
    /// Variants for the trait [TryTupToArr](trait.TryTupToArr.html).
    #[cfg(any(feature = "try_tup_to_arr8", feature = "try_tup_to_arr16"))]
    TryTupToArrError(TryTupToArrErr),
    /// Variants for the trait [TryFromIntStr](trait.TryFromIntStr.html).
    #[cfg(feature = "try_from_int_str")]
    TryFromIntStrError(TryFromIntStrErr),
    /// Variants for the core trait [`TryFrom`].
    TryFromIntErr(TryFromIntError),
    /// Variants for the method [`parse`](https://doc.rust-lang.org/nightly/core/primitive.str.html#method.parse).
    ParseIntErr(ParseIntError),
    /// Variants for the core trait [TryFromByAdd](trait.TryFromByAdd.html).
    TryFromByAddErr,
    /// Variants for the core trait [TryIntoByAdd](trait.TryIntoByAdd.html).
    TryIntoByAddErr,
}

/// Structure for storing miscellaneous errors.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ConvertErrors {
    pub(crate) source: MultiErrors,
}

//impl ConvertErrors {
//    pub fn set_multi_err(err: MultiErrors) -> Self {
//        Self{ source: err }
//    }
//    pub fn get_multi_err(&self) -> &MultiErrors {
//        &self.source
//    }
//}

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
            MultiErrors::TryFromByAddErr => {
                write!(f, "an attempt to convert an integral type outside the valid range")
            }
            MultiErrors::TryIntoByAddErr => {
                write!(f, "an attempt to convert an integral type outside the valid range")
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

impl From<MultiErrors> for ConvertErrors {
    fn from(err: MultiErrors) -> Self {
            Self { source: err, }
    }
}

