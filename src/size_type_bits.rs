/// Generic trait to define the size of the integers type in bits.
///
/// # Usage
/// Basic use of the trait.
///
/// ```
/// use num_convert::Tbits;
///
/// assert_eq!(i8::tbits(), 8_u32);
/// assert_eq!(u64::tbits(), 64_u32);
/// ```
///
/// # Examples
///
/// ```
/// # use num_convert::Tbits;
/// fn get_bits_from_type<T: Tbits>(value: T) -> u32 {
///     T::tbits()
/// }
///
/// assert_eq!(get_bits_from_type(-128_i8), 8_u32);
/// assert_eq!(get_bits_from_type(isize::MIN), 64_u32);
/// ```
pub trait Tbits {
    /// Returns the size of the integer type in bits.
    fn tbits() -> u32;
}

macro_rules! tbits_impls {
    ($($value_type:ty),*) => {
        $(
            impl Tbits for $value_type {
                #[doc = concat!("The size of ", stringify!($value_type), " integer type in bits.")]
                #[inline]
                fn tbits() -> u32 {
                    <$value_type>::BITS
                }
            }
        )*
    }
}

tbits_impls! { i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, i128, u128 }

/// The Sbit trait to define the size of integer value in bits.
///
/// # Usage
/// Basic use of the trait.
///
/// ```
/// use num_convert::Sbits;
///
/// assert_eq!(127_i8.sbits(), 8_u32);
/// assert_eq!(0_u64.sbits(), 64_u32);
/// ```
///
/// # Examples
///
/// ```
/// # use num_convert::Sbits;
/// fn get_bits_from_value<T: Sbits>(value: T) -> u32 {
///     value.sbits()
/// }
///
/// assert_eq!(get_bits_from_value(65_535_u16), 16_u32);
/// assert_eq!(get_bits_from_value(u128::MAX), 128_u32);
/// ```
pub trait Sbits {
    /// Returns the size of the integer value in bits.
    fn sbits(&self) -> u32;
}

macro_rules! sbits_impls {
    ($($value_type:ty),*) => {
        $(
            impl Sbits for $value_type {
                #[doc = concat!("The size of ", stringify!($value_type), " integer value in bits.")]
                #[inline]
                fn sbits(&self) -> u32 {
                    <$value_type>::BITS
                }
            }
        )*
    }
}

sbits_impls! { i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, i128, u128 }
