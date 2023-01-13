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
//! - The TryFromDigits trait for converting from digits as a number of possible value types.
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
//!
//! fn convert_into_u16<T: ToByAdd>(min: T, max: T) -> (u16, u16) {
//!     (min.into_u16(), max.into_u16())
//! }
//! assert_eq!((u16::MIN, u16::MAX), convert_into_u16(i16::MIN, i16::MAX));
//! ```
//! Usage:
//!
//! Convert the u8 type to a generic T type.
//! ```
//! # use num_convert::{FromAs, IntoAs};
//! # use std::ops::Div;
//!
//! fn primitive_type_len<T>(mut num: T) -> usize
//! where
//!     // Using the Std library.
//!     //T: Eq + Div<Output = T> + TryFrom<u8> + Copy + IntoAs<T>,
//!     //<T as TryFrom<u8>>::Error: Debug,
//!     T: Eq + Copy + Div<Output = T> + IntoAs<T>,
//! {
//!     let mut count = 0;
//!
//!     // Using the Std library.
//!     //let ten = <T as TryFrom<u8>>::try_from(10u8).unwrap();
//!     // There will never be a conversion error here.
//!     let ten = 10u8.into_as();
//!
//!     // Using the Std library.
//!     //let zero = <T as TryFrom<u8>>::try_from(0u8).unwrap();
//!     // There will never be a conversion error here.
//!     let zero = 0u8.into_as();
//!     while num != zero {
//!         num = num / ten;
//!         count += 1;
//!     }
//!     if count == 0 {
//!         1
//!     } else {
//!         count
//!     }
//! }
//! ```

mod convert_from;
mod convert_from_as;
mod convert_to;
mod convert_to_as;
mod convert_try_from;
mod convert_try_from_digits;
mod convert_try_to;
/// Module for implementation upper and lower bounds of types.
pub mod min_zero_max;
mod size_type_bits;
pub use crate::convert_from::FromByAdd;
pub use crate::convert_from_as::FromAs;
pub use crate::convert_to::ToByAdd;
pub use crate::convert_to_as::IntoAs;
pub use crate::convert_try_from::TryFromByAdd;
pub use crate::convert_try_from_digits::TryFromDigits;
pub use crate::convert_try_to::TryToByAdd;
pub use crate::size_type_bits::Sbits;
pub use crate::size_type_bits::Tbits;
