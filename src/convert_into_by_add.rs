/// A generic trait for converting into possible types.
///
/// # Examples
/// Usage:
///
/// ```
/// # use num_convert::IntoByAdd;
/// //fn convert_into_u8<T: IntoByAdd>(min: T, max: T) -> (u8, u8) {
/// //    (min.into_by_add(), max.into_by_add())
/// //}
/// //assert_eq!(convert_into_u8(i8::MIN, i8::MAX), (u8::MIN, u8::MAX));
///
/// assert_eq!(<i16 as IntoByAdd<u16>>::into_by_add(  i16::MIN), u16::MIN as u16);
/// assert_eq!(<i32 as IntoByAdd<u128>>::into_by_add( i32::MIN), u32::MIN as u128);
/// //assert_eq!(IntoByAdd::into_by_add(u128::MIN), i128::MIN);
/// //assert_eq!(IntoByAdd::into_by_add(i8::MAX), u8::MAX as u64);
/// //```
pub trait IntoByAdd<T> {
    /// Converts the value of `self` to an `T`.
    fn into_by_add(self) -> T;
}

macro_rules! signed_or_unsigned_impls {
    ( $( $for_type:ty; $($into_type:ty),* );* ) => {
        //signed = to signed
        //unsigned = to unsigned
        $(
            impl IntoByAdd<$for_type> for $for_type {
                #[inline]
                fn into_by_add(self) -> $for_type {
                    self
                }
            }

            $(
                //signed < to signed
                //unsigned < to unsigned
                impl IntoByAdd<$into_type> for $for_type {
                    #[inline]
                    fn into_by_add(self) -> $into_type {
                        self as $into_type
                    }
                }
            )*
        )*
    }
}

macro_rules! unsigned_to_signed_impls {
    ( $for_type:ty, $type:ty; $($as_type:ty, $into_type:ty);* ) => {
        //unsigned = to signed
        impl IntoByAdd<$type> for $for_type {
            #[inline]
            fn into_by_add(self) -> $type {
                ((self as $type).wrapping_add(<$type>::MAX)).wrapping_add(1)
            }
        }
        $(
            //unsigned < to signed
            impl IntoByAdd<$into_type> for $for_type {
                #[inline]
                fn into_by_add(self) -> $into_type {
                    ((self as $as_type).wrapping_add(<$as_type>::MAX)).wrapping_add(1) as $into_type
                }
            }
        )*
    };
}

macro_rules! signed_to_unsigned_impls {
    //signed < to unsigned
    ( $for_type:ty; $add_value:expr; $($into_type:ty),*) => {
        $(
            impl IntoByAdd<$into_type> for $for_type {
                #[inline]
                fn into_by_add(self) -> $into_type {
                    (self as $into_type).wrapping_add($add_value)
                }
            }
        )*
    };
}

signed_or_unsigned_impls!{ i8; i16, i32, i64, isize, i128; i16; i32, i64, isize, i128;
i32; i64, isize, i128; i64; isize, i128; isize; i64, i128; i128; }
signed_or_unsigned_impls!{ u8; u16, u32, u64, usize, u128; u16; u32, u64, usize, u128;
u32; u64, usize, u128; u64; usize, u128; usize; u64, u128; u128; }

signed_to_unsigned_impls!{ i8; 128; u8, u16, u32, u64, usize, u128 }
signed_to_unsigned_impls!{ i16; 32_768; u16, u32, u64, usize, u128 }
signed_to_unsigned_impls!{ i32; 2_147_483_648; u32, u64, usize, u128 }
signed_to_unsigned_impls!{ i64; 9_223_372_036_854_775_808; u64, usize, u128 }
signed_to_unsigned_impls!{ isize; 9_223_372_036_854_775_808; u64, usize, u128 }
signed_to_unsigned_impls!{ i128; 170_141_183_460_469_231_731_687_303_715_884_105_728; u128 }

unsigned_to_signed_impls!{ u8, i8; u16, i16; u32, i32; u64, i64; usize, isize; u128, i128 }
unsigned_to_signed_impls!{ u16, i16; u32, i32; u64, i64; usize, isize; u128, i128 }
unsigned_to_signed_impls!{ u32, i32; u64, i64; usize, isize; u128, i128 }
unsigned_to_signed_impls!{ u64, i64; usize, isize; u128, i128 }
unsigned_to_signed_impls!{ usize, isize; u64, i64; u128, i128 }
unsigned_to_signed_impls!{ u128, i128; }
