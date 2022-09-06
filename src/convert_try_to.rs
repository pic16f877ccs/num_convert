use core::{i128, i16, i32, i64, i8, isize};
use core::{u128, u16, u32, u64, u8, usize};
use paste::paste;

///
/// Convert from signed integers to signed values, see below for details.    
/// -128_i8 -> -128_i8 ... -128_i8 -> -128_i128, -128_i128 -> -128_i8
/// 127_i8 -> 127_i8 ... 127_i8 -> 127_i128, 127_i128 -> 127_i8
/// convert from unsigned integers to unsigned values, see below for details.
/// 0_u8 -> 0_u8 ... 0_u8 -> 0_u128, 0_u128 -> 0_u8
/// 255_u8 -> 255_u8 ... 255_u8 -> 255_u128, 255_u128 -> 255_u8
/// Convert from signed integers to unsigned values, see below for details.
/// -128_i8 -> 0_u8 ... -128_i8 -> 0_u128, -128_i128 -> 0_u8
/// 127_i8 -> 255_u8 ... 127_i8 -> 255_u128, 127_i128 -> 255_u8
/// convert from unsigned integers to signed values, see below for details.
/// 0_u8 -> -128_i8 ... 0_u8 -> -128_i128, 255_u128 -> -128_i8
/// 255_u8 -> 127_i8 ... 255_u8 -> 127_i128, 255_u128 -> 127_i8
///
/// # A generic trait for converting value types.
///
/// # Examples
///
/// ```
/// use num_convert::TryToByAdd;
/// use std::fmt::Debug;
///
/// fn convert_i8_to_u8<T>(min: T, max: T) -> (u8, u8)
/// where
///     T: TryToByAdd,
///     <T as TryToByAdd>::Error: Debug,
/// {
///
///     (min.try_into_u8().unwrap(), max.try_into_u8().unwrap())
/// }
/// assert_eq!((u8::MIN, u8::MAX), convert_i8_to_u8(i8::MIN, i8::MAX));
///
/// assert_eq!(i8::MIN, TryToByAdd::try_into_i8(&i8::MIN).unwrap());
/// assert_eq!(i8::MAX, TryToByAdd::try_into_i8(&i8::MAX).unwrap());
/// assert_eq!(u8::MIN, TryToByAdd::try_into_u8(&i8::MIN).unwrap());
/// assert_eq!(u8::MAX, TryToByAdd::try_into_u8(&i8::MAX).unwrap());
/// assert_eq!(i8::MIN, TryToByAdd::try_into_i8(&u8::MIN).unwrap());
/// assert_eq!(i8::MAX, TryToByAdd::try_into_i8(&u8::MAX).unwrap());
/// assert_eq!(u8::MIN, TryToByAdd::try_into_u8(&u8::MIN).unwrap());
/// assert_eq!(u8::MAX, TryToByAdd::try_into_u8(&u8::MAX).unwrap());
///
/// ```

pub trait TryToByAdd: Sized {
    type Error;

    fn try_into_i8(&self) -> Result<i8, Self::Error>;
    fn try_into_u8(&self) -> Result<u8, Self::Error>;
    fn try_into_i16(&self) -> Result<i16, Self::Error>;
    fn try_into_u16(&self) -> Result<u16, Self::Error>;
    fn try_into_i32(&self) -> Result<i32, Self::Error>;
    fn try_into_u32(&self) -> Result<u32, Self::Error>;
    fn try_into_i64(&self) -> Result<i64, Self::Error>;
    fn try_into_u64(&self) -> Result<u64, Self::Error>;
    fn try_into_isize(&self) -> Result<isize, Self::Error>;
    fn try_into_usize(&self) -> Result<usize, Self::Error>;
    fn try_into_i128(&self) -> Result<i128, Self::Error>;
    fn try_into_u128(&self) -> Result<u128, Self::Error>;
}

macro_rules! signed_or_unsigned {
    ( $($to_type:ty),+ ) => {
        $( paste! {
            fn [<try_into_$to_type>](&self) -> Result<$to_type, Self::Error> {
                Ok(*self as $to_type)
            }
        })*
    }
}

macro_rules! if_signed {
    ( $($value_min:expr, $value_max:expr),+; $($to_type:ty),+ ) => {
        $( paste! {
            fn [<try_into_$to_type>](&self) -> Result<$to_type, Self::Error> {
                if *self < $value_min || *self > $value_max {
                    Err("Cannot be converted")
                } else {
                    Ok(*self as $to_type)
                }
            }
        })*
    }
}

macro_rules! signed_to_unsigned {
    ( $for_type:expr; $($to_type:ty),+ ) => {
        $( paste! {
            fn [<try_into_$to_type>](&self) -> Result<$to_type, Self::Error> {
                Ok((*self as $to_type).wrapping_add($for_type))
            }
        }
        )*
    }
}

macro_rules! if_signed_to_unsigned {
    (  $($value_min:expr, $value_max:expr; $add_value:expr),+; $($to_type:ty),+ ) => {
        $( paste! {
            fn [<try_into_$to_type>](&self) -> Result<$to_type, Self::Error> {
                if *self < $value_min || *self > $value_max {
                    Err("Cannot be converted")
                } else {
                    Ok((*self as $to_type).wrapping_add($add_value))
                }
            }
        }
        )*
    }
}

macro_rules! if_unsigned {
    ( $($value_max:expr),+; $($to_type:ty),+ ) => {
        $( paste! {
            fn [<try_into_$to_type>](&self) -> Result<$to_type, Self::Error> {
                if *self > $value_max {
                    Err("Cannot be converted")
                } else {
                    Ok(*self as $to_type)
                }
            }
        })*
    }
}

macro_rules! unsigned_to_signed {
    ( $for_type:ty; $($to_type:ty),+ ) => {
        $( paste! {
            fn [<try_into_$to_type>](&self) -> Result<$to_type, Self::Error> {
                Ok(((*self as $for_type).wrapping_add(<$for_type>::MAX)).wrapping_add(1) as $to_type)
            }
        })*
    }
}

macro_rules! if_unsigned_to_signed {
    ( $($value_max:expr; $for_type:ty),+; $($to_type:ty),+ ) => {
        $( paste! {
            fn [<try_into_$to_type>](&self) -> Result<$to_type, Self::Error> {
                if *self > $value_max {
                    Err("Cannot be converted")
                } else {
                    Ok(((*self as $for_type).wrapping_add(<$for_type>::MAX)).wrapping_add(1) as $to_type)
                }
            }
        })*
    }
}

macro_rules! signed_impls {
    ( $type_i8:ty, $type_i16:ty, $type_i32:ty, $type_i64:ty, $type_isize:ty, $type_i128:ty;
      $type_u8:ty, $type_u16:ty, $type_u32:ty, $type_u64:ty, $type_usize:ty, $type_u128:ty;
      $val_1:expr, $val_2:expr, $val_3:expr, $val_4:expr, $val_5:expr, $val_6:expr,
      $val_7:expr, $val_8:expr, $val_9:expr, $val_10:expr, $val_11:expr, $val_12:expr) => {

        impl TryToByAdd for $type_i8 {
            type Error = &'static str;

            signed_or_unsigned!($type_i8, $type_i16, $type_i32, $type_i64, $type_isize, $type_i128);
            signed_to_unsigned!($val_3; $type_u8, $type_u16, $type_u32, $type_u64, $type_usize, $type_u128);
        }

        impl TryToByAdd for $type_i16 {
            type Error = &'static str;

            if_signed!($val_1, $val_2; $type_i8);
            signed_or_unsigned!($type_i16, $type_i32, $type_i64, $type_isize, $type_i128);
            if_signed_to_unsigned!($val_1, $val_2; $val_3; $type_u8);
            signed_to_unsigned!($val_6; $type_u16, $type_u32, $type_u64, $type_usize, $type_u128);
        }

        impl TryToByAdd for $type_i32 {
            type Error = &'static str;

            if_signed!($val_1, $val_2, $val_4, $val_5; $type_i8, $type_i16);
            signed_or_unsigned!($type_i32, $type_i64, $type_isize, $type_i128);
            if_signed_to_unsigned!($val_1, $val_2; $val_3, $val_4, $val_5; $val_6; $type_u8, $type_u16);
            signed_to_unsigned!($val_9; $type_u32, $type_u64, $type_usize, $type_u128);
        }

        impl TryToByAdd for $type_i64 {
            type Error = &'static str;

            if_signed!($val_1, $val_2, $val_4, $val_5, $val_7, $val_8; $type_i8, $type_i16, $type_i32);
            signed_or_unsigned!($type_i64, $type_isize, $type_i128);
            if_signed_to_unsigned!($val_1, $val_2; $val_3, $val_4, $val_5; $val_6, $val_7, $val_8; $val_9; $type_u8, $type_u16, $type_u32);
            signed_to_unsigned!($val_12; $type_u64, $type_usize, $type_u128);
        }

        impl TryToByAdd for $type_isize {
            type Error = &'static str;

            if_signed!($val_1, $val_2, $val_4, $val_5, $val_7, $val_8; $type_i8, $type_i16, $type_i32);
            signed_or_unsigned!($type_i64, $type_isize, $type_i128);
            if_signed_to_unsigned!($val_1, $val_2; $val_3, $val_4, $val_5; $val_6, $val_7, $val_8; $val_9; $type_u8, $type_u16, $type_u32);
            signed_to_unsigned!($val_12; $type_u64, $type_usize, $type_u128);
        }

        impl TryToByAdd for $type_i128 {
            type Error = &'static str;

            if_signed!($val_1, $val_2, $val_4, $val_5, $val_7, $val_8, $val_10, $val_11, $val_10, $val_11;
                $type_i8, $type_i16, $type_i32, $type_i64, $type_isize);
            signed_or_unsigned!($type_i128);
            if_signed_to_unsigned!($val_1, $val_2; $val_3, $val_4, $val_5; $val_6, $val_7, $val_8; $val_9,
                $val_10, $val_11; $val_12, $val_10, $val_11; $val_12; $type_u8, $type_u16, $type_u32, $type_u64, $type_usize);
            signed_to_unsigned!(170_141_183_460_469_231_731_687_303_715_884_105_728; $type_u128);
        }
    }
}

macro_rules! unsigned_impls {
    ( $type_i8:ty, $type_i16:ty, $type_i32:ty, $type_i64:ty, $type_isize:ty, $type_i128:ty;
      $type_u8:ty, $type_u16:ty, $type_u32:ty, $type_u64:ty, $type_usize:ty, $type_u128:ty;
      $value_8bit: expr, $value_16bit: expr, $value_32bit:expr, $value_64bit:expr ) => {

        impl TryToByAdd for $type_u8 {
            type Error = &'static str;

            unsigned_to_signed!($type_i8; $type_i8, $type_i16, $type_i32, $type_i64, $type_isize, $type_i128);
            signed_or_unsigned!($type_u8, $type_u16, $type_u32, $type_u64, $type_usize, $type_u128);
        }

        impl TryToByAdd for $type_u16 {
            type Error = &'static str;

            if_unsigned_to_signed!($value_8bit; $type_i8; $type_i8);
            unsigned_to_signed!($type_i16; $type_i16, $type_i32, $type_i64, $type_isize, $type_i128);
            if_unsigned!($value_8bit; $type_u8);
            signed_or_unsigned!($type_u16, $type_u32, $type_u64, $type_usize, $type_u128);
        }

        impl TryToByAdd for $type_u32 {
            type Error = &'static str;

            if_unsigned_to_signed!($value_8bit; $type_i8, $value_16bit; $type_i16; $type_i8, $type_i16);
            unsigned_to_signed!($type_i32; $type_i32, $type_i64, $type_isize, $type_i128);
            if_unsigned!($value_8bit, $value_16bit; $type_u8, $type_u16);
            signed_or_unsigned!($type_u32, $type_u64, $type_usize, $type_u128);
        }

        impl TryToByAdd for $type_u64 {
            type Error = &'static str;

            if_unsigned_to_signed!($value_8bit; $type_i8, $value_16bit; $type_i16, $value_32bit; $type_i32; $type_i8, $type_i16, $type_i32);
            unsigned_to_signed!($type_i64; $type_i64, $type_isize, $type_i128);
            if_unsigned!($value_8bit, $value_16bit, $value_32bit; $type_u8, $type_u16, $type_u32);
            signed_or_unsigned!($type_u64, $type_usize, $type_u128);
        }

        impl TryToByAdd for $type_usize {
            type Error = &'static str;

            if_unsigned_to_signed!($value_8bit; $type_i8, $value_16bit; $type_i16, $value_32bit; $type_i32; $type_i8, $type_i16, $type_i32);
            unsigned_to_signed!($type_isize; $type_i64, $type_isize, $type_i128);
            if_unsigned!($value_8bit, $value_16bit, $value_32bit; $type_u8, $type_u16, $type_u32);
            signed_or_unsigned!($type_u64, $type_usize, $type_u128);
        }

        impl TryToByAdd for $type_u128 {
            type Error = &'static str;

            if_unsigned_to_signed!($value_8bit; $type_i8, $value_16bit; $type_i16, $value_32bit; $type_i32,
                $value_64bit; $type_i64, $value_64bit; $type_isize; $type_i8, $type_i16, $type_i32, $type_i64, $type_isize);
            unsigned_to_signed!($type_i128; $type_i128);
            if_unsigned!($value_8bit, $value_16bit, $value_32bit, $value_64bit, $value_64bit;
                $type_u8, $type_u16, $type_u32, $type_u64, $type_usize);
            signed_or_unsigned!($type_u128);
        }
    }
}

signed_impls!(
    i8, i16, i32, i64, isize, i128;
    u8, u16, u32, u64, usize, u128;
    -128, 127, 128,
    -32_768, 32_767, 32_768,
    -2_147_483_648, 2_147_483_647, 2_147_483_648,
    -9_223_372_036_854_775_808, 9_223_372_036_854_775_807, 9_223_372_036_854_775_808
);

unsigned_impls!(
    i8, i16, i32, i64, isize, i128;
    u8, u16, u32, u64, usize, u128;
    255, 65_535, 4_294_967_295, 18_446_744_073_709_551_615
);
