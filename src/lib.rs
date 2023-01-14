#![forbid(unsafe_code)]
#![no_std]
#![warn(missing_docs)]
//! # Generic traits for conversions between integer types.
//!
//! - The FromByAdd trait for converting from negative integers to positive or vice versa.
//! - The ToByAdd trait for converting into negative integers to positive or vice versa.
//! - The TryFromByAdd trait for converting from negative integers to positive or vice versa, that can fail.
//! - The TryToByAdd trait for converting into negative integers to positive or vice versa, that can fail.
//! - The FromAs trait for conversion from integers with overflow.
//! - The IntoAs trait for conversion into integers with overflow.
//! - The TryFromDigits trait for converting from digits as a number, with possible value types.
//!
//! # Generic traits for define the size of the integers in bits.
//!
//! - The Sbit trait for define the size of integer value in bits.
//! - The Tbit trait for define the size of integer type in bits.
//!
//! # Examples
//! Usage:
//!
//! Convert a negative value to a positive.
//! ```
//! # use num_convert::ToByAdd;
//! fn convert_into_u16<T: ToByAdd>(min: T, max: T) -> (u16, u16) {
//!     (min.into_u16(), max.into_u16())
//! }
//! assert_eq!((u16::MIN, u16::MAX), convert_into_u16(i16::MIN, i16::MAX));
//! ```
//!
//! Usage:
//!
//! Conversions type U16 in U8 without overflow and with overflow.
//! ```
//! # use num_convert::{IntoAs, FromAs};
//! assert_eq!(<u16 as IntoAs<u8>>::into_as(255u16), 255u8);
//! assert_eq!(<u16 as IntoAs<u8>>::into_as(258u16), 2u8);
//! assert_eq!(<u8 as FromAs<u16>>::from_as(261u16), 5u8);
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

mod convert_from;
mod convert_from_as;
mod convert_to;
mod convert_to_as;
mod convert_try_from;
mod convert_try_from_digits;
mod convert_try_to;
mod size_type_bits;

/// Module for implementation upper and lower bounds of types.
#[cfg(feature = "bounds")]
pub mod min_zero_max;

pub use crate::convert_from::FromByAdd;
pub use crate::convert_from_as::FromAs;
pub use crate::convert_to::ToByAdd;
pub use crate::convert_to_as::IntoAs;
pub use crate::convert_try_from::TryFromByAdd;
pub use crate::convert_try_from_digits::TryFromDigits;
pub use crate::convert_try_to::TryToByAdd;
pub use crate::size_type_bits::Sbits;
pub use crate::size_type_bits::Tbits;
