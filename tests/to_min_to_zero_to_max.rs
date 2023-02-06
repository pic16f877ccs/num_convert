use num_convert::{ToMin, ToMax, ToZero};
use paste::paste;

macro_rules! to_min_to_zero_to_max {
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

to_min_to_zero_to_max! { ToZero, to_zero; i8, i16, i32, i64, isize, i128, u8, u16, u32, u64, usize, u128; 0 }

to_min_to_zero_to_max! { ToMin, to_min, i8; i8, i16, i32, i64, isize, i128; -128 }
to_min_to_zero_to_max! { ToMin, to_min, i16; i16, i32, i64, isize, i128; -32_768 }
to_min_to_zero_to_max! { ToMin, to_min, i32; i32, i64, isize, i128; -2_147_483_648 }
to_min_to_zero_to_max! { ToMin, to_min, i64; i64, isize, i128; -9_223_372_036_854_775_808 }
to_min_to_zero_to_max! { ToMin, to_min, isize; i64, isize, i128; -9_223_372_036_854_775_808 }
to_min_to_zero_to_max! { ToMin, to_min, i128; i128; -170_141_183_460_469_231_731_687_303_715_884_105_728 }

to_min_to_zero_to_max! { ToMax, to_max, i8; i8, i16, i32, i64, isize, i128, u8, u16, u32, u64, usize, u128; 127 }
to_min_to_zero_to_max! { ToMax, to_max, i16; i16, i32, i64, isize, i128, u16, u32, u64, usize, u128; 32_767 }
to_min_to_zero_to_max! { ToMax, to_max, i32; i32, i64, isize, i128, u32, u64, usize, u128; 2_147_483_647 }
to_min_to_zero_to_max! { ToMax, to_max, i64; i64, isize, i128, u64, usize, u128; 9_223_372_036_854_775_807 }
to_min_to_zero_to_max! { ToMax, to_max, isize; i64, isize, i128, u64, usize, u128; 9_223_372_036_854_775_807 }
to_min_to_zero_to_max! { ToMax, to_max, i128; i128; 170_141_183_460_469_231_731_687_303_715_884_105_727 }

to_min_to_zero_to_max! { ToMax, to_max, u8; i16, i32, i64, isize, i128, u8, u16, u32, u64, usize, u128; 255 }
to_min_to_zero_to_max! { ToMax, to_max, u16; i32, i64, isize, i128, u16, u32, u64, usize, u128; 65_535 }
to_min_to_zero_to_max! { ToMax, to_max, u32; i64, isize, i128, u32, u64, usize, u128; 4_294_967_295 }
to_min_to_zero_to_max! { ToMax, to_max, u64; i128, u64, usize, u128; 18_446_744_073_709_551_615 }
to_min_to_zero_to_max! { ToMax, to_max, usize; i128, u64, usize, u128; 18_446_744_073_709_551_615 }
to_min_to_zero_to_max! { ToMax, to_max, u128; u128; 340_282_366_920_938_463_463_374_607_431_768_211_455 }

to_min_to_zero_to_max! { ToMin, to_min, u8; u8; 0 }
to_min_to_zero_to_max! { ToMin, to_min, u16; u16; 0 }
to_min_to_zero_to_max! { ToMin, to_min, u32; u32; 0 }
to_min_to_zero_to_max! { ToMin, to_min, u64; u64; 0 }
to_min_to_zero_to_max! { ToMin, to_min, usize; usize; 0 }
to_min_to_zero_to_max! { ToMin, to_min, u128; u128; 0 }
