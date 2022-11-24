/// ## Examples
///
/// ```
/// use num_convert::Ebits;
///
/// fn get_bits<T: Ebits>(value: T) -> u32 {
///     let bits = T::EBITS;
///     T::get_bits()
/// }
///
/// ```
macro_rules! bits_extra {
    ($trait_name:ident; $($value_type:ty, $value:expr);*) => {
        pub trait $trait_name {
            const EBITS: u32;
            fn get_bits() -> u32;
        }

        $(
            impl $trait_name for $value_type {
                const EBITS: u32 = $value;
                fn get_bits() -> u32 {
                    $value
                }
            }
        )*
    }
}

bits_extra! { Ebits; i8, 8; u8, 8; i16, 16; u16, 16; i32, 32; u32, 32;
i64, 64; u64, 64; isize, 64; usize, 64; i128, 128; u128, 128 }

