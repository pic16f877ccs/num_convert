use num_convert::min_zero_max::*;
use paste::paste;

macro_rules! min_zero_max_tests {
    ( $fn_name:ident; $($type:ty),*; $value:expr ) => {
        $( paste! {
            #[test]
            fn [<$type _$fn_name>]() {
                assert_eq!($type::$fn_name(), $value);
            }
        })*
    }
}

min_zero_max_tests! { zero; i8, i16, i32, i64, isize, i128, u8, u16, u32, u64, usize, u128; 0 }

min_zero_max_tests! { min_i8; i8, i16, i32, i64, isize, i128; -128 }
min_zero_max_tests! { min_i16; i16, i32, i64, isize, i128; -32_768 }
min_zero_max_tests! { min_i32; i32, i64, isize, i128; -2_147_483_648 }
min_zero_max_tests! { min_i64; i64, isize, i128; -9_223_372_036_854_775_808 }
min_zero_max_tests! { min_isize; i64, isize, i128; -9_223_372_036_854_775_808 }
min_zero_max_tests! { min_i128; i128; -170_141_183_460_469_231_731_687_303_715_884_105_728 }

min_zero_max_tests! { max_i8; i8, i16, i32, i64, isize, i128; 127 }
min_zero_max_tests! { max_i16; i16, i32, i64, isize, i128; 32_767 }
min_zero_max_tests! { max_i32; i32, i64, isize, i128; 2_147_483_647 }
min_zero_max_tests! { max_i64; i64, isize, i128; 9_223_372_036_854_775_807 }
min_zero_max_tests! { max_isize; i64, isize, i128; 9_223_372_036_854_775_807 }
min_zero_max_tests! { max_i128; i128; 170_141_183_460_469_231_731_687_303_715_884_105_727 }

min_zero_max_tests! { max_u8; u8, u16, u32, u64, usize, u128; 255 }
min_zero_max_tests! { max_u16; u16, u32, u64, usize, u128; 65_535 }
min_zero_max_tests! { max_u32; u32, u64, usize, u128; 4_294_967_295 }
min_zero_max_tests! { max_u64; u64, usize, u128; 18_446_744_073_709_551_615 }
min_zero_max_tests! { max_usize; u64, usize, u128; 18_446_744_073_709_551_615 }
min_zero_max_tests! { max_u128; u128; 340_282_366_920_938_463_463_374_607_431_768_211_455 }

min_zero_max_tests! { min_u8; u8; 0 }
min_zero_max_tests! { min_u16; u16; 0 }
min_zero_max_tests! { min_u32; u32; 0 }
min_zero_max_tests! { min_u64; u64; 0 }
min_zero_max_tests! { min_usize; u64; 0 }
min_zero_max_tests! { min_u128; u128; 0 }
