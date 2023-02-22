/// A generic trait for converting from possible types.
///
/// # Examples
/// Usage:
///
/// ```
/// # use num_convert::FromByAdd;
/// // -128_i8 -> 0_u32
/// assert_eq!(<u32 as FromByAdd<i8>>::from_by_add(<i8>::MIN), <u8>::MIN as u32);
/// // 127_i8 -> 255_u64
/// assert_eq!(<u64 as FromByAdd<i8>>::from_by_add(<i8>::MAX), <u8>::MAX as u64);
/// ```
pub trait FromByAdd<T> {
    /// Converts the value from `T` to `self`.
    fn from_by_add(n: T) -> Self;
}

macro_rules! signed_or_unsigned_impls {
    ( $( $from_type:ty; $($into_type:ty),* );* ) => {
        $(
            //from signed = to signed
            //from unsigned = to unsigned
            impl FromByAdd<$from_type> for $from_type {
                #[doc = concat!("Returns the same type ", stringify!($from_type), ".")]
                #[inline]
                fn from_by_add(n: Self) -> Self {
                    n
                }
            }

            $(
                //from signed < to signed
                //from unsigned < to unsigned
                impl FromByAdd<$from_type> for $into_type {
                    #[doc = concat!("Converts ", stringify!($from_type), " to ", stringify!($into_type), " losslessly.")]
                    #[inline]
                    fn from_by_add(n: $from_type) -> Self {
                        n as Self
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

macro_rules! unsigned_to_signed_impls {
    ( $from_type:ty, $as_type:ty; $($into_type:ty),* ) => {
        //from unsigned = to signed
        impl FromByAdd<$from_type> for $as_type {
            #[doc = concat!("Converts ", stringify!($from_type), " to equvalent ", stringify!($as_type), ".")]
            #[inline]
            fn from_by_add(n: $from_type) -> Self {
                ((n as Self).wrapping_add(<Self>::MAX)).wrapping_add(1)
            }
        }
        $(
            //from unsigned < to signed
            impl FromByAdd<$from_type> for $into_type {
                #[doc = concat!("Converts ", stringify!($from_type), " to equvalent ", stringify!($into_type), ".")]
                #[inline]
                fn from_by_add(n: $from_type) -> Self {
                    ((n as $as_type).wrapping_add(<$as_type>::MAX)).wrapping_add(1) as Self
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
            impl FromByAdd<$from_type> for $into_type {
                #[doc = concat!("Converts ", stringify!($from_type), " to equvalent ", stringify!($into_type), ".")]
                #[inline]
                fn from_by_add(n: $from_type) -> Self {
                    (n as Self).wrapping_add($add_value)
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
