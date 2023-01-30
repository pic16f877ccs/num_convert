use num_convert::TryToByAdd;
use paste::paste;

macro_rules! into_by_add_tests_ok {
        ( $from:ty; $($left_type:ty, $into:ty, $right_type:ty);+ ) => {
            $(
                paste! {
                    #[test]
                    fn [<ok_$from _try_into_$into _min>]() {
                       assert_eq!(<$left_type>::MIN as $into,
                           paste! {TryToByAdd::[<try_into_$into>](<$right_type>::MIN as $from).unwrap()});
                    }

                    #[test]
                    fn [<ok_$from _try_into_$into _max>]() {
                       assert_eq!(<$left_type>::MAX as $into,
                           paste! {TryToByAdd::[<try_into_$into>](<$right_type>::MAX as $from).unwrap()});
                    }
                }
            )*
        }
    }

macro_rules! into_by_add_tests_min_err {
        ( $from:ty; $($into:ty, $right_type:ty);+ ) => {
            $(
                paste! {
                    #[test]
                    #[should_panic]
                    fn [<err_$from _try_into_$into _min>]() {
                       assert_eq!(<$into>::MIN,
                           paste! {TryToByAdd::[<try_into_$into>]((<$right_type>::MIN as $from) - 1).unwrap()});
                    }
                }
            )*
        }
    }

macro_rules! into_by_add_tests_max_err {
        ( $from:ty; $($into:ty, $right_type:ty);+ ) => {
            $(
                paste! {
                    #[test]
                    #[should_panic]
                    fn [<err_$from _try_into_$into _max>]() {
                       assert_eq!(<$into>::MAX,
                           paste! {TryToByAdd::[<try_into_$into>]((<$right_type>::MAX as $from) + 1).unwrap()});
                    }
                }
            )*
        }
    }

into_by_add_tests_ok! {i8;    i8, i8, i8; i8,  i16, i8;  i8,  i32, i8;  i8,  i64, i8;  i8,    isize, i8;    i8,    i128, i8}
into_by_add_tests_ok! {i16;   i8, i8, i8; i16, i16, i16; i16, i32, i16; i16, i64, i16; i16,   isize, i16;   i16,   i128, i16}
into_by_add_tests_ok! {i32;   i8, i8, i8; i16, i16, i16; i32, i32, i32; i32, i64, i32; i32,   isize, i32;   i32,   i128, i32}
into_by_add_tests_ok! {i64;   i8, i8, i8; i16, i16, i16; i32, i32, i32; i64, i64, i64; isize, isize, isize;   i64,   i128, i64}
#[cfg(target_pointer_width = "32")]
into_by_add_tests_ok! {isize; i8, i8, i8; i16, i16, i16; i32, i32, i32; isize, i64, isize; isize, isize, isize; isize, i128, isize}
#[cfg(target_pointer_width = "64")]
into_by_add_tests_ok! {isize; i8, i8, i8; i16, i16, i16; i32, i32, i32; i64, i64, i64; isize, isize, isize; isize, i128, isize}
into_by_add_tests_ok! {i128;  i8, i8, i8; i16, i16, i16; i32, i32, i32; i64, i64, i64; isize, isize, isize; i128,  i128, i128}

into_by_add_tests_ok! {i8;    u8, u8, i8; u8,  u16, i8;  u8,  u32, i8;  u8,  u64, i8;  u8,    usize, i8;    u8,    u128, i8}
into_by_add_tests_ok! {i16;   u8, u8, i8; u16, u16, i16; u16, u32, i16; u16, u64, i16; u16,   usize, i16;   u16,   u128, i16}
into_by_add_tests_ok! {i32;   u8, u8, i8; u16, u16, i16; u32, u32, i32; u32, u64, i32; u32,   usize, i32;   u32,   u128, i32}
#[cfg(target_pointer_width = "32")]
into_by_add_tests_ok! {i64;   u8, u8, i8; u16, u16, i16; u32, u32, i32; u64, u64, i64; usize, usize, isize; u64, u128, i64}
#[cfg(target_pointer_width = "64")]
into_by_add_tests_ok! {i64;   u8, u8, i8; u16, u16, i16; u32, u32, i32; u64, u64, i64; u64,   usize, i64;   u64,   u128, i64}
#[cfg(target_pointer_width = "32")]
into_by_add_tests_ok! {isize; u8, u8, i8; u16, u16, i16; u32, u32, i32; usize, u64, isize; usize, usize, isize; usize, u128, isize}
#[cfg(target_pointer_width = "64")]
into_by_add_tests_ok! {isize; u8, u8, i8; u16, u16, i16; u32, u32, i32; u64, u64, i64; usize, usize, isize; usize, u128, isize}
into_by_add_tests_ok! {i128;  u8, u8, i8; u16, u16, i16; u32, u32, i32; u64, u64, i64; usize, usize, isize; u128,  u128, i128}

into_by_add_tests_ok! {u8;    u8, u8, u8; u8,  u16, u8;  u8,  u32, u8;  u8,  u64, u8;  u8,    usize, u8;    u8,    u128, u8}
into_by_add_tests_ok! {u16;   u8, u8, u8; u16, u16, u16; u16, u32, u16; u16, u64, u16; u16,   usize, u16;   u16,   u128, u16}
into_by_add_tests_ok! {u32;   u8, u8, u8; u16, u16, u16; u32, u32, u32; u32, u64, u32; u32,   usize, u32;   u32,   u128, u32}
#[cfg(target_pointer_width = "32")]
into_by_add_tests_ok! {u64;   u8, u8, u8; u16, u16, u16; u32, u32, u32; u64, u64, u64; usize, usize, usize;   u64,   u128, u64}
#[cfg(target_pointer_width = "64")]
into_by_add_tests_ok! {u64;   u8, u8, u8; u16, u16, u16; u32, u32, u32; u64, u64, u64; u64,   usize, u64;   u64,   u128, u64}
#[cfg(target_pointer_width = "32")]
into_by_add_tests_ok! {usize; u8, u8, u8; u16, u16, u16; u32, u32, u32; usize, u64, usize; usize, usize, usize; usize, u128, usize}
#[cfg(target_pointer_width = "64")]
into_by_add_tests_ok! {usize; u8, u8, u8; u16, u16, u16; u32, u32, u32; u64, u64, u64; usize, usize, usize; usize, u128, usize}
into_by_add_tests_ok! {u128;  u8, u8, u8; u16, u16, u16; u32, u32, u32; u64, u64, u64; usize, usize, usize; u128,  u128, u128}

into_by_add_tests_ok! {u8;    i8, i8, u8; i8,  i16, u8;  i8,  i32, u8;  i8,  i64, u8;  i8,    isize, u8;    i8,    i128, u8}
into_by_add_tests_ok! {u16;   i8, i8, u8; i16, i16, u16; i16, i32, u16; i16, i64, u16; i16,   isize, u16;   i16,   i128, u16}
into_by_add_tests_ok! {u32;   i8, i8, u8; i16, i16, u16; i32, i32, u32; i32, i64, u32; i32,   isize, u32;   i32,   i128, u32}
#[cfg(target_pointer_width = "32")]
into_by_add_tests_ok! {u64;   i8, i8, u8; i16, i16, u16; i32, i32, u32; i64, i64, u64; isize,   isize, usize;   i64,   i128, u64}
#[cfg(target_pointer_width = "64")]
into_by_add_tests_ok! {u64;   i8, i8, u8; i16, i16, u16; i32, i32, u32; i64, i64, u64; i64,   isize, u64;   i64,   i128, u64}
#[cfg(target_pointer_width = "32")]
into_by_add_tests_ok! {usize; i8, i8, u8; i16, i16, u16; i32, i32, u32; isize, i64, usize; isize, isize, usize; isize, i128, usize}
#[cfg(target_pointer_width = "64")]
into_by_add_tests_ok! {usize; i8, i8, u8; i16, i16, u16; i32, i32, u32; i64, i64, u64; isize, isize, usize; isize, i128, usize}
into_by_add_tests_ok! {u128;  i8, i8, u8; i16, i16, u16; i32, i32, u32; i64, i64, u64; isize, isize, usize; i128,  i128, u128}

into_by_add_tests_max_err! {u16; i8, u8; u8, u8}
into_by_add_tests_max_err! {u32; i8, u8; u8, u8; i16, u16; u16, u16}
#[cfg(target_pointer_width = "32")]
into_by_add_tests_max_err! {u64; i8, u8; u8, u8; i16, u16; u16, u16; i32, u32; u32, u32; isize, usize; usize, usize}
#[cfg(target_pointer_width = "64")]
into_by_add_tests_max_err! {u64; i8, u8; u8, u8; i16, u16; u16, u16; i32, u32; u32, u32}
#[cfg(target_pointer_width = "32")]
into_by_add_tests_max_err! {usize; i8, u8; u8, u8; i16, u16; u16, u16}
#[cfg(target_pointer_width = "64")]
into_by_add_tests_max_err! {usize; i8, u8; u8, u8; i16, u16; u16, u16; i32, u32; u32, u32}
into_by_add_tests_max_err! {u128; i8, u8; u8, u8; i16, u16; u16, u16; i32, u32; u32, u32; i64, u64; u64, u64; isize, usize; usize, usize}

into_by_add_tests_max_err! {i16; i8, u8; u8, u8}
into_by_add_tests_max_err! {i32; i8, u8; u8, u8; i16, u16; u16, u16}
#[cfg(target_pointer_width = "32")]
into_by_add_tests_max_err! {i64; i8, u8; u8, u8; i16, u16; u16, u16; i32, u32; u32, u32; isize, usize; usize, usize}
#[cfg(target_pointer_width = "64")]
into_by_add_tests_max_err! {i64; i8, u8; u8, u8; i16, u16; u16, u16; i32, u32; u32, u32}
#[cfg(target_pointer_width = "32")]
into_by_add_tests_max_err! {isize; i8, u8; u8, u8; i16, u16; u16, u16}
#[cfg(target_pointer_width = "64")]
into_by_add_tests_max_err! {isize; i8, u8; u8, u8; i16, u16; u16, u16; i32, u32; u32, u32}
into_by_add_tests_max_err! {i128; i8, u8; u8, u8; i16, u16; u16, u16; i32, u32; u32, u32; i64, u64; u64, u64; isize, usize; usize, usize}

into_by_add_tests_min_err! {i16; i8, u8}
into_by_add_tests_min_err! {i32; i8, u8; i16, u16}
#[cfg(target_pointer_width = "32")]
into_by_add_tests_min_err! {i64; i8, u8; i16, u16; i32, u32; isize, usize}
#[cfg(target_pointer_width = "64")]
into_by_add_tests_min_err! {i64; i8, u8; i16, u16; i32, u32}
#[cfg(target_pointer_width = "32")]
into_by_add_tests_min_err! {isize; i8, u8; i16, u16}
#[cfg(target_pointer_width = "64")]
into_by_add_tests_min_err! {isize; i8, u8; i16, u16; i32, u32}
into_by_add_tests_min_err! {i128; i8, u8; i16, u16; i32, u32; i64, u64; isize, usize}
