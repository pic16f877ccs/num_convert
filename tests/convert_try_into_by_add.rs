use num_convert::TryIntoByAdd;
use paste::paste;

macro_rules! try_into_by_add_ok {
        ( $from:ty; $($right_type:ty, $into:ty, $left_type:ty);+ ) => {
            $(
                paste! {
                    #[test]
                    fn [<ok_$from _try_into_$into _min>]() {
                       assert_eq!(TryIntoByAdd::try_into_by_add(<$left_type>::MIN as $from), Some(<$right_type>::MIN as $into));
                    }

                    #[test]
                    fn [<ok_$from _try_into_$into _max>]() {
                       assert_eq!(TryIntoByAdd::try_into_by_add(<$left_type>::MAX as $from), Some(<$right_type>::MAX as $into));
                    }
                }
            )*
        }
    }

macro_rules! try_into_by_add_min_err {
    ( $from:ty; $($into:ty, $try_type:ty);+ ) => {
        $(
            paste! {
                #[test]
                #[should_panic]
                fn [<err_$from _try_into_$into _min>]() {
                   assert_eq!(TryIntoByAdd::try_into_by_add((<$try_type>::MIN as $from) - 1), Some(<$into>::MIN));
                }
            }
        )*
    }
}

macro_rules! try_into_by_add_max_err {
    ( $from:ty; $($into:ty, $try_type:ty);+ ) => {
        $(
            paste! {
                #[test]
                #[should_panic]
                fn [<err_$from _try_into_$into _max>]() {
                   assert_eq!(TryIntoByAdd::try_into_by_add((<$try_type>::MAX as $from) + 1), Some(<$into>::MAX));
                }
            }
        )*
    }
}

try_into_by_add_ok! {i8; i8, i8, i8; i8, i16, i8; i8, i32, i8; i8, i64, i8; i8, isize, i8; i8, i128, i8}
try_into_by_add_ok! {i16; i8, i8, i8; i16, i16, i16; i16, i32, i16; i16, i64, i16; i16, isize, i16; i16, i128, i16}
try_into_by_add_ok! {i32; i8, i8, i8; i16, i16, i16; i32, i32, i32; i32, i64, i32; i32, isize, i32; i32, i128, i32}
try_into_by_add_ok! {i64; i8, i8, i8; i16, i16, i16; i32, i32, i32; i64, i64, i64; isize, isize, isize; i64, i128, i64}
try_into_by_add_ok! {isize; i8, i8, i8; i16, i16, i16; i32, i32, i32; i64, i64, i64; isize, isize, isize; isize, i128, isize}
try_into_by_add_ok! {i128; i8, i8, i8; i16, i16, i16; i32, i32, i32; i64, i64, i64; isize, isize, isize; i128, i128, i128}

//try_into_by_add_ok! {i8;    u8, u8, i8; u8,  u16, i8;  u8,  u32, i8;  u8,  u64, i8;  u8,    usize, i8;    u8,    u128, i8}
//try_into_by_add_ok! {i16;   u8, u8, i8; u16, u16, i16; u16, u32, i16; u16, u64, i16; u16,   usize, i16;   u16,   u128, i16}
//try_into_by_add_ok! {i32;   u8, u8, i8; u16, u16, i16; u32, u32, i32; u32, u64, i32; u32,   usize, i32;   u32,   u128, i32}
//try_into_by_add_ok! {i64;   u8, u8, i8; u16, u16, i16; u32, u32, i32; u64, u64, i64; u64,   usize, i64;   u64,   u128, i64}
//try_into_by_add_ok! {isize; u8, u8, i8; u16, u16, i16; u32, u32, i32; u64, u64, i64; usize, usize, isize; usize, u128, isize}
//try_into_by_add_ok! {i128;  u8, u8, i8; u16, u16, i16; u32, u32, i32; u64, u64, i64; usize, usize, isize; u128,  u128, i128}

try_into_by_add_ok! {u8; u8, u8, u8; u8, u16, u8; u8, u32, u8; u8, u64, u8; u8, usize, u8; u8, u128, u8}
try_into_by_add_ok! {u16;   u8, u8, u8; u16, u16, u16; u16, u32, u16; u16, u64, u16; u16,   usize, u16;   u16,   u128, u16}
try_into_by_add_ok! {u32;   u8, u8, u8; u16, u16, u16; u32, u32, u32; u32, u64, u32; u32,   usize, u32;   u32,   u128, u32}
try_into_by_add_ok! {u64;   u8, u8, u8; u16, u16, u16; u32, u32, u32; u64, u64, u64; u64,   usize, u64;   u64,   u128, u64}
try_into_by_add_ok! {usize; u8, u8, u8; u16, u16, u16; u32, u32, u32; u64, u64, u64; usize, usize, usize; usize, u128, usize}
try_into_by_add_ok! {u128;  u8, u8, u8; u16, u16, u16; u32, u32, u32; u64, u64, u64; usize, usize, usize; u128,  u128, u128}

//try_into_by_add_ok! {u8;    i8, i8, u8; i8,  i16, u8;  i8,  i32, u8;  i8,  i64, u8;  i8,    isize, u8;    i8,    i128, u8}
//try_into_by_add_ok! {u16;   i8, i8, u8; i16, i16, u16; i16, i32, u16; i16, i64, u16; i16,   isize, u16;   i16,   i128, u16}
//try_into_by_add_ok! {u32;   i8, i8, u8; i16, i16, u16; i32, i32, u32; i32, i64, u32; i32,   isize, u32;   i32,   i128, u32}
//try_into_by_add_ok! {u64;   i8, i8, u8; i16, i16, u16; i32, i32, u32; i64, i64, u64; i64,   isize, u64;   i64,   i128, u64}
//try_into_by_add_ok! {usize; i8, i8, u8; i16, i16, u16; i32, i32, u32; i64, i64, u64; isize, isize, usize; isize, i128, usize}
//try_into_by_add_ok! {u128;  i8, i8, u8; i16, i16, u16; i32, i32, u32; i64, i64, u64; isize, isize, usize; i128,  i128, u128}

//try_into_by_add_max_err! {u16; i8, u8; u8, u8}
//try_into_by_add_max_err! {u32; i8, u8; u8, u8; i16, u16; u16, u16}
//try_into_by_add_max_err! {u64; i8, u8; u8, u8; i16, u16; u16, u16; i32, u32; u32, u32}
//try_into_by_add_max_err! {usize; i8, u8; u8, u8; i16, u16; u16, u16; i32, u32; u32, u32}
//try_into_by_add_max_err! {u128; i8, u8; u8, u8; i16, u16; u16, u16; i32, u32; u32, u32; i64, u64; u64, u64; isize, usize; usize, usize}
//try_into_by_add_max_err! {i16; i8, u8; u8, u8}
//try_into_by_add_max_err! {i32; i8, u8; u8, u8; i16, u16; u16, u16}
//try_into_by_add_max_err! {i64; i8, u8; u8, u8; i16, u16; u16, u16; i32, u32; u32, u32}
//try_into_by_add_max_err! {isize; i8, u8; u8, u8; i16, u16; u16, u16; i32, u32; u32, u32}
//try_into_by_add_max_err! {i128; i8, u8; u8, u8; i16, u16; u16, u16; i32, u32; u32, u32; i64, u64; u64, u64; isize, usize; usize, usize}
//try_into_by_add_min_err! {i16; i8, u8}
//try_into_by_add_min_err! {i32; i8, u8; i16, u16}
//try_into_by_add_min_err! {i64; i8, u8; i16, u16; i32, u32}
//try_into_by_add_min_err! {isize; i8, u8; i16, u16; i32, u32}
//try_into_by_add_min_err! {i128; i8, u8; i16, u16; i32, u32; i64, u64; isize, usize}
