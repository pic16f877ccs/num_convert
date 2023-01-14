use paste::paste;

/// A trait that converts negative integers to positive and positive to negative.
///
/// # Examples
/// Usage:
///
/// ```
/// # use num_convert::TryToByAdd;
/// fn convert_into_u8<T>(min: T, max: T) -> (u8, u8)
/// where
///     T: TryToByAdd,
/// {
///     (min.try_into_u8().unwrap(), max.try_into_u8().unwrap())
/// }
/// assert_eq!((u8::MIN, u8::MAX), convert_into_u8(i8::MIN, i8::MAX));
/// assert_eq!((u8::MIN, u8::MAX), convert_into_u8(i8::MIN as i64, i8::MAX as i64));
///
/// assert_eq!(u8::MAX, TryToByAdd::try_into_u8(i8::MAX).unwrap());
/// assert_eq!(i8::MIN, TryToByAdd::try_into_i8(u8::MIN).unwrap());
/// assert_eq!(u8::MIN, TryToByAdd::try_into_u8(u8::MIN).unwrap());
/// ```
/// The conversion is within the possible range of values.

pub trait TryToByAdd {
    /// Converting into negative integers to positive or vice versa, that can fail.
    fn try_into_i8(self) -> Option<i8>;
    /// Converting into negative integers to positive or vice versa, that can fail.
    fn try_into_u8(self) -> Option<u8>;
    /// Converting into negative integers to positive or vice versa, that can fail.
    fn try_into_i16(self) -> Option<i16>;
    /// Converting into negative integers to positive or vice versa, that can fail.
    fn try_into_u16(self) -> Option<u16>;
    /// Converting into negative integers to positive or vice versa, that can fail.
    fn try_into_i32(self) -> Option<i32>;
    /// Converting into negative integers to positive or vice versa, that can fail.
    fn try_into_u32(self) -> Option<u32>;
    /// Converting into negative integers to positive or vice versa, that can fail.
    fn try_into_i64(self) -> Option<i64>;
    /// Converting into negative integers to positive or vice versa, that can fail.
    fn try_into_u64(self) -> Option<u64>;
    /// Converting into negative integers to positive or vice versa, that can fail.
    fn try_into_isize(self) -> Option<isize>;
    /// Converting into negative integers to positive or vice versa, that can fail.
    fn try_into_usize(self) -> Option<usize>;
    /// Converting into negative integers to positive or vice versa, that can fail.
    fn try_into_i128(self) -> Option<i128>;
    /// Converting into negative integers to positive or vice versa, that can fail.
    fn try_into_u128(self) -> Option<u128>;
}

//the type self is equivalent to intotype
macro_rules! signed_or_unsigned_eq {
    ( $to_type:ty ) => {
        paste! {
            #[inline]
            fn [<try_into_$to_type>](self) -> Option<$to_type> {
                Some(self)
            }
        }
    };
}

//self is less than infotype
macro_rules! signed_or_unsigned_le {
    ( $($to_type:ty),+ ) => {
        $( paste! {
            #[inline]
            fn [<try_into_$to_type>](self) -> Option<$to_type> {
                Some(self as $to_type)
            }
        })*
    }
}

//if the type of self is greater than intotype then None
macro_rules! signed_to_signed_gt {
    ( $($to_type:ty, $value_min:expr, $value_max:expr);+ ) => {
        $( paste! {
            #[inline]
            fn [<try_into_$to_type>](self) -> Option<$to_type> {
                if self < $value_min || self > $value_max {
                    None
                } else {
                    Some(self as $to_type)
                }
            }
        })*
    }
}

//the type self is equivalent or less to intotype
macro_rules! signed_to_unsigned_eq_le {
    ( $($to_type:ty),+; $add_type:expr ) => {
        $( paste! {
            #[inline]
            fn [<try_into_$to_type>](self) -> Option<$to_type> {
                Some((self as $to_type).wrapping_add($add_type))
            }
        }
        )*
    }
}

//if the type of self is greater than intotype then None
macro_rules! signed_to_unsigned_gt {
    (  $($to_type:ty, $value_min:expr, $value_max:expr, $add_value:expr);+ ) => {
        $( paste! {
            #[inline]
            fn [<try_into_$to_type>](self) -> Option<$to_type> {
                if self < $value_min || self > $value_max {
                    None
                } else {
                    Some((self as $to_type).wrapping_add($add_value))
                }
            }
        }
        )*
    }
}

//unsigned impls
//if the type of self is greater than intotype then None
macro_rules! unsigned_to_signed_gt {
    ( $($to_type:ty, $value_max:expr);+ ) => {
        $( paste! {
            #[inline]
            fn [<try_into_$to_type>](self) -> Option<$to_type> {
                if self > $value_max {
                    None
                } else {
                    Some(((self as $to_type).wrapping_add(<$to_type>::MAX)).wrapping_add(1))
                }
            }
        })*
    }
}

//if the type of self is greater than intotype then None
macro_rules! unsigned_to_unsigned_gt {
    ( $($to_type:ty, $value_max:expr);+ ) => {
        $( paste! {
            #[inline]
            fn [<try_into_$to_type>](self) -> Option<$to_type> {
                if self > $value_max {
                    None
                } else {
                    Some(self as $to_type)
                }
            }
        })*
    }
}

//self is less than infotype
macro_rules! unsigned_to_signed_eq {
    ( $($to_type:ty),+ ) => {
       $( paste! {
            #[inline]
            fn [<try_into_$to_type>](self) -> Option<$to_type> {
                Some(((self as $to_type).wrapping_add(<$to_type>::MAX)).wrapping_add(1))
            }
        })*
    }
}

//self is less than infotype
macro_rules! unsigned_to_signed_le {
    ( $($to_type:ty),+; $add_type:ty ) => {
        $( paste! {
            #[inline]
            fn [<try_into_$to_type>](self) -> Option<$to_type> {
                Some(((self as $add_type).wrapping_add(<$add_type>::MAX)).wrapping_add(1) as $to_type)
            }
        })*
    }
}

macro_rules! signed_impls {
    ( $type_i8:ty, $type_i16:ty, $type_i32:ty, $type_i64:ty, $type_isize:ty, $type_i128:ty;
        $type_u8:ty, $type_u16:ty, $type_u32:ty, $type_u64:ty, $type_usize:ty, $type_u128:ty;
            $min_i8:expr, $min_i16:expr, $min_i32:expr, $min_i64:expr, $min_isize:expr;
                $max_i8:expr, $max_i16:expr, $max_i32:expr, $max_i64:expr, $max_isize:expr;
                    $add_i8:expr, $add_i16:expr, $add_i32:expr, $add_i64:expr, $add_isize:expr) => {

        impl TryToByAdd for $type_i8 {
            signed_or_unsigned_eq!($type_i8);
            signed_or_unsigned_le!($type_i16, $type_i32, $type_i64, $type_isize, $type_i128);
            signed_to_unsigned_eq_le!($type_u8, $type_u16, $type_u32, $type_u64, $type_usize, $type_u128; $add_i8);
        }

        impl TryToByAdd for $type_i16 {
            signed_to_signed_gt!($type_i8, $min_i8, $max_i8);
            signed_or_unsigned_eq!($type_i16);
            signed_or_unsigned_le!($type_i32, $type_i64, $type_isize, $type_i128);
            signed_to_unsigned_gt!($type_u8, $min_i8, $max_i8, $add_i8);
            signed_to_unsigned_eq_le!($type_u16, $type_u32, $type_u64, $type_usize, $type_u128; $add_i16);
        }

        impl TryToByAdd for $type_i32 {
            signed_to_signed_gt!($type_i8, $min_i8, $max_i8; $type_i16, $min_i16, $max_i16);
            signed_or_unsigned_eq!($type_i32);
            signed_or_unsigned_le!($type_i64, $type_isize, $type_i128);
            signed_to_unsigned_gt!($type_u8, $min_i8, $max_i8, $add_i8; $type_u16, $min_i16, $max_i16, $add_i16);
            signed_to_unsigned_eq_le!($type_u32, $type_u64, $type_usize, $type_u128; $add_i32);
        }

        #[cfg(target_pointer_width = "32")]
        impl TryToByAdd for $type_i64 {
            signed_to_signed_gt!($type_i8, $min_i8, $max_i8; $type_i16, $min_i16, $max_i16; $type_i32, $min_i32, $max_i32;
                $type_isize, $min_isize, $max_isize);
            signed_or_unsigned_eq!($type_i64);
            signed_or_unsigned_le!($type_i128);
            signed_to_unsigned_gt!($type_u8, $min_i8, $max_i8, $add_i8; $type_u16, $min_i16, $max_i16, $add_i16;
                $type_u32, $min_i32, $max_i32, $add_i32; $type_usize, $min_isize, $max_isize, $add_isize);
            signed_to_unsigned_eq_le!($type_u64, $type_u128; $add_i64);
        }

        #[cfg(target_pointer_width = "64")]
        impl TryToByAdd for $type_i64 {
            signed_to_signed_gt!($type_i8, $min_i8, $max_i8; $type_i16, $min_i16, $max_i16; $type_i32, $min_i32, $max_i32);
            signed_or_unsigned_eq!($type_i64);
            signed_or_unsigned_le!($type_isize, $type_i128);
            signed_to_unsigned_gt!($type_u8, $min_i8, $max_i8, $add_i8; $type_u16, $min_i16, $max_i16, $add_i16;
                $type_u32, $min_i32, $max_i32, $add_i32);
            signed_to_unsigned_eq_le!($type_u64, $type_usize, $type_u128; $add_i64);
        }

        #[cfg(target_pointer_width = "32")]
        impl TryToByAdd for $type_isize {
            signed_to_signed_gt!($type_i8, $min_i8, $max_i8; $type_i16, $min_i16, $max_i16);
            signed_or_unsigned_eq!($type_isize);
            signed_or_unsigned_le!($type_i32, $type_i64, $type_i128);
            signed_to_unsigned_gt!($type_u8, $min_i8, $max_i8, $add_i8; $type_u16, $min_i16, $max_i16, $add_i16);
            signed_to_unsigned_eq_le!($type_u32, $type_u64, $type_usize, $type_u128; $add_isize);
        }

        #[cfg(target_pointer_width = "64")]
        impl TryToByAdd for $type_isize {
            signed_to_signed_gt!($type_i8, $min_i8, $max_i8; $type_i16, $min_i16, $max_i16; $type_i32, $min_i32, $max_i32);
            signed_or_unsigned_eq!($type_isize);
            signed_or_unsigned_le!($type_i64, $type_i128);
            signed_to_unsigned_gt!($type_u8, $min_i8, $max_i8, $add_i8; $type_u16, $min_i16, $max_i16, $add_i16;
                $type_u32, $min_i32, $max_i32, $add_i32);
            signed_to_unsigned_eq_le!($type_u64, $type_usize, $type_u128; $add_isize);
        }

        impl TryToByAdd for $type_i128 {
            signed_to_signed_gt!($type_i8, $min_i8, $max_i8; $type_i16, $min_i16, $max_i16; $type_i32, $min_i32, $max_i32;
                $type_i64, $min_i64, $max_i64; $type_isize, $min_isize, $max_isize);
            signed_or_unsigned_eq!($type_i128);
            signed_to_unsigned_gt!($type_u8, $min_i8, $max_i8, $add_i8; $type_u16, $min_i16, $max_i16, $add_i16;
                $type_u32, $min_i32, $max_i32, $add_i32; $type_u64, $min_i64, $max_i64, $add_i64; $type_usize, $min_isize, $max_isize, $add_isize);
            signed_to_unsigned_eq_le!($type_u128; 170_141_183_460_469_231_731_687_303_715_884_105_728);
        }
    }
}

macro_rules! unsigned_impls {
    ( $type_i8:ty, $type_i16:ty, $type_i32:ty, $type_i64:ty, $type_isize:ty, $type_i128:ty;
        $type_u8:ty, $type_u16:ty, $type_u32:ty, $type_u64:ty, $type_usize:ty, $type_u128:ty;
            $max_u8:expr, $max_u16:expr, $max_u32:expr, $max_u64:expr, $max_usize:expr) => {

        impl TryToByAdd for $type_u8 {
            unsigned_to_signed_eq!($type_i8);
            unsigned_to_signed_le!($type_i16, $type_i32, $type_i64, $type_isize, $type_i128; $type_i8);
            signed_or_unsigned_eq!($type_u8);
            signed_or_unsigned_le!($type_u16, $type_u32, $type_u64, $type_usize, $type_u128);
        }

        impl TryToByAdd for $type_u16 {
            unsigned_to_signed_gt!($type_i8, $max_u8);
            unsigned_to_signed_eq!($type_i16);
            unsigned_to_signed_le!($type_i32, $type_i64, $type_isize, $type_i128; $type_i16);
            unsigned_to_unsigned_gt!($type_u8, $max_u8);
            signed_or_unsigned_eq!($type_u16);
            signed_or_unsigned_le!($type_u32, $type_u64, $type_usize, $type_u128);
        }

        #[cfg(target_pointer_width = "32")]
        impl TryToByAdd for $type_u32 {
            unsigned_to_signed_gt!($type_i8, $max_u8; $type_i16, $max_u16);
            unsigned_to_signed_eq!($type_i32, $type_isize);
            unsigned_to_signed_le!($type_i64, $type_i128; $type_i32);
            unsigned_to_unsigned_gt!($type_u8, $max_u8; $type_u16, $max_u16);
            signed_or_unsigned_eq!($type_u32);
            signed_or_unsigned_le!($type_u64, $type_usize, $type_u128);
        }

        #[cfg(target_pointer_width = "64")]
        impl TryToByAdd for $type_u32 {
            unsigned_to_signed_gt!($type_i8, $max_u8; $type_i16, $max_u16);
            unsigned_to_signed_eq!($type_i32);
            unsigned_to_signed_le!($type_i64, $type_isize, $type_i128; $type_i32);
            unsigned_to_unsigned_gt!($type_u8, $max_u8; $type_u16, $max_u16);
            signed_or_unsigned_eq!($type_u32);
            signed_or_unsigned_le!($type_u64, $type_usize, $type_u128);
        }

        #[cfg(target_pointer_width = "32")]
        impl TryToByAdd for $type_u64 {
            unsigned_to_signed_gt!($type_i8, $max_u8; $type_i16, $max_u16; $type_i32, $max_u32; $type_isize, $max_usize);
            unsigned_to_signed_eq!($type_i64);
            unsigned_to_signed_le!($type_i128; $type_i64);
            unsigned_to_unsigned_gt!($type_u8, $max_u8; $type_u16, $max_u16; $type_u32, $max_u32; $type_usize, $max_usize);
            signed_or_unsigned_eq!($type_u64);
            signed_or_unsigned_le!($type_u128);
        }

        #[cfg(target_pointer_width = "64")]
        impl TryToByAdd for $type_u64 {
            unsigned_to_signed_gt!($type_i8, $max_u8; $type_i16, $max_u16; $type_i32, $max_u32);
            unsigned_to_signed_eq!($type_i64, $type_isize);
            unsigned_to_signed_le!($type_i128; $type_i64);
            unsigned_to_unsigned_gt!($type_u8, $max_u8; $type_u16, $max_u16; $type_u32, $max_u32);
            signed_or_unsigned_eq!($type_u64);
            signed_or_unsigned_le!($type_usize, $type_u128);
        }

        #[cfg(target_pointer_width = "32")]
        impl TryToByAdd for $type_usize {
            unsigned_to_signed_gt!($type_i8, $max_u8; $type_i16, $max_u16; $type_i32, $max_u32);
            unsigned_to_signed_eq!($type_isize);
            unsigned_to_signed_le!($type_i64, $type_i128; $type_isize);
            unsigned_to_unsigned_gt!($type_u8, $max_u8; $type_u16, $max_u16; $type_u32, $max_u32);
            signed_or_unsigned_eq!($type_usize);
            signed_or_unsigned_le!($type_u64, $type_u128);
        }

        #[cfg(target_pointer_width = "64")]
        impl TryToByAdd for $type_usize {
            unsigned_to_signed_gt!($type_i8, $max_u8; $type_i16, $max_u16; $type_i32, $max_u32);
            unsigned_to_signed_eq!($type_isize, $type_i64);
            unsigned_to_signed_le!($type_i128; $type_isize);
            unsigned_to_unsigned_gt!($type_u8, $max_u8; $type_u16, $max_u16; $type_u32, $max_u32);
            signed_or_unsigned_eq!($type_usize);
            signed_or_unsigned_le!($type_u64, $type_u128);
        }

        impl TryToByAdd for $type_u128 {
            unsigned_to_signed_gt!($type_i8, $max_u8; $type_i16, $max_u16; $type_i32, $max_u32; $type_i64, $max_u64; $type_isize, $max_usize);
            unsigned_to_signed_eq!($type_i128);
            unsigned_to_unsigned_gt!($type_u8, $max_u8; $type_u16, $max_u16; $type_u32, $max_u32; $type_u64, $max_u64; $type_usize, $max_usize);
            signed_or_unsigned_eq!($type_u128);
        }
    }
}

#[cfg(target_pointer_width = "64")]
signed_impls!(
    i8, i16, i32, i64, isize, i128;
        u8, u16, u32, u64, usize, u128;
            -128, -32_768, -2_147_483_648, -9_223_372_036_854_775_808, -9_223_372_036_854_775_808;
                127, 32_767, 2_147_483_647, 9_223_372_036_854_775_807, 9_223_372_036_854_775_807;
                    128, 32_768, 2_147_483_648, 9_223_372_036_854_775_808, 9_223_372_036_854_775_808
);

#[cfg(target_pointer_width = "64")]
unsigned_impls!(
    i8, i16, i32, i64, isize, i128;
        u8, u16, u32, u64, usize, u128;
            255, 65_535, 4_294_967_295, 18_446_744_073_709_551_615, 18_446_744_073_709_551_615
);

#[cfg(target_pointer_width = "32")]
signed_impls!(
    i8, i16, i32, i64, isize, i128;
        u8, u16, u32, u64, usize, u128;
            -128, -32_768, -2_147_483_648, -9_223_372_036_854_775_808, -2_147_483_648;
                127, 32_767, 2_147_483_647, 9_223_372_036_854_775_807, 2_147_483_647;
                    128, 32_768, 2_147_483_648, 9_223_372_036_854_775_808, 2_147_483_648
);

#[cfg(target_pointer_width = "32")]
unsigned_impls!(
    i8, i16, i32, i64, isize, i128;
        u8, u16, u32, u64, usize, u128;
            255, 65_535, 4_294_967_295, 18_446_744_073_709_551_615, 4_294_967_295
);
