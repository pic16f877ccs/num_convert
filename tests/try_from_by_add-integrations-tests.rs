use num_convert::*;
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

#[macro_export]
macro_rules! try_from {
    ( $left_type:ty; $to_type:ty; $($to_types:ty),+; $($from_types:ty),+; $try_from:ident($from_type:ty); $right_type:ty ) => {
        $(
            let to_var = if <$to_types>::MAX as $left_type < <$to_type>::MAX as $left_type {
                (<$to_types>::MIN as $to_types, <$to_types>::MAX as $to_types) 
            } else {
                (<$to_type>::MIN as $to_types, <$to_type>::MAX as $to_types)
            };

            let from_var = if <$from_types>::MAX as $right_type < <$from_type>::MAX as $right_type {
                (<$from_types>::MIN as $from_type, <$from_types>::MAX as $from_type) 
            } else {
                (<$from_type>::MIN as $from_type, <$from_type>::MAX as $from_type)
            };

            assert_eq!(to_var.0, <$to_types as TryFromByAdd>::$try_from(from_var.0 as $from_type).unwrap());
            assert_eq!(to_var.1, <$to_types as TryFromByAdd>::$try_from(from_var.1 as $from_type).unwrap());
        )+
    };
}

macro_rules! from_signed_to_signed {
    ( $($try_from:ident($arg_type:ty)),+ ) => {
        { $( try_from!(i128; $arg_type; i8, i16, i32, i64, isize, i128; i8, i16, i32, i64, isize, i128; $try_from($arg_type); i128); )+ }
    };
}

macro_rules! from_unsigned_to_unsigned {
    ( $($try_from:ident($arg_type:ty)),+ ) => {
        { $( try_from!(u128; $arg_type; u8, u16, u32, u64, usize, u128; u8, u16, u32, u64, usize, u128; $try_from($arg_type); u128); )+ }
    };
}

macro_rules! from_unsigned_to_signed {
    ( $($try_from:ident($arg_type:ty), $type:ty);+ ) => {
        { $( try_from!(i128; $type; i8, i16, i32, i64, isize, i128; u8, u16, u32, u64, usize, u128; $try_from($arg_type); u128); )+ }
    };
}

macro_rules! from_signed_to_unsigned {
    ( $($try_from:ident($arg_type:ty), $type:ty);+ ) => {
        { $( try_from!(u128; $type; u8, u16, u32, u64, usize, u128; i8, i16, i32, i64, isize, i128; $try_from($arg_type); i128); )+ }
    };
}

#[test]
fn from_signed_to_signed() {
    from_signed_to_signed!(try_from_i8(i8), try_from_i16(i16), try_from_i32(i32), try_from_i64(i64),
    try_from_isize(isize), try_from_i128(i128));
}

#[test]
fn from_unsigned_to_unsigned() {
    from_unsigned_to_unsigned!(try_from_u8(u8), try_from_u16(u16), try_from_u32(u32), try_from_u64(u64),
    try_from_usize(usize), try_from_u128(u128));
}

#[test]
fn from_unsigned_to_signed() {
    from_unsigned_to_signed!(try_from_u8(u8), i8; try_from_u16(u16), i16; try_from_u32(u32), i32; try_from_u64(u64), i64;
    try_from_usize(usize), isize; try_from_u128(u128), i128);
}

#[test]
fn from_signed_to_unsigned() {
    from_signed_to_unsigned!(try_from_i8(i8), u8; try_from_i16(i16), u16; try_from_i32(i32), u32; try_from_i64(i64), u64;
    try_from_isize(isize), usize; try_from_i128(i128), u128);
}

#[test]
fn from_out_i8() {
    assert_eq!(i8::MIN, <i8 as    TryFromByAdd>::try_from_i8(i8::MIN).unwrap());
    assert_eq!(i8::MAX, <i8 as    TryFromByAdd>::try_from_i8(i8::MAX).unwrap());
    assert_eq!(i8::MIN, <i8 as    TryFromByAdd>::try_from_u8(u8::MIN).unwrap());
    assert_eq!(i8::MAX, <i8 as    TryFromByAdd>::try_from_u8(u8::MAX).unwrap());
    assert_eq!(i8::MIN, <i8 as   TryFromByAdd>::try_from_i16(i8::MIN as i16).unwrap());
    assert_eq!(i8::MAX, <i8 as   TryFromByAdd>::try_from_i16(i8::MAX as i16).unwrap());
    assert_eq!(i8::MIN, <i8 as   TryFromByAdd>::try_from_u16(u8::MIN as u16).unwrap());
    assert_eq!(i8::MAX, <i8 as   TryFromByAdd>::try_from_u16(u8::MAX as u16).unwrap());
    assert_eq!(i8::MIN, <i8 as   TryFromByAdd>::try_from_i32(i8::MIN as i32).unwrap());
    assert_eq!(i8::MAX, <i8 as   TryFromByAdd>::try_from_i32(i8::MAX as i32).unwrap());
    assert_eq!(i8::MIN, <i8 as   TryFromByAdd>::try_from_u32(u8::MIN as u32).unwrap());
    assert_eq!(i8::MAX, <i8 as   TryFromByAdd>::try_from_u32(u8::MAX as u32).unwrap());
    assert_eq!(i8::MIN, <i8 as   TryFromByAdd>::try_from_i64(i8::MIN as i64).unwrap());
    assert_eq!(i8::MAX, <i8 as   TryFromByAdd>::try_from_i64(i8::MAX as i64).unwrap());
    assert_eq!(i8::MIN, <i8 as   TryFromByAdd>::try_from_u64(u8::MIN as u64).unwrap());
    assert_eq!(i8::MAX, <i8 as   TryFromByAdd>::try_from_u64(u8::MAX as u64).unwrap());
    assert_eq!(i8::MIN, <i8 as TryFromByAdd>::try_from_isize(i8::MIN as isize).unwrap());
    assert_eq!(i8::MAX, <i8 as TryFromByAdd>::try_from_isize(i8::MAX as isize).unwrap());
    assert_eq!(i8::MIN, <i8 as TryFromByAdd>::try_from_usize(u8::MIN as usize).unwrap());
    assert_eq!(i8::MAX, <i8 as TryFromByAdd>::try_from_usize(u8::MAX as usize).unwrap());
    assert_eq!(i8::MIN, <i8 as  TryFromByAdd>::try_from_i128(i8::MIN as i128).unwrap());
    assert_eq!(i8::MAX, <i8 as  TryFromByAdd>::try_from_i128(i8::MAX as i128).unwrap());
    assert_eq!(i8::MIN, <i8 as  TryFromByAdd>::try_from_u128(u8::MIN as u128).unwrap());
    assert_eq!(i8::MAX, <i8 as  TryFromByAdd>::try_from_u128(u8::MAX as u128).unwrap());
}

#[test]
fn from_out_u8() {
    assert_eq!(u8::MIN, <u8 as    TryFromByAdd>::try_from_i8(i8::MIN).unwrap());
    assert_eq!(u8::MAX, <u8 as    TryFromByAdd>::try_from_i8(i8::MAX).unwrap());
    assert_eq!(u8::MIN, <u8 as    TryFromByAdd>::try_from_u8(u8::MIN).unwrap());
    assert_eq!(u8::MAX, <u8 as    TryFromByAdd>::try_from_u8(u8::MAX).unwrap());
    assert_eq!(u8::MIN, <u8 as   TryFromByAdd>::try_from_i16(i8::MIN as i16).unwrap());
    assert_eq!(u8::MAX, <u8 as   TryFromByAdd>::try_from_i16(i8::MAX as i16).unwrap());
    assert_eq!(u8::MIN, <u8 as   TryFromByAdd>::try_from_u16(u8::MIN as u16).unwrap());
    assert_eq!(u8::MAX, <u8 as   TryFromByAdd>::try_from_u16(u8::MAX as u16).unwrap());
    assert_eq!(u8::MIN, <u8 as   TryFromByAdd>::try_from_i32(i8::MIN as i32).unwrap());
    assert_eq!(u8::MAX, <u8 as   TryFromByAdd>::try_from_i32(i8::MAX as i32).unwrap());
    assert_eq!(u8::MIN, <u8 as   TryFromByAdd>::try_from_u32(u8::MIN as u32).unwrap());
    assert_eq!(u8::MAX, <u8 as   TryFromByAdd>::try_from_u32(u8::MAX as u32).unwrap());
    assert_eq!(u8::MIN, <u8 as   TryFromByAdd>::try_from_i64(i8::MIN as i64).unwrap());
    assert_eq!(u8::MAX, <u8 as   TryFromByAdd>::try_from_i64(i8::MAX as i64).unwrap());
    assert_eq!(u8::MIN, <u8 as   TryFromByAdd>::try_from_u64(u8::MIN as u64).unwrap());
    assert_eq!(u8::MAX, <u8 as   TryFromByAdd>::try_from_u64(u8::MAX as u64).unwrap());
    assert_eq!(u8::MIN, <u8 as TryFromByAdd>::try_from_isize(i8::MIN as isize).unwrap());
    assert_eq!(u8::MAX, <u8 as TryFromByAdd>::try_from_isize(i8::MAX as isize).unwrap());
    assert_eq!(u8::MIN, <u8 as TryFromByAdd>::try_from_usize(u8::MIN as usize).unwrap());
    assert_eq!(u8::MAX, <u8 as TryFromByAdd>::try_from_usize(u8::MAX as usize).unwrap());
    assert_eq!(u8::MIN, <u8 as  TryFromByAdd>::try_from_i128(i8::MIN as i128).unwrap());
    assert_eq!(u8::MAX, <u8 as  TryFromByAdd>::try_from_i128(i8::MAX as i128).unwrap());
    assert_eq!(u8::MIN, <u8 as  TryFromByAdd>::try_from_u128(u8::MIN as u128).unwrap());
    assert_eq!(u8::MAX, <u8 as  TryFromByAdd>::try_from_u128(u8::MAX as u128).unwrap());
}


#[test]
fn from_out_i16() {
    assert_eq!(i8::MIN  as i16, <i16 as    TryFromByAdd>::try_from_i8(i8::MIN  as i8).unwrap());
    assert_eq!(i8::MAX  as i16, <i16 as    TryFromByAdd>::try_from_i8(i8::MAX  as i8).unwrap());
    assert_eq!(i8::MIN  as i16, <i16 as    TryFromByAdd>::try_from_u8(u8::MIN  as u8).unwrap());
    assert_eq!(i8::MAX  as i16, <i16 as    TryFromByAdd>::try_from_u8(u8::MAX  as u8).unwrap());
    assert_eq!(i16::MIN as i16, <i16 as   TryFromByAdd>::try_from_i16(i16::MIN as i16).unwrap());
    assert_eq!(i16::MAX as i16, <i16 as   TryFromByAdd>::try_from_i16(i16::MAX as i16).unwrap());
    assert_eq!(i16::MIN as i16, <i16 as   TryFromByAdd>::try_from_u16(u16::MIN as u16).unwrap());
    assert_eq!(i16::MAX as i16, <i16 as   TryFromByAdd>::try_from_u16(u16::MAX as u16).unwrap());
    assert_eq!(i16::MIN as i16, <i16 as   TryFromByAdd>::try_from_i32(i16::MIN as i32).unwrap());
    assert_eq!(i16::MAX as i16, <i16 as   TryFromByAdd>::try_from_i32(i16::MAX as i32).unwrap());
    assert_eq!(i16::MIN as i16, <i16 as   TryFromByAdd>::try_from_u32(u16::MIN as u32).unwrap());
    assert_eq!(i16::MAX as i16, <i16 as   TryFromByAdd>::try_from_u32(u16::MAX as u32).unwrap());
    assert_eq!(i16::MIN as i16, <i16 as   TryFromByAdd>::try_from_i64(i16::MIN as i64).unwrap());
    assert_eq!(i16::MAX as i16, <i16 as   TryFromByAdd>::try_from_i64(i16::MAX as i64).unwrap());
    assert_eq!(i16::MIN as i16, <i16 as   TryFromByAdd>::try_from_u64(u16::MIN as u64).unwrap());
    assert_eq!(i16::MAX as i16, <i16 as   TryFromByAdd>::try_from_u64(u16::MAX as u64).unwrap());
    assert_eq!(i16::MIN as i16, <i16 as TryFromByAdd>::try_from_isize(i16::MIN as isize).unwrap());
    assert_eq!(i16::MAX as i16, <i16 as TryFromByAdd>::try_from_isize(i16::MAX as isize).unwrap());
    assert_eq!(i16::MIN as i16, <i16 as TryFromByAdd>::try_from_usize(u16::MIN as usize).unwrap());
    assert_eq!(i16::MAX as i16, <i16 as TryFromByAdd>::try_from_usize(u16::MAX as usize).unwrap());
    assert_eq!(i16::MIN as i16, <i16 as  TryFromByAdd>::try_from_i128(i16::MIN as i128).unwrap());
    assert_eq!(i16::MAX as i16, <i16 as  TryFromByAdd>::try_from_i128(i16::MAX as i128).unwrap());
    assert_eq!(i16::MIN as i16, <i16 as  TryFromByAdd>::try_from_u128(u16::MIN as u128).unwrap());
    assert_eq!(i16::MAX as i16, <i16 as  TryFromByAdd>::try_from_u128(u16::MAX as u128).unwrap());
}

#[test]
fn from_out_u16() {
    assert_eq!(u8::MIN as u16, <u16 as    TryFromByAdd>::try_from_i8(i8::MIN).unwrap());
    assert_eq!(u8::MAX as u16, <u16 as    TryFromByAdd>::try_from_i8(i8::MAX).unwrap());
    assert_eq!(u8::MIN as u16, <u16 as    TryFromByAdd>::try_from_u8(u8::MIN).unwrap());
    assert_eq!(u8::MAX as u16, <u16 as    TryFromByAdd>::try_from_u8(u8::MAX).unwrap());
    assert_eq!(u16::MIN,       <u16 as   TryFromByAdd>::try_from_i16(i16::MIN).unwrap());
    assert_eq!(u16::MAX,       <u16 as   TryFromByAdd>::try_from_i16(i16::MAX).unwrap());
    assert_eq!(u16::MIN,       <u16 as   TryFromByAdd>::try_from_u16(u16::MIN).unwrap());
    assert_eq!(u16::MAX,       <u16 as   TryFromByAdd>::try_from_u16(u16::MAX).unwrap());
    assert_eq!(u16::MIN,       <u16 as   TryFromByAdd>::try_from_i32(i16::MIN as i32).unwrap());
    assert_eq!(u16::MAX,       <u16 as   TryFromByAdd>::try_from_i32(i16::MAX as i32).unwrap());
    assert_eq!(u16::MIN,       <u16 as   TryFromByAdd>::try_from_u32(u16::MIN as u32).unwrap());
    assert_eq!(u16::MAX,       <u16 as   TryFromByAdd>::try_from_u32(u16::MAX as u32).unwrap());
    assert_eq!(u16::MIN,       <u16 as   TryFromByAdd>::try_from_i64(i16::MIN as i64).unwrap());
    assert_eq!(u16::MAX,       <u16 as   TryFromByAdd>::try_from_i64(i16::MAX as i64).unwrap());
    assert_eq!(u16::MIN,       <u16 as   TryFromByAdd>::try_from_u64(u16::MIN as u64).unwrap());
    assert_eq!(u16::MAX,       <u16 as   TryFromByAdd>::try_from_u64(u16::MAX as u64).unwrap());
    assert_eq!(u16::MIN,       <u16 as TryFromByAdd>::try_from_isize(i16::MIN as isize).unwrap());
    assert_eq!(u16::MAX,       <u16 as TryFromByAdd>::try_from_isize(i16::MAX as isize).unwrap());
    assert_eq!(u16::MIN,       <u16 as TryFromByAdd>::try_from_usize(u16::MIN as usize).unwrap());
    assert_eq!(u16::MAX,       <u16 as TryFromByAdd>::try_from_usize(u16::MAX as usize).unwrap());
    assert_eq!(u16::MIN,       <u16 as  TryFromByAdd>::try_from_i128(i16::MIN as i128).unwrap());
    assert_eq!(u16::MAX,       <u16 as  TryFromByAdd>::try_from_i128(i16::MAX as i128).unwrap());
    assert_eq!(u16::MIN,       <u16 as  TryFromByAdd>::try_from_u128(u16::MIN as u128).unwrap());
    assert_eq!(u16::MAX,       <u16 as  TryFromByAdd>::try_from_u128(u16::MAX as u128).unwrap());
}

#[test]
fn from_out_i32() {
    assert_eq!(i8::MIN  as i32, <i32 as    TryFromByAdd>::try_from_i8(i8::MIN).unwrap());
    assert_eq!(i8::MAX  as i32, <i32 as    TryFromByAdd>::try_from_i8(i8::MAX).unwrap());
    assert_eq!(i8::MIN  as i32, <i32 as    TryFromByAdd>::try_from_u8(u8::MIN).unwrap());
    assert_eq!(i8::MAX  as i32, <i32 as    TryFromByAdd>::try_from_u8(u8::MAX).unwrap());
    assert_eq!(i16::MIN as i32, <i32 as   TryFromByAdd>::try_from_i16(i16::MIN).unwrap());
    assert_eq!(i16::MAX as i32, <i32 as   TryFromByAdd>::try_from_i16(i16::MAX).unwrap());
    assert_eq!(i16::MIN as i32, <i32 as   TryFromByAdd>::try_from_u16(u16::MIN).unwrap());
    assert_eq!(i16::MAX as i32, <i32 as   TryFromByAdd>::try_from_u16(u16::MAX).unwrap());
    assert_eq!(i32::MIN,        <i32 as   TryFromByAdd>::try_from_i32(i32::MIN).unwrap());
    assert_eq!(i32::MAX,        <i32 as   TryFromByAdd>::try_from_i32(i32::MAX).unwrap());
    assert_eq!(i32::MIN,        <i32 as   TryFromByAdd>::try_from_u32(u32::MIN).unwrap());
    assert_eq!(i32::MAX,        <i32 as   TryFromByAdd>::try_from_u32(u32::MAX).unwrap());
    assert_eq!(i32::MIN,        <i32 as   TryFromByAdd>::try_from_i64(i32::MIN as i64).unwrap());
    assert_eq!(i32::MAX,        <i32 as   TryFromByAdd>::try_from_i64(i32::MAX as i64).unwrap());
    assert_eq!(i32::MIN,        <i32 as   TryFromByAdd>::try_from_u64(u32::MIN as u64).unwrap());
    assert_eq!(i32::MAX,        <i32 as   TryFromByAdd>::try_from_u64(u32::MAX as u64).unwrap());
    assert_eq!(i32::MIN,        <i32 as TryFromByAdd>::try_from_isize(i32::MIN as isize).unwrap());
    assert_eq!(i32::MAX,        <i32 as TryFromByAdd>::try_from_isize(i32::MAX as isize).unwrap());
    assert_eq!(i32::MIN,        <i32 as TryFromByAdd>::try_from_usize(u32::MIN as usize).unwrap());
    assert_eq!(i32::MAX,        <i32 as TryFromByAdd>::try_from_usize(u32::MAX as usize).unwrap());
    assert_eq!(i32::MIN,        <i32 as  TryFromByAdd>::try_from_i128(i32::MIN as i128).unwrap());
    assert_eq!(i32::MAX,        <i32 as  TryFromByAdd>::try_from_i128(i32::MAX as i128).unwrap());
    assert_eq!(i32::MIN,        <i32 as  TryFromByAdd>::try_from_u128(u32::MIN as u128).unwrap());
    assert_eq!(i32::MAX,        <i32 as  TryFromByAdd>::try_from_u128(u32::MAX as u128).unwrap());
}


#[test]
fn from_out_u32() {
    assert_eq!(u8::MIN  as u32, <u32 as    TryFromByAdd>::try_from_i8(i8::MIN).unwrap());
    assert_eq!(u8::MAX  as u32, <u32 as    TryFromByAdd>::try_from_i8(i8::MAX).unwrap());
    assert_eq!(u8::MIN  as u32, <u32 as    TryFromByAdd>::try_from_u8(u8::MIN).unwrap());
    assert_eq!(u8::MAX  as u32, <u32 as    TryFromByAdd>::try_from_u8(u8::MAX).unwrap());
    assert_eq!(u16::MIN as u32, <u32 as   TryFromByAdd>::try_from_i16(i16::MIN).unwrap());
    assert_eq!(u16::MAX as u32, <u32 as   TryFromByAdd>::try_from_i16(i16::MAX).unwrap());
    assert_eq!(u16::MIN as u32, <u32 as   TryFromByAdd>::try_from_u16(u16::MIN).unwrap());
    assert_eq!(u16::MAX as u32, <u32 as   TryFromByAdd>::try_from_u16(u16::MAX).unwrap());
    assert_eq!(u32::MIN,        <u32 as   TryFromByAdd>::try_from_i32(i32::MIN).unwrap());
    assert_eq!(u32::MAX,        <u32 as   TryFromByAdd>::try_from_i32(i32::MAX).unwrap());
    assert_eq!(u32::MIN,        <u32 as   TryFromByAdd>::try_from_u32(u32::MIN).unwrap());
    assert_eq!(u32::MAX,        <u32 as   TryFromByAdd>::try_from_u32(u32::MAX).unwrap());
    assert_eq!(u32::MIN,        <u32 as   TryFromByAdd>::try_from_i64(i32::MIN as i64).unwrap());
    assert_eq!(u32::MAX,        <u32 as   TryFromByAdd>::try_from_i64(i32::MAX as i64).unwrap());
    assert_eq!(u32::MIN,        <u32 as   TryFromByAdd>::try_from_u64(u32::MIN as u64).unwrap());
    assert_eq!(u32::MAX,        <u32 as   TryFromByAdd>::try_from_u64(u32::MAX as u64).unwrap());
    assert_eq!(u32::MIN,        <u32 as TryFromByAdd>::try_from_isize(i32::MIN as isize).unwrap());
    assert_eq!(u32::MAX,        <u32 as TryFromByAdd>::try_from_isize(i32::MAX as isize).unwrap());
    assert_eq!(u32::MIN,        <u32 as TryFromByAdd>::try_from_usize(u32::MIN as usize).unwrap());
    assert_eq!(u32::MAX,        <u32 as TryFromByAdd>::try_from_usize(u32::MAX as usize).unwrap());
    assert_eq!(u32::MIN,        <u32 as  TryFromByAdd>::try_from_i128(i32::MIN as i128).unwrap());
    assert_eq!(u32::MAX,        <u32 as  TryFromByAdd>::try_from_i128(i32::MAX as i128).unwrap());
    assert_eq!(u32::MIN,        <u32 as  TryFromByAdd>::try_from_u128(u32::MIN as u128).unwrap());
    assert_eq!(u32::MAX,        <u32 as  TryFromByAdd>::try_from_u128(u32::MAX as u128).unwrap());
}


#[test]
fn from_out_i64() {
    assert_eq!(i8::MIN  as i64, <i64 as    TryFromByAdd>::try_from_i8(i8::MIN).unwrap());
    assert_eq!(i8::MAX  as i64, <i64 as    TryFromByAdd>::try_from_i8(i8::MAX).unwrap());
    assert_eq!(i8::MIN  as i64, <i64 as    TryFromByAdd>::try_from_u8(u8::MIN).unwrap());
    assert_eq!(i8::MAX  as i64, <i64 as    TryFromByAdd>::try_from_u8(u8::MAX).unwrap());
    assert_eq!(i16::MIN as i64, <i64 as   TryFromByAdd>::try_from_i16(i16::MIN).unwrap());
    assert_eq!(i16::MAX as i64, <i64 as   TryFromByAdd>::try_from_i16(i16::MAX).unwrap());
    assert_eq!(i16::MIN as i64, <i64 as   TryFromByAdd>::try_from_u16(u16::MIN).unwrap());
    assert_eq!(i16::MAX as i64, <i64 as   TryFromByAdd>::try_from_u16(u16::MAX).unwrap());
    assert_eq!(i32::MIN as i64, <i64 as   TryFromByAdd>::try_from_i32(i32::MIN).unwrap());
    assert_eq!(i32::MAX as i64, <i64 as   TryFromByAdd>::try_from_i32(i32::MAX).unwrap());
    assert_eq!(i32::MIN as i64, <i64 as   TryFromByAdd>::try_from_u32(u32::MIN).unwrap());
    assert_eq!(i32::MAX as i64, <i64 as   TryFromByAdd>::try_from_u32(u32::MAX).unwrap());
    assert_eq!(i64::MIN,        <i64 as   TryFromByAdd>::try_from_i64(i64::MIN).unwrap());
    assert_eq!(i64::MAX,        <i64 as   TryFromByAdd>::try_from_i64(i64::MAX).unwrap());
    assert_eq!(i64::MIN,        <i64 as   TryFromByAdd>::try_from_u64(u64::MIN).unwrap());
    assert_eq!(i64::MAX,        <i64 as   TryFromByAdd>::try_from_u64(u64::MAX).unwrap());
    assert_eq!(i64::MIN,        <i64 as TryFromByAdd>::try_from_isize(i64::MIN as isize).unwrap());
    assert_eq!(i64::MAX,        <i64 as TryFromByAdd>::try_from_isize(i64::MAX as isize).unwrap());
    assert_eq!(i64::MIN,        <i64 as TryFromByAdd>::try_from_usize(u64::MIN as usize).unwrap());
    assert_eq!(i64::MAX,        <i64 as TryFromByAdd>::try_from_usize(u64::MAX as usize).unwrap());
    assert_eq!(i64::MIN,        <i64 as  TryFromByAdd>::try_from_i128(i64::MIN as i128).unwrap());
    assert_eq!(i64::MAX,        <i64 as  TryFromByAdd>::try_from_i128(i64::MAX as i128).unwrap());
    assert_eq!(i64::MIN,        <i64 as  TryFromByAdd>::try_from_u128(u64::MIN as u128).unwrap());
    assert_eq!(i64::MAX,        <i64 as  TryFromByAdd>::try_from_u128(u64::MAX as u128).unwrap());
}

#[test]
fn from_out_u64() {
    assert_eq!(u8::MIN  as u64, <u64 as    TryFromByAdd>::try_from_i8(i8::MIN).unwrap());
    assert_eq!(u8::MAX  as u64, <u64 as    TryFromByAdd>::try_from_i8(i8::MAX).unwrap());
    assert_eq!(u8::MIN  as u64, <u64 as    TryFromByAdd>::try_from_u8(u8::MIN).unwrap());
    assert_eq!(u8::MAX  as u64, <u64 as    TryFromByAdd>::try_from_u8(u8::MAX).unwrap());
    assert_eq!(u16::MIN as u64, <u64 as   TryFromByAdd>::try_from_i16(i16::MIN).unwrap());
    assert_eq!(u16::MAX as u64, <u64 as   TryFromByAdd>::try_from_i16(i16::MAX).unwrap());
    assert_eq!(u16::MIN as u64, <u64 as   TryFromByAdd>::try_from_u16(u16::MIN).unwrap());
    assert_eq!(u16::MAX as u64, <u64 as   TryFromByAdd>::try_from_u16(u16::MAX).unwrap());
    assert_eq!(u32::MIN as u64, <u64 as   TryFromByAdd>::try_from_i32(i32::MIN).unwrap());
    assert_eq!(u32::MAX as u64, <u64 as   TryFromByAdd>::try_from_i32(i32::MAX).unwrap());
    assert_eq!(u32::MIN as u64, <u64 as   TryFromByAdd>::try_from_u32(u32::MIN).unwrap());
    assert_eq!(u32::MAX as u64, <u64 as   TryFromByAdd>::try_from_u32(u32::MAX).unwrap());
    assert_eq!(u64::MIN,        <u64 as   TryFromByAdd>::try_from_i64(i64::MIN).unwrap());
    assert_eq!(u64::MAX,        <u64 as   TryFromByAdd>::try_from_i64(i64::MAX).unwrap());
    assert_eq!(u64::MIN,        <u64 as   TryFromByAdd>::try_from_u64(u64::MIN).unwrap());
    assert_eq!(u64::MAX,        <u64 as   TryFromByAdd>::try_from_u64(u64::MAX).unwrap());
    assert_eq!(u64::MIN,        <u64 as TryFromByAdd>::try_from_isize(i64::MIN as isize).unwrap());
    assert_eq!(u64::MAX,        <u64 as TryFromByAdd>::try_from_isize(i64::MAX as isize).unwrap());
    assert_eq!(u64::MIN,        <u64 as TryFromByAdd>::try_from_usize(u64::MIN as usize).unwrap());
    assert_eq!(u64::MAX,        <u64 as TryFromByAdd>::try_from_usize(u64::MAX as usize).unwrap());
    assert_eq!(u64::MIN,        <u64 as  TryFromByAdd>::try_from_i128(i64::MIN as i128).unwrap());
    assert_eq!(u64::MAX,        <u64 as  TryFromByAdd>::try_from_i128(i64::MAX as i128).unwrap());
    assert_eq!(u64::MIN,        <u64 as  TryFromByAdd>::try_from_u128(u64::MIN as u128).unwrap());
    assert_eq!(u64::MAX,        <u64 as  TryFromByAdd>::try_from_u128(u64::MAX as u128).unwrap());
}
