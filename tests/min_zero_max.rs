use num_convert::min_zero_max::*;
use paste::paste;

macro_rules! min_zero_max_tests {
    ( $trait_name:ident, $fn_name:ident, $for_type:ty; $($type:ty),*; $value:expr ) => {
        $( paste! {
            #[test]
            fn [<$type _$fn_name _for_$for_type>]() {
                assert_eq!(<$for_type as $trait_name<$type>>::$fn_name(), $value);
            }
        })*
    };
    
    ( $trait_name:ident, $fn_name:ident; $($type:ty),*; $value:expr ) => {
        $( paste! {
            #[test]
            fn [<$type _$fn_name _for_$type>]() {
                assert_eq!(<$type as $trait_name<$type>>::$fn_name(), $value);
            }
        })*
    }
}

min_zero_max_tests! { Tzero, tzero; i8, i16, i32, i64, isize, i128, u8, u16, u32, u64, usize, u128; 0 }

min_zero_max_tests! { Tmin, tmin, i8; i8, i16, i32, i64, isize, i128; -128 }
min_zero_max_tests! { Tmin, tmin, i16; i16, i32, i64, isize, i128; -32_768 }
min_zero_max_tests! { Tmin, tmin, i32; i32, i64, isize, i128; -2_147_483_648 }
min_zero_max_tests! { Tmin, tmin, i64; i64, isize, i128; -9_223_372_036_854_775_808 }
min_zero_max_tests! { Tmin, tmin, isize; i64, isize, i128; -9_223_372_036_854_775_808 }
min_zero_max_tests! { Tmin, tmin, i128; i128; -170_141_183_460_469_231_731_687_303_715_884_105_728 }

min_zero_max_tests! { Tmax, tmax, i8; i8, i16, i32, i64, isize, i128, u8, u16, u32, u64, usize, u128; 127 }
min_zero_max_tests! { Tmax, tmax, i16; i16, i32, i64, isize, i128, u16, u32, u64, usize, u128; 32_767 }
min_zero_max_tests! { Tmax, tmax, i32; i32, i64, isize, i128, u32, u64, usize, u128; 2_147_483_647 }
min_zero_max_tests! { Tmax, tmax, i64; i64, isize, i128, u64, usize, u128; 9_223_372_036_854_775_807 }
min_zero_max_tests! { Tmax, tmax, isize; i64, isize, i128, u64, usize, u128; 9_223_372_036_854_775_807 }
min_zero_max_tests! { Tmax, tmax, i128; i128; 170_141_183_460_469_231_731_687_303_715_884_105_727 }

min_zero_max_tests! { Tmax, tmax, u8; i16, i32, i64, isize, i128, u8, u16, u32, u64, usize, u128; 255 }
min_zero_max_tests! { Tmax, tmax, u16; i32, i64, isize, i128, u16, u32, u64, usize, u128; 65_535 }
min_zero_max_tests! { Tmax, tmax, u32; i64, isize, i128, u32, u64, usize, u128; 4_294_967_295 }
min_zero_max_tests! { Tmax, tmax, u64; i128, u64, usize, u128; 18_446_744_073_709_551_615 }
min_zero_max_tests! { Tmax, tmax, usize; i128, u64, usize, u128; 18_446_744_073_709_551_615 }
min_zero_max_tests! { Tmax, tmax, u128; u128; 340_282_366_920_938_463_463_374_607_431_768_211_455 }

min_zero_max_tests! { Tmin, tmin, u8; u8; 0 }
min_zero_max_tests! { Tmin, tmin, u16; u16; 0 }
min_zero_max_tests! { Tmin, tmin, u32; u32; 0 }
min_zero_max_tests! { Tmin, tmin, u64; u64; 0 }
min_zero_max_tests! { Tmin, tmin, usize; usize; 0 }
min_zero_max_tests! { Tmin, tmin, u128; u128; 0 }
