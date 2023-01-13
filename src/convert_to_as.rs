use paste::paste;

pub trait CastFrom {
    fn from_i8(n: i8) -> Self;
    fn from_i16(n: i16) -> Self;
    fn from_i32(n: i32) -> Self;
    fn from_i64(n: i64) -> Self;
    fn from_isize(n: isize) -> Self;
    fn from_i128(n: i128) -> Self;
    fn from_u8(n: u8) -> Self;
    fn from_u16(n: u16) -> Self;
    fn from_u32(n: u32) -> Self;
    fn from_u64(n: u64) -> Self;
    fn from_usize(n: usize) -> Self;
    fn from_u128(n: u128) -> Self;
}

macro_rules! cast_from_impl {
    ($($from_type:ty),+; $type:ty ) => {
        impl CastFrom for $type {
            paste!{
                #[inline]
                fn [<from_$type>](n: $type) -> Self {
                    n
                }
            }
            $(
            paste!{
                #[inline]
                fn [<from_$from_type>](n: $from_type) -> Self {
                    n as Self
                }
            }
            )*
        }
    }
}

cast_from_impl! {i16, i32, i64, isize, i128, u8, u16, u32, u64, usize, u128; i8}
cast_from_impl! {i8, i32, i64, isize, i128, u8, u16, u32, u64, usize, u128; i16}
cast_from_impl! {i8, i16, i64, isize, i128, u8, u16, u32, u64, usize, u128; i32}
cast_from_impl! {i8, i16, i32, isize, i128, u8, u16, u32, u64, usize, u128; i64}
cast_from_impl! {i8, i16, i32, i64, i128, u8, u16, u32, u64, usize, u128; isize}
cast_from_impl! {i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, u128; i128}
cast_from_impl! {i8, i16, i32, i64, isize, i128, u16, u32, u64, usize, u128; u8}
cast_from_impl! {i8, i16, i32, i64, isize, i128, u8, u32, u64, usize, u128; u16}
cast_from_impl! {i8, i16, i32, i64, isize, i128, u8, u16, u64, usize, u128; u32}
cast_from_impl! {i8, i16, i32, i64, isize, i128, u8, u16, u32, usize, u128; u64}
cast_from_impl! {i8, i16, i32, i64, isize, i128, u8, u16, u32, u64, u128; usize}
cast_from_impl! {i8, i16, i32, i64, isize, i128, u8, u16, u32, u64, usize; u128}

/// The IntoAs trait for conversion from integers with overflow. 
pub trait IntoAs<T>: CastFrom {
    /// Conversion into integers with overflow.
    fn into_as(self) -> T;
}

macro_rules! into_as_impl {
    ($($type:ty),*) => {
        $( paste! {
            impl<T: CastFrom> IntoAs<T> for $type {
                #[inline]
                fn into_as(self) -> T {
                    <T as CastFrom>::[<from_$type>](self)
                }
            }
        })*
    }
}

into_as_impl! { i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, i128, u128 }

#[cfg(test)]
mod cast_from_test {
    use super::*;
    use paste::paste;

    macro_rules! cast_from_tests {
        ( $($from_type:ty),* ;$type:ty) => {
            cast_from_tests!{$type}

            paste! {
                $(
                    #[test]
                    fn [<$type _cast_from_test_$from_type>]() {
                        assert_eq!( <$type as CastFrom>::[<from_$from_type>](($type::MAX as $from_type) + 1), $type::MIN);
                    }
                )*
            }
        };

        ($type:ty) => {
            paste! {
                #[test]
                fn [<$type cast_from_test_$type>]() {
                    assert_eq!( <$type as CastFrom>::[<from_$type>]($type::MAX), $type::MAX);
                }
            }
        };

        ($($from_type:ty, $type:ty);*) => {
            paste! {
                $(
                    #[test]
                    fn [<$type cast_from_test_$from_type>]() {
                        assert_eq!( <$type as CastFrom>::[<from_$from_type>]($from_type::MAX), $from_type::MAX as $type);
                }
                )*
            }
        };
    }

    cast_from_tests! {u8, i8; i8, i16; u8, i16; u16, i16; i8, i32; u8, i32;
        i16, i32; u16, i32; u32, i32}
    cast_from_tests! {i8, i64; u8, i64; i16, i64; u16, i64; i32, i64; u32, i64;
        isize, i64; usize, i64; u64, i64}
    cast_from_tests! {i8, isize; u8, isize; i16, isize; u16, isize; i32, isize;
        u32, isize; i64, isize; u64, isize; usize, isize}
    cast_from_tests! {i8, i128; u8, i128; i16, i128; u16, i128; i32, i128; u32, i128;
        i64, i128; u64, i128; isize, i128; usize, i128; u128, i128}

    cast_from_tests! {i16, i32, i64, isize, i128, u16, u32, u64, usize, u128; i8}
    cast_from_tests! {i32, i64, isize, i128, u32, u64, usize, u128; i16}
    cast_from_tests! {i64, isize, i128, u64, usize, u128; i32}
    cast_from_tests! {i128, u128; i64}
    cast_from_tests! {i128, u128; isize}
    cast_from_tests! {i128}

    cast_from_tests! {i8, u8; i8, u16; u8, u16; i16, u16; i8, u32; u8, u32; i16, u32;
        u16, u32; i32, u32}
    cast_from_tests! {i8, u64; u8, u64; i16, u64; u16, u64; i32, u64; u32, u64;
        i64, u64; isize, u64; usize, u64}
    cast_from_tests! {i8, usize; u8, usize; i16, usize; u16, usize; i32, usize;
        u32, usize; i64, usize; u64, usize; isize, usize}
    cast_from_tests! {i8, u128; u8, u128; i16, u128; u16, u128; i32, u128; u32, u128;
        i64, u128; u64, u128; isize, u128; usize, u128; i128, u128}

    cast_from_tests! {i16, i32, i64, isize, i128, u16, u32, u64, usize, u128; u8}
    cast_from_tests! {i32, i64, isize, i128, u32, u64, usize, u128; u16}
    cast_from_tests! {i64, isize, i128, u64, usize, u128; u32}
    cast_from_tests! {i128, u128; u64}
    cast_from_tests! {i128, u128; usize}
    cast_from_tests! {u128}
}
