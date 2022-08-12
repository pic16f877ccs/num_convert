use core::{i128, i16, i32, i64, i8, isize};
use core::{u128, u16, u32, u64, u8, usize};

///
/// Convert from signed integers to signed values, see below for details.    
/// -128_i8 -> -128_i8 ... -128_i8 -> -128_i128, -128_i128 -> -128_i8
/// 127_i8 -> 127_i8 ... 127_i8 -> 127_i128, 127_i128 -> 127_i8
/// convert from unsigned integers to unsigned values, see below for details.
/// 0_u8 -> 0_u8 ... 0_u8 -> 0_u128, 0_u128 -> 0_u8
/// 255_u8 -> 255_u8 ... 255_u8 -> 255_u128, 255_u128 -> 255_u8
/// Convert from signed integers to unsigned values, see below for details.
/// -128_i8 -> 0_u8 ... -128_i8 -> 0_u128, -128_i128 -> 0_u8
/// 127_i8 -> 255_u8 ... 127_i8 -> 255_u128, 127_i128 -> 255_u8
/// convert from unsigned integers to signed values, see below for details.
/// 0_u8 -> -128_i8 ... 0_u8 -> -128_i128, 255_u128 -> -128_i8
/// 255_u8 -> 127_i8 ... 255_u8 -> 127_i128, 255_u128 -> 127_i8
///
/// # A generic trait for converting value types.
///
/// # Examples
///
/// ```
///
///
///
/// ```

pub trait TryFromByAdd: Sized {
    type Error;

    fn try_from_i8(n: i8) -> Result<Self, Self::Error>;
    fn try_from_u8(n: u8) -> Result<Self, Self::Error>;
    fn try_from_i16(n: i16) -> Result<Self, Self::Error>;
    fn try_from_u16(n: u16) -> Result<Self, Self::Error>;
    fn try_from_i32(n: i32) -> Result<Self, Self::Error>;
    fn try_from_u32(n: u32) -> Result<Self, Self::Error>;
    fn try_from_i64(n: i64) -> Result<Self, Self::Error>;
    fn try_from_u64(n: u64) -> Result<Self, Self::Error>;
    fn try_from_isize(n: isize) -> Result<Self, Self::Error>;
    fn try_from_usize(n: usize) -> Result<Self, Self::Error>;
    fn try_from_i128(n: i128) -> Result<Self, Self::Error>;
    fn try_from_u128(n: u128) -> Result<Self, Self::Error>;
}

impl TryFromByAdd for i8 {
    type Error = &'static str;
    /// Returns an `i8` for compatibility.
    #[inline]
    fn try_from_i8(n: i8) -> Result<i8, Self::Error> {
        Ok(n)
    }

    /// Converts the value of `u8` to an `i8`.
    #[inline]
    fn try_from_u8(n: u8) -> Result<i8, Self::Error> {
        Ok(((n as i8).wrapping_add(i8::MAX)).wrapping_add(1))
    }

    /// Converts the value of `i16` to an `i8`.
    #[inline]
    fn try_from_i16(n: i16) -> Result<i8, Self::Error> {
        if n >= -128 && n <= 127 {
            Ok(n as i8)
        } else {
            Err("Cannot be converted to type i8")
        }
    }

    /// Converts the value of `u16` to an `i8`.
    #[inline]
    fn try_from_u16(n: u16) -> Result<i8, Self::Error> {
        if n <= 255 {
            Ok(((n as i8).wrapping_add(i8::MAX)).wrapping_add(1))
        } else {
            Err("Cannot be converted to type i8")
        }
    }

    /// Converts the value of `i32` to an `i8`.
    #[inline]
    fn try_from_i32(n: i32) -> Result<i8, Self::Error> {
        if n >= -128 && n <= 127 {
            Ok(n as i8)
        } else {
            Err("Cannot be converted to type i8")
        }
    }

    /// Converts the value of `u32` to an `i8`.
    #[inline]
    fn try_from_u32(n: u32) -> Result<i8, Self::Error> {
        if n <= 255 {
            Ok(((n as i8).wrapping_add(i8::MAX)).wrapping_add(1))
        } else {
            Err("Cannot be converted to type i8")
        }
    }

    /// Converts the value of `i64` to an `i8`.
    #[inline]
    fn try_from_i64(n: i64) -> Result<i8, Self::Error> {
        if n >= -128 && n <= 127 {
            Ok(n as i8)
        } else {
            Err("Cannot be converted to type i8")
        }
    }

    /// Converts the value of `u64` to an `i8`.
    #[inline]
    fn try_from_u64(n: u64) -> Result<i8, Self::Error> {
        if n <= 255 {
            Ok(((n as i8).wrapping_add(i8::MAX)).wrapping_add(1))
        } else {
            Err("Cannot be converted to type i8")
        }
    }

    /// Converts the value of `isize` to an `i8`.
    #[inline]
    fn try_from_isize(n: isize) -> Result<i8, Self::Error> {
        if n >= -128 && n <= 127 {
            Ok(n as i8)
        } else {
            Err("Cannot be converted to type i8")
        }
    }

    /// Converts the value of `usize` to an `i8`.
    #[inline]
    fn try_from_usize(n: usize) -> Result<i8, Self::Error> {
        if n <= 255 {
            Ok(((n as i8).wrapping_add(i8::MAX)).wrapping_add(1))
        } else {
            Err("Cannot be converted to type i8")
        }
    }

    /// Converts the value of `i128` to an `i8`.
    #[inline]
    fn try_from_i128(n: i128) -> Result<i8, Self::Error> {
        if n >= -128 && n <= 127 {
            Ok(n as i8)
        } else {
            Err("Cannot be converted to type i8")
        }
    }

    /// Converts the value of `u128` to an `i8`.
    #[inline]
    fn try_from_u128(n: u128) -> Result<i8, Self::Error> {
        if n <= 255 {
            Ok(((n as i8).wrapping_add(i8::MAX)).wrapping_add(1))
        } else {
            Err("Cannot be converted to type i8")
        }
    }
}

//impl TryFromByAdd for u8 {
//    type Error = &'static str;
//    /// Converts the value of `i8` to an `u8`.
//    #[inline]
//    fn try_into_u8(&self) -> Result<u8, Self::Error> {
//        Ok((n as u8).wrapping_add(128))
//    }
//
//    /// Returns an `u8` for compatibility.
//    #[inline]
//    fn try_into_u8(&self) -> Result<u8, Self::Error> {
//        Ok(n)
//    }
//
//    /// Converts the value of `u8` to an `i16`.
//    #[inline]
//    fn try_into_i16(&self) -> Result<i16, Self::Error> {
//        Ok(((n as i8).wrapping_add(i8::MAX)).wrapping_add(1) as i16)
//    }
//
//    /// Converts the value of `u8` to an `u16`.
//    #[inline]
//    fn try_into_u16(&self) -> Result<u16, Self::Error> {
//        Ok(n as u16)
//    }
//
//    /// Converts the value of `u8` to an `i32`.
//    #[inline]
//    fn try_into_i32(&self) -> Result<i32, Self::Error> {
//        Ok(((n as i8).wrapping_add(i8::MAX)).wrapping_add(1) as i32)
//    }
//
//    /// Converts the value of `u8` to an `u32`.
//    #[inline]
//    fn try_into_u32(&self) -> Result<u32, Self::Error> {
//        Ok(n as u32)
//    }
//
//    /// Converts the value of `u8` to an `i64`.
//    #[inline]
//    fn try_into_i64(&self) -> Result<i64, Self::Error> {
//        Ok(((n as i8).wrapping_add(i8::MAX)).wrapping_add(1) as i64)
//    }
//
//    fn try_into_u64(&self) -> Result<u64, Self::Error> {
//        Ok(n as u64)
//    }
//
//    /// Converts the value of `u8` to an `isize`.
//    #[inline]
//    fn try_into_isize(&self) -> Result<isize, Self::Error> {
//        Ok(((n as i8).wrapping_add(i8::MAX)).wrapping_add(1) as isize)
//    }
//
//    /// Converts the value of `u8` to an `usize`.
//    #[inline]
//    fn try_into_usize(&self) -> Result<usize, Self::Error> {
//        Ok(n as usize)
//    }
//
//    /// Converts the value of `u8` to an `i128`.
//    #[inline]
//    fn try_into_i128(&self) -> Result<i128, Self::Error> {
//        Ok(((n as i8).wrapping_add(i8::MAX)).wrapping_add(1) as i128)
//    }
//
//    /// Converts the value of `u8` to an `u128`.
//    #[inline]
//    fn try_into_u128(&self) -> Result<u128, Self::Error> {
//        Ok(n as u128)
//    }
//}
//
//impl TryFromByAdd for i16 {
//    type Error = &'static str;
//
//    /// Converts the value of `i8` to an `i16`.
//    #[inline]
//    fn try_into_i16(&self) -> Result<i16, Self::Error> {
//        Ok(n as i16)
//    }
//
//    /// Converts the value of `i16` to an `u8`.
//    #[inline]
//    fn try_into_u8(&self) -> Result<u8, Self::Error> {
//        if n >= -128 && n <= 127 {
//            Ok((n as u8).wrapping_add(128))
//        } else {
//            Err("Cannot be converted to type u8")
//        }
//    }
//
//    /// Returns an `i16` for compatibility.
//    #[inline]
//    fn try_into_i16(&self) -> Result<i16, Self::Error> {
//        Ok(n)
//    }
//
//    /// Converts the value of `i16` to an `u16`.
//    #[inline]
//    fn try_into_u16(&self) -> Result<u16, Self::Error> {
//        Ok((n as u16).wrapping_add(32_768))
//    }
//
//    /// Converts the value of `i16` to an `i32`.
//    #[inline]
//    fn try_into_i32(&self) -> Result<i32, Self::Error> {
//        Ok(n as i32)
//    }
//
//    /// Converts the value of `i16` to an `u32`.
//    #[inline]
//    fn try_into_u32(&self) -> Result<u32, Self::Error> {
//        Ok((n as u32).wrapping_add(32_768))
//    }
//
//    /// Converts the value of `i16` to an `i64`.
//    #[inline]
//    fn try_into_i64(&self) -> Result<i64, Self::Error> {
//        Ok(n as i64)
//    }
//
//    /// Converts the value of `i16` to an `u64`.
//    #[inline]
//    fn try_into_u64(&self) -> Result<u64, Self::Error> {
//        Ok((n as u64).wrapping_add(32_768))
//    }
//
//    /// Converts the value of `i16` to an `isize`.
//    #[inline]
//    fn try_into_isize(&self) -> Result<isize, Self::Error> {
//        Ok(n as isize)
//    }
//
//    /// Converts the value of `i16` to an `usize`.
//    #[inline]
//    fn try_into_usize(&self) -> Result<usize, Self::Error> {
//        Ok((n as usize).wrapping_add(32_768))
//    }
//
//    /// Converts the value of `i16` to an `i128`.
//    #[inline]
//    fn try_into_i128(&self) -> Result<i128, Self::Error> {
//        Ok(n as i128)
//    }
//
//    /// Converts the value of `i16` to an `u128`.
//    #[inline]
//    fn try_into_u128(&self) -> Result<u128, Self::Error> {
//        Ok((n as u128).wrapping_add(32_768))
//    }
//}
//
//impl TryFromByAdd for u16 {
//    type Error = &'static str;
//
//    /// Converts the value of `i8` to an `u16`.
//    #[inline]
//    fn try_into_u16(&self) -> Result<u16, Self::Error> {
//        Ok((n as u16).wrapping_add(128))
//    }
//
//    /// Converts the value of `u16` to an `u8`.
//    #[inline]
//    fn try_into_u8(&self) -> Result<u8, Self::Error> {
//        if n <= 255 {
//            Ok(n as u8)
//        } else {
//            Err("Cannot be converted to type u8")
//        }
//    }
//
//    /// Converts the value of `u16` to an `i16`.
//    #[inline]
//    fn try_into_i16(&self) -> Result<i16, Self::Error> {
//        Ok(((n as i16).wrapping_add(i16::MAX)).wrapping_add(1))
//    }
//
//    /// Returns an `u16` for compatibility.
//    #[inline]
//    fn try_into_u16(&self) -> Result<u16, Self::Error> {
//        Ok(n)
//    }
//
//    /// Converts the value of `u16` to an `i32`.
//    #[inline]
//    fn try_into_i32(&self) -> Result<i32, Self::Error> {
//        Ok(((n as i16).wrapping_add(i16::MAX)).wrapping_add(1) as i32)
//    }
//
//    /// Converts the value of `u16` to an `u32`.
//    #[inline]
//    fn try_into_u32(&self) -> Result<u32, Self::Error> {
//        Ok(n as u32)
//    }
//
//    /// Converts the value of `u16` to an `i64`.
//    #[inline]
//    fn try_into_i64(&self) -> Result<i64, Self::Error> {
//        Ok(((n as i16).wrapping_add(i16::MAX)).wrapping_add(1) as i64)
//    }
//
//    /// Converts the value of `u16` to an `u64`.
//    #[inline]
//    fn try_into_u64(&self) -> Result<u64, Self::Error> {
//        Ok(n as u64)
//    }
//
//    /// Converts the value of `u16` to an `isize`.
//    #[inline]
//    fn try_into_isize(&self) -> Result<isize, Self::Error> {
//        Ok(((n as i16).wrapping_add(i16::MAX)).wrapping_add(1) as isize)
//    }
//
//    /// Converts the value of `u16` to an `usize`.
//    #[inline]
//    fn try_into_usize(&self) -> Result<usize, Self::Error> {
//        Ok(n as usize)
//    }
//
//    /// Converts the value of `u16` to an `i128`.
//    #[inline]
//    fn try_into_i128(&self) -> Result<i128, Self::Error> {
//        Ok(((n as i16).wrapping_add(i16::MAX)).wrapping_add(1) as i128)
//    }
//
//    /// Converts the value of `u16` to an `u128`.
//    #[inline]
//    fn try_into_u128(&self) -> Result<u128, Self::Error> {
//        Ok(n as u128)
//    }
//}
//
//impl TryFromByAdd for i32 {
//    type Error = &'static str;
//
//
//    /// Converts the value of `i32` to an `u8`.
//    #[inline]
//    fn try_into_u8(&self) -> Result<u8, Self::Error> {
//        if n >= -128 && n <= 127 {
//            Ok((n as u8).wrapping_add(128))
//        } else {
//            Err("Cannot be converted to type u8")
//        }
//    }
//
//    /// Converts the value of `i32` to an `i16`.
//    #[inline]
//    fn try_into_i16(&self) -> Result<i16, Self::Error> {
//        if n >= -32_768 && n <= 32_767 {
//            Ok(n as i16)
//        } else {
//            Err("Cannot be converted to type i16")
//        }
//    }
//
//    /// Converts the value of `i32` to an `u16`.
//    #[inline]
//    fn try_into_u16(&self) -> Result<u16, Self::Error> {
//        if n >= -32_768 && n <= 32_767 {
//            Ok((n as u16).wrapping_add(32_768))
//        } else {
//            Err("Cannot be converted to type u16")
//        }
//    }
//
//    /// Returns an `i32` for compatibility.
//    #[inline]
//    fn try_into_i32(&self) -> Result<i32, Self::Error> {
//        Ok(n)
//    }
//
//    /// Converts the value of `i32` to an `u32`.
//    #[inline]
//    fn try_into_u32(&self) -> Result<u32, Self::Error> {
//        Ok((n as u32).wrapping_add(2_147_483_648))
//    }
//
//    /// Converts the value of `i32` to an `i64`.
//    #[inline]
//    fn try_into_i64(&self) -> Result<i64, Self::Error> {
//        Ok(n as i64)
//    }
//
//    /// Converts the value of `i32` to an `u64`.
//    #[inline]
//    fn try_into_u64(&self) -> Result<u64, Self::Error> {
//        Ok((n as u64).wrapping_add(2_147_483_648))
//    }
//
//    /// Converts the value of `i32` to an `isize`.
//    #[inline]
//    fn try_into_isize(&self) -> Result<isize, Self::Error> {
//        Ok(n as isize)
//    }
//
//    /// Converts the value of `i32` to an `usize`.
//    #[inline]
//    fn try_into_usize(&self) -> Result<usize, Self::Error> {
//        Ok((n as usize).wrapping_add(2_147_483_648))
//    }
//
//    /// Converts the value of `i32` to an `i128`.
//    #[inline]
//    fn try_into_i128(&self) -> Result<i128, Self::Error> {
//        Ok(n as i128)
//    }
//
//    /// Converts the value of `i32` to an `u128`.
//    #[inline]
//    fn try_into_u128(&self) -> Result<u128, Self::Error> {
//        Ok((n as u128).wrapping_add(2_147_483_648))
//    }
//}
//
//impl TryFromByAdd for u32 {
//    type Error = &'static str;
//
//    /// Converts the value of `i8` to an `u32`.
//    #[inline]
//    fn try_into_u32(&self) -> Result<u32, Self::Error> {
//        Ok((n as u32).wrapping_add(128))
//    }
//
//    /// Converts the value of `u32` to an `u8`.
//    #[inline]
//    fn try_into_u8(&self) -> Result<u8, Self::Error> {
//        if n <= 255 {
//            Ok(n as u8)
//        } else {
//            Err("Cannot be converted to type u8")
//        }
//    }
//
//    /// Converts the value of `u32` to an `i16`.
//    #[inline]
//    fn try_into_i16(&self) -> Result<i16, Self::Error> {
//        if n <= 65_535 {
//            Ok(((n as i16).wrapping_add(i16::MAX)).wrapping_add(1))
//        } else {
//            Err("Cannot be converted to type i16")
//        }
//    }
//
//    /// Converts the value of `u32` to an `u16`.
//    #[inline]
//    fn try_into_u16(&self) -> Result<u16, Self::Error> {
//        if n <= 65_535 {
//            Ok(n as u16)
//        } else {
//            Err("Cannot be converted to type u16")
//        }
//    }
//
//    /// Converts the value of `u32` to an `i32`.
//    #[inline]
//    fn try_into_i32(&self) -> Result<i32, Self::Error> {
//        Ok(((n as i32).wrapping_add(i32::MAX)).wrapping_add(1))
//    }
//
//    /// Returns an `u32` for compatibility.
//    #[inline]
//    fn try_into_u32(&self) -> Result<u32, Self::Error> {
//        Ok(n)
//    }
//
//    /// Converts the value of `u32` to an `i64`.
//    #[inline]
//    fn try_into_i64(&self) -> Result<i64, Self::Error> {
//        Ok(((n as i32).wrapping_add(i32::MAX)).wrapping_add(1) as i64)
//    }
//
//    /// Converts the value of `u32` to an `u64`.
//    #[inline]
//    fn try_into_u64(&self) -> Result<u64, Self::Error> {
//        Ok(n as u64)
//    }
//
//    /// Converts the value of `u32` to an `isize`.
//    #[inline]
//    fn try_into_isize(&self) -> Result<isize, Self::Error> {
//        Ok(((n as i32).wrapping_add(i32::MAX)).wrapping_add(1) as isize)
//    }
//
//    /// Converts the value of `u32` to an `usize`.
//    #[inline]
//    fn try_into_usize(&self) -> Result<usize, Self::Error> {
//        Ok(n as usize)
//    }
//
//    /// Converts the value of `u32` to an `i128`.
//    #[inline]
//    fn try_into_i128(&self) -> Result<i128, Self::Error> {
//        Ok(((n as i32).wrapping_add(i32::MAX)).wrapping_add(1) as i128)
//    }
//
//    /// Converts the value of `u32` to an `u128`.
//    #[inline]
//    fn try_into_u128(&self) -> Result<u128, Self::Error> {
//        Ok(n as u128)
//    }
//}
//
//impl TryFromByAdd for i64 {
//    type Error = &'static str;
//
//    /// Converts the value of `i8` to an `i64`.
//    #[inline]
//    fn try_into_i64(&self) -> Result<i64, Self::Error> {
//        Ok(n as i64)
//    }
//
//    /// Converts the value of `i64` to an `u8`.
//    #[inline]
//    fn try_into_u8(&self) -> Result<u8, Self::Error> {
//        if n >= -128 && n <= 127 {
//            Ok((n as u8).wrapping_add(128))
//        } else {
//            Err("Cannot be converted to type u8")
//        }
//    }
//
//    /// Converts the value of `i64` to an `i16`.
//    #[inline]
//    fn try_into_i16(&self) -> Result<i16, Self::Error> {
//        if n >= -32_768 && n <= 32_767 {
//            Ok(n as i16)
//        } else {
//            Err("Cannot be converted to type i16")
//        }
//    }
//
//    /// Converts the value of `i64` to an `u16`.
//    #[inline]
//    fn try_into_u16(&self) -> Result<u16, Self::Error> {
//        if n >= -32_768 && n <= 32_767 {
//            Ok((n as u16).wrapping_add(32_768))
//        } else {
//            Err("Cannot be converted to type u16")
//        }
//    }
//
//    /// Converts the value of `i64` to an `i32`.
//    #[inline]
//    fn try_into_i32(&self) -> Result<i32, Self::Error> {
//        if n >= -2_147_483_648 && n <= 2_147_483_647 {
//            Ok(n as i32)
//        } else {
//            Err("Cannot be converted to type i32")
//        }
//    }
//
//    /// Converts the value of `i64` to an `u32`.
//    #[inline]
//    fn try_into_u32(&self) -> Result<u32, Self::Error> {
//        if n >= -2_147_483_648 && n <= 2_147_483_647 {
//            Ok((n as u32).wrapping_add(2_147_483_648))
//        } else {
//            Err("Cannot be converted to type u32")
//        }
//    }
//
//    /// Returns an `i64` for compatibility.
//    #[inline]
//    fn try_into_i64(&self) -> Result<i64, Self::Error> {
//        Ok(n)
//    }
//
//    /// Converts the value of `i64` to an `u64`.
//    #[inline]
//    fn try_into_u64(&self) -> Result<u64, Self::Error> {
//        Ok((n as u64).wrapping_add(9_223_372_036_854_775_808))
//    }
//
//    /// Converts the value of `i64` to an `isize`.
//    #[inline]
//    fn try_into_isize(&self) -> Result<isize, Self::Error> {
//        Ok(n as isize)
//    }
//
//    /// Converts the value of `i64` to an `usize`.
//    #[inline]
//    fn try_into_usize(&self) -> Result<usize, Self::Error> {
//        Ok((n as usize).wrapping_add(9_223_372_036_854_775_808))
//    }
//
//    /// Converts the value of `i64` to an `i128`.
//    #[inline]
//    fn try_into_i128(&self) -> Result<i128, Self::Error> {
//        Ok(n as i128)
//    }
//
//    /// Converts the value of `i64` to an `u128`.
//    #[inline]
//    fn try_into_u128(&self) -> Result<u128, Self::Error> {
//        Ok((n as u128).wrapping_add(9_223_372_036_854_775_808))
//    }
//}
//
//impl TryFromByAdd for u64 {
//    type Error = &'static str;
//
//    /// Converts the value of `i8` to an `u64`.
//    #[inline]
//    fn try_into_u64(&self) -> Result<u64, Self::Error> {
//        Ok((n as u64).wrapping_add(128))
//    }
//
//    /// Converts the value of `u64` to an `u8`.
//    #[inline]
//    fn try_into_u8(&self) -> Result<u8, Self::Error> {
//        if n <= 255 {
//            Ok(n as u8)
//        } else {
//            Err("Cannot be converted to type u8")
//        }
//    }
//
//    /// Converts the value of `u64` to an `i16`.
//    #[inline]
//    fn try_into_i16(&self) -> Result<i16, Self::Error> {
//        if n <= 65_535 {
//            Ok(((n as i16).wrapping_add(i16::MAX)).wrapping_add(1))
//        } else {
//            Err("Cannot be converted to type i16")
//        }
//    }
//
//    /// Converts the value of `u64` to an `u16`.
//    #[inline]
//    fn try_into_u16(&self) -> Result<u16, Self::Error> {
//        if n <= 65_535 {
//            Ok(n as u16)
//        } else {
//            Err("Cannot be converted to type u16")
//        }
//    }
//
//    /// Converts the value of `u64` to an `i32`.
//    #[inline]
//    fn try_into_i32(&self) -> Result<i32, Self::Error> {
//        if n <= 4_294_967_295 {
//            Ok(((n as i32).wrapping_add(i32::MAX)).wrapping_add(1))
//        } else {
//            Err("Cannot be converted to type i32")
//        }
//    }
//
//    /// Converts the value of `u64` to an `u32`.
//    #[inline]
//    fn try_into_u32(&self) -> Result<u32, Self::Error> {
//        if n <= 4_294_967_295 {
//            Ok(n as u32)
//        } else {
//            Err("Cannot be converted to type u32")
//        }
//    }
//
//    /// Converts the value of `u64` to an `i64`.
//    #[inline]
//    fn try_into_i64(&self) -> Result<i64, Self::Error> {
//        Ok(((n as i64).wrapping_add(i64::MAX)).wrapping_add(1))
//    }
//
//    /// Returns an `u64` for compatibility.
//    #[inline]
//    fn try_into_u64(&self) -> Result<u64, Self::Error> {
//        Ok(n)
//    }
//
//    /// Converts the value of `u64` to an `isize`.
//    #[inline]
//    fn try_into_isize(&self) -> Result<isize, Self::Error> {
//        Ok(((n as isize).wrapping_add(isize::MAX)).wrapping_add(1))
//    }
//
//    /// Converts the value of `u64` to an `usize`.
//    #[inline]
//    fn try_into_usize(&self) -> Result<usize, Self::Error> {
//        Ok(n as usize)
//    }
//
//    /// Converts the value of `u64` to an `i128`.
//    #[inline]
//    fn try_into_i128(&self) -> Result<i128, Self::Error> {
//        Ok(((n as i64).wrapping_add(i64::MAX)).wrapping_add(1) as i128)
//    }
//
//    /// Converts the value of `u64` to an `u128`.
//    #[inline]
//    fn try_into_u128(&self) -> Result<u128, Self::Error> {
//        Ok(n as u128)
//    }
//}
//
//impl TryFromByAdd for isize {
//    type Error = &'static str;
//
//    /// Converts the value of `i8` to an `isize`.
//    #[inline]
//    fn try_into_isize(&self) -> Result<isize, Self::Error> {
//        Ok(n as isize)
//    }
//
//    /// Converts the value of `isize` to an `u8`.
//    #[inline]
//    fn try_into_u8(&self) -> Result<u8, Self::Error> {
//        if n >= -128 && n <= 127 {
//            Ok((n as u8).wrapping_add(128))
//        } else {
//            Err("Cannot be converted to type u8")
//        }
//    }
//
//    /// Converts the value of `isize` to an `i16`.
//    #[inline]
//    fn try_into_i16(&self) -> Result<i16, Self::Error> {
//        if n >= -32_768 && n <= 32_767 {
//            Ok(n as i16)
//        } else {
//            Err("Cannot be converted to type i16")
//        }
//    }
//
//    /// Converts the value of `isize` to an `u16`.
//    #[inline]
//    fn try_into_u16(&self) -> Result<u16, Self::Error> {
//        if n >= -32_768 && n <= 32_767 {
//            Ok((n as u16).wrapping_add(32_768))
//        } else {
//            Err("Cannot be converted to type u16")
//        }
//    }
//
//    /// Converts the value of `isize` to an `i32`.
//    #[inline]
//    fn try_into_i32(&self) -> Result<i32, Self::Error> {
//        if n >= -2_147_483_648 && n <= 2_147_483_647 {
//            Ok(n as i32)
//        } else {
//            Err("Cannot be converted to type i32")
//        }
//    }
//
//    /// Converts the value of `isize` to an `u32`.
//    #[inline]
//    fn try_into_u32(&self) -> Result<u32, Self::Error> {
//        if n >= -2_147_483_648 && n <= 2_147_483_647 {
//            Ok((n as u32).wrapping_add(2_147_483_648))
//        } else {
//            Err("Cannot be converted to type u32")
//        }
//    }
//
//    /// Converts the value of `isize` to an `i64`.
//    #[inline]
//    fn try_into_i64(&self) -> Result<i64, Self::Error> {
//        Ok(n as i64)
//    }
//
//    /// Converts the value of `isize` to an `u64`.
//    #[inline]
//    fn try_into_u64(&self) -> Result<u64, Self::Error> {
//        Ok((n as u64).wrapping_add(9_223_372_036_854_775_808))
//    }
//
//    /// Returns an `isize` for compatibility.
//    #[inline]
//    fn try_into_isize(&self) -> Result<isize, Self::Error> {
//        Ok(n)
//    }
//
//    /// Converts the value of `isize` to an `usize`.
//    #[inline]
//    fn try_into_usize(&self) -> Result<usize, Self::Error> {
//        Ok((n as usize).wrapping_add(9_223_372_036_854_775_808))
//    }
//
//    /// Converts the value of `isize` to an `i128`.
//    #[inline]
//    fn try_into_i128(&self) -> Result<i128, Self::Error> {
//        Ok(n as i128)
//    }
//
//    /// Converts the value of `isize` to an `u128`.
//    #[inline]
//    fn try_into_u128(&self) -> Result<u128, Self::Error> {
//        Ok((n as u128).wrapping_add(9_223_372_036_854_775_808))
//    }
//}
//
//impl TryFromByAdd for usize {
//    type Error = &'static str;
//
//    /// Converts the value of `i8` to an `usize`.
//    #[inline]
//    fn try_into_usize(&self) -> Result<usize, Self::Error> {
//        Ok((n as usize).wrapping_add(128))
//    }
//
//    /// Converts the value of `usize` to an `u8`.
//    #[inline]
//    fn try_into_u8(&self) -> Result<u8, Self::Error> {
//        if n <= 255 {
//            Ok(n as u8)
//        } else {
//            Err("Cannot be converted to type u8")
//        }
//    }
//
//    /// Converts the value of `usize` to an `i16`.
//    #[inline]
//    fn try_into_i16(&self) -> Result<i16, Self::Error> {
//        if n <= 65_535 {
//            Ok(((n as i16).wrapping_add(i16::MAX)).wrapping_add(1))
//        } else {
//            Err("Cannot be converted to type i16")
//        }
//    }
//
//    /// Converts the value of `usize` to an `u16`.
//    #[inline]
//    fn try_into_u16(&self) -> Result<u16, Self::Error> {
//        if n <= 65_535 {
//            Ok(n as u16)
//        } else {
//            Err("Cannot be converted to type u16")
//        }
//    }
//
//    /// Converts the value of `usize` to an `i32`.
//    #[inline]
//    fn try_into_i32(&self) -> Result<i32, Self::Error> {
//        if n <= 4_294_967_295 {
//            Ok(((n as i32).wrapping_add(i32::MAX)).wrapping_add(1))
//        } else {
//            Err("Cannot be converted to type i32")
//        }
//    }
//
//    /// Converts the value of `usize` to an `u32`.
//    #[inline]
//    fn try_into_u32(&self) -> Result<u32, Self::Error> {
//        if n <= 4_294_967_295 {
//            Ok(n as u32)
//        } else {
//            Err("Cannot be converted to type u32")
//        }
//    }
//
//    /// Converts the value of `usize` to an `i64`.
//    #[inline]
//    fn try_into_i64(&self) -> Result<i64, Self::Error> {
//        Ok(((n as i64).wrapping_add(i64::MAX)).wrapping_add(1))
//    }
//
//    /// Converts the value of `usize` to an `u64`.
//    #[inline]
//    fn try_into_u64(&self) -> Result<u64, Self::Error> {
//        Ok(n as u64)
//    }
//
//    /// Converts the value of `usize` to an `isize`.
//    #[inline]
//    fn try_into_isize(&self) -> Result<isize, Self::Error> {
//        Ok(((n as isize).wrapping_add(isize::MAX)).wrapping_add(1))
//    }
//
//    /// Returns an `usize` for compatibility.
//    #[inline]
//    fn try_into_usize(&self) -> Result<usize, Self::Error> {
//        Ok(n)
//    }
//
//    /// Converts the value of `usize` to an `i128`.
//    #[inline]
//    fn try_into_i128(&self) -> Result<i128, Self::Error> {
//        Ok(((n as isize).wrapping_add(isize::MAX)).wrapping_add(1) as i128)
//    }
//
//    /// Converts the value of `usize` to an `u128`.
//    #[inline]
//    fn try_into_u128(&self) -> Result<u128, Self::Error> {
//        Ok(n as u128)
//    }
//}
//
//impl TryFromByAdd for i128 {
//    type Error = &'static str;
//    /// Converts the value of `i8` to an `i128`.
//    #[inline]
//    fn try_into_i128(&self) -> Result<i128, Self::Error> {
//        Ok(n as i128)
//    }
//
//    /// Converts the value of `i128` to an `u8`.
//    #[inline]
//    fn try_into_u8(&self) -> Result<u8, Self::Error> {
//        if n >= -128 && n <= 127 {
//            Ok((n as u8).wrapping_add(128))
//        } else {
//            Err("Cannot be converted to type u8")
//        }
//    }
//
//    /// Converts the value of `i128` to an `i16`.
//    #[inline]
//    fn try_into_i16(&self) -> Result<i16, Self::Error> {
//        if n >= -32_768 && n <= 32_767 {
//            Ok(n as i16)
//        } else {
//            Err("Cannot be converted to type i16")
//        }
//    }
//
//    /// Converts the value of `i128` to an `u16`.
//    #[inline]
//    fn try_into_u16(&self) -> Result<u16, Self::Error> {
//        if n >= -32_768 && n <= 32_767 {
//            Ok((n as u16).wrapping_add(32_768))
//        } else {
//            Err("Cannot be converted to type u16")
//        }
//    }
//
//    /// Converts the value of `i128` to an `i32`.
//    #[inline]
//    fn try_into_i32(&self) -> Result<i32, Self::Error> {
//        if n >= -2_147_483_648 && n <= 2_147_483_647 {
//            Ok(n as i32)
//        } else {
//            Err("Cannot be converted to type i32")
//        }
//    }
//
//    /// Converts the value of `i128` to an `u32`.
//    #[inline]
//    fn try_into_u32(&self) -> Result<u32, Self::Error> {
//        if n >= -2_147_483_648 && n <= 2_147_483_647 {
//            Ok((n as u32).wrapping_add(2_147_483_648))
//        } else {
//            Err("Cannot be converted to type u32")
//        }
//    }
//
//    /// Converts the value of `i128` to an `i64`.
//    #[inline]
//    fn try_into_i64(&self) -> Result<i64, Self::Error> {
//        if n >= -9_223_372_036_854_775_808 && n <= 9_223_372_036_854_775_807 {
//            Ok(n as i64)
//        } else {
//            Err("Cannot be converted to type i64")
//        }
//    }
//
//    /// Converts the value of `i128` to an `u64`.
//    #[inline]
//    fn try_into_u64(&self) -> Result<u64, Self::Error> {
//        if n >= -9_223_372_036_854_775_808 && n <= 9_223_372_036_854_775_807 {
//            Ok((n as u64).wrapping_add(9_223_372_036_854_775_808))
//        } else {
//            Err("Cannot be converted to type u64")
//        }
//    }
//
//    /// Converts the value of `i128` to an `isize`.
//    #[inline]
//    fn try_into_isize(&self) -> Result<isize, Self::Error> {
//        if n >= -9_223_372_036_854_775_808 && n <= 9_223_372_036_854_775_807 {
//            Ok(n as isize)
//        } else {
//            Err("Cannot be converted to type isize")
//        }
//    }
//
//    /// Converts the value of `i128` to an `usize`.
//    #[inline]
//    fn try_into_usize(&self) -> Result<usize, Self::Error> {
//        if n >= -9_223_372_036_854_775_808 && n <= 9_223_372_036_854_775_807 {
//            Ok((n as usize).wrapping_add(9_223_372_036_854_775_808))
//        } else {
//            Err("Cannot be converted to type usize")
//        }
//    }
//
//    /// Returns an `i128` for compatibility.
//    #[inline]
//    fn try_into_i128(&self) -> Result<i128, Self::Error> {
//        Ok(n)
//    }
//
//    /// Converts the value of `i128` to an `u128`.
//    #[inline]
//    fn try_into_u128(&self) -> Result<u128, Self::Error> {
//        Ok((n as u128).wrapping_add(170_141_183_460_469_231_731_687_303_715_884_105_728))
//    }
//}
//
//impl TryFromByAdd for u128 {
//    type Error = &'static str;
//
//    /// Converts the value of `i8` to an `u128`.
//    #[inline]
//    fn try_into_u128(&self) -> Result<u128, Self::Error> {
//        Ok((n as u128).wrapping_add(128))
//    }
//
//    /// Converts the value of `u128` to an `u8`.
//    #[inline]
//    fn try_into_u8(&self) -> Result<u8, Self::Error> {
//        if n <= 255 {
//            Ok(n as u8)
//        } else {
//            Err("Cannot be converted to type u8")
//        }
//    }
//
//    /// Converts the value of `u128` to an `i16`.
//    #[inline]
//    fn try_into_i16(&self) -> Result<i16, Self::Error> {
//        if n <= 65_535 {
//            Ok(((n as i16).wrapping_add(i16::MAX)).wrapping_add(1))
//        } else {
//            Err("Cannot be converted to type i16")
//        }
//    }
//
//    /// Converts the value of `u128` to an `u16`.
//    #[inline]
//    fn try_into_u16(&self) -> Result<u16, Self::Error> {
//        if n <= 65_535 {
//            Ok(n as u16)
//        } else {
//            Err("Cannot be converted to type u16")
//        }
//    }
//
//    /// Converts the value of `u128` to an `i32`.
//    #[inline]
//    fn try_into_i32(&self) -> Result<i32, Self::Error> {
//        if n <= 4_294_967_295 {
//            Ok(((n as i32).wrapping_add(i32::MAX)).wrapping_add(1))
//        } else {
//            Err("Cannot be converted to type i32")
//        }
//    }
//
//    /// Converts the value of `u128` to an `u32`.
//    #[inline]
//    fn try_into_u32(&self) -> Result<u32, Self::Error> {
//        if n <= 4_294_967_295 {
//            Ok(n as u32)
//        } else {
//            Err("Cannot be converted to type u32")
//        }
//    }
//
//    /// Converts the value of `u128` to an `i64`.
//    #[inline]
//    fn try_into_i64(&self) -> Result<i64, Self::Error> {
//        if n <= 18_446_744_073_709_551_615 {
//            Ok(((n as i64).wrapping_add(i64::MAX)).wrapping_add(1))
//        } else {
//            Err("Cannot be converted to type i64")
//        }
//    }
//
//    /// Converts the value of `u128` to an `u64`.
//    #[inline]
//    fn try_into_u64(&self) -> Result<u64, Self::Error> {
//        if n <= 18_446_744_073_709_551_615 {
//            Ok(n as u64)
//        } else {
//            Err("Cannot be converted to type u64")
//        }
//    }
//
//    /// Converts the value of `u128` to an `isize`.
//    #[inline]
//    fn try_into_isize(&self) -> Result<isize, Self::Error> {
//        if n <= 18_446_744_073_709_551_615 {
//            Ok(((n as isize).wrapping_add(isize::MAX)).wrapping_add(1))
//        } else {
//            Err("Cannot be converted to type isize")
//        }
//    }
//
//    /// Converts the value of `u128` to an `usize`.
//    #[inline]
//    fn try_into_usize(&self) -> Result<usize, Self::Error> {
//        if n <= 18_446_744_073_709_551_615 {
//            Ok(n as usize)
//        } else {
//            Err("Cannot be converted to type usize")
//        }
//    }
//
//    /// Converts the value of `u128` to an `i128`.
//    #[inline]
//    fn try_into_i128(&self) -> Result<i128, Self::Error> {
//        Ok(((n as i128).wrapping_add(i128::MAX)).wrapping_add(1))
//    }
//
//    /// Returns an `u128` for compatibility.
//    #[inline]
//    fn try_into_u128(&self) -> Result<u128, Self::Error> {
//        Ok(n)
//    }
//}
