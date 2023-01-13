/// Generic traits for define the size of the integers in bits.
/// # Examples
/// Usage:
///
/// ```
/// # use num_convert::{ Tbits, Sbits };
/// fn get_bits<T: Tbits + Sbits>(value: T) -> u32 {
///     assert_eq!(T::tbits(), value.sbits());
///     T::tbits() + value.sbits()
/// }
///
/// ```

macro_rules! bits_extra {
    ($trait_name:ident; $($value_type:ty),*) => {
        /// The Tbit trait for define the size of integer type in bits.
        pub trait $trait_name {
            /// Associated function returns the size of integer type in bits.
            fn tbits() -> u32;
        }

        $(
            impl $trait_name for $value_type {
                #[inline]
                fn tbits() -> u32 {
                    <$value_type>::BITS
                }
            }
        )*
    }
}

bits_extra! { Tbits; i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, i128, u128 }

/// The Sbit trait for define the size of integer value in bits.
pub trait Sbits {
    /// Method returns the size of integers value in bits.
    fn sbits(&self) -> u32;
}

macro_rules! sbits_impls {
    ($($type:ty),*) => {
        $(
            impl Sbits for $type {
                fn sbits(&self) -> u32 {
                    <$type>::BITS
                }
            }
        )*
    }
}

sbits_impls! { i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, i128, u128 }
