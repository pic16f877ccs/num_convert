use num_convert::ToByAdd;
use paste::paste;

macro_rules! to_by_add_tests {
        ( $from:ty; $($into:ty),+; $type:ty ) => {
            $(
                paste! {
                    #[test]
                    fn [<$from _into_$into _min>]() {
                       assert_eq!(<$type>::MIN as $into, paste! {ToByAdd::[<into_$into>](<$from>::MIN)});
                    }

                    #[test]
                    fn [<$from _into_$into _max>]() {
                       assert_eq!(<$type>::MAX as $into, paste! {ToByAdd::[<into_$into>](<$from>::MAX)});
                    }
                }
            )*
        }
    }

to_by_add_tests! {i8; i8, i16, i32, i64, isize, i128; i8}
to_by_add_tests! {i8; u8, u16, u32, u64, usize, u128; u8}

to_by_add_tests! {u8; i8, i16, i32, i64, isize, i128; i8}
to_by_add_tests! {u8; u8, u16, u32, u64, usize, u128; u8}

to_by_add_tests! {i16; i16, i32, i64, isize, i128; i16}
to_by_add_tests! {i16; u16, u32, u64, usize, u128; u16}

to_by_add_tests! {u16; i16, i32, i64, isize, i128; i16}
to_by_add_tests! {u16; u16, u32, u64, usize, u128; u16}

to_by_add_tests! {i32; i32, i64, isize, i128; i32}
to_by_add_tests! {i32; u32, u64, usize, u128; u32}

to_by_add_tests! {u32; i32, i64, isize, i128; i32}
to_by_add_tests! {u32; u32, u64, usize, u128; u32}

#[cfg(target_pointer_width = "32")]
to_by_add_tests! {i64; i64, i128; i64}
#[cfg(target_pointer_width = "32")]
to_by_add_tests! {i64; u64, u128; u64}

#[cfg(target_pointer_width = "64")]
to_by_add_tests! {i64; i64, isize, i128; i64}
#[cfg(target_pointer_width = "64")]
to_by_add_tests! {i64; u64, usize, u128; u64}

#[cfg(target_pointer_width = "32")]
to_by_add_tests! {u64; i64, i128; i64}
#[cfg(target_pointer_width = "32")]
to_by_add_tests! {u64; u64, u128; u64}

#[cfg(target_pointer_width = "64")]
to_by_add_tests! {u64; i64, isize, i128; i64}
#[cfg(target_pointer_width = "64")]
to_by_add_tests! {u64; u64, usize, u128; u64}

#[cfg(target_pointer_width = "32")]
to_by_add_tests! {isize; i32, i64, isize, i128; isize}
#[cfg(target_pointer_width = "32")]
to_by_add_tests! {isize; u32, u64, usize, u128; usize}

#[cfg(target_pointer_width = "64")]
to_by_add_tests! {isize; i64, isize, i128; isize}
#[cfg(target_pointer_width = "64")]
to_by_add_tests! {isize; u64, usize, u128; usize}

#[cfg(target_pointer_width = "32")]
to_by_add_tests! {usize; i32, i64, isize, i128; isize}
#[cfg(target_pointer_width = "32")]
to_by_add_tests! {usize; u32, u64, usize, u128; usize}

#[cfg(target_pointer_width = "64")]
to_by_add_tests! {usize; i64, isize, i128; isize}
#[cfg(target_pointer_width = "64")]
to_by_add_tests! {usize; u64, usize, u128; usize}

to_by_add_tests! {i128; i128; i128}
to_by_add_tests! {i128; u128; u128}

to_by_add_tests! {u128; i128; i128}
to_by_add_tests! {u128; u128; u128}
