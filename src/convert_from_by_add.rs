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
    ( $( $for_type:ty; $($from_type:ty),* );* ) => {
        //from signed = to signed
        //from unsigned = to unsigned
        $(
            impl FromByAdd<$for_type> for $for_type {
                #[inline]
                fn from_by_add(n: Self) -> Self {
                    n
                }
            }

            $(
                //from signed < to signed
                //from unsigned < to unsigned
                impl FromByAdd<$from_type> for $for_type {
                    #[inline]
                    fn from_by_add(n: $from_type) -> Self {
                        n as Self
                    }
                }
            )*
        )*
    }
}

macro_rules! unsigned_to_signed_impls {
    ( $for_type:ty, $type:ty; $($as_type:ty, $from_type:ty);* ) => {
        //from unsigned = to signed
        impl FromByAdd<$type> for $for_type {
            #[inline]
            fn from_by_add(n: $type) -> Self {
                ((n as Self).wrapping_add(<Self>::MAX)).wrapping_add(1)
            }
        }
        $(
            //from unsigned < to signed
            impl FromByAdd<$from_type> for $for_type {
                #[inline]
                fn from_by_add(n: $from_type) -> Self {
                    ((n as $as_type).wrapping_add(<$as_type>::MAX)).wrapping_add(1) as Self
                }
            }
        )*
    };
}

macro_rules! signed_to_unsigned_impls {
    //from signed < to unsigned
    ( $for_type:ty; $($add_value:expr, $from_type:ty);*) => {
        $(
            impl FromByAdd<$from_type> for $for_type {
                #[inline]
                fn from_by_add(n: $from_type) -> Self {
                    (n as Self).wrapping_add($add_value)
                }
            }
        )*
    };
}

signed_or_unsigned_impls!{ i8; ; i16; i8; i32; i8, i16; i64; i8, i16, i32, isize; isize; i8, i16, i32, i64; i128; i8, i16, i32, i64, isize }
signed_or_unsigned_impls!{ u8; ; u16; u8; u32; u8, u16; u64; u8, u16, u32, usize; usize; u8, u16, u32, u64; u128; u8, u16, u32, u64, usize }

signed_to_unsigned_impls!{ u8; 128, i8}
signed_to_unsigned_impls!{ u16; 128, i8; 32_768, i16 }
signed_to_unsigned_impls!{ u32; 128, i8; 32_768, i16; 2_147_483_648, i32 }
signed_to_unsigned_impls!{ u64; 128, i8; 32_768, i16; 2_147_483_648, i32; 9_223_372_036_854_775_808, i64; 9_223_372_036_854_775_808, isize }
signed_to_unsigned_impls!{ usize; 128, i8; 32_768, i16; 2_147_483_648, i32; 9_223_372_036_854_775_808, i64; 9_223_372_036_854_775_808, isize }
signed_to_unsigned_impls!{ u128; 128, i8; 32_768, i16; 2_147_483_648, i32; 9_223_372_036_854_775_808, i64; 9_223_372_036_854_775_808, isize;
170_141_183_460_469_231_731_687_303_715_884_105_728, i128 }

unsigned_to_signed_impls!{ i8, u8; }
unsigned_to_signed_impls!{ i16, u16; i8, u8}
unsigned_to_signed_impls!{ i32, u32; i8, u8; i16, u16}
unsigned_to_signed_impls!{ i64, u64; i8, u8; i16, u16; i32, u32; isize, usize }
unsigned_to_signed_impls!{ isize, usize; i8, u8; i16, u16; i32, u32; i64, u64 }
unsigned_to_signed_impls!{ i128, u128; i8, u8; i16, u16; i32, u32; i64, u64; isize, usize }
