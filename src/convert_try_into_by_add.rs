/// A generic trait for converting into possible types.
///
/// # Examples
/// Usage:
///
/// ```
/// # use num_convert::TryIntoByAdd;
/// fn convert_try_into_u8<T: TryIntoByAdd<u8>>(min: T, max: T) -> (u8, u8) {
///     (min.try_into_by_add().unwrap(), max.try_into_by_add().unwrap())
/// }
/// assert_eq!(convert_try_into_u8(i8::MIN, i8::MAX), (u8::MIN, u8::MAX));
///
/// assert_eq!(TryIntoByAdd::<u8>::try_into_by_add(127_i16), Some(255u8));
/// assert_eq!(TryIntoByAdd::<u8>::try_into_by_add(128_i16), None);
/// assert_eq!(TryIntoByAdd::<u8>::try_into_by_add(-128_i16), Some(0_u8));
/// assert_eq!(TryIntoByAdd::<u8>::try_into_by_add(-129_i16), None);
/// ```
pub trait TryIntoByAdd<T> {
    /// Converts the value of `self` to an `T`.
    fn try_into_by_add(self) -> Option<T>;
}

macro_rules! signed_or_unsigned_impls {
    ( $( $for_type:ty; $($into_type:ty),* );* ) => {
        //signed = to signed
        //unsigned = to unsigned
        $(
            impl TryIntoByAdd<$for_type> for $for_type {
                #[doc = concat!("Returns the same type ", stringify!($for_type), ".")]
                #[inline]
                fn try_into_by_add(self) -> Option<$for_type> {
                    Some(self)
                }
            }

            $(
                //signed < to signed
                //unsigned < to unsigned
                impl TryIntoByAdd<$into_type> for $for_type {
                    #[doc = concat!("Converts ", stringify!($for_type), " to ", stringify!($into_type), " losslessly.")]
                    #[inline]
                    fn try_into_by_add(self) -> Option<$into_type> {
                        Some(self as $into_type)
                    }
                }
            )*
        )*
    }
}

signed_or_unsigned_impls! { i8; i16, i32, i64, isize, i128; i16; i32, i64, isize, i128 }
signed_or_unsigned_impls! { i32; i64, isize, i128; i64; isize, i128; isize; i64, i128; i128; }
signed_or_unsigned_impls! { u8; u16, u32, u64, usize, u128; u16; u32, u64, usize, u128 }
signed_or_unsigned_impls! { u32; u64, usize, u128; u64; usize, u128; usize; u64, u128; u128; }

macro_rules! signed_to_signed_gt {
    ( $into_type:ty, $value_min:expr, $value_max:expr; $($for_type:ty),* ) => {
        $(
            impl TryIntoByAdd<$into_type> for $for_type {
                #[doc = concat!("Converts ", stringify!($for_type), " to ", stringify!($into_type), ". Conversion can fail.")]
                #[inline]
                fn try_into_by_add(self) -> Option<$into_type> {
                    if self < $value_min || self > $value_max {
                        None
                    } else {
                        Some(self as $into_type)
                    }
                }
            }
        )*
    }
}

signed_to_signed_gt! { i8, -128, 127; i16, i32, i64, isize, i128 }
signed_to_signed_gt! { i16, -32_768, 32_767; i32, i64, isize, i128 }
signed_to_signed_gt! { i32, -2_147_483_648, 2_147_483_647; i64, isize, i128 }
signed_to_signed_gt! { i64, -9_223_372_036_854_775_808, 9_223_372_036_854_775_807; i128 }
signed_to_signed_gt! { isize, -9_223_372_036_854_775_808, 9_223_372_036_854_775_807; i128 }

macro_rules! signed_to_unsigned_gt {
    ( $into_type:ty, $value_min:expr, $value_max:expr, $add_value:expr; $($for_type:ty),* ) => {
        $(
            impl TryIntoByAdd<$into_type> for $for_type {
                #[doc = concat!("Converts ", stringify!($for_type), " to equvalent ", stringify!($into_type), ". Conversion can fail.")]
                #[inline]
                fn try_into_by_add(self) -> Option<$into_type> {
                    if self < $value_min || self > $value_max {
                        None
                    } else {
                        Some((self as $into_type).wrapping_add($add_value))
                    }
                }
            }
        )*
    }
}

signed_to_unsigned_gt! { u8, -128, 127, 128; i16, i32, i64, isize, i128 }
signed_to_unsigned_gt! { u16, -32_768, 32_767, 32_768; i32, i64, isize, i128 }
signed_to_unsigned_gt! { u32, -2_147_483_648, 2_147_483_647, 2_147_483_648; i64, isize, i128 }
signed_to_unsigned_gt! { u64, -9_223_372_036_854_775_808, 9_223_372_036_854_775_807, 9_223_372_036_854_775_808; i128 }
signed_to_unsigned_gt! { usize, -9_223_372_036_854_775_808, 9_223_372_036_854_775_807, 9_223_372_036_854_775_808; i128 }

macro_rules! unsigned_to_unsigned_gt {
    ( $into_type:ty, $value_max:expr; $($for_type:ty),* ) => {
        $(
            impl TryIntoByAdd<$into_type> for $for_type {
                #[doc = concat!("Converts ", stringify!($for_type), " to ", stringify!($into_type), ". Conversion can fail.")]
                #[inline]
                fn try_into_by_add(self) -> Option<$into_type> {
                    if self > $value_max {
                        None
                    } else {
                        Some(self as $into_type)
                    }
                }
            }
        )*
    }
}

unsigned_to_unsigned_gt! { u8, 255; u16, u32, u64, usize, u128 }
unsigned_to_unsigned_gt! { u16, 65_535; u32, u64, usize, u128 }
unsigned_to_unsigned_gt! { u32, 4_294_967_295; u64, usize, u128 }
unsigned_to_unsigned_gt! { u64, 18_446_744_073_709_551_615; u128 }
unsigned_to_unsigned_gt! { usize, 18_446_744_073_709_551_615; u128 }

macro_rules! unsigned_to_signed_gt {
    ( $into_type:ty, $value_max:expr; $($for_type:ty),* ) => {
        $(
            impl TryIntoByAdd<$into_type> for $for_type {
                #[doc = concat!("Converts ", stringify!($for_type), " to equvalent ", stringify!($into_type), ". Conversion can fail.")]
                #[inline]
                fn try_into_by_add(self) -> Option<$into_type> {
                    if self > $value_max {
                        None
                    } else {
                        Some(((self as $into_type).wrapping_add(<$into_type>::MAX)).wrapping_add(1))
                    }
                }
            }
        )*
    }
}

unsigned_to_signed_gt! { i8, 255; u16, u32, u64, usize, u128 }
unsigned_to_signed_gt! { i16, 65_535; u32, u64, usize, u128 }
unsigned_to_signed_gt! { i32, 4_294_967_295; u64, usize, u128 }
unsigned_to_signed_gt! { i64, 18_446_744_073_709_551_615; u128 }
unsigned_to_signed_gt! { isize, 18_446_744_073_709_551_615; u128 }

macro_rules! unsigned_to_signed_impls {
    ( $( $for_type:ty, $as_type:ty; $($into_type:ty),* );* ) => {
        //unsigned = to signed
        $(
            impl TryIntoByAdd<$as_type> for $for_type {
                #[doc = concat!("Converts ", stringify!($for_type), " to equvalent ", stringify!($as_type), ".")]
                #[inline]
                fn try_into_by_add(self) -> Option<$as_type> {
                    Some(((self as $as_type).wrapping_add(<$as_type>::MAX)).wrapping_add(1))
                }
            }

            $(
                //unsigned < to signed
                impl TryIntoByAdd<$into_type> for $for_type {
                    #[doc = concat!("Converts ", stringify!($for_type), " to equvalent ", stringify!($into_type), ".")]
                    #[inline]
                    fn try_into_by_add(self) -> Option<$into_type> {
                        Some(((self as $as_type).wrapping_add(<$as_type>::MAX)).wrapping_add(1) as $into_type)
                    }
                }
            )*
        )*
    };
}

unsigned_to_signed_impls! { u8, i8; i16, i32, i64, isize, i128; u16, i16; i32, i64, isize, i128 }
unsigned_to_signed_impls! { u32, i32; i64, isize, i128; u64, i64; isize, i128; usize, isize; i64, i128; u128, i128; }

macro_rules! signed_to_unsigned_impls {
    //signed < to unsigned
    ( $for_type:ty; $add_value:expr; $($into_type:ty),*) => {
        $(
            impl TryIntoByAdd<$into_type> for $for_type {
                #[doc = concat!("Converts ", stringify!($for_type), " to equvalent ", stringify!($into_type), ".")]
                #[inline]
                fn try_into_by_add(self) -> Option<$into_type> {
                    Some((self as $into_type).wrapping_add($add_value))
                }
            }
        )*
    };
}

signed_to_unsigned_impls! { i8; 128; u8, u16, u32, u64, usize, u128 }
signed_to_unsigned_impls! { i16; 32_768; u16, u32, u64, usize, u128 }
signed_to_unsigned_impls! { i32; 2_147_483_648; u32, u64, usize, u128 }
signed_to_unsigned_impls! { i64; 9_223_372_036_854_775_808; u64, usize, u128 }
signed_to_unsigned_impls! { isize; 9_223_372_036_854_775_808; u64, usize, u128 }
signed_to_unsigned_impls! { i128; 170_141_183_460_469_231_731_687_303_715_884_105_728; u128 }
