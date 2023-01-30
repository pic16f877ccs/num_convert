#![allow(non_snake_case)]
use num_convert::TryFromDigits;
use paste::paste;

const ZEROS_SIGNED: i128 = -100_000_000_000_000_000_000;
const ZEROS_UNSIGNED: u128 = 100_000_000_000_000_000_000;
const ZERO_SIGNED: i128 = -0;
const ZERO_UNSIGNED: u128 = 0;
const OK_I128: i128 = 215_575_341_561_156_311_124;
const OK_I64: i64 = 1_561_156_311_124;
const OK_I32: i32 = 1_156_311_124;
const OK_I16: i16 = 11_124;
const OK_U128: u128 = 215_575_341_561_156_311_124;
const OK_U64: u64 = 1_561_156_311_124;
const OK_U32: u32 = 1_156_311_124;
const OK_U16: u16 = 11_124;

macro_rules! from_digits_tests {
        ( $from_value:expr; $from_type:ty; $($to_type:ty, $to_value:expr);* ) => {
        $(
            paste! {
                #[test]
                fn [<from_$from_value:lower _digits_$to_type>]() {
                    assert_eq!(<$to_type as TryFromDigits<$from_type>>::from_digits($from_value), Ok($to_value));
                }
            }
        )*
        }
    }

from_digits_tests! { ZERO_SIGNED; i128; i8, 0; u8, 0; i16, 0; u16, 0; i32, 0; u32, 0; i64, 0; u64, 0; isize, 0; usize, 0 }
from_digits_tests! { ZERO_UNSIGNED; u128; i8, 0; u8, 0; i16, 0; u16, 0; i32, 0; u32, 0; i64, 0; u64, 0; isize, 0; usize, 0 }
from_digits_tests! { ZEROS_SIGNED; i128; i8, 0; u8, 0; i16, 0; u16, 0; i32, 0; u32, 0; i64, 0; u64, 0; isize, 0; usize, 0 }
from_digits_tests! { ZEROS_UNSIGNED; u128; i8, 0; u8, 0; i16, 0; u16, 0; i32, 0; u32, 0; i64, 0; u64, 0; isize, 0; usize, 0 }

from_digits_tests! { OK_I128; i128; i8, 124i8; u8, 124u8; i16, 11_124i16; u16, 11_124u16; i32, 1_156_311_124i32; u32, 1_156_311_124_u32;
i64, 5_575_341_561_156_311_124i64; u64, 15_575_341_561_156_311_124u64; isize, 5_575_341_561_156_311_124isize;
usize, 15_575_341_561_156_311_124usize }

from_digits_tests! { OK_I64; i64; i8, 124i8; u8, 124u8; i16, 11_124i16; u16, 11_124u16; i32, 1_156_311_124i32; u32, 1_156_311_124_u32 }

from_digits_tests! { OK_I32; i32; i8, 124i8; u8, 124u8; i16, 11_124i16; u16, 11_124u16 }

from_digits_tests! { OK_I16; i16; i8, 124i8; u8, 124u8 }

from_digits_tests! { OK_U128; u128; i8, 124i8; u8, 124u8; i16, 11_124i16; u16, 11_124u16; i32, 1_156_311_124i32; u32, 1_156_311_124_u32;
i64, 5_575_341_561_156_311_124i64; u64, 15_575_341_561_156_311_124u64; isize, 5_575_341_561_156_311_124isize;
usize, 15_575_341_561_156_311_124usize }

from_digits_tests! { OK_U64; u64; i8, 124i8; u8, 124u8; i16, 11_124i16; u16, 11_124u16; i32, 1_156_311_124i32; u32, 1_156_311_124_u32 }

from_digits_tests! { OK_U32; u32; i8, 124i8; u8, 124u8; i16, 11_124i16; u16, 11_124u16 }

from_digits_tests! { OK_U16; u16; i8, 124i8; u8, 124u8 }

macro_rules! from_digits_tests_err {
        ( $from_value:expr; $from_type:ty; $($to_type:ty);* ) => {
        $(
            paste! {
                #[test]
                fn [<from_$from_value:lower _digits_$to_type _err>]() {
                    assert!(<$to_type as TryFromDigits<$from_type>>::from_digits($from_value).is_err());
                }
            }
        )*
        }
    }

const ERR_I128: i128 = 19_900_000_005_000_066_256;
const ERR_I64: i64 = 5_000_066_256;
const ERR_I32: i32 = 66_256;
const ERR_I16: i16 = 256;
const ERR_U128: u128 = 19_900_000_005_000_066_256;
const ERR_U64: u64 = 5_000_066_256;
const ERR_U32: u32 = 66_256;
const ERR_U16: u16 = 256;

from_digits_tests_err! { ERR_I128; i128; i8; u8; i16; u16; i32; u32; i64; u64; isize; usize }

from_digits_tests_err! { ERR_I64; i64; i8; u8; i16; u16; i32; u32 }

from_digits_tests_err! { ERR_I32; i32; i8; u8; i16; u16 }

from_digits_tests_err! { ERR_I16; i16; i8; u8 }

from_digits_tests_err! { ERR_U128; u128; i8; u8; i16; u16; i32; u32; i64; u64; isize; usize }

from_digits_tests_err! { ERR_U64; u64; i8; u8; i16; u16; i32; u32 }

from_digits_tests_err! { ERR_U32; u32; i8; u8; i16; u16 }

from_digits_tests_err! { ERR_U16; u16; i8; u8 }
