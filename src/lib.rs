//! # Traits for conversions between integer types.
//!
//! - The FromByAdd trait for conversion from integers 
//! - The ToByAdd trait for conversion into integers
//! - The TryFromByAdd trait for converting from integer values that can fail
//! - The TryToByAdd trait for converting into integer values that can fail.
//!
//! When converting from a negative or positive value to a positive or negative value,
//! the type and value are converted.
//! For example -128_i8 -> 0_u8, 127_i8 -> 255_u8, 0_u8 -> 127_i8.
//!
//! ## Examples
//!
//! ```
//! use num_convert::ToByAdd;
//!
//! fn convert_into_u16<T: ToByAdd>(min: T, max: T) -> (u16, u16) {
//!     (min.into_u16(), max.into_u16())
//! }
//! assert_eq!((u16::MIN, u16::MAX), convert_into_u16(i16::MIN, i16::MAX));

mod convert_from;
mod convert_to;
mod convert_from_as;
mod convert_to_as;
mod convert_try_from;
mod convert_try_to;
mod size_type_bits;
pub mod min_zero_max;
pub use crate::convert_from::FromByAdd;
pub use crate::convert_to::ToByAdd;
pub use crate::convert_from_as::FromAs;
pub use crate::convert_to_as::IntoAs;
pub use crate::convert_try_from::TryFromByAdd;
pub use crate::convert_try_to::TryToByAdd;
pub use crate::size_type_bits::Tbits;
