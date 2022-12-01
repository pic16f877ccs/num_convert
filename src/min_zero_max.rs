use paste::paste;
use std::cmp::PartialEq;

macro_rules! min_zero_max_impl {
    ( $trait_name:ident, $fn_name:ident; $($value_type:ty),*; $value:expr) => {
        pub trait $trait_name: Sized + PartialEq + Copy {
            fn $fn_name() -> Self;
        }
        $(
        impl $trait_name for $value_type {
            #[inline]
            fn $fn_name() -> $value_type {
                $value
            }
        }
        )*
    }
}

min_zero_max_impl! { Tzero,    zero; i8, i16, i32, i64, isize, i128, u8, u16, u32, u64, usize, u128; 0 }

min_zero_max_impl! { TminI8,    min_i8; i8, i16, i32, i64, isize, i128; -128 }
min_zero_max_impl! { TminI16,   min_i16; i16, i32, i64, isize, i128; -32_768 }
min_zero_max_impl! { TminI32,   min_i32; i32, i64, isize, i128; -2_147_483_648 }
min_zero_max_impl! { TminI64,   min_i64; i64, isize, i128; -9_223_372_036_854_775_808 }
min_zero_max_impl! { TminIsize, min_isize; i64, isize, i128; -9_223_372_036_854_775_808 }
min_zero_max_impl! { TminI128,  min_i128; i128; -170_141_183_460_469_231_731_687_303_715_884_105_728 }

min_zero_max_impl! { TmaxI8,    max_i8; i8, i16, i32, i64, isize, i128; 127 }
min_zero_max_impl! { TmaxI16,   max_i16; i16, i32, i64, isize, i128; 32_767 }
min_zero_max_impl! { TmaxI32,   max_i32; i32, i64, isize, i128; 2_147_483_647 }
min_zero_max_impl! { TmaxI64,   max_i64; i64, isize, i128; 9_223_372_036_854_775_807 }
min_zero_max_impl! { TmaxIsize, max_isize; i64, isize, i128; 9_223_372_036_854_775_807 }
min_zero_max_impl! { TmaxI128,  max_i128; i128; 170_141_183_460_469_231_731_687_303_715_884_105_727 }

min_zero_max_impl! { TmaxU8,    max_u8; u8, u16, u32, u64, usize, u128; 255 }
min_zero_max_impl! { TmaxU16,   max_u16; u16, u32, u64, usize, u128; 65_535 }
min_zero_max_impl! { TmaxU32,   max_u32; u32, u64, usize, u128; 4_294_967_295 }
min_zero_max_impl! { TmaxU64,   max_u64; u64, usize, u128; 18_446_744_073_709_551_615 }
min_zero_max_impl! { TmaxUsize, max_usize; u64, usize, u128; 18_446_744_073_709_551_615 }
min_zero_max_impl! { TmaxU128,  max_u128; u128; 340_282_366_920_938_463_463_374_607_431_768_211_455 }

min_zero_max_impl! { TminU8,    min_u8;    u8; 0 }
min_zero_max_impl! { TminU16,   min_u16;   u16; 0 }
min_zero_max_impl! { TminU32,   min_u32;   u32; 0 }
min_zero_max_impl! { TminU64,   min_u64;   u64; 0 }
min_zero_max_impl! { TminUsize, min_usize; u64; 0 }
min_zero_max_impl! { TminU128,  min_u128;  u128; 0 }
