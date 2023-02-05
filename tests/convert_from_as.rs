use num_convert::FromAs;
use paste::paste;

macro_rules! from_as_tests {
    ( $($from_type:ty),* ;$type:ty) => {
        from_as_tests!{$type}

        paste! {
            $(
                #[test]
                fn [<$type _from_as_$from_type>]() {
                    assert_eq!(
                        <$type as FromAs<$from_type>>::from_as(($type::MAX as $from_type) + 1),
                        $type::MIN);
                }
            )*
        }
    };

    ($type:ty) => {
        paste! {
            #[test]
            fn [<$type _from_as_$type>]() {
                assert_eq!( <$type as FromAs<$type>>::from_as($type::MAX), $type::MAX);
            }
        }
    };

    ($($from_type:ty, $type:ty);*) => {
        paste! {
            $(
                #[test]
                fn [<$type _from_as_$from_type>]() {
                    assert_eq!(
                        <$type as FromAs<$from_type>>::from_as($from_type::MAX),
                        $from_type::MAX as $type);
                }
            )*
        }
    };
}

from_as_tests! {u8, i8; i8, i16; u8, i16; u16, i16; i8, i32; u8, i32; i16, i32;
u16, i32; u32, i32}
from_as_tests! {i8, i64; u8, i64; i16, i64; u16, i64; i32, i64; u32, i64;
isize, i64; usize, i64; u64, i64}
from_as_tests! {i8, isize; u8, isize; i16, isize; u16, isize; i32, isize;
u32, isize; i64, isize; u64, isize; usize, isize}
from_as_tests! {i8, i128; u8, i128; i16, i128; u16, i128; i32, i128; u32, i128;
i64, i128; u64, i128; isize, i128; usize, i128; u128, i128}

from_as_tests! {i16, i32, i64, isize, i128, u16, u32, u64, usize, u128; i8}
from_as_tests! {i32, i64, isize, i128, u32, u64, usize, u128; i16}
from_as_tests! {i64, isize, i128, u64, usize, u128; i32}
from_as_tests! {i128, u128; i64}
from_as_tests! {i128, u128; isize}
from_as_tests! {i128}

from_as_tests! {i8, u8; i8, u16; u8, u16; i16, u16; i8, u32; u8, u32; i16, u32;
u16, u32; i32, u32}
from_as_tests! {i8, u64; u8, u64; i16, u64; u16, u64; i32, u64; u32, u64; i64, u64;
isize, u64; usize, u64}
from_as_tests! {i8, usize; u8, usize; i16, usize; u16, usize; i32, usize; u32, usize;
i64, usize; u64, usize; isize, usize}
from_as_tests! {i8, u128; u8, u128; i16, u128; u16, u128; i32, u128; u32, u128; i64, u128;
u64, u128; isize, u128; usize, u128; i128, u128}

from_as_tests! {i16, i32, i64, isize, i128, u16, u32, u64, usize, u128; u8}
from_as_tests! {i32, i64, isize, i128, u32, u64, usize, u128; u16}
from_as_tests! {i64, isize, i128, u64, usize, u128; u32}
from_as_tests! {i128, u128; u64}
from_as_tests! {i128, u128; usize}
from_as_tests! {u128}

