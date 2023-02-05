#![forbid(unsafe_code)]
#![no_std]
#![warn(missing_docs)]
//! # Generic traits for conversions between integer types.
//!
//! - The FromByAdd trait for converting from negative integers to positive or vice versa.
//! - The IntoByAdd trait for converting into negative integers to positive or vice versa.
//! - The TryFromByAdd trait for converting from negative integers to positive or vice versa, that can fail.
//! - The TryIntoByAdd trait for converting into negative integers to positive or vice versa, that can fail.
//! - The FromAs generic trait for conversion from integers with possible overflow.
//! - The IntoAs generic trait for conversion into integers with possible overflow.
//! - The TryFromDigits trait for converting from digits as a number, with possible value types.
//!
//! # Other traits for integers.
//!
//! - A trait IntegerLen to determine the number of digits of integers.
//! - The Sbits trait for define the size of integer value in bits.
//! - The Tbits trait for define the size of integer type in bits.
//!
//! # Examples
//! Usage:
//!
//! Convert from negative into positive and positive into negative.
//! ```
//! # use num_convert::FromByAdd;
//! assert_eq!(<u8>::from_by_add(-128i8), 0u8);
//! assert_eq!(<i8>::from_by_add(255u8), 127i8);
//! ```
//!
//! Usage:
//!
//! Convert into between integer types.
//! ```
//! # use num_convert::IntoByAdd;
//! fn integer_to_integer<T1: IntoByAdd<U1>, T2: IntoByAdd<U2>, U1, U2>(min: T1, max: T2) -> (U1, U2) {
//!     (min.into_by_add(), max.into_by_add())
//! }
//! assert_eq!(integer_to_integer(i16::MIN, u16::MAX), (u16::MIN, i16::MAX));
//! assert_eq!(integer_to_integer(u16::MIN, u16::MAX), (i16::MIN, i16::MAX));
//! ```
//!
//! Usage:
//!
//! Conversions type U16 in U8 without overflow and with overflow.
//! ```
//! # use num_convert::{IntoAs, FromAs};
//! assert_eq!(<u16 as IntoAs<u8>>::into_as(255u16), 255u8);
//! assert_eq!(<u16 as IntoAs<u8>>::into_as(258u16), 2u8);
//! assert_eq!(<u8>::from_as(261u16), 5u8);
//! ```
//!
//! Usage:
//!
//! Converting from digits as a number.
//! ```
//! # use num_convert::TryFromDigits;
//! let arr: [u32; 6] = [25_635_071, 25_634_091, 25_633_334, 25_636_309, 25_637_101, 25_636_243];
//! let val: Vec<u8> = arr.iter().map(|var| u8::from_digits(*var).unwrap_or(255u8) ).collect::<_>();
//! assert_eq!(val, [71, 91, 255, 255, 101, 243]);
//! ```
//!
//! Usage:
//!
//! The size of integer values in bits.
//! ```
//! # use num_convert::Sbits;
//! assert_eq!((-128i8).sbits(), 8u32);
//! assert_eq!(u64::MAX.sbits(), 64u32);
//! ```
//!
//! Usage:
//!
//! The size of integer type in bits.
//! ```
//! # use num_convert::Tbits;
//! assert_eq!(i8::tbits(), 8u32);
//! assert_eq!(u64::tbits(), 64u32);
//! ```
//!
//! Usage:
//!
//! Determining the number of digits of integers.
//! ```
//! # use num_convert::IntegerLen;
//! assert_eq!(0i8.len(), 1usize);
//! assert_eq!(i8::MAX.len(), 3usize);
//! assert_eq!(i128::MAX.len(), 39usize);
//! assert_eq!(u128::MAX.len(), 39usize);
//! ```

mod convert_from_by_add;
mod convert_into_by_add;
mod convert_from_as;
mod convert_into_as;
mod convert_try_from_by_add;
mod convert_try_into_by_add;
mod convert_try_from_digits;
mod extra_traits;

#[cfg(feature = "try_fm_by_add")]
mod convert_try_from;

#[cfg(feature = "try_to_by_add")]
mod convert_try_into;

#[cfg(feature = "cast_from_as")]
mod cast_from_as;

#[cfg(feature = "cast_into_as")]
mod cast_into_as;

#[cfg(feature = "bits")]
mod size_type_bits;

#[cfg(feature = "bounds")]
pub mod min_zero_max;

pub use crate::convert_from_by_add::FromByAdd;
pub use crate::convert_into_by_add::IntoByAdd;
pub use crate::convert_from_as::FromAs;
pub use crate::convert_into_as::IntoAs;
pub use crate::convert_try_into_by_add::TryIntoByAdd;
pub use crate::convert_try_from_by_add::TryFromByAdd;
pub use crate::convert_try_from_digits::TryFromDigits;
pub use crate::extra_traits::IntegerLen;

#[cfg(feature = "try_fm_by_add")]
pub use crate::convert_try_from::TryFmByAdd;
#[cfg(feature = "try_to_by_add")]
pub use crate::convert_try_into::TryToByAdd;

#[cfg(feature = "type_info")]
pub use crate::extra_traits::TypeInfo;

#[cfg(feature = "val_type_info")]
pub use crate::extra_traits::ValTypeInfo;

#[cfg(feature = "cast_into_as")]
pub use crate::cast_into_as::CastIntoAs;
#[cfg(feature = "cast_from_as")]
pub use crate::cast_from_as::CastFromAs;

#[cfg(feature = "bits")]
pub use crate::size_type_bits::Sbits;
#[cfg(feature = "bits")]
pub use crate::size_type_bits::Tbits;
