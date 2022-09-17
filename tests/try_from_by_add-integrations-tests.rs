use num_convert::TryFromByAdd;
use paste::paste;

macro_rules! from_by_add_tests {
        ( $into:ty; $($left_type:ty, $from:ty, $right_type:ty);+ ) => {
            $(
                paste! {
                    #[test]
                    fn [<$into _try_from_$from _min>]() {
                       assert_eq!(<$left_type>::MIN as $into,
                           paste! {<$into as TryFromByAdd>::[<try_from_$from>](<$right_type>::MIN as $from).unwrap()});
                    }

                    #[test]
                    fn [<$into _try_from_$from _max>]() {
                       assert_eq!(<$left_type>::MAX as $into,
                           paste! {<$into as TryFromByAdd>::[<try_from_$from>](<$right_type>::MAX as $from).unwrap()});
                    }
                }
            )*
        }
    }

from_by_add_tests! {i8; i8, i8, i8; i8, i16, i8; i8, i32, i8; i8, i64, i8; i8, isize, i8; i8, i128, i8}
from_by_add_tests! {i16; i8, i8, i8; i16, i16, i16; i16, i32, i16; i16, i64, i16; i16, isize, i16; i16, i128, i16}
from_by_add_tests! {i32; i8, i8, i8; i16, i16, i16; i32, i32, i32; i32, i64, i32; i32, isize, i32; i32, i128, i32}
from_by_add_tests! {i64; i8, i8, i8; i16, i16, i16; i32, i32, i32; i64, i64, i64; i64, isize, i64; i64, i128, i64}
from_by_add_tests! {isize; i8, i8, i8; i16, i16, i16; i32, i32, i32; i64, i64, i64; isize, isize, isize; isize, i128, isize}
from_by_add_tests! {i128; i8, i8, i8; i16, i16, i16; i32, i32, i32; i64, i64, i64; isize, isize, isize; i128, i128, i128}

from_by_add_tests! {u8;    u8, i8, i8; u8,  i16, i8;  u8,  i32, i8;  u8,  i64, i8;  u8,    isize, i8;    u8,    i128, i8}
from_by_add_tests! {u16;   u8, i8, i8; u16, i16, i16; u16, i32, i16; u16, i64, i16; u16,   isize, i16;   u16,   i128, i16}
from_by_add_tests! {u32;   u8, i8, i8; u16, i16, i16; u32, i32, i32; u32, i64, i32; u32,   isize, i32;   u32,   i128, i32}
from_by_add_tests! {u64;   u8, i8, i8; u16, i16, i16; u32, i32, i32; u64, i64, i64; u64,   isize, i64;   u64,   i128, i64}
from_by_add_tests! {usize; u8, i8, i8; u16, i16, i16; u32, i32, i32; u64, i64, i64; usize, isize, isize; usize, i128, isize}
from_by_add_tests! {u128;  u8, i8, i8; u16, i16, i16; u32, i32, i32; u64, i64, i64; usize, isize, isize; u128,  i128, i128}

from_by_add_tests! {i8;    i8, u8, u8; i8,  u16, u8;  i8,  u32, u8;  i8,  u64, u8;  i8,    usize, u8;    i8,    u128, u8}
from_by_add_tests! {i16;   i8, u8, u8; i16, u16, u16; i16, u32, u16; i16, u64, u16; i16,   usize, u16;   i16,   u128, u16}
from_by_add_tests! {i32;   i8, u8, u8; i16, u16, u16; i32, u32, u32; i32, u64, u32; i32,   usize, u32;   i32,   u128, u32}
from_by_add_tests! {i64;   i8, u8, u8; i16, u16, u16; i32, u32, u32; i64, u64, u64; i64,   usize, u64;   i64,   u128, u64}
from_by_add_tests! {isize; i8, u8, u8; i16, u16, u16; i32, u32, u32; i64, u64, u64; isize, usize, usize; isize, u128, usize}
from_by_add_tests! {i128;  i8, u8, u8; i16, u16, u16; i32, u32, u32; i64, u64, u64; isize, usize, usize; i128,  u128, u128}

from_by_add_tests! {u8;    u8, u8, u8; u8,  u16, u8;  u8,  u32, u8;  u8,  u64, u8;  u8,    usize, u8;    u8,    u128, u8}
from_by_add_tests! {u16;   u8, u8, u8; u16, u16, u16; u16, u32, u16; u16, u64, u16; u16,   usize, u16;   u16,   u128, u16}
from_by_add_tests! {u32;   u8, u8, u8; u16, u16, u16; u32, u32, u32; u32, u64, u32; u32,   usize, u32;   u32,   u128, u32}
from_by_add_tests! {u64;   u8, u8, u8; u16, u16, u16; u32, u32, u32; u64, u64, u64; u64,   usize, u64;   u64,   u128, u64}
from_by_add_tests! {usize; u8, u8, u8; u16, u16, u16; u32, u32, u32; u64, u64, u64; usize, usize, usize; usize, u128, usize}
from_by_add_tests! {u128;  u8, u8, u8; u16, u16, u16; u32, u32, u32; u64, u64, u64; usize, usize, usize; u128,  u128, u128}

