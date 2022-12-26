/// ## Examples
///
/// ```
/// use num_convert::{ Tbits, Sbits };
///
/// fn get_bits<T: Tbits + Sbits>(value: T) -> u32 {
///     T::tbits() + value.sbits()
/// }
///
/// ```

macro_rules! bits_extra {
    ($trait_name:ident; $($value_type:ty),*) => {
        pub trait $trait_name {
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

pub trait Sbits {
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
