use num_convert::CastIntoAs;
use paste::paste;

macro_rules! cast_into_as {
    ( $type:ty; $($into_type:ty),*) => {
        paste!{
            $(
                #[test]
                fn [<$type _into_$into_type>]() {
                    assert_eq!(<$type>::[<into_$into_type>]($type::MAX), $type::MAX as $into_type);
                }
            )*
        }
    }
}

cast_into_as! {i8; i8, i16, i32, i64, isize, i128, u8, u16, u32, u64, usize, u128}
cast_into_as! {u8; i8, i16, i32, i64, isize, i128, u8, u16, u32, u64, usize, u128}
cast_into_as! {i16; i8, i16, i32, i64, isize, i128, u8, u16, u32, u64, usize, u128}
cast_into_as! {u16; i8, i16, i32, i64, isize, i128, u8, u16, u32, u64, usize, u128}
cast_into_as! {i32; i8, i16, i32, i64, isize, i128, u8, u16, u32, u64, usize, u128}
cast_into_as! {u32; i8, i16, i32, i64, isize, i128, u8, u16, u32, u64, usize, u128}
cast_into_as! {i64; i8, i16, i32, i64, isize, i128, u8, u16, u32, u64, usize, u128}
cast_into_as! {u64; i8, i16, i32, i64, isize, i128, u8, u16, u32, u64, usize, u128}
cast_into_as! {isize; i8, i16, i32, i64, isize, i128, u8, u16, u32, u64, usize, u128}
cast_into_as! {usize; i8, i16, i32, i64, isize, i128, u8, u16, u32, u64, usize, u128}
cast_into_as! {i128; i8, i16, i32, i64, isize, i128, u8, u16, u32, u64, usize, u128}
cast_into_as! {u128; i8, i16, i32, i64, isize, i128, u8, u16, u32, u64, usize, u128}
