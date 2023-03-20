#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![no_std]
#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]
#![allow(clippy::manual_range_contains)]
//! # Generic traits for conversions between integer types.
//! This library provides various ways to convert to integer type from other types.
//!
//! # Examples
//!
//! ```
//! # use num_convert::*;
//! assert_eq!(<u8 as FromByAdd<i8>>::from_by_add(-128_i8), 0_u8);
//! assert_eq!(<i16 as IntoByAdd<u16>>::into_by_add(i16::MIN), u16::MIN);
//!
//! assert_eq!(<i8 as TryFromByAdd<u8>>::try_from_by_add(0_u8), Some(-128_i8));
//! assert_eq!(<i128 as TryIntoByAdd<u128>>::try_into_by_add(i128::MIN), Some(u128::MIN));
//!
//! assert_eq!(<u8 as FromAs<i8>>::from_as(-128_i8), 128_u8);
//! assert_eq!(<u16 as IntoAs<u8>>::into_as(258_u16), 2_u8);
//!
//! assert_eq!(<u8 as TryFromDigits<u16>>::from_digits(65_255_u16), Ok(255_u8));
//!
//! assert_eq!(u64::MAX.sbits(), 64_u32);
//! assert_eq!(i8::tbits(), 8_u32);
//!
//! assert_eq!(0i8.len(), 1_usize);
//! assert_eq!(u128::MAX.len(), 39_usize);
//!
//! assert_eq!(<i32 as FromTuple>::from_5((true, false, 45_u8, 2023_u16, -60_i8,)),
//! [1_i32, 0_i32, 45_i32, 2023_i32, -60_i32]);
//!
//! assert_eq!(<i16 as TryFromIntStr<&str>>::try_from_int_str("-2023"), Ok(-2023_i16));
//! assert_eq!(<u16 as TryFromIntStr<u128>>::try_from_int_str(1975_u128), Ok(1975_u16));
//! ```

#[doc = include_str!("../README.md")]
mod convert_from_as;
mod convert_from_by_add;
mod convert_into_as;
mod convert_into_by_add;
mod convert_try_from_by_add;
mod convert_try_from_digits;
mod convert_try_into_by_add;
mod extra_traits;

#[cfg(any(
    feature = "try_tup_to_arr8",
    feature = "try_tup_to_arr16",
    feature = "try_from_int_str"
))]
mod convert_errors;

#[cfg(feature = "try_from_int_str")]
mod convert_try_from_int_str;

#[cfg(feature = "cast_from_as")]
mod cast_from_as;

#[cfg(feature = "cast_into_as")]
mod cast_into_as;

#[cfg(feature = "bits")]
mod size_type_bits;

#[cfg(any(feature = "tup8", feature = "tup16"))]
mod convert_from_tup;

#[cfg(any(feature = "try_tup_to_arr8", feature = "try_tup_to_arr16"))]
mod convert_try_tup_to_arr;

#[cfg(any(feature = "to_min", feature = "to_max", feature = "to_zero"))]
mod to_min_to_zero_to_max;

pub use crate::convert_from_as::FromAs;
pub use crate::convert_from_by_add::FromByAdd;
pub use crate::convert_into_as::IntoAs;
pub use crate::convert_into_by_add::IntoByAdd;
pub use crate::convert_try_from_by_add::TryFromByAdd;
pub use crate::convert_try_from_digits::TryFromDigits;
pub use crate::convert_try_into_by_add::TryIntoByAdd;
pub use crate::extra_traits::IntegerLen;

#[cfg_attr(docsrs, doc(cfg(feature = "try_from_int_str")))]
#[cfg_attr(docsrs, doc(cfg(feature = "try_tup_to_arr8")))]
#[cfg_attr(docsrs, doc(cfg(feature = "try_tup_to_arr16")))]
#[cfg(any(
    feature = "try_tup_to_arr8",
    feature = "try_tup_to_arr16",
    feature = "try_from_int_str"
))]
pub use crate::convert_errors::{ConvertErrors, MultiErrors};

#[cfg(feature = "try_from_int_str")]
#[cfg_attr(docsrs, doc(cfg(feature = "try_from_int_str")))]
pub use crate::convert_try_from_int_str::{IntStrError, TryFromIntStr, TryFromIntStrErr};

#[cfg(any(feature = "tup8", feature = "tup16"))]
#[cfg_attr(docsrs, doc(cfg(feature = "tup8")))]
#[cfg_attr(docsrs, doc(cfg(feature = "tup16")))]
pub use crate::convert_from_tup::FromTuple;

#[cfg(any(feature = "try_tup_to_arr8", feature = "try_tup_to_arr16"))]
#[cfg_attr(docsrs, doc(cfg(feature = "try_tup_to_arr8")))]
#[cfg_attr(docsrs, doc(cfg(feature = "try_tup_to_arr16")))]
pub use crate::convert_try_tup_to_arr::{TryTupToArr, TryTupToArrErr};

#[cfg(feature = "to_min")]
#[cfg_attr(docsrs, doc(cfg(feature = "to_min")))]
pub use crate::to_min_to_zero_to_max::ToMin;

#[cfg(feature = "to_max")]
#[cfg_attr(docsrs, doc(cfg(feature = "to_max")))]
pub use crate::to_min_to_zero_to_max::ToMax;

#[cfg(feature = "to_zero")]
#[cfg_attr(docsrs, doc(cfg(feature = "to_zero")))]
pub use crate::to_min_to_zero_to_max::ToZero;

#[cfg(feature = "type_info")]
#[cfg_attr(docsrs, doc(cfg(feature = "type_info")))]
pub use crate::extra_traits::TypeInfo;

#[cfg(feature = "val_type_info")]
#[cfg_attr(docsrs, doc(cfg(feature = "val_type_info")))]
pub use crate::extra_traits::ValTypeInfo;

#[cfg(feature = "cast_from_as")]
#[cfg_attr(docsrs, doc(cfg(feature = "cast_from_as")))]
pub use crate::cast_from_as::CastFromAs;

#[cfg(feature = "cast_into_as")]
#[cfg_attr(docsrs, doc(cfg(feature = "cast_into_as")))]
pub use crate::cast_into_as::CastIntoAs;

#[cfg(feature = "bits")]
#[cfg_attr(docsrs, doc(cfg(feature = "bits")))]
pub use crate::size_type_bits::Sbits;

#[cfg(feature = "bits")]
#[cfg_attr(docsrs, doc(cfg(feature = "bits")))]
pub use crate::size_type_bits::Tbits;
