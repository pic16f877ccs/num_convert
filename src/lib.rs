use core::{i16, i32, i64, i8, isize};
use core::{u16, u32, u64, u8, usize};

/// Const for adding `usize` 32 bit.
#[cfg(target_pointer_width = "32")]
const ADD_VALUE_USIZE: usize = 2_147_483_648;

/// Const for adding `usize` 64 bit.
#[cfg(target_pointer_width = "64")]
const ADD_VALUE_USIZE: usize = 9_223_372_036_854_775_808;

///
/// Convert from signed integers to unsigned in the full range of values.
///
/// # A generic trait for converting value types.
///
/// # Examples
///
/// ```
/// use num_convert::ToByAdd;
///
/// fn convert_i8_to_u8<T: ToByAdd>(min: T, max: T) -> (u8, u8) {
///     (min.to_u8(), max.to_u8())
/// }
/// assert_eq!((u8::MIN, u8::MAX), convert_i8_to_u8(i8::MIN, i8::MAX));
///
/// assert_eq!(i8::MIN, ToByAdd::to_i8(&i8::MIN));
/// assert_eq!(i8::MAX, ToByAdd::to_i8(&i8::MAX));
/// assert_eq!(u8::MIN, ToByAdd::to_u8(&i8::MIN));
/// assert_eq!(u8::MAX, ToByAdd::to_u8(&i8::MAX));
/// ```
pub trait ToByAdd {
    /// Converts the value of `self` to an `i8`.
    fn to_i8(&self) -> i8;

    /// Converts the value of `self` to an `u8`.
    fn to_u8(&self) -> u8;

    /// Converts the value of `self` to an `i16`.
    fn to_i16(&self) -> i16;

    /// Converts the value of `self` to an `u16`.
    fn to_u16(&self) -> u16;

    /// Converts the value of `self` to an `i32`.
    fn to_i32(&self) -> i32;

    /// Converts the value of `self` to an `u32`.
    fn to_u32(&self) -> u32;

    /// Converts the value of `self` to an `i64`.
    fn to_i64(&self) -> i64;

    /// Converts the value of `self` to an `u64`.
    fn to_u64(&self) -> u64;

    /// Converts the value of `self` to an `isize`.
    fn to_isize(&self) -> isize;

    /// Converts the value of `self` to an `usize`.
    fn to_usize(&self) -> usize;
}

impl ToByAdd for i8 {
    /// Returns an `i8` for compatibility.
    #[inline]
    fn to_i8(&self) -> i8 {
        *self
    }

    /// Converts the value of `i8` to an `u8`.
    #[inline]
    fn to_u8(&self) -> u8 {
        (*self as u8).wrapping_add(128)
    }

    /// Converts the value of `i8` to an `i16`.
    #[inline]
    fn to_i16(&self) -> i16 {
        *self as i16
    }

    /// Converts the value of `i8` to an `u16`.
    #[inline]
    fn to_u16(&self) -> u16 {
        (*self as u16).wrapping_add(128)
    }

    /// Converts the value of `i8` to an `i32`.
    #[inline]
    fn to_i32(&self) -> i32 {
        *self as i32
    }

    /// Converts the value of `i8` to an `u32`.
    #[inline]
    fn to_u32(&self) -> u32 {
        (*self as u32).wrapping_add(128)
    }

    /// Converts the value of `i8` to an `i64`.
    #[inline]
    fn to_i64(&self) -> i64 {
        *self as i64
    }

    /// Converts the value of `i8` to an `u64`.
    #[inline]
    fn to_u64(&self) -> u64 {
        (*self as u64).wrapping_add(128)
    }

    /// Converts the value of `i8` to an `isize`.
    #[inline]
    fn to_isize(&self) -> isize {
        *self as isize
    }

    /// Converts the value of `i8` to an `usize`.
    #[inline]
    fn to_usize(&self) -> usize {
        (*self as usize).wrapping_add(128)
    }
}

impl ToByAdd for i16 {
    ///This value cannot be represented as an `i8`
    fn to_i8(&self) -> i8 {
        todo!();
    }

    ///This value cannot be represented as an `u8`
    fn to_u8(&self) -> u8 {
        todo!();
    }

    /// Returns an `i16` for compatibility.
    #[inline]
    fn to_i16(&self) -> i16 {
        *self
    }

    /// Converts the value of `i16` to an `u16`.
    #[inline]
    fn to_u16(&self) -> u16 {
        (*self as u16).wrapping_add(32_768)
    }

    /// Converts the value of `i16` to an `i32`.
    #[inline]
    fn to_i32(&self) -> i32 {
        *self as i32
    }

    /// Converts the value of `i16` to an `u32`.
    #[inline]
    fn to_u32(&self) -> u32 {
        (*self as u32).wrapping_add(32_768)
    }

    /// Converts the value of `i16` to an `i64`.
    #[inline]
    fn to_i64(&self) -> i64 {
        *self as i64
    }

    /// Converts the value of `i16` to an `u64`.
    #[inline]
    fn to_u64(&self) -> u64 {
        (*self as u64).wrapping_add(32_768)
    }

    /// Converts the value of `i16` to an `isize`.
    #[inline]
    fn to_isize(&self) -> isize {
        *self as isize
    }
    /// Converts the value of `i16` to an `usize`.
    #[inline]
    fn to_usize(&self) -> usize {
        (*self as usize).wrapping_add(32_768)
    }
}

impl ToByAdd for i32 {
    ///This value cannot be represented as an `i8`
    fn to_i8(&self) -> i8 {
        todo!();
    }

    ///This value cannot be represented as an `u8`
    fn to_u8(&self) -> u8 {
        todo!();
    }

    ///This value cannot be represented as an `i16`
    fn to_i16(&self) -> i16 {
        todo!();
    }

    ///This value cannot be represented as an `u16`
    fn to_u16(&self) -> u16 {
        todo!();
    }

    /// Returns an `i32` for compatibility.
    #[inline]
    fn to_i32(&self) -> i32 {
        *self
    }

    /// Converts the value of `i32` to an `u32`.
    #[inline]
    fn to_u32(&self) -> u32 {
        (*self as u32).wrapping_add(2_147_483_648)
    }

    /// Converts the value of `i32` to an `i64`.
    #[inline]
    fn to_i64(&self) -> i64 {
        *self as i64
    }

    /// Converts the value of `i32` to an `u64`.
    #[inline]
    fn to_u64(&self) -> u64 {
        (*self as u64).wrapping_add(2_147_483_648)
    }

    /// Converts the value of `i32` to an `isize`.
    #[inline]
    fn to_isize(&self) -> isize {
        *self as isize
    }

    /// Converts the value of `i32` to an `usize`.
    #[inline]
    fn to_usize(&self) -> usize {
        (*self as usize).wrapping_add(2_147_483_648)
    }
}

impl ToByAdd for i64 {
    ///This value cannot be represented as an `i8`
    fn to_i8(&self) -> i8 {
        todo!();
    }

    ///This value cannot be represented as an `u8`
    fn to_u8(&self) -> u8 {
        todo!();
    }

    ///This value cannot be represented as an `i16`
    fn to_i16(&self) -> i16 {
        todo!();
    }

    ///This value cannot be represented as an `u16`
    fn to_u16(&self) -> u16 {
        todo!();
    }

    ///This value cannot be represented as an `u16`
    #[inline]
    fn to_i32(&self) -> i32 {
        todo!();
    }

    ///This value cannot be represented as an `u16`
    #[inline]
    fn to_u32(&self) -> u32 {
        todo!();
    }

    /// Converts the value of `i64` to an `i64`.
    #[inline]
    fn to_i64(&self) -> i64 {
        *self
    }

    /// Converts the value of `i64` to an `u64`.
    #[inline]
    fn to_u64(&self) -> u64 {
        (*self as u64).wrapping_add(9_223_372_036_854_775_808)
    }

    /// Converts the value of `i64` to an `isize`.
    #[inline]
    fn to_isize(&self) -> isize {
        *self as isize
    }

    /// Converts the value of `i64` to an `usize`, 32 or 64 bit arch.
    #[inline]
    fn to_usize(&self) -> usize {
        (*self as usize).wrapping_add(ADD_VALUE_USIZE)
    }
}

impl ToByAdd for isize {
    ///This value cannot be represented as an `i8`
    fn to_i8(&self) -> i8 {
        todo!();
    }

    ///This value cannot be represented as an `u8`
    fn to_u8(&self) -> u8 {
        todo!();
    }

    ///This value cannot be represented as an `i16`
    fn to_i16(&self) -> i16 {
        todo!();
    }

    ///This value cannot be represented as an `u16`
    fn to_u16(&self) -> u16 {
        todo!();
    }

    ///This value cannot be represented as an `u16`
    #[inline]
    fn to_i32(&self) -> i32 {
        todo!();
    }

    ///This value cannot be represented as an `u16`
    #[inline]
    fn to_u32(&self) -> u32 {
        todo!();
    }

    /// Converts the value of `isize` to an `i64`.
    #[inline]
    fn to_i64(&self) -> i64 {
        *self as i64
    }

    /// Converts the value of `isize` to an `u64`.
    #[inline]
    fn to_u64(&self) -> u64 {
        (*self as u64).wrapping_add(9_223_372_036_854_775_808)
    }

    /// Converts the value of `isize` to an `isize`.
    #[inline]
    fn to_isize(&self) -> isize {
        *self
    }

    /// Converts the value of `isize` to an `usize`, 32 or 64 bit arch.
    #[inline]
    fn to_usize(&self) -> usize {
        (*self as usize).wrapping_add(ADD_VALUE_USIZE)
    }
}
