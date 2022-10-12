/// # A generic trait for converting from value types.
/// A trait that converts integers from negative to positive and positive to negative.
/// For the compatibility of negative to negative and positive to positive.
/// The conversion is within the possible range of values.
///
/// ## Examples
///
/// ```
/// use num_convert::TryFromByAdd;
/// 
/// // -128_i8 -> 0_u8
/// assert_eq!(<u8>::MIN, <u8 as TryFromByAdd>::try_from_i8(<i8>::MIN).unwrap());
/// // 127_i8 -> 255_u8
/// assert_eq!(<u8>::MAX, <u8 as TryFromByAdd>::try_from_i8(<i8>::MAX).unwrap());
///
/// ```

pub trait TryFromByAdd: Sized {

    fn try_from_i8(n: i8) -> Option<Self>;
    fn try_from_u8(n: u8) -> Option<Self>;
    fn try_from_i16(n: i16) -> Option<Self>;
    fn try_from_u16(n: u16) -> Option<Self>;
    fn try_from_i32(n: i32) -> Option<Self>;
    fn try_from_u32(n: u32) -> Option<Self>;
    fn try_from_i64(n: i64) -> Option<Self>;
    fn try_from_u64(n: u64) -> Option<Self>;
    fn try_from_isize(n: isize) -> Option<Self>;
    fn try_from_usize(n: usize) -> Option<Self>;
    fn try_from_i128(n: i128) -> Option<Self>;
    fn try_from_u128(n: u128) -> Option<Self>;
}

impl TryFromByAdd for i8 {
    /// Returns an `i8` for compatibility.
    #[inline]
    fn try_from_i8(n: i8) -> Option<i8> {
        Some(n)
    }

    /// Converts the value of `u8` to an `i8`.
    #[inline]
    fn try_from_u8(n: u8) -> Option<i8> {
        Some(((n as i8).wrapping_add(i8::MAX)).wrapping_add(1))
    }

    /// Converts the value of `i16` to an `i8`.
    #[inline]
    fn try_from_i16(n: i16) -> Option<i8> {
        if n >= -128 && n <= 127 {
            Some(n as i8)
        } else {
            None
        }
    }

    /// Converts the value of `u16` to an `i8`.
    #[inline]
    fn try_from_u16(n: u16) -> Option<i8> {
        if n <= 255 {
            Some(((n as i8).wrapping_add(i8::MAX)).wrapping_add(1))
        } else {
            None
        }
    }

    /// Converts the value of `i32` to an `i8`.
    #[inline]
    fn try_from_i32(n: i32) -> Option<i8> {
        if n >= -128 && n <= 127 {
            Some(n as i8)
        } else {
            None
        }
    }

    /// Converts the value of `u32` to an `i8`.
    #[inline]
    fn try_from_u32(n: u32) -> Option<i8> {
        if n <= 255 {
            Some(((n as i8).wrapping_add(i8::MAX)).wrapping_add(1))
        } else {
            None
        }
    }

    /// Converts the value of `i64` to an `i8`.
    #[inline]
    fn try_from_i64(n: i64) -> Option<i8> {
        if n >= -128 && n <= 127 {
            Some(n as i8)
        } else {
            None
        }
    }

    /// Converts the value of `u64` to an `i8`.
    #[inline]
    fn try_from_u64(n: u64) -> Option<i8> {
        if n <= 255 {
            Some(((n as i8).wrapping_add(i8::MAX)).wrapping_add(1))
        } else {
            None
        }
    }

    /// Converts the value of `isize` to an `i8`.
    #[inline]
    fn try_from_isize(n: isize) -> Option<i8> {
        if n >= -128 && n <= 127 {
            Some(n as i8)
        } else {
            None
        }
    }

    /// Converts the value of `usize` to an `i8`.
    #[inline]
    fn try_from_usize(n: usize) -> Option<i8> {
        if n <= 255 {
            Some(((n as i8).wrapping_add(i8::MAX)).wrapping_add(1))
        } else {
            None
        }
    }

    /// Converts the value of `i128` to an `i8`.
    #[inline]
    fn try_from_i128(n: i128) -> Option<i8> {
        if n >= -128 && n <= 127 {
            Some(n as i8)
        } else {
            None
        }
    }

    /// Converts the value of `u128` to an `i8`.
    #[inline]
    fn try_from_u128(n: u128) -> Option<i8> {
        if n <= 255 {
            Some(((n as i8).wrapping_add(i8::MAX)).wrapping_add(1))
        } else {
            None
        }
    }
}

impl TryFromByAdd for u8 {
    /// Converts the value of `i8` to an `u8`.
    #[inline]
    fn try_from_i8(n: i8) -> Option<u8> {
        Some((n as u8).wrapping_add(128))
    }

    /// Returns an `u8` for compatibility.
    #[inline]
    fn try_from_u8(n: u8) -> Option<u8> {
        Some(n)
    }

    /// Converts the value of `i16` to an `u8`.
    #[inline]
    fn try_from_i16(n: i16) -> Option<u8> {
        if n >= -128 && n <= 127 {
            Some((n as u8).wrapping_add(128))
        } else {
            None
        }
    }

    /// Converts the value of `u16` to an `u8`.
    #[inline]
    fn try_from_u16(n: u16) -> Option<u8> {
        if n <= 255 {
            Some(n as u8)
        } else {
            None
        }
    }

    /// Converts the value of `i32` to an `u8`.
    #[inline]
    fn try_from_i32(n: i32) -> Option<u8> {
        if n >= -128 && n <= 127 {
            Some((n as u8).wrapping_add(128))
        } else {
            None
        }
    }

    /// Converts the value of `u32` to an `u8`.
    #[inline]
    fn try_from_u32(n: u32) -> Option<u8> {
        if n <= 255 {
            Some(n as u8)
        } else {
            None
        }
    }

    /// Converts the value of `i64` to an `u8`.
    #[inline]
    fn try_from_i64(n: i64) -> Option<u8> {
        if n >= -128 && n <= 127 {
            Some((n as u8).wrapping_add(128))
        } else {
            None
        }
    }

    /// Converts the value of `u64` to an `u8`.
    #[inline]
    fn try_from_u64(n: u64) -> Option<u8> {
        if n <= 255 {
            Some(n as u8)
        } else {
            None
        }
    }

    /// Converts the value of `isize` to an `u8`.
    #[inline]
    fn try_from_isize(n: isize) -> Option<u8> {
        if n >= -128 && n <= 127 {
            Some((n as u8).wrapping_add(128))
        } else {
            None
        }
    }

    /// Converts the value of `usize` to an `u8`.
    #[inline]
    fn try_from_usize(n: usize) -> Option<u8> {
        if n <= 255 {
            Some(n as u8)
        } else {
            None
        }
    }

    /// Converts the value of `i128` to an `u8`.
    #[inline]
    fn try_from_i128(n: i128) -> Option<u8> {
        if n >= -128 && n <= 127 {
            Some((n as u8).wrapping_add(128))
        } else {
            None
        }
    }

    /// Converts the value of `u128` to an `u8`.
    #[inline]
    fn try_from_u128(n: u128) -> Option<u8> {
        if n <= 255 {
            Some(n as u8)
        } else {
            None
        }
    }
}

impl TryFromByAdd for i16 {
    /// Converts the value of `i8` to an `i16`.
    #[inline]
    fn try_from_i8(n: i8) -> Option<i16> {
        Some(n as i16)
    }

    /// Converts the value of `u8` to an `i16`.
    #[inline]
    fn try_from_u8(n: u8) -> Option<i16> {
        Some(((n as i8).wrapping_add(i8::MAX)).wrapping_add(1) as i16)
    }

    /// Returns an `i16` for compatibility.
    #[inline]
    fn try_from_i16(n: i16) -> Option<i16> {
        Some(n)
    }

    /// Converts the value of `u16` to an `i16`.
    #[inline]
    fn try_from_u16(n: u16) -> Option<i16> {
        Some(((n as i16).wrapping_add(i16::MAX)).wrapping_add(1))
    }

    /// Converts the value of `i32` to an `i16`.
    #[inline]
    fn try_from_i32(n: i32) -> Option<i16> {
        if n >= -32_768 && n <= 32_767 {
            Some(n as i16)
        } else {
            None
        }
    }

    /// Converts the value of `u32` to an `i16`.
    #[inline]
    fn try_from_u32(n: u32) -> Option<i16> {
        if n <= 65_535 {
            Some(((n as i16).wrapping_add(i16::MAX)).wrapping_add(1))
        } else {
            None
        }
    }

    /// Converts the value of `i64` to an `i16`.
    #[inline]
    fn try_from_i64(n: i64) -> Option<i16> {
        if n >= -32_768 && n <= 32_767 {
            Some(n as i16)
        } else {
            None
        }
    }

    /// Converts the value of `u64` to an `i16`.
    #[inline]
    fn try_from_u64(n: u64) -> Option<i16> {
        if n <= 65_535 {
            Some(((n as i16).wrapping_add(i16::MAX)).wrapping_add(1))
        } else {
            None
        }
    }

    /// Converts the value of `isize` to an `i16`.
    #[inline]
    fn try_from_isize(n: isize) -> Option<i16> {
        if n >= -32_768 && n <= 32_767 {
            Some(n as i16)
        } else {
            None
        }
    }

    /// Converts the value of `usize` to an `i16`.
    #[inline]
    fn try_from_usize(n: usize) -> Option<i16> {
        if n <= 65_535 {
            Some(((n as i16).wrapping_add(i16::MAX)).wrapping_add(1))
        } else {
            None
        }
    }

    /// Converts the value of `i128` to an `i16`.
    #[inline]
    fn try_from_i128(n: i128) -> Option<i16> {
        if n >= -32_768 && n <= 32_767 {
            Some(n as i16)
        } else {
            None
        }
    }

    /// Converts the value of `u128` to an `i16`.
    #[inline]
    fn try_from_u128(n: u128) -> Option<i16> {
        if n <= 65_535 {
            Some(((n as i16).wrapping_add(i16::MAX)).wrapping_add(1))
        } else {
            None
        }
    }
}

impl TryFromByAdd for u16 {
    /// Converts the value of `i8` to an `u16`.
    #[inline]
    fn try_from_i8(n: i8) -> Option<u16> {
        Some((n as u16).wrapping_add(128))
    }

    /// Converts the value of `u8` to an `u16`.
    #[inline]
    fn try_from_u8(n: u8) -> Option<u16> {
        Some(n as u16)
    }

    /// Converts the value of `i16` to an `u16`.
    #[inline]
    fn try_from_i16(n: i16) -> Option<u16> {
        Some((n as u16).wrapping_add(32_768))
    }


    /// Returns an `u16` for compatibility.
    #[inline]
    fn try_from_u16(n: u16) -> Option<u16> {
        Some(n)
    }

    /// Converts the value of `i32` to an `u16`.
    #[inline]
    fn try_from_i32(n: i32) -> Option<u16> {
        if n >= -32_768 && n <= 32_767 {
            Some((n as u16).wrapping_add(32_768))
        } else {
            None
        }
    }

    /// Converts the value of `u32` to an `u16`.
    #[inline]
    fn try_from_u32(n: u32) -> Option<u16> {
        if n <= 65_535 {
            Some(n as u16)
        } else {
            None
        }
    }

    /// Converts the value of `i64` to an `u16`.
    #[inline]
    fn try_from_i64(n: i64) -> Option<u16> {
        if n >= -32_768 && n <= 32_767 {
            Some((n as u16).wrapping_add(32_768))
        } else {
            None
        }
    }

    /// Converts the value of `u64` to an `u16`.
    #[inline]
    fn try_from_u64(n: u64) -> Option<u16> {
        if n <= 65_535 {
            Some(n as u16)
        } else {
            None
        }
    }

    /// Converts the value of `isize` to an `u16`.
    #[inline]
    fn try_from_isize(n: isize) -> Option<u16> {
        if n >= -32_768 && n <= 32_767 {
            Some((n as u16).wrapping_add(32_768))
        } else {
            None
        }
    }

    /// Converts the value of `usize` to an `u16`.
    #[inline]
    fn try_from_usize(n: usize) -> Option<u16> {
        if n <= 65_535 {
            Some(n as u16)
        } else {
            None
        }
    }

    /// Converts the value of `i128` to an `u16`.
    #[inline]
    fn try_from_i128(n: i128) -> Option<u16> {
        if n >= -32_768 && n <= 32_767 {
            Some((n as u16).wrapping_add(32_768))
        } else {
            None
        }
    }

    /// Converts the value of `u128` to an `u16`.
    #[inline]
    fn try_from_u128(n: u128) -> Option<u16> {
        if n <= 65_535 {
            Some(n as u16)
        } else {
            None
        }
    }
}

impl TryFromByAdd for i32 {
    /// Converts the value of `i8` to an `i32`.
    #[inline]
    fn try_from_i8(n: i8) -> Option<i32> {
        Some(n as i32)
    }

    /// Converts the value of `u8` to an `i32`.
    #[inline]
    fn try_from_u8(n: u8) -> Option<i32> {
        Some(((n as i8).wrapping_add(i8::MAX)).wrapping_add(1) as i32)
    }

    /// Converts the value of `i16` to an `i32`.
    #[inline]
    fn try_from_i16(n: i16) -> Option<i32> {
        Some(n as i32)
    }

    /// Converts the value of `u16` to an `i32`.
    #[inline]
    fn try_from_u16(n: u16) -> Option<i32> {
        Some(((n as i16).wrapping_add(i16::MAX)).wrapping_add(1) as i32)
    }

    /// Returns an `i32` for compatibility.
    #[inline]
    fn try_from_i32(n: i32) -> Option<i32> {
        Some(n)
    }

    /// Converts the value of `u32` to an `i32`.
    #[inline]
    fn try_from_u32(n: u32) -> Option<i32> {
        Some(((n as i32).wrapping_add(i32::MAX)).wrapping_add(1))
    }

    /// Converts the value of `i64` to an `i32`.
    #[inline]
    fn try_from_i64(n: i64) -> Option<i32> {
        if n >= -2_147_483_648 && n <= 2_147_483_647 {
            Some(n as i32)
        } else {
            None
        }
    }

    /// Converts the value of `u64` to an `i32`.
    #[inline]
    fn try_from_u64(n: u64) -> Option<i32> {
        if n <= 4_294_967_295 {
            Some(((n as i32).wrapping_add(i32::MAX)).wrapping_add(1))
        } else {
            None
        }
    }

    /// Converts the value of `isize` to an `i32`.
    #[inline]
    fn try_from_isize(n: isize) -> Option<i32> {
        if n >= -2_147_483_648 && n <= 2_147_483_647 {
            Some(n as i32)
        } else {
            None
        }
    }

    /// Converts the value of `usize` to an `i32`.
    #[inline]
    fn try_from_usize(n: usize) -> Option<i32> {
        if n <= 4_294_967_295 {
            Some(((n as i32).wrapping_add(i32::MAX)).wrapping_add(1))
        } else {
            None
        }
    }

    /// Converts the value of `i128` to an `i32`.
    #[inline]
    fn try_from_i128(n: i128) -> Option<i32> {
        if n >= -2_147_483_648 && n <= 2_147_483_647 {
            Some(n as i32)
        } else {
            None
        }
    }

    /// Converts the value of `u128` to an `i32`.
    #[inline]
    fn try_from_u128(n: u128) -> Option<i32> {
        if n <= 4_294_967_295 {
            Some(((n as i32).wrapping_add(i32::MAX)).wrapping_add(1))
        } else {
            None
        }
    }
}

impl TryFromByAdd for u32 {
    /// Converts the value of `i8` to an `u32`.
    #[inline]
    fn try_from_i8(n: i8) -> Option<u32> {
        Some((n as u32).wrapping_add(128))
    }

    /// Converts the value of `u8` to an `u32`.
    #[inline]
    fn try_from_u8(n: u8) -> Option<u32> {
        Some(n as u32)
    }

    /// Converts the value of `i16` to an `u32`.
    #[inline]
    fn try_from_i16(n: i16) -> Option<u32> {
        Some((n as u32).wrapping_add(32_768))
    }

    /// Converts the value of `u16` to an `u32`.
    #[inline]
    fn try_from_u16(n: u16) -> Option<u32> {
        Some(n as u32)
    }

    /// Converts the value of `i32` to an `u32`.
    #[inline]
    fn try_from_i32(n: i32) -> Option<u32> {
        Some((n as u32).wrapping_add(2_147_483_648))
    }

    /// Returns an `u32` for compatibility.
    #[inline]
    fn try_from_u32(n: u32) -> Option<u32> {
        Some(n)
    }

    /// Converts the value of `i64` to an `u32`.
    #[inline]
    fn try_from_i64(n: i64) -> Option<u32> {
        if n >= -2_147_483_648 && n <= 2_147_483_647 {
            Some((n as u32).wrapping_add(2_147_483_648))
        } else {
            None
        }
    }

    /// Converts the value of `u64` to an `u32`.
    #[inline]
    fn try_from_u64(n: u64) -> Option<u32> {
        if n <= 4_294_967_295 {
            Some(n as u32)
        } else {
            None
        }
    }

    /// Converts the value of `isize` to an `u32`, 32 bit.
    #[inline]
    #[cfg(target_pointer_width = "32")]
    fn try_from_isize(n: isize) -> Option<u32> {
        Some((n as u32).wrapping_add(2_147_483_648))
    }

    /// Converts the value of `isize` to an `u32`, 64 bit.
    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn try_from_isize(n: isize) -> Option<u32> {
        if n >= -2_147_483_648 && n <= 2_147_483_647 {
            Some((n as u32).wrapping_add(2_147_483_648))
        } else {
            None
        }
    }

    /// Converts the value of `usize` to an `u32`, 32 bit.
    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn try_from_usize(n: usize) -> Option<u32> {
        Some(n as u32)
    }

    /// Converts the value of `usize` to an `u32`, 64 bit.
    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn try_from_usize(n: usize) -> Option<u32> {
        if n <= 4_294_967_295 {
            Some(n as u32)
        } else {
            None
        }
    }

    /// Converts the value of `i128` to an `u32`.
    #[inline]
    fn try_from_i128(n: i128) -> Option<u32> {
        if n >= -2_147_483_648 && n <= 2_147_483_647 {
            Some((n as u32).wrapping_add(2_147_483_648))
        } else {
            None
        }
    }

    /// Converts the value of `u128` to an `u32`.
    #[inline]
    fn try_from_u128(n: u128) -> Option<u32> {
        if n <= 4_294_967_295 {
            Some(n as u32)
        } else {
            None
        }
    }
}

impl TryFromByAdd for i64 {
    /// Converts the value of `i8` to an `i64`.
    #[inline]
    fn try_from_i8(n: i8) -> Option<i64> {
        Some(n as i64)
    }

    /// Converts the value of `u8` to an `i64`.
    #[inline]
    fn try_from_u8(n: u8) -> Option<i64> {
        Some(((n as i8).wrapping_add(i8::MAX)).wrapping_add(1) as i64)
    }

    /// Converts the value of `i16` to an `i64`.
    #[inline]
    fn try_from_i16(n: i16) -> Option<i64> {
        Some(n as i64)
    }

    /// Converts the value of `u16` to an `i64`.
    #[inline]
    fn try_from_u16(n: u16) -> Option<i64> {
        Some(((n as i16).wrapping_add(i16::MAX)).wrapping_add(1) as i64)
    }

    /// Converts the value of `i32` to an `i64`.
    #[inline]
    fn try_from_i32(n: i32) -> Option<i64> {
        Some(n as i64)
    }

    /// Converts the value of `u32` to an `i64`.
    #[inline]
    fn try_from_u32(n: u32) -> Option<i64> {
        Some(((n as i32).wrapping_add(i32::MAX)).wrapping_add(1) as i64)
    }

    /// Returns an `i64` for compatibility.
    #[inline]
    fn try_from_i64(n: i64) -> Option<i64> {
        Some(n)
    }

    /// Converts the value of `u64` to an `i64`.
    #[inline]
    fn try_from_u64(n: u64) -> Option<i64> {
        Some(((n as i64).wrapping_add(i64::MAX)).wrapping_add(1))
    }

    /// Converts the value of `isize` to an `i64`.
    #[inline]
    fn try_from_isize(n: isize) -> Option<i64> {
        Some(n as i64)
    }

    /// Converts the value of `usize` to an `i64`, 32 bit.
    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn try_from_usize(n: usize) -> Option<i64> {
        Some(((n as isize).wrapping_add(isize::MAX)).wrapping_add(1) as i64)
    }

    /// Converts the value of `usize` to an `i64`, 64 bit.
    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn try_from_usize(n: usize) -> Option<i64> {
        Some(((n as i64).wrapping_add(i64::MAX)).wrapping_add(1))
    }

    /// Converts the value of `i128` to an `i64`.
    #[inline]
    fn try_from_i128(n: i128) -> Option<i64> {
        if n >= -9_223_372_036_854_775_808 && n <= 9_223_372_036_854_775_807 {
            Some(n as i64)
        } else {
            None
        }
    }

    /// Converts the value of `u128` to an `i64`.
    #[inline]
    fn try_from_u128(n: u128) -> Option<i64> {
        if n <= 18_446_744_073_709_551_615 {
            Some(((n as i64).wrapping_add(i64::MAX)).wrapping_add(1))
        } else {
            None
        }
    }
}

impl TryFromByAdd for u64 {

    /// Converts the value of `i8` to an `u64`.
    #[inline]
    fn try_from_i8(n: i8) -> Option<u64> {
        Some((n as u64).wrapping_add(128))
    }

    /// Converts the value of `u8` to an `u64`.
    #[inline]
    fn try_from_u8(n: u8) -> Option<u64> {
        Some(n as u64)
    }

    /// Converts the value of `i16` to an `u64`.
    #[inline]
    fn try_from_i16(n: i16) -> Option<u64> {
        Some((n as u64).wrapping_add(32_768))
    }

    /// Converts the value of `u16` to an `u64`.
    #[inline]
    fn try_from_u16(n: u16) -> Option<u64> {
        Some(n as u64)
    }

    /// Converts the value of `i32` to an `u64`.
    #[inline]
    fn try_from_i32(n: i32) -> Option<u64> {
        Some((n as u64).wrapping_add(2_147_483_648))
    }

    /// Converts the value of `u32` to an `u64`.
    #[inline]
    fn try_from_u32(n: u32) -> Option<u64> {
        Some(n as u64)
    }

    /// Converts the value of `i64` to an `u64`.
    #[inline]
    fn try_from_i64(n: i64) -> Option<u64> {
        Some((n as u64).wrapping_add(9_223_372_036_854_775_808))
    }

    /// Returns an `u64` for compatibility.
    #[inline]
    fn try_from_u64(n: u64) -> Option<u64> {
        Some(n)
    }

    /// Converts the value of `isize` to an `u64`, 32 bit.
    #[inline]
    #[cfg(target_pointer_width = "32")]
    fn try_from_isize(n: isize) -> Option<u64> {
        Some((n as u64).wrapping_add(2_147_483_648))
    }

    /// Converts the value of `isize` to an `u64`, 32 bit.
    #[inline]
    #[cfg(target_pointer_width = "64")]
    fn try_from_isize(n: isize) -> Option<u64> {
        Some((n as u64).wrapping_add(9_223_372_036_854_775_808))
    }

    /// Converts the value of `usize` to an `u64`.
    #[inline]
    fn try_from_usize(n: usize) -> Option<u64> {
        Some(n as u64)
    }

    /// Converts the value of `i128` to an `u64`.
    #[inline]
    fn try_from_i128(n: i128) -> Option<u64> {
        if n >= -9_223_372_036_854_775_808 && n <= 9_223_372_036_854_775_807 {
            Some((n as u64).wrapping_add(9_223_372_036_854_775_808))
        } else {
            None
        }
    }

    /// Converts the value of `u128` to an `u64`.
    #[inline]
    fn try_from_u128(n: u128) -> Option<u64> {
        if n <= 18_446_744_073_709_551_615 {
            Some(n as u64)
        } else {
            None
        }
    }
}

impl TryFromByAdd for isize {

    /// Converts the value of `i8` to an `isize`.
    #[inline]
    fn try_from_i8(n: i8) -> Option<isize> {
        Some(n as isize)
    }

    /// Converts the value of `u8` to an `isize`.
    #[inline]
    fn try_from_u8(n: u8) -> Option<isize> {
        Some(((n as i8).wrapping_add(i8::MAX)).wrapping_add(1) as isize)
    }

    /// Converts the value of `i16` to an `isize`.
    #[inline]
    fn try_from_i16(n: i16) -> Option<isize> {
        Some(n as isize)
    }

    /// Converts the value of `u16` to an `isize`.
    #[inline]
    fn try_from_u16(n: u16) -> Option<isize> {
        Some(((n as i16).wrapping_add(i16::MAX)).wrapping_add(1) as isize)
    }

    /// Converts the value of `i32` to an `isize`.
    #[inline]
    fn try_from_i32(n: i32) -> Option<isize> {
        Some(n as isize)
    }

    /// Converts the value of `u32` to an `isize`.
    #[inline]
    fn try_from_u32(n: u32) -> Option<isize> {
        Some(((n as i32).wrapping_add(i32::MAX)).wrapping_add(1) as isize)
    }

    /// Converts the value of `i64` to an `isize`, 32 bit.
    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn try_from_i64(n: i64) -> Option<isize> {
        if n >= -2_147_483_648 && n <= 2_147_483_647 {
            Some(n as isize)
        } else {
            None
        }
    }

    /// Converts the value of `i64` to an `isize`, 64 bit.
    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn try_from_i64(n: i64) -> Option<isize> {
        Some(n as isize)
    }

    #[inline]
    fn try_from_u64(n: u64) -> Option<isize> {
        Some(((n as isize).wrapping_add(isize::MAX)).wrapping_add(1))
    }

    /// Returns an `isize` for compatibility.
    #[inline]
    fn try_from_isize(n: isize) -> Option<isize> {
        Some(n)
    }

    /// Converts the value of `usize` to an `isize`.
    #[inline]
    fn try_from_usize(n: usize) -> Option<isize> {
        Some(((n as isize).wrapping_add(isize::MAX)).wrapping_add(1))
    }

    /// Converts the value of `i128` to an `isize`, 32 bit.
    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn try_from_i128(n: i128) -> Option<isize> {
        if n >= -2_147_483_648 && n <= 2_147_483_647 {
            Some(n as isize)
        } else {
            None
        }
    }

    /// Converts the value of `i128` to an `isize`, 64 bit.
    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn try_from_i128(n: i128) -> Option<isize> {
        if n >= -9_223_372_036_854_775_808 && n <= 9_223_372_036_854_775_807 {
            Some(n as isize)
        } else {
            None
        }
    }

    /// Converts the value of `u128` to an `isize`, 32 bit.
    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn try_from_u128(n: u128) -> Option<isize> {
        if n <= 4_294_967_295 {
            Some(((n as isize).wrapping_add(isize::MAX)).wrapping_add(1))
        } else {
            None
        }
    }

    /// Converts the value of `u128` to an `isize`, 64 bit.
    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn try_from_u128(n: u128) -> Option<isize> {
        if n <= 18_446_744_073_709_551_615 {
            Some(((n as isize).wrapping_add(isize::MAX)).wrapping_add(1))
        } else {
            None
        }
    }
}

impl TryFromByAdd for usize {

    /// Converts the value of `i8` to an `usize`.
    #[inline]
    fn try_from_i8(n: i8) -> Option<usize> {
        Some((n as usize).wrapping_add(128))
    }

    /// Converts the value of `u8` to an `usize`.
    #[inline]
    fn try_from_u8(n: u8) -> Option<usize> {
        Some(n as usize)
    }

    /// Converts the value of `i16` to an `usize`.
    #[inline]
    fn try_from_i16(n: i16) -> Option<usize> {
        Some((n as usize).wrapping_add(32_768))
    }

    /// Converts the value of `u16` to an `usize`.
    #[inline]
    fn try_from_u16(n: u16) -> Option<usize> {
        Some(n as usize)
    }

    /// Converts the value of `i32` to an `usize`.
    #[inline]
    fn try_from_i32(n: i32) -> Option<usize> {
        Some((n as usize).wrapping_add(2_147_483_648))
    }

    /// Converts the value of `u32` to an `usize`.
    #[inline]
    fn try_from_u32(n: u32) -> Option<usize> {
        Some(n as usize)
    }

    /// Converts the value of `i64` to an `usize`, 32 bit.
    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn try_from_i64(n: i64) -> Option<usize> {
        Some((n as usize).wrapping_add(2_147_483_648))
    }

    /// Converts the value of `i64` to an `usize`, 64 bit.
    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn try_from_i64(n: i64) -> Option<usize> {
        Some((n as usize).wrapping_add(9_223_372_036_854_775_808))
    }

    /// Converts the value of `u64` to an `usize`, 32 bit.
    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn try_from_u64(n: u64) -> Option<usize> {
        if n <= 4_294_967_295 {
            Some(n as usize)
        } else {
            None
        }
    }

    /// Converts the value of `u64` to an `usize`, 64 bit.
    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn try_from_u64(n: u64) -> Option<usize> {
        Some(n as usize)
    }

    /// Converts the value of `isize` to an `usize`, 32 bit.
    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn try_from_isize(n: isize) -> Option<usize> {
        Some((n as usize).wrapping_add(2_147_483_648))
    }

    /// Converts the value of `isize` to an `usize`, 64 bit.
    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn try_from_isize(n: isize) -> Option<usize> {
        Some((n as usize).wrapping_add(9_223_372_036_854_775_808))
    }

    /// Returns an `usize` for compatibility.
    #[inline]
    fn try_from_usize(n: usize) -> Option<usize> {
        Some(n)
    }

    /// Converts the value of `i128` to an `usize`, 32 bit.
    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn try_from_i128(n: i128) -> Option<usize> {
        if n >= -2_147_483_648 && n <= 2_147_483_647 {
            Some((n as usize).wrapping_add(2_147_483_648))
        } else {
            None
        }
    }

    /// Converts the value of `i128` to an `usize`, 64 bit.
    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn try_from_i128(n: i128) -> Option<usize> {
        if n >= -9_223_372_036_854_775_808 && n <= 9_223_372_036_854_775_807 {
            Some((n as usize).wrapping_add(9_223_372_036_854_775_808))
        } else {
            None
        }
    }

    /// Converts the value of `u128` to an `usize`, 32 bit.
    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn try_from_u128(n: u128) -> Option<usize> {
        if n <= 4_294_967_295 {
            Some(n as usize)
        } else {
            None
        }
    }

    /// Converts the value of `u128` to an `usize`, 64 bit.
    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn try_from_u128(n: u128) -> Option<usize> {
        if n <= 18_446_744_073_709_551_615 {
            Some(n as usize)
        } else {
            None
        }
    }
}

impl TryFromByAdd for i128 {
    /// Converts the value of `i8` to an `i128`.
    #[inline]
    fn try_from_i8(n: i8) -> Option<i128> {
        Some(n as i128)
    }

    /// Converts the value of `u8` to an `i128`.
    #[inline]
    fn try_from_u8(n: u8) -> Option<i128> {
        Some(((n as i8).wrapping_add(i8::MAX)).wrapping_add(1) as i128)
    }

    /// Converts the value of `i16` to an `i128`.
    #[inline]
    fn try_from_i16(n: i16) -> Option<i128> {
        Some(n as i128)
    }

    /// Converts the value of `u16` to an `i128`.
    #[inline]
    fn try_from_u16(n: u16) -> Option<i128> {
        Some(((n as i16).wrapping_add(i16::MAX)).wrapping_add(1) as i128)
    }

    /// Converts the value of `i32` to an `i128`.
    #[inline]
    fn try_from_i32(n: i32) -> Option<i128> {
        Some(n as i128)
    }

    /// Converts the value of `u32` to an `i128`.
    #[inline]
    fn try_from_u32(n: u32) -> Option<i128> {
        Some(((n as i32).wrapping_add(i32::MAX)).wrapping_add(1) as i128)
    }

    /// Converts the value of `i64` to an `i128`.
    #[inline]
    fn try_from_i64(n: i64) -> Option<i128> {
        Some(n as i128)
    }

    /// Converts the value of `u64` to an `i128`.
    #[inline]
    fn try_from_u64(n: u64) -> Option<i128> {
        Some(((n as i64).wrapping_add(i64::MAX)).wrapping_add(1) as i128)
    }

    /// Converts the value of `isize` to an `i128`.
    #[inline]
    fn try_from_isize(n: isize) -> Option<i128> {
        Some(n as i128)
    }

    /// Converts the value of `usize` to an `i128`.
    #[inline]
    fn try_from_usize(n: usize) -> Option<i128> {
        Some(((n as isize).wrapping_add(isize::MAX)).wrapping_add(1) as i128)
    }

    /// Returns an `i128` for compatibility.
    #[inline]
    fn try_from_i128(n: i128) -> Option<i128> {
        Some(n)
    }

    /// Converts the value of `u128` to an `i128`.
    #[inline]
    fn try_from_u128(n: u128) -> Option<i128> {
        Some(((n as i128).wrapping_add(i128::MAX)).wrapping_add(1))
    }
}

impl TryFromByAdd for u128 {

    /// Converts the value of `i8` to an `u128`.
    #[inline]
    fn try_from_i8(n: i8) -> Option<u128> {
        Some((n as u128).wrapping_add(128))
    }

    /// Converts the value of `u8` to an `u128`.
    #[inline]
    fn try_from_u8(n: u8) -> Option<u128> {
        Some(n as u128)
    }

    /// Converts the value of `i16` to an `u128`.
    #[inline]
    fn try_from_i16(n: i16) -> Option<u128> {
        Some((n as u128).wrapping_add(32_768))
    }

    /// Converts the value of `u16` to an `u128`.
    #[inline]
    fn try_from_u16(n: u16) -> Option<u128> {
        Some(n as u128)
    }

    /// Converts the value of `i32` to an `u128`.
    #[inline]
    fn try_from_i32(n: i32) -> Option<u128> {
        Some((n as u128).wrapping_add(2_147_483_648))
    }

   /// Converts the value of `u32` to an `u128`.
   #[inline]
   fn try_from_u32(n: u32) -> Option<u128> {
       Some(n as u128)
   }


    /// Converts the value of `i64` to an `u128`.
    #[inline]
    fn try_from_i64(n: i64) -> Option<u128> {
        Some((n as u128).wrapping_add(9_223_372_036_854_775_808))
    }


    /// Converts the value of `u64` to an `u128`.
    #[inline]
    fn try_from_u64(n: u64) -> Option<u128> {
        Some(n as u128)
    }

    /// Converts the value of `isize` to an `u128`, 32 bit.
    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn try_from_isize(n: isize) -> Option<u128> {
        Some((n as u128).wrapping_add(2_147_483_648))
    }

    /// Converts the value of `isize` to an `u128`, 64 bit.
    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn try_from_isize(n: isize) -> Option<u128> {
        Some((n as u128).wrapping_add(9_223_372_036_854_775_808))
    }

    /// Converts the value of `usize` to an `u128`.
    #[inline]
    fn try_from_usize(n: usize) -> Option<u128> {
        Some(n as u128)
    }

    /// Converts the value of `i128` to an `u128`.
    #[inline]
    fn try_from_i128(n: i128) -> Option<u128> {
        Some((n as u128).wrapping_add(170_141_183_460_469_231_731_687_303_715_884_105_728))
    }

    /// Returns an `u128` for compatibility.
    #[inline]
    fn try_from_u128(n: u128) -> Option<u128> {
        Some(n)
    }
}
