use paste::paste;

/// A generic trait for converting into possible types.
///
/// # Examples
/// Usage:
///
/// ```
/// # use num_convert::ToByAdd;
/// fn convert_into_u8<T: ToByAdd>(min: T, max: T) -> (u8, u8) {
///     (min.into_u8(), max.into_u8())
/// }
/// assert_eq!(convert_into_u8(i8::MIN, i8::MAX), (u8::MIN, u8::MAX));
///
/// assert_eq!(ToByAdd::into_i128(u128::MIN), i128::MIN);
/// assert_eq!(ToByAdd::into_u64(i8::MAX), u8::MAX as u64);
/// ```
pub trait ToByAdd {
    /// Converts the value of `self` to an `i8`.
    fn into_i8(self) -> i8;

    /// Converts the value of `self` to an `u8`.
    fn into_u8(self) -> u8;

    /// Converts the value of `self` to an `i16`.
    fn into_i16(self) -> i16;

    /// Converts the value of `self` to an `u16`.
    fn into_u16(self) -> u16;

    /// Converts the value of `self` to an `i32`.
    fn into_i32(self) -> i32;

    /// Converts the value of `self` to an `u32`.
    fn into_u32(self) -> u32;

    /// Converts the value of `self` to an `i64`.
    fn into_i64(self) -> i64;

    /// Converts the value of `self` to an `u64`.
    fn into_u64(self) -> u64;

    /// Converts the value of `self` to an `isize`.
    fn into_isize(self) -> isize;

    /// Converts the value of `self` to an `usize`.
    fn into_usize(self) -> usize;

    /// Converts the value of `self` to an `i128`.
    fn into_i128(self) -> i128;

    /// Converts the value of `self` to an `u128`.
    fn into_u128(self) -> u128;
}

macro_rules! return_self_value {
    ( $($to_type:ty),+ ) => {
        $( paste! {
            #[inline]
            fn [<into_$to_type>](self) -> $to_type {
                self
            }
        })*
    }
}

macro_rules! signed_or_unsigned {
    ( $($to_type:ty),+ ) => {
        $( paste! {
            #[inline]
            fn [<into_$to_type>](self) -> $to_type {
                self as $to_type
            }
        })*
    }
}

macro_rules! unreach_func {
    ( $($to_type:ty),+ ) => {
        $( paste! {
            #[inline]
            fn [<into_$to_type>](self) -> $to_type {
                unimplemented!();
            }
        })*
    }
}

macro_rules! signed_to_unsigned {
    ( $for_type:expr; $($to_type:ty),+ ) => {
        $( paste! {
            #[inline]
            fn [<into_$to_type>](self) -> $to_type {
                (self as $to_type).wrapping_add($for_type)
            }
        })*
    }
}

macro_rules! unsigned_to_signed {
    ( $for_type:ty; $($to_type:ty),+ ) => {
        $( paste! {
            #[inline]
            fn [<into_$to_type>](self) -> $to_type {
                ((self as $for_type).wrapping_add(<$for_type>::MAX)).wrapping_add(1) as $to_type
            }
        })*
    }
}

macro_rules! signed_impls {
    ( $type_i8:ty, $type_i16:ty, $type_i32:ty, $type_i64:ty, $type_isize:ty, $type_i128:ty;
        $type_u8:ty, $type_u16:ty, $type_u32:ty, $type_u64:ty, $type_usize:ty, $type_u128:ty;
            $value_8bit:expr, $value_16bit:expr, $value_32bit:expr, $value_64bit:expr, $value_32or64bit:expr ) => {

        impl ToByAdd for $type_i8 {
            return_self_value!($type_i8);
            signed_or_unsigned!($type_i16, $type_i32, $type_i64, $type_isize, $type_i128);
            signed_to_unsigned!($value_8bit; $type_u8, $type_u16, $type_u32, $type_u64, $type_usize, $type_u128);
        }

        impl ToByAdd for $type_i16 {
            unreach_func!($type_i8);
            return_self_value!($type_i16);
            signed_or_unsigned!($type_i32, $type_i64, $type_isize, $type_i128);
            unreach_func!($type_u8);
            signed_to_unsigned!($value_16bit; $type_u16, $type_u32, $type_u64, $type_usize, $type_u128);
        }

        impl ToByAdd for $type_i32 {
            unreach_func!($type_i8, $type_i16);
            return_self_value!($type_i32);
            signed_or_unsigned!($type_i64, $type_isize, $type_i128);
            unreach_func!($type_u8, $type_u16);
            signed_to_unsigned!($value_32bit; $type_u32, $type_u64, $type_usize, $type_u128);
        }

        #[cfg(target_pointer_width = "32")]
        impl ToByAdd for $type_i64 {
            unreach_func!($type_i8, $type_i16, $type_i32, $type_isize);
            return_self_value!($type_i64);
            signed_or_unsigned!($type_i128);
            unreach_func!($type_u8, $type_u16, $type_u32, $type_usize);
            signed_to_unsigned!($value_64bit; $type_u64, $type_u128);
        }

        #[cfg(target_pointer_width = "64")]
        impl ToByAdd for $type_i64 {
            unreach_func!($type_i8, $type_i16, $type_i32);
            return_self_value!($type_i64);
            signed_or_unsigned!($type_isize, $type_i128);
            unreach_func!($type_u8, $type_u16, $type_u32);
            signed_to_unsigned!($value_64bit; $type_u64, $type_usize, $type_u128);
        }

        #[cfg(target_pointer_width = "32")]
        impl ToByAdd for $type_isize {
            unreach_func!($type_i8, $type_i16);
            return_self_value!($type_isize);
            signed_or_unsigned!($type_i32, $type_i64, $type_i128);
            unreach_func!($type_u8, $type_u16);
            signed_to_unsigned!($value_32or64bit; $type_u32, $type_u64, $type_usize, $type_u128);
        }

        #[cfg(target_pointer_width = "64")]
        impl ToByAdd for $type_isize {
            unreach_func!($type_i8, $type_i16, $type_i32);
            return_self_value!($type_isize);
            signed_or_unsigned!($type_i64, $type_i128);
            unreach_func!($type_u8, $type_u16, $type_u32);
            signed_to_unsigned!($value_32or64bit; $type_u64, $type_usize, $type_u128);
        }

        impl ToByAdd for $type_i128 {
            unreach_func!($type_i8, $type_i16, $type_i32, $type_i64, $type_isize);
            return_self_value!($type_i128);
            unreach_func!($type_u8, $type_u16, $type_u32, $type_u64, $type_usize);
            signed_to_unsigned!(170_141_183_460_469_231_731_687_303_715_884_105_728; $type_u128);
        }
    }
}

macro_rules! unsigned_impls {
    ( $type_i8:ty, $type_i16:ty, $type_i32:ty, $type_i64:ty, $type_isize:ty, $type_i128:ty;
        $type_u8:ty, $type_u16:ty, $type_u32:ty, $type_u64:ty, $type_usize:ty, $type_u128:ty; ) => {

        impl ToByAdd for $type_u8 {
            unsigned_to_signed!($type_i8;$type_i8, $type_i16, $type_i32, $type_i64, $type_isize, $type_i128);
            return_self_value!($type_u8);
            signed_or_unsigned!($type_u16, $type_u32, $type_u64, $type_usize, $type_u128);
        }

        impl ToByAdd for $type_u16 {
            unreach_func!($type_i8);
            unsigned_to_signed!($type_i16; $type_i16, $type_i32, $type_i64, $type_isize, $type_i128);
            unreach_func!($type_u8);
            return_self_value!($type_u16);
            signed_or_unsigned!($type_u32, $type_u64, $type_usize, $type_u128);
        }

        impl ToByAdd for $type_u32 {
            unreach_func!($type_i8, $type_i16);
            unsigned_to_signed!($type_i32; $type_i32, $type_i64, $type_isize, $type_i128);
            unreach_func!($type_u8, $type_u16);
            return_self_value!($type_u32);
            signed_or_unsigned!($type_u64, $type_usize, $type_u128);
        }

        #[cfg(target_pointer_width = "32")]
        impl ToByAdd for $type_u64 {
            unreach_func!($type_i8, $type_i16, $type_i32, $type_isize );
            unsigned_to_signed!($type_i64; $type_i64, $type_i128);
            unreach_func!($type_u8, $type_u16, $type_u32, $type_usize);
            return_self_value!($type_u64);
            signed_or_unsigned!($type_u128);
        }

        #[cfg(target_pointer_width = "64")]
        impl ToByAdd for $type_u64 {
            unreach_func!($type_i8, $type_i16, $type_i32);
            unsigned_to_signed!($type_i64; $type_i64, $type_isize, $type_i128);
            unreach_func!($type_u8, $type_u16, $type_u32);
            return_self_value!($type_u64);
            signed_or_unsigned!($type_usize, $type_u128);
        }

        #[cfg(target_pointer_width = "32")]
        impl ToByAdd for $type_usize {
            unreach_func!($type_i8, $type_i16);
            unsigned_to_signed!($type_isize; $type_i32, $type_i64, $type_isize, $type_i128);
            unreach_func!($type_u8, $type_u16);
            return_self_value!($type_usize);
            signed_or_unsigned!($type_u32, $type_u64, $type_u128);
        }

        #[cfg(target_pointer_width = "64")]
        impl ToByAdd for $type_usize {
            unreach_func!($type_i8, $type_i16, $type_i32);
            unsigned_to_signed!($type_isize; $type_i64, $type_isize, $type_i128);
            unreach_func!($type_u8, $type_u16, $type_u32);
            return_self_value!($type_usize);
            signed_or_unsigned!($type_u64, $type_u128);
        }

        impl ToByAdd for $type_u128 {
            unreach_func!($type_i8, $type_i16, $type_i32, $type_i64, $type_isize);
            unsigned_to_signed!($type_i128; $type_i128);
            unreach_func!($type_u8, $type_u16, $type_u32, $type_u64, $type_usize);
            return_self_value!($type_u128);
        }
    }
}

// For adding 32 bit arch.
#[cfg(target_pointer_width = "32")]
signed_impls!(
    i8, i16, i32, i64, isize, i128;
        u8, u16, u32, u64, usize, u128;
            128, 32_768, 2_147_483_648,
                9_223_372_036_854_775_808, 2_147_483_648
);

// For adding 64 bit arch.
#[cfg(target_pointer_width = "64")]
signed_impls!(
    i8, i16, i32, i64, isize, i128;
        u8, u16, u32, u64, usize, u128;
            128, 32_768, 2_147_483_648,
                9_223_372_036_854_775_808, 9_223_372_036_854_775_808
);

unsigned_impls!(
    i8, i16, i32, i64, isize, i128;
        u8, u16, u32, u64, usize, u128;
);
