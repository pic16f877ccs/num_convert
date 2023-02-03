/// A generic trait for converting from possible types.
///
/// # Examples
/// Usage:
///
/// ```
/// # use num_convert::TryFromByAdd;
/// // -128_i8 -> 0_u32
/// assert_eq!(<u32 as TryFromByAdd<i8>>::try_from_by_add(<i8>::MIN), <u8>::MIN as u32);
/// // 127_i8 -> 255_u64
/// assert_eq!(<u64 as TryFromByAdd<i8>>::try_from_by_add(<i8>::MAX), <u8>::MAX as u64);
/// ```
pub trait TryFromByAdd<T> {
    /// Converts the value from `T` to `self`.
    fn try_from_by_add(n: T) -> Option<Self> where Self: Sized;
}

macro_rules! signed_or_unsigned_impls {
    ( $( $for_type:ty; $($from_type:ty),* );* ) => {
        //from signed = to signed
        //from unsigned = to unsigned
        $(
            impl TryFromByAdd<$for_type> for $for_type {
                #[inline]
                fn try_from_by_add(n: Self) -> Option<Self> {
                    Some(n)
                }
            }

            $(
                //from signed < to signed
                //from unsigned < to unsigned
                impl TryFromByAdd<$from_type> for $for_type {
                    #[inline]
                    fn try_from_by_add(n: $from_type) -> Option<Self> {
                        Some(n as Self)
                    }
                }
            )*
        )*
    }
}

signed_or_unsigned_impls!{ i8; ; i16; i8; i32; i8, i16; i64; i8, i16, i32, isize; isize; i8, i16, i32, i64; i128; i8, i16, i32, i64, isize }
signed_or_unsigned_impls!{ u8; ; u16; u8; u32; u8, u16; u64; u8, u16, u32, usize; usize; u8, u16, u32, u64; u128; u8, u16, u32, u64, usize }

macro_rules! signed_gt_signed_impls {
    ( $( $for_type:ty; $($from_type:ty),* );* ) => {
        $(
            $(
                //from signed > to signed
                impl TryFromByAdd<$from_type> for $for_type {
                    #[inline]
                    fn try_from_by_add(n: $from_type) -> Option<Self> {
                        if n >= <$from_type>::MIN && n <= <$from_type>::MAX {
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

signed_gt_signed_impls!{ i8; i16, i32, i64, isize, i128; i16; i32, i64, isize, i128; i32; i64, isize, i128; i64; i128; isize; i128  }

macro_rules! signed_gt_unsigned_impls {
    ( $( $for_type:ty, $max_value:expr; $($from_type:ty),* );* ) => {
        $(
            $(
                //from signed > to unsigned
                impl TryFromByAdd<$from_type> for $for_type {
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

signed_gt_unsigned_impls!{ i8, 255; u16, u32, u64, usize, u128; i16, 65_535; u32, u64, usize, u128; i32, 4_294_967_295; u64, usize, u128;
    i64, 18_446_744_073_709_551_615; u128; isize, 18_446_744_073_709_551_615; u128  }

macro_rules! unsigned_gt_unsigned_impls {
    ( $( $for_type:ty; $($from_type:ty),* );* ) => {
        $(
            $(
                //from unsigned > to unsigned
                impl TryFromByAdd<$from_type> for $for_type {
                    #[inline]
                    fn try_from_by_add(n: $from_type) -> Option<Self> {
                        if n <= <$from_type>::MAX {
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

unsigned_gt_unsigned_impls!{ u8; u16, u32, u64, usize, u128; u16; u32, u64, usize, u128; u32; u64, usize, u128; u64; u128; usize; u128  }

macro_rules! unsigned_gt_signed_impls {
    ( $( $for_type:ty, $min_value:expr, $max_value:expr, $add_value:expr; $($from_type:ty),* );* ) => {
        $(
            $(
                //from unsigned < to signed
                impl TryFromByAdd<$from_type> for $for_type {
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
        )*
    }
}

unsigned_gt_signed_impls!{ u8, -128, 127, 128; i16, i32, i64, isize, i128; u16, -32_768, 32_767, 32_768; i32, i64, isize, i128;
    u32, -2_147_483_648, 2_147_483_647, 2_147_483_648; i64, isize, i128; u64, -9_223_372_036_854_775_808, 9_223_372_036_854_775_807, 9_223_372_036_854_775_808; i128; usize, -9_223_372_036_854_775_808, 9_223_372_036_854_775_807, 9_223_372_036_854_775_808; i128  }

macro_rules! unsigned_to_signed_impls {
    ( $for_type:ty, $type:ty; $($as_type:ty, $from_type:ty);* ) => {
        //from unsigned = to signed
        impl TryFromByAdd<$type> for $for_type {
            #[inline]
            fn try_from_by_add(n: $type) -> Option<Self> {
                Some(((n as Self).wrapping_add(<Self>::MAX)).wrapping_add(1))
            }
        }
        $(
            //from unsigned < to signed
            impl TryFromByAdd<$from_type> for $for_type {
                #[inline]
                fn try_from_by_add(n: $from_type) -> Option<Self> {
                    Some(((n as $as_type).wrapping_add(<$as_type>::MAX)).wrapping_add(1) as Self)
                }
            }
        )*
    };
}

macro_rules! signed_to_unsigned_impls {
    //from signed < to unsigned
    ( $for_type:ty; $($add_value:expr, $from_type:ty);*) => {
        $(
            impl TryFromByAdd<$from_type> for $for_type {
                #[inline]
                fn try_from_by_add(n: $from_type) -> Option<Self> {
                    Some((n as Self).wrapping_add($add_value))
                }
            }
        )*
    };
}


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
