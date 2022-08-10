use core::{i16, i32, i64, i8, isize};
use core::{u16, u32, u64, u8, usize};

/// Const for adding `usize` 32 bit.
#[cfg(target_pointer_width = "32")]
const ADD_VALUE_USIZE: usize = 2_147_483_648;

/// Const for adding `usize` 64 bit.
#[cfg(target_pointer_width = "64")]
const ADD_VALUE_USIZE: usize = 9_223_372_036_854_775_808;

pub trait TryFromByAdd: Sized {
    type Error;
    fn from_i8(n: i8) -> Result<Self, Self::Error>;
    fn from_u8(n: u8) -> Result<Self, Self::Error>;
    //fn from_i16(n: i16) -> Result<Self, Self::Error>;
    fn from_u16(n: u16) -> Result<Self, Self::Error>;
    //fn from_i32(n: i32) -> Result<Self, Self::Error>;
    fn from_u32(n: u32) -> Result<Self, Self::Error>;
    //fn from_i64(n: i64) -> Result<Self, Self::Error>;
    fn from_u64(n: u64) -> Result<Self, Self::Error>;
    //fn from_isize(n: isize) -> Result<Self, Self::Error>;
    fn from_usize(n: usize) -> Result<Self, Self::Error>;
}

impl TryFromByAdd for i8 {
    type Error = &'static str;

    fn from_i8(n: i8) -> Result<i8, Self::Error> {
        Ok(n)
    }

    fn from_u8(n: u8) -> Result<i8, Self::Error> {
        Ok(((n as i8).wrapping_add(i8::MAX)).wrapping_add(1))
    }

    fn from_u16(n: u16) -> Result<i8, Self::Error> {
        if n <= 255 {
            Ok(((n as i8).wrapping_add(i8::MAX)).wrapping_add(1))
        } else {
            Err("Cannot be converted to type i8")
        }
    }

    fn from_u32(n: u32) -> Result<i8, Self::Error> {
        if n <= 255 {
            Ok(((n as i8).wrapping_add(i8::MAX)).wrapping_add(1))
        } else {
            Err("Cannot be converted to type i8")
        }
    }

    fn from_u64(n: u64) -> Result<i8, Self::Error> {
        if n <= 255 {
            Ok(((n as i8).wrapping_add(i8::MAX)).wrapping_add(1))
        } else {
            Err("Cannot be converted to type i8")
        }
    }

    fn from_usize(n: usize) -> Result<i8, Self::Error> {
        if n <= 255 {
            Ok(((n as i8).wrapping_add(i8::MAX)).wrapping_add(1))
        } else {
            Err("Cannot be converted to type i8")
        }
    }
}

impl TryFromByAdd for u8 {
    type Error = &'static str;

    fn from_i8(n: i8) -> Result<u8, Self::Error> {
        Ok((n as u8).wrapping_add(128))
    }

    fn from_u8(n: u8) -> Result<u8, Self::Error> {
        Ok(n)
    }

    fn from_u16(n: u16) -> Result<u8, Self::Error> {
        if n <= 255 {
            Ok(n as u8)
        } else {
            Err("Cannot be converted to type u8")
        }
    }

    fn from_u32(n: u32) -> Result<u8, Self::Error> {
        if n <= 255 {
            Ok(n as u8)
        } else {
            Err("Cannot be converted to type u8")
        }
    }

    fn from_u64(n: u64) -> Result<u8, Self::Error> {
        if n <= 255 {
            Ok(n as u8)
        } else {
            Err("Cannot be converted to type u8")
        }
    }

    fn from_usize(n: usize) -> Result<u8, Self::Error> {
        if n <= 255 {
            Ok(n as u8)
        } else {
            Err("Cannot be converted to type u8")
        }
    }
}

impl TryFromByAdd for i16 {
    type Error = &'static str;

    fn from_i8(n: i8) -> Result<i16, Self::Error> {
        Ok(n as i16)
    }

    fn from_u8(n: u8) -> Result<i16, Self::Error> {
        Ok(((n as i8).wrapping_add(i8::MAX)).wrapping_add(1) as i16)
    }

    fn from_u16(n: u16) -> Result<i16, Self::Error> {
        Ok(((n as i16).wrapping_add(32_767)).wrapping_add(1))
    }

    fn from_u32(n: u32) -> Result<i16, Self::Error> {
        if n <= 65_535 {
            Ok(((n as i16).wrapping_add(i16::MAX)).wrapping_add(1))
        } else {
            Err("Cannot be converted to type i16")
        }
    }

    fn from_u64(n: u64) -> Result<i16, Self::Error> {
        if n <= 65_535 {
            Ok(((n as i16).wrapping_add(i16::MAX)).wrapping_add(1))
        } else {
            Err("Cannot be converted to type i16")
        }
    }

    fn from_usize(n: usize) -> Result<i16, Self::Error> {
        if n <= 65_535 {
            Ok(((n as i16).wrapping_add(i16::MAX)).wrapping_add(1))
        } else {
            Err("Cannot be converted to type i16")
        }
    }
}

impl TryFromByAdd for u16 {
    type Error = &'static str;

    fn from_i8(n: i8) -> Result<u16, Self::Error> {
        Ok((n as u16).wrapping_add(128))
    }

    fn from_u8(n: u8) -> Result<u16, Self::Error> {
        Ok(n as u16)
    }

    fn from_u16(n: u16) -> Result<u16, Self::Error> {
        Ok(n)
    }

    fn from_u32(n: u32) -> Result<u16, Self::Error> {
        if n <= 65_535 {
            Ok(n as u16)
        } else {
            Err("Cannot be converted to type u16")
        }
    }

    fn from_u64(n: u64) -> Result<u16, Self::Error> {
        if n <= 65_535 {
            Ok(n as u16)
        } else {
            Err("Cannot be converted to type u16")
        }
    }

    fn from_usize(n: usize) -> Result<u16, Self::Error> {
        if n <= 65_535 {
            Ok(n as u16)
        } else {
            Err("Cannot be converted to type u16")
        }
    }
}

impl TryFromByAdd for i32 {
    type Error = &'static str;

    fn from_i8(n: i8) -> Result<i32, Self::Error> {
        Ok(n as i32)
    }

    fn from_u8(n: u8) -> Result<i32, Self::Error> {
        Ok(((n as i8).wrapping_add(i8::MAX)).wrapping_add(1) as i32)
    }

    fn from_u16(n: u16) -> Result<i32, Self::Error> {
        Ok(((n as i16).wrapping_add(i16::MAX)).wrapping_add(1) as i32)
    }

    fn from_u32(n: u32) -> Result<i32, Self::Error> {
        Ok(((n as i32).wrapping_add(i32::MAX)).wrapping_add(1))
    }

    fn from_u64(n: u64) -> Result<i32, Self::Error> {
        if n <= 4_294_967_295 {
            Ok(((n as i32).wrapping_add(i32::MAX)).wrapping_add(1))
        } else {
            Err("Cannot be converted to type i32")
        }
    }

    // fixme
    /// Target `usize` 32 bit.
    #[cfg(target_pointer_width = "32")]
    fn from_usize(n: usize) -> Result<i32, Self::Error> {
        Ok(((n as i32).wrapping_add(i32::MAX)).wrapping_add(1))
    }

    // fixme
    /// Target `usize` 64 bit.
    #[cfg(target_pointer_width = "64")]
    fn from_usize(n: usize) -> Result<i32, Self::Error> {
        if n <= 4_294_967_295 {
            Ok(((n as i32).wrapping_add(i32::MAX)).wrapping_add(1))
        } else {
            Err("Cannot be converted to type i32")
        }
    }
}

impl TryFromByAdd for u32 {
    type Error = &'static str;

    fn from_i8(n: i8) -> Result<u32, Self::Error> {
        Ok((n as u32).wrapping_add(128))
    }

    fn from_u8(n: u8) -> Result<u32, Self::Error> {
        Ok(n as u32)
    }

    fn from_u16(n: u16) -> Result<u32, Self::Error> {
        Ok(n as u32)
    }

    fn from_u32(n: u32) -> Result<u32, Self::Error> {
        Ok(n)
    }

    fn from_u64(n: u64) -> Result<u32, Self::Error> {
        if n <= 4_294_967_295 {
            Ok(n as u32)
        } else {
            Err("Cannot be converted to type u32")
        }
    }

    fn from_usize(n: usize) -> Result<u32, Self::Error> {
        if n <= 4_294_967_295 {
            Ok(n as u32)
        } else {
            Err("Cannot be converted to type u32")
        }
    }
}

impl TryFromByAdd for i64 {
    type Error = &'static str;

    fn from_i8(n: i8) -> Result<i64, Self::Error> {
        Ok(n as i64)
    }

    fn from_u8(n: u8) -> Result<i64, Self::Error> {
        Ok(((n as i8).wrapping_add(i8::MAX)).wrapping_add(1) as i64)
    }

    fn from_u16(n: u16) -> Result<i64, Self::Error> {
        Ok(((n as i16).wrapping_add(i16::MAX)).wrapping_add(1) as i64)
    }

    fn from_u32(n: u32) -> Result<i64, Self::Error> {
        Ok(((n as i32).wrapping_add(i32::MAX)).wrapping_add(1) as i64)
    }

    fn from_u64(n: u64) -> Result<i64, Self::Error> {
        Ok(((n as i64).wrapping_add(i64::MAX)).wrapping_add(1))
    }

    // fixme
    /// Target `usize` 32 bit.
    #[cfg(target_pointer_width = "32")]
    fn from_usize(n: usize) -> Result<i64, Self::Error> {
        if n <= 4_294_967_295 {
            Ok(((n as i64).wrapping_add(i64::MAX)).wrapping_add(1))
        } else {
            Err("Cannot be converted to type i64")
        }
    }

    // fixme
    /// Target `usize` 64 bit.
    #[cfg(target_pointer_width = "64")]
    fn from_usize(n: usize) -> Result<i64, Self::Error> {
        Ok(((n as i64).wrapping_add(i64::MAX)).wrapping_add(1))
    }
}

impl TryFromByAdd for u64 {
    type Error = &'static str;

    fn from_i8(n: i8) -> Result<u64, Self::Error> {
        Ok((n as u64).wrapping_add(128))
    }

    fn from_u8(n: u8) -> Result<u64, Self::Error> {
        Ok(n as u64)
    }

    fn from_u16(n: u16) -> Result<u64, Self::Error> {
        Ok(n as u64)
    }

    fn from_u32(n: u32) -> Result<u64, Self::Error> {
        Ok(n as u64)
    }

    fn from_u64(n: u64) -> Result<u64, Self::Error> {
        Ok(n)
    }

    fn from_usize(n: usize) -> Result<u64, Self::Error> {
        Ok(n as u64)
    }
}

impl TryFromByAdd for isize {
    type Error = &'static str;

    fn from_i8(n: i8) -> Result<isize, Self::Error> {
        Ok(n as isize)
    }

    fn from_u8(n: u8) -> Result<isize, Self::Error> {
        Ok(((n as i8).wrapping_add(i8::MAX)).wrapping_add(1) as isize)
    }

    fn from_u16(n: u16) -> Result<isize, Self::Error> {
        Ok(((n as i16).wrapping_add(i16::MAX)).wrapping_add(1) as isize)
    }

    fn from_u32(n: u32) -> Result<isize, Self::Error> {
        Ok(((n as i32).wrapping_add(i32::MAX)).wrapping_add(1) as isize)
    }
    // fixme
    /// Target `usize` 32 bit.
    #[cfg(target_pointer_width = "32")]
    fn from_usize(n: u64) -> Result<isize, Self::Error> {
        if n <= 4_294_967_295 {
            Ok(((n as isize).wrapping_add(isize::MAX)).wrapping_add(1))
        } else {
            Err("Cannot be converted to type isize")
        }
    }

    // fixme
    /// Target `usize` 64 bit.
    #[cfg(target_pointer_width = "64")]
    fn from_u64(n: u64) -> Result<isize, Self::Error> {
        Ok(((n as isize).wrapping_add(isize::MAX)).wrapping_add(1))
    }

    fn from_usize(n: usize) -> Result<isize, Self::Error> {
        Ok(((n as isize).wrapping_add(isize::MAX)).wrapping_add(1))
    }
}

impl TryFromByAdd for usize {
    type Error = &'static str;

    fn from_i8(n: i8) -> Result<usize, Self::Error> {
        Ok((n as usize).wrapping_add(128))
    }

    fn from_u8(n: u8) -> Result<usize, Self::Error> {
        Ok(n as usize)
    }

    fn from_u16(n: u16) -> Result<usize, Self::Error> {
        Ok(n as usize)
    }

    fn from_u32(n: u32) -> Result<usize, Self::Error> {
        Ok(n as usize)
    }

    fn from_u64(n: u64) -> Result<usize, Self::Error> {
        Ok(n as usize)
    }

    fn from_usize(n: usize) -> Result<usize, Self::Error> {
        Ok(n)
    }
}
