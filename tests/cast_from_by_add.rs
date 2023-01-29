use num_convert::CastFromByAdd;
use paste::paste;

macro_rules! from_by_add_tests {
        ( $into:ty; $($from:ty, $type:ty);+ ) => {
            $(
                paste! {
                    #[test]
                    fn [<$into _from_$from _min>]() {
                       assert_eq!(<$type>::MIN as $into, paste! {<$into as CastFromByAdd>::[<from_$from>](<$from>::MIN)});
                    }

                    #[test]
                    fn [<$into _form_$from _max>]() {
                       assert_eq!(<$type>::MAX as $into, paste! {<$into as CastFromByAdd>::[<from_$from>](<$from>::MAX)});
                    }
                }
            )*
        }
    }

from_by_add_tests! {i8;   i8, i8}
from_by_add_tests! {i8;   u8, i8}
from_by_add_tests! {i16;  i8, i8; i16, i16}
from_by_add_tests! {i16;  u8, i8; u16, i16}

// For adding 32 bit arch.
#[cfg(target_pointer_width = "32")]
from_by_add_tests! {i32;  i8, i8; i16, i16; i32, i32; isize, isize}
// For adding 32 bit arch.
#[cfg(target_pointer_width = "32")]
from_by_add_tests! {i32;  u8, i8; u16, i16; u32, i32; usize, isize}

// For adding 64 bit arch
#[cfg(target_pointer_width = "64")]
from_by_add_tests! {i32;  i8, i8; i16, i16; i32, i32}
// For adding 64 bit arch
#[cfg(target_pointer_width = "64")]
from_by_add_tests! {i32;  u8, i8; u16, i16; u32, i32}

from_by_add_tests! {i64;  i8, i8; i16, i16; i32, i32; i64, i64; isize, isize}
from_by_add_tests! {i64;  u8, i8; u16, i16; u32, i32; u64, i64; usize, isize}

// For adding 32 bit arch.
#[cfg(target_pointer_width = "32")]
from_by_add_tests! {isize;  i8, i8; i16, i16; i32, i32; isize, isize}
// For adding 32 bit arch.
#[cfg(target_pointer_width = "32")]
from_by_add_tests! {isize;  u8, i8; u16, i16; u32, i32; usize, isize}

// For adding 64 bit arch
#[cfg(target_pointer_width = "64")]
from_by_add_tests! {isize;  i8, i8; i16, i16; i32, i32; i64, i64; isize, isize}
// For adding 64 bit arch
#[cfg(target_pointer_width = "64")]
from_by_add_tests! {isize;  u8, i8; u16, i16; u32, i32; u64, i64; usize, isize}

from_by_add_tests! {i128; i8, i8; i16, i16; i32, i32; i64, i64; isize, isize; i128, i128}
from_by_add_tests! {i128; u8, i8; u16, i16; u32, i32; u64, i64; usize, isize; u128, i128}

from_by_add_tests! {u8;   i8, u8}
from_by_add_tests! {u8;   u8, u8}
from_by_add_tests! {u16;  i8, u8; i16, u16}
from_by_add_tests! {u16;  u8, u8; u16, u16}

// For adding 32 bit arch.
#[cfg(target_pointer_width = "32")]
from_by_add_tests! {u32;  i8, u8; i16, u16; i32, u32; isize, usize}
// For adding 32 bit arch.
#[cfg(target_pointer_width = "32")]
from_by_add_tests! {u32;  u8, u8; u16, u16; u32, u32; usize, usize}

// For adding 64 bit arch
#[cfg(target_pointer_width = "64")]
from_by_add_tests! {u32;  i8, u8; i16, u16; i32, u32}
// For adding 64 bit arch
#[cfg(target_pointer_width = "64")]
from_by_add_tests! {u32;  u8, u8; u16, u16; u32, u32}

from_by_add_tests! {u64;  i8, u8; i16, u16; i32, u32; i64, u64; isize, usize}
from_by_add_tests! {u64;  u8, u8; u16, u16; u32, u32; u64, u64; usize, usize}

// For adding 32 bit arch.
#[cfg(target_pointer_width = "32")]
from_by_add_tests! {usize;  i8, u8; i16, u16; i32, u32; isize, usize}
// For adding 32 bit arch.
#[cfg(target_pointer_width = "32")]
from_by_add_tests! {usize;  u8, u8; u16, u16; u32, u32; usize, usize}

// For adding 64 bit arch
#[cfg(target_pointer_width = "64")]
from_by_add_tests! {usize;  i8, u8; i16, u16; i32, u32; i64, u64; isize, usize}
// For adding 64 bit arch
#[cfg(target_pointer_width = "64")]
from_by_add_tests! {usize;  u8, u8; u16, u16; u32, u32; u64, u64; usize, usize}

from_by_add_tests! {u128; i8, u8; i16, u16; i32, u32; i64, u64; isize, usize; i128, u128}
from_by_add_tests! {u128; u8, u8; u16, u16; u32, u32; u64, u64; usize, usize; u128, u128}
