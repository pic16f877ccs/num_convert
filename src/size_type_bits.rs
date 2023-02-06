/// Generic trait for define the size of the integers type in bits.
///
/// # Examples
/// Usage:
///
/// ```
/// # use num_convert::Tbits;
/// fn get_bits_from_type<T: Tbits>(value: T) -> u32 {
///     T::tbits()
/// }
/// ```
pub trait Tbits {
    /// Associated function returns the size of integer type in bits.
    fn tbits() -> u32;
}

macro_rules! tbits_impls {
    ($($value_type:ty),*) => {
        $(
            impl Tbits for $value_type {
                #[inline]
                fn tbits() -> u32 {
                    <$value_type>::BITS
                }
            }
        )*
    }
}

tbits_impls! { i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, i128, u128 }

/// The Sbit trait for define the size of integer value in bits.
///
/// # Examples
/// Usage:
///
/// ```
/// # use num_convert::Sbits;
/// fn get_bits_from_value<T: Sbits>(value: T) -> u32 {
///     value.sbits()
/// }
///
/// assert_eq!(get_bits_from_value(-128_i8), 8_u32);
/// assert_eq!(get_bits_from_value(65535_u16), 16_u32);
/// ```
pub trait Sbits {
    /// Method returns the size of integers value in bits.
    fn sbits(&self) -> u32;
}

macro_rules! sbits_impls {
    ($($type:ty),*) => {
        $(
            impl Sbits for $type {
                #[inline]
                fn sbits(&self) -> u32 {
                    <$type>::BITS
                }
            }
        )*
    }
}

sbits_impls! { i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, i128, u128 }
