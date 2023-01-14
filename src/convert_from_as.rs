use paste::paste;

// issue #34537 https://github.com/rust-lang/rust/issues/34537
pub trait CastInto {
    fn into_i8(self) -> i8;
    fn into_i16(self) -> i16;
    fn into_i32(self) -> i32;
    fn into_i64(self) -> i64;
    fn into_isize(self) -> isize;
    fn into_i128(self) -> i128;
    fn into_u8(self) -> u8;
    fn into_u16(self) -> u16;
    fn into_u32(self) -> u32;
    fn into_u64(self) -> u64;
    fn into_usize(self) -> usize;
    fn into_u128(self) -> u128;
}

macro_rules! cast_into_impl {
    ($($from_type:ty),+; $type:ty ) => {
        impl CastInto for $type {
            paste!{
                #[inline]
                fn [<into_$type>](self) -> $type {
                    self
                }
            }
            $(
            paste!{
                #[inline]
                fn [<into_$from_type>](self) -> $from_type {
                    self as $from_type
                }
            }
            )*
        }
    }
}

cast_into_impl! {i16, i32, i64, isize, i128, u8, u16, u32, u64, usize, u128; i8}
cast_into_impl! {i8, i32, i64, isize, i128, u8, u16, u32, u64, usize, u128; i16}
cast_into_impl! {i8, i16, i64, isize, i128, u8, u16, u32, u64, usize, u128; i32}
cast_into_impl! {i8, i16, i32, isize, i128, u8, u16, u32, u64, usize, u128; i64}
cast_into_impl! {i8, i16, i32, i64, i128, u8, u16, u32, u64, usize, u128; isize}
cast_into_impl! {i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, u128; i128}
cast_into_impl! {i8, i16, i32, i64, isize, i128, u16, u32, u64, usize, u128; u8}
cast_into_impl! {i8, i16, i32, i64, isize, i128, u8, u32, u64, usize, u128; u16}
cast_into_impl! {i8, i16, i32, i64, isize, i128, u8, u16, u64, usize, u128; u32}
cast_into_impl! {i8, i16, i32, i64, isize, i128, u8, u16, u32, usize, u128; u64}
cast_into_impl! {i8, i16, i32, i64, isize, i128, u8, u16, u32, u64, u128; usize}
cast_into_impl! {i8, i16, i32, i64, isize, i128, u8, u16, u32, u64, usize; u128}

/// The FromAs trait for convert from value between integer types with possible overflow. 
/// ```
/// # use num_convert::FromAs;
/// assert_eq!(<u8 as FromAs<u16>>::from_as(255u16), 255u8);
/// assert_eq!(<u8 as FromAs<u16>>::from_as(258u16), 2u8);
/// ```

pub trait FromAs<T>: CastInto {

    /// Convert from value between integer types with possible overflow. 
    fn from_as(n: T) -> Self;
}

macro_rules! from_as_impl {
    ($($type:ty),*) => {
        $( paste! {
            impl<T: CastInto> FromAs<T> for $type {
                #[inline]
                fn from_as(n: T) -> Self {
                    <T as CastInto>::[<into_$type>](n)
                }
            }
        })*
    }
}

from_as_impl! { i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, i128, u128 }

#[cfg(test)]
mod cast_into_test {
    use super::*;
    use paste::paste;

    macro_rules! cast_into_tests {
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

    cast_into_tests! {i8; i8, i16, i32, i64, isize, i128, u8, u16, u32, u64, usize, u128}
    cast_into_tests! {u8; i8, i16, i32, i64, isize, i128, u8, u16, u32, u64, usize, u128}
    cast_into_tests! {i16; i8, i16, i32, i64, isize, i128, u8, u16, u32, u64, usize, u128}
    cast_into_tests! {u16; i8, i16, i32, i64, isize, i128, u8, u16, u32, u64, usize, u128}
    cast_into_tests! {i32; i8, i16, i32, i64, isize, i128, u8, u16, u32, u64, usize, u128}
    cast_into_tests! {u32; i8, i16, i32, i64, isize, i128, u8, u16, u32, u64, usize, u128}
    cast_into_tests! {i64; i8, i16, i32, i64, isize, i128, u8, u16, u32, u64, usize, u128}
    cast_into_tests! {u64; i8, i16, i32, i64, isize, i128, u8, u16, u32, u64, usize, u128}
    cast_into_tests! {isize; i8, i16, i32, i64, isize, i128, u8, u16, u32, u64, usize, u128}
    cast_into_tests! {usize; i8, i16, i32, i64, isize, i128, u8, u16, u32, u64, usize, u128}
    cast_into_tests! {i128; i8, i16, i32, i64, isize, i128, u8, u16, u32, u64, usize, u128}
    cast_into_tests! {u128; i8, i16, i32, i64, isize, i128, u8, u16, u32, u64, usize, u128}
}
