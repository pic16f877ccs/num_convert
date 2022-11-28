/// ## Examples
///
/// ```
/// use num_convert::Tbits;
///
/// fn get_bits<T: Tbits>(value: T) -> u32 {
///     T::get_bits()
/// }
///
/// ```
macro_rules! bits_extra {
    ($trait_name:ident; $($value_type:ty, $value:expr);*) => {
        pub trait $trait_name {
            fn get_bits() -> u32;
        }

        $(
            impl $trait_name for $value_type {
                #[inline]
                fn get_bits() -> u32 {
                    $value
                }
            }
        )*
    }
}

bits_extra! { Tbits; i8, 8; u8, 8; i16, 16; u16, 16; i32, 32; u32, 32;
i64, 64; u64, 64; isize, 64; usize, 64; i128, 128; u128, 128 }

