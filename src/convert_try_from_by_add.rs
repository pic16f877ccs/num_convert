/// Generic trait for converting between integer types within bounds of possible values.
/// - For all integer types in the range of possible values.
/// - Returns an error if the value is out of range.
/// - Converting types with different signs of the same size in bits.
/// - A normal conversion of same-signed types to a larger integral type.
/// - It is possible to convert types with different signs to a larger integer type.
///
/// # Usage
/// Basic use of the trait.
///
/// ```
/// use num_convert::TryFromByAdd;
///
/// assert_eq!(<u8 as TryFromByAdd<i16>>::try_from_by_add(-128_i16).unwrap(), 0_u8);
/// assert_eq!(<u8>::try_from_by_add(127_i8).unwrap(), 255_u8);
/// ```
///
/// # Examples
///
/// Converting without error.
/// ```
/// # use num_convert::TryFromByAdd;
/// assert_eq!(<u32 as TryFromByAdd<i8>>::try_from_by_add(<i8>::MIN), Some(<u8>::MIN as u32));
/// assert_eq!(<u64 as TryFromByAdd<i8>>::try_from_by_add(<i8>::MAX), Some(<u8>::MAX as u64));
/// ```
///
/// Converting with error.
/// ```
/// # use num_convert::TryFromByAdd;
/// assert_eq!(<u32>::try_from_by_add(<u64>::MAX), None);
/// assert_eq!(<u8>::try_from_by_add(128_i16), None);
/// ```
pub trait TryFromByAdd<T> {
    /// Converts the value from `T` to `self`.
    fn try_from_by_add(n: T) -> Option<Self>
    where
        Self: Sized;
}

macro_rules! signed_or_unsigned_impls {
    ( $( $from_type:ty; $($into_type:ty),* );* ) => {
        $(
            //from signed = to signed
            //from unsigned = to unsigned
            impl TryFromByAdd<$from_type> for $from_type {
                #[doc = concat!("Returns the same type ", stringify!($from_type), ".")]
                #[inline]
                fn try_from_by_add(n: Self) -> Option<Self> {
                    Some(n)
                }
            }

            $(
                //from signed < to signed
                //from unsigned < to unsigned
                impl TryFromByAdd<$from_type> for $into_type {
                    #[doc = concat!("Converts ", stringify!($from_type), " to ", stringify!($into_type), " losslessly.")]
                    #[inline]
                    fn try_from_by_add(n: $from_type) -> Option<Self> {
                        Some(n as Self)
                    }
                }
            )*
        )*
    }
}

signed_or_unsigned_impls! { i8; i16, i32, i64, isize, i128; i16; i32, i64, isize, i128;
i32; i64, isize, i128; i64; isize, i128; isize; i64, i128; i128; }
signed_or_unsigned_impls! { u8; u16, u32, u64, usize, u128; u16; u32, u64, usize, u128;
u32; u64, usize, u128; u64; usize, u128; usize; u64, u128; u128; }

macro_rules! signed_gt_signed_impls {
    ( $( $into_type:ty, $min_type:expr, $max_type:expr; $($from_type:ty),* );* ) => {
        $(
            $(
                //from signed > to signed
                impl TryFromByAdd<$from_type> for $into_type {
                    #[doc = concat!("Converts ", stringify!($from_type), " to ", stringify!($into_type), ". Conversion can fail.")]
                    #[inline]
                    fn try_from_by_add(n: $from_type) -> Option<Self> {
                        if n >= $min_type && n <= $max_type {
                            Some(n as Self)
                        } else {
                            None
                        }
                    }
                }
            )*
        )*
    }
}

signed_gt_signed_impls! { i8, -128, 127; i16, i32, i64, isize, i128; i16, -32_768, 32_767; i32, i64, isize, i128;
i32, -2_147_483_648, 2_147_483_647; i64, isize, i128; i64, -9_223_372_036_854_775_808, 9_223_372_036_854_775_807; i128; isize, -9_223_372_036_854_775_808, 9_223_372_036_854_775_807; i128  }

macro_rules! signed_gt_unsigned_impls {
    ( $( $into_type:ty, $max_value:expr; $($from_type:ty),* );* ) => {
        $(
            $(
                //from signed > to unsigned
                impl TryFromByAdd<$from_type> for $into_type {
                    #[doc = concat!("Converts ", stringify!($from_type), " to equivalent ", stringify!($into_type), ". Conversion can fail.")]
                    #[inline]
                    fn try_from_by_add(n: $from_type) -> Option<Self> {
                        if n <= $max_value {
                            Some(((n as Self).wrapping_add(Self::MAX)).wrapping_add(1))
                        } else {
                            None
                        }
                    }
                }
            )*
        )*
    }
}

signed_gt_unsigned_impls! { i8, 255; u16, u32, u64, usize, u128; i16, 65_535; u32, u64, usize, u128;
i32, 4_294_967_295; u64, usize, u128; i64, 18_446_744_073_709_551_615; u128; isize, 18_446_744_073_709_551_615; u128 }

macro_rules! unsigned_gt_unsigned_impls {
    ( $( $into_type:ty, $max_type:expr; $($from_type:ty),* );* ) => {
        $(
            $(
                //from unsigned > to unsigned
                impl TryFromByAdd<$from_type> for $into_type {
                    #[doc = concat!("Converts ", stringify!($from_type), " to ", stringify!($into_type), ". Conversion can fail.")]
                    #[inline]
                    fn try_from_by_add(n: $from_type) -> Option<Self> {
                        if n <= $max_type {
                            Some(n as Self)
                        } else {
                            None
                        }
                    }
                }
            )*
        )*
    }
}

unsigned_gt_unsigned_impls! { u8, 255; u16, u32, u64, usize, u128; u16, 65_535; u32, u64, usize, u128;
u32, 4_294_967_295; u64, usize, u128; u64, 18_446_744_073_709_551_615; u128; usize, 18_446_744_073_709_551_615; u128 }

macro_rules! unsigned_gt_signed_impls {
    ( $into_type:ty, $min_value:expr, $max_value:expr, $add_value:expr; $($from_type:ty),* ) => {
        $(
            //from unsigned < to signed
            impl TryFromByAdd<$from_type> for $into_type {
                #[doc = concat!("Converts ", stringify!($from_type), " to equivalent ", stringify!($into_type), ". Conversion can fail.")]
                #[inline]
                fn try_from_by_add(n: $from_type) -> Option<Self> {
                    if n >= $min_value && n <= $max_value {
                        Some((n as Self).wrapping_add($add_value))
                    } else {
                        None
                    }
                }
            }
        )*
    }
}

unsigned_gt_signed_impls! { u8, -128, 127, 128; i16, i32, i64, isize, i128 }
unsigned_gt_signed_impls! { u16, -32_768, 32_767, 32_768; i32, i64, isize, i128 }
unsigned_gt_signed_impls! { u32, -2_147_483_648, 2_147_483_647, 2_147_483_648; i64, isize, i128 }
unsigned_gt_signed_impls! { u64, -9_223_372_036_854_775_808, 9_223_372_036_854_775_807, 9_223_372_036_854_775_808; i128 }
unsigned_gt_signed_impls! { usize, -9_223_372_036_854_775_808, 9_223_372_036_854_775_807, 9_223_372_036_854_775_808; i128 }

macro_rules! unsigned_to_signed_impls {
    ( $from_type:ty, $as_type:ty; $($into_type:ty),* ) => {
        //from unsigned = to signed
        impl TryFromByAdd<$from_type> for $as_type {
            #[doc = concat!("Converts ", stringify!($from_type), " to equivalent ", stringify!($as_type), ".")]
            #[inline]
            fn try_from_by_add(n: $from_type) -> Option<Self> {
                Some(((n as Self).wrapping_add(<Self>::MAX)).wrapping_add(1))
            }
        }

        $(
            //from unsigned < to signed
            impl TryFromByAdd<$from_type> for $into_type {
                #[doc = concat!("Converts ", stringify!($from_type), " to equivalent ", stringify!($into_type), ".")]
                #[inline]
                fn try_from_by_add(n: $from_type) -> Option<Self> {
                    Some(((n as $as_type).wrapping_add(<$as_type>::MAX)).wrapping_add(1) as Self)
                }
            }
        )*
    };
}

unsigned_to_signed_impls! { u8, i8; i16, i32, i64, isize, i128 }
unsigned_to_signed_impls! { u16, i16; i32, i64, isize, i128 }
unsigned_to_signed_impls! { u32, i32; i64, isize, i128 }
unsigned_to_signed_impls! { u64, i64; isize, i128 }
unsigned_to_signed_impls! { usize, isize; i64, i128 }
unsigned_to_signed_impls! { u128, i128; }

macro_rules! signed_to_unsigned_impls {
    //from signed < to unsigned
    ( $from_type:ty, $add_value:expr; $($into_type:ty),*) => {
        $(
            impl TryFromByAdd<$from_type> for $into_type {
                #[doc = concat!("Converts ", stringify!($from_type), " to equivalent ", stringify!($into_type), ".")]
                #[inline]
                fn try_from_by_add(n: $from_type) -> Option<Self> {
                    Some((n as Self).wrapping_add($add_value))
                }
            }
        )*
    };
}

signed_to_unsigned_impls! { i8, 128; u8, u16, u32, u64, usize, u128 }
signed_to_unsigned_impls! { i16, 32_768; u16, u32, u64, usize, u128 }
signed_to_unsigned_impls! { i32, 2_147_483_648; u32, u64, usize, u128 }
signed_to_unsigned_impls! { i64, 9_223_372_036_854_775_808; u64, usize, u128 }
signed_to_unsigned_impls! { isize, 9_223_372_036_854_775_808; u64, usize, u128 }
signed_to_unsigned_impls! { i128, (i128::MAX as u128) + 1; u128 }
