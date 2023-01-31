use core::ops::Rem;

macro_rules! try_from_digits_impls {
    ( $($type:ty, $trait_name:ident);* ) => {
    /// A generic trait for converting from digits into possible types.
    ///
    /// # Examples
    /// Usage:
    ///
    /// ```
    /// # use num_convert::TryFromDigits;
    /// // 65_255u16 -> 255_u8
    /// assert_eq!(<u8 as TryFromDigits<u16>>::from_digits(65_255u16), Ok(255u8));
    /// // 100_000u32 -> 0_u8
    /// assert_eq!(<u8 as TryFromDigits<u32>>::from_digits(100_000u32), Ok(0u8));
    /// // 10_000_965_535_i64 -> 65535u16
    /// assert_eq!(<u16 as TryFromDigits<i64>>::from_digits(10_000_965_535i64), Ok(65_535_u16));
    /// // 10_000_000_256u64 -> Error
    /// assert!(<u8 as TryFromDigits<u64>>::from_digits(10_000_000_256u64).is_err());
    /// ```
    pub trait TryFromDigits<T> {
        /// Returns digits as a number into possible types.
        fn from_digits(n: T) -> Result<Self, <Self as TryFrom<T>>::Error> where Self: TryFrom<T>;
    }
        $(
            impl<T> TryFromDigits<T> for $type
            where
                T: Rem<T, Output = T> + $trait_name,
                Self: TryFrom<T>,
            {
                #[inline]
                fn from_digits(n: T) -> Result<Self, <Self as TryFrom<T>>::Error>
                {
                    <Self as TryFrom<T>>::try_from(n % T::num())
                }
            }
        )*
    }
}

try_from_digits_impls! { i8,Thousands; u8, Thousands; i16, HundredThousands; u16, HundredThousands;
i32, TenBillions; u32, TenBillions; i64, BigTen; u64, BigHundred; isize, BigTen; usize, BigHundred }

trait Thousands {
    fn num() -> Self;
}

macro_rules! thousands_impls {
    ( $($type:ty),* ; $value:expr ) => {
        $(
            impl Thousands for $type {
                #[inline]
                fn num() -> Self {
                    $value
                }
            }
        )*
    }
}

thousands_impls! { i16, u16, i32, u32, i64, u64, isize, usize, i128, u128; 1000 }

trait HundredThousands {
    fn num() -> Self;
}

macro_rules! hundred_thousands_impls {
    ( $($type:ty),* ; $value:expr ) => {
        $(
            impl HundredThousands for $type {
                #[inline]
                fn num() -> Self {
                    $value
                }
            }
        )*
    }
}

hundred_thousands_impls! { i32, u32, i64, u64, isize, usize, i128, u128; 100_000 }

trait TenBillions {
    fn num() -> Self;
}

macro_rules! ten_billions_impls {
    ( $($type:ty),* ; $value:expr ) => {
        $(
            impl TenBillions for $type {
                #[inline]
                fn num() -> Self {
                    $value
                }
            }
        )*
    }
}

ten_billions_impls! { i64, u64, isize, usize, i128, u128; 10_000_000_000 }

trait BigTen {
    fn num() -> Self;
}

trait BigHundred {
    fn num() -> Self;
}

macro_rules! big_impls {
    ( $( $trait_name:ident, $type:ty, $value:expr );* ) => {
        $(
            impl $trait_name for $type {
                #[inline]
                fn num() -> Self {
                    $value
                }
            }
        )*
    }
}

big_impls! { BigTen, i128, 10_000_000_000_000_000_000; BigHundred, u128, 100_000_000_000_000_000_000 }
big_impls! { BigTen, u128, 10_000_000_000_000_000_000; BigHundred, i128, 100_000_000_000_000_000_000 }

#[cfg(test)]
mod tests {
    use super::*;
    use paste::paste;

    macro_rules! try_from_digits {
        ( $trait_name:ident; $($type:ty),* ; $value:expr ) => {
            $( paste! {
                #[test]
                fn [<digit_$type _value_$value _test>]() {
                    assert_eq!(<$type as $trait_name>::num(), $value);
                }

            })*
        }
    }

    try_from_digits! { Thousands; i16, u16, i32, u32, i64, u64, isize, usize, i128, u128; 1000 }
    try_from_digits! { HundredThousands; i32, u32, i64, u64, isize, usize, i128, u128; 100_000 }
    try_from_digits! { TenBillions; i64, u64, isize, usize, i128, u128; 10_000_000_000 }
    try_from_digits! { BigTen; i128; 10_000_000_000_000_000_000 }
    try_from_digits! { BigTen; u128; 10_000_000_000_000_000_000 }
    try_from_digits! { BigHundred; u128; 100_000_000_000_000_000_000 }
    try_from_digits! { BigHundred; i128; 100_000_000_000_000_000_000 }
}
