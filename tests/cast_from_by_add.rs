use num_convert::CastFromByAdd;
use paste::paste;

macro_rules! cast_from_by_add {
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

cast_from_by_add! { i8;   i8, i8 }
cast_from_by_add! { i8;   u8, i8 }
cast_from_by_add! { i16;  i8, i8; i16, i16 }
cast_from_by_add! { i16;  u8, i8; u16, i16 }

// For adding 32 bit arch.
#[cfg(target_pointer_width = "32")]
cast_from_by_add! {i32;  i8, i8; i16, i16; i32, i32; isize, isize}
// For adding 32 bit arch.
#[cfg(target_pointer_width = "32")]
cast_from_by_add! {i32;  u8, i8; u16, i16; u32, i32; usize, isize}

// For adding 64 bit arch
#[cfg(target_pointer_width = "64")]
cast_from_by_add! {i32;  i8, i8; i16, i16; i32, i32}
// For adding 64 bit arch
#[cfg(target_pointer_width = "64")]
cast_from_by_add! {i32;  u8, i8; u16, i16; u32, i32}

cast_from_by_add! {i64;  i8, i8; i16, i16; i32, i32; i64, i64; isize, isize}
cast_from_by_add! {i64;  u8, i8; u16, i16; u32, i32; u64, i64; usize, isize}

// For adding 32 bit arch.
#[cfg(target_pointer_width = "32")]
cast_from_by_add! {isize;  i8, i8; i16, i16; i32, i32; isize, isize}
// For adding 32 bit arch.
#[cfg(target_pointer_width = "32")]
cast_from_by_add! {isize;  u8, i8; u16, i16; u32, i32; usize, isize}

// For adding 64 bit arch
#[cfg(target_pointer_width = "64")]
cast_from_by_add! {isize;  i8, i8; i16, i16; i32, i32; i64, i64; isize, isize}
// For adding 64 bit arch
#[cfg(target_pointer_width = "64")]
cast_from_by_add! {isize;  u8, i8; u16, i16; u32, i32; u64, i64; usize, isize}

cast_from_by_add! {i128; i8, i8; i16, i16; i32, i32; i64, i64; isize, isize; i128, i128}
cast_from_by_add! {i128; u8, i8; u16, i16; u32, i32; u64, i64; usize, isize; u128, i128}

cast_from_by_add! {u8;   i8, u8}
cast_from_by_add! {u8;   u8, u8}
cast_from_by_add! {u16;  i8, u8; i16, u16}
cast_from_by_add! {u16;  u8, u8; u16, u16}

// For adding 32 bit arch.
#[cfg(target_pointer_width = "32")]
cast_from_by_add! {u32;  i8, u8; i16, u16; i32, u32; isize, usize}
// For adding 32 bit arch.
#[cfg(target_pointer_width = "32")]
cast_from_by_add! {u32;  u8, u8; u16, u16; u32, u32; usize, usize}

// For adding 64 bit arch
#[cfg(target_pointer_width = "64")]
cast_from_by_add! {u32;  i8, u8; i16, u16; i32, u32}
// For adding 64 bit arch
#[cfg(target_pointer_width = "64")]
cast_from_by_add! {u32;  u8, u8; u16, u16; u32, u32}

cast_from_by_add! {u64;  i8, u8; i16, u16; i32, u32; i64, u64; isize, usize}
cast_from_by_add! {u64;  u8, u8; u16, u16; u32, u32; u64, u64; usize, usize}

// For adding 32 bit arch.
#[cfg(target_pointer_width = "32")]
cast_from_by_add! {usize;  i8, u8; i16, u16; i32, u32; isize, usize}
// For adding 32 bit arch.
#[cfg(target_pointer_width = "32")]
cast_from_by_add! {usize;  u8, u8; u16, u16; u32, u32; usize, usize}

// For adding 64 bit arch
#[cfg(target_pointer_width = "64")]
cast_from_by_add! {usize;  i8, u8; i16, u16; i32, u32; i64, u64; isize, usize}
// For adding 64 bit arch
#[cfg(target_pointer_width = "64")]
cast_from_by_add! {usize;  u8, u8; u16, u16; u32, u32; u64, u64; usize, usize}

cast_from_by_add! {u128; i8, u8; i16, u16; i32, u32; i64, u64; isize, usize; i128, u128}
cast_from_by_add! {u128; u8, u8; u16, u16; u32, u32; u64, u64; usize, usize; u128, u128}
