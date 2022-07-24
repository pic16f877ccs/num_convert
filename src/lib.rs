use core::{i16, i32, i8, isize};
use core::{u16, u32, u8, usize};

/// A generic trait for converting value types.
/// Convert from signed integers to unsigned in the full range of values.
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
