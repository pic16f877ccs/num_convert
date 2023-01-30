use num_convert::{Sbits, Tbits};
use paste::paste;

macro_rules! tbits_tests {
   ( $($value_type:ty),* ) => {
       $( paste! {

           #[test]
           fn [<test_tbits_$value_type>]() {
               assert_eq!($value_type::tbits(), <$value_type>::BITS);
           }
       })*
   }
}

tbits_tests! { i8,  u8,  i16,  u16,  i32,  u32,  i64,  u64,  isize,  usize,  i128,  u128 }

macro_rules! sbits_tests_eq {
    ($($value:expr, $bits:expr);*) => {
        $(
            paste!{
                #[test]
                fn [<sbits_$value _bits_$bits>]() {
                    assert_eq!($value.sbits(), $bits);
                }
            }
        )*
    }
}

sbits_tests_eq! { 1i8, 8; 2u8, 8; 3i16, 16; 4u16, 16; 5i32, 32; 6u32, 32;
7i64, 64; 8u64, 64; 9isize, 64; 10usize, 64; 11i128, 128; 12u128, 128 }

macro_rules! sbits_tests_ne {
    ($($value:expr, $bits:expr);*) => {
        $(
            paste!{
                #[test]
                fn [<sbits_$value _bits_$bits>]() {
                    assert_ne!($value.sbits(), $bits);
                }
            }
        )*
    }
}

sbits_tests_ne! { 1i8, 1; 2u8, 2; 3i16, 3; 4u16, 4; 5i32, 5; 6u32, 6;
7i64, 7; 8u64, 8; 9isize, 9; 10usize, 10; 11i128, 11; 12u128, 12 }
