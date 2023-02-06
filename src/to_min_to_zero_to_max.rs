use core::cmp::PartialEq;

/// Trait to implement lower bounds on types.
pub trait ToMin<T>: Sized + PartialEq + Copy {
    /// Returns lower bounds of types.
    fn to_min() -> T;
}

/// Trait to implement upper bounds on types.
pub trait ToMax<T>: Sized + PartialEq + Copy {
    /// Returns upper bounds of types.
    fn to_max() -> T;
}

/// Trait for implementing the null value of types.
pub trait ToZero<T>: Sized + PartialEq + Copy {
    /// Returns zero of types.
    fn to_zero() -> T;
}

macro_rules! min_zero_max_impl {
    ( $trait_name:ident, $fn_name:ident, $for_type:ty; $($value_type:ty),*; $value:expr) => {
        $(
            impl $trait_name<$value_type> for $for_type {
                #[inline]
                fn $fn_name() -> $value_type {
                    $value
                }
            }
        )*
    }
}

min_zero_max_impl! { ToMin, to_min, i8; i8, i16, i32, i64, isize, i128; -128 }
min_zero_max_impl! { ToMin, to_min, i16; i16, i32, i64, isize, i128; -32_768 }
min_zero_max_impl! { ToMin, to_min, i32; i32, i64, isize, i128; -2_147_483_648 }
min_zero_max_impl! { ToMin, to_min, i64; i64, isize, i128; -9_223_372_036_854_775_808 }
min_zero_max_impl! { ToMin, to_min, isize; isize, i64, i128; -9_223_372_036_854_775_808 }
min_zero_max_impl! { ToMin, to_min, i128; i128; i128::MIN }

min_zero_max_impl! { ToMax, to_max, i8; i8, i16, i32, i64, isize, i128, u8, u16, u32, u64, usize, u128; 127 }
min_zero_max_impl! { ToMax, to_max, i16; i16, i32, i64, isize, i128, u16, u32, u64, usize, u128; 32_767 }
min_zero_max_impl! { ToMax, to_max, i32; i32, i64, isize, i128, u32, u64, usize, u128; 2_147_483_647 }
min_zero_max_impl! { ToMax, to_max, i64; i64, isize, i128, u64, usize, u128; 9_223_372_036_854_775_807 }
min_zero_max_impl! { ToMax, to_max, isize; i64, isize, i128, u64, usize, u128; 9_223_372_036_854_775_807 }
min_zero_max_impl! { ToMax, to_max, i128; i128, u128; 170_141_183_460_469_231_731_687_303_715_884_105_727 }

min_zero_max_impl! { ToMax, to_max, u8; i16, i32, i64, isize, i128, u8, u16, u32, u64, usize, u128; 255 }
min_zero_max_impl! { ToMax, to_max, u16; i32, i64, isize, i128, u16, u32, u64, usize, u128; 65_535 }
min_zero_max_impl! { ToMax, to_max, u32; i64, isize, i128, u32, u64, usize, u128; 4_294_967_295 }
min_zero_max_impl! { ToMax, to_max, u64; i128, u64, usize, u128; 18_446_744_073_709_551_615 }
min_zero_max_impl! { ToMax, to_max, usize; i128, u64, usize, u128; 18_446_744_073_709_551_615 }
min_zero_max_impl! { ToMax, to_max, u128; u128; u128::MAX }

min_zero_max_impl! { ToMin, to_min, u8; u8; 0_u8 }
min_zero_max_impl! { ToMin, to_min, u16; u16; 0_u16 }
min_zero_max_impl! { ToMin, to_min, u32; u32; 0_u32 }
min_zero_max_impl! { ToMin, to_min, u64; u64; 0_u64 }
min_zero_max_impl! { ToMin, to_min, usize; usize; 0_usize }
min_zero_max_impl! { ToMin, to_min, u128; u128; 0_u128 }

min_zero_max_impl! { ToZero, to_zero, u8; u8; 0_u8 }
min_zero_max_impl! { ToZero, to_zero, u16; u16; 0_u16 }
min_zero_max_impl! { ToZero, to_zero, u32; u32; 0_u32 }
min_zero_max_impl! { ToZero, to_zero, u64; u64; 0_u64 }
min_zero_max_impl! { ToZero, to_zero, usize; usize; 0_usize }
min_zero_max_impl! { ToZero, to_zero, u128; u128; 0_u128 }
min_zero_max_impl! { ToZero, to_zero, i8; i8; 0_i8 }
min_zero_max_impl! { ToZero, to_zero, i16; i16; 0_i16 }
min_zero_max_impl! { ToZero, to_zero, i32; i32; 0_i32 }
min_zero_max_impl! { ToZero, to_zero, i64; i64; 0_i64 }
min_zero_max_impl! { ToZero, to_zero, isize; isize; 0_isize }
min_zero_max_impl! { ToZero, to_zero, i128; i128; 0_i128 }
