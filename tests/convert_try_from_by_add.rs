use num_convert::TryFromByAdd;
use paste::paste;

macro_rules! try_from_by_add_ok {
    ( $into_type:ty, $type:ty; $( $from_type:ty),+ ) => {
        $( paste! {
                #[test]
                fn [<$into_type _try_from_$from_type _min>]() {
                   assert_eq!(<$into_type>::try_from_by_add(<$type>::MIN as $from_type), Some(<$into_type>::MIN));
                }

                #[test]
                fn [<$into_type _try_from_$from_type _max>]() {
                   assert_eq!(<$into_type>::try_from_by_add(<$type>::MAX as $from_type), Some(<$into_type>::MAX));
                }
            }
        )+
    };

    (  $from_type:ty; $($into_type:ty),+; $type:ty ) => {
        $( paste! {
                #[test]
                fn [<$into_type _try_from_$from_type _min>]() {
                   assert_eq!(<$into_type>::try_from_by_add(<$from_type>::MIN), Some(<$type>::MIN as $into_type));
                }

                #[test]
                fn [<$into_type _try_from_$from_type _max>]() {
                   assert_eq!(<$into_type>::try_from_by_add(<$from_type>::MAX), Some(<$type>::MAX as $into_type));
                }
            }
        )+
    };
}

try_from_by_add_ok! { i8, u8; u16, u32, u64, usize, u128 }
try_from_by_add_ok! { i16, u16; u32, u64, usize, u128 }
try_from_by_add_ok! { i32, u32; u64, usize, u128 }
try_from_by_add_ok! { i64, u64; u128 }
try_from_by_add_ok! { isize, usize; u128 }

try_from_by_add_ok! { u8, i8; i16, i32, i64, isize, i128 }
try_from_by_add_ok! { u16, i16; i32, i64, isize, i128 }
try_from_by_add_ok! { u32, i32; i64, isize, i128 }
try_from_by_add_ok! { u64, i64; i128 }
try_from_by_add_ok! { usize, isize; i128 }

try_from_by_add_ok! { i8; i8, i16, i32, i64, isize, i128; i8 }
try_from_by_add_ok! { i16; i16, i32, i64, isize, i128; i16 }
try_from_by_add_ok! { i32; i32, i64, isize, i128; i32 }
try_from_by_add_ok! { i64; i64, isize, i128; i64 }
try_from_by_add_ok! { isize; i64, isize, i128; isize }

try_from_by_add_ok! { i8, i8; i16, i32, i64, isize, i128 }
try_from_by_add_ok! { i16, i16; i32, i64, isize, i128 }
try_from_by_add_ok! { i32, i32; i64, isize, i128 }
try_from_by_add_ok! { i64, i64; i128 }
try_from_by_add_ok! { isize, isize; i128 }
try_from_by_add_ok! { i128, i128; i128 }

try_from_by_add_ok! { i8; u8, u16, u32, u64, usize, u128; u8 }
try_from_by_add_ok! { i16; u16, u32, u64, usize, u128; u16 }
try_from_by_add_ok! { i32; u32, u64, usize, u128; u32 }
try_from_by_add_ok! { i64; u64, usize, u128; u64 }
try_from_by_add_ok! { isize; u64, usize, u128; usize }
try_from_by_add_ok! { i128; u128; u128 }

try_from_by_add_ok! { u8; u8, u16, u32, u64, usize, u128; u8 }
try_from_by_add_ok! { u16; u16, u32, u64, usize, u128; u16 }
try_from_by_add_ok! { u32; u32, u64, usize, u128; u32 }
try_from_by_add_ok! { u64; u64, usize, u128; u64 }
try_from_by_add_ok! { usize; u64, usize, u128; usize }

try_from_by_add_ok! { u8, u8; u16, u32, u64, usize, u128 }
try_from_by_add_ok! { u16, u16; u32, u64, usize, u128 }
try_from_by_add_ok! { u32, u32; u64, usize, u128 }
try_from_by_add_ok! { u64, u64; u128 }
try_from_by_add_ok! { usize, usize; u128 }
try_from_by_add_ok! { u128, u128; u128 }

try_from_by_add_ok! { u8; i8, i16, i32, i64, isize, i128; i8 }
try_from_by_add_ok! { u16; i16, i32, i64, isize, i128; i16 }
try_from_by_add_ok! { u32; i32, i64, isize, i128; i32 }
try_from_by_add_ok! { u64; i64, isize, i128; i64 }
try_from_by_add_ok! { usize; i64, isize, i128; isize }
try_from_by_add_ok! { u128; i128; i128 }

macro_rules! try_from_by_add_min_none {
   ( $into_type:ty, $type:ty; $($from_type:ty),+ ) => {
       $(
           paste! {
               #[test]
               fn [<none_$into_type _try_from_$from_type _min>]() {
                  assert_eq!(<$into_type>::try_from_by_add((<$type>::MIN as $from_type) - 1), None);
               }
           }
       )*
   }
}

macro_rules! try_from_by_add_max_none {
   ( $into_type:ty, $type:ty; $($from_type:ty),+ ) => {
       $(
           paste! {
               #[test]
               fn [<none_$into_type _try_from_$from_type _max>]() {
                  assert_eq!(<$into_type>::try_from_by_add((<$type>::MAX as $from_type) + 1), None);
               }
           }
       )*
   }
}

try_from_by_add_min_none! { u8, i8; i16, i32, i64, isize, i128 }
try_from_by_add_min_none! { u16, i16; i32, i64, isize, i128 }
try_from_by_add_min_none! { u32, i32; i64, isize, i128 }
try_from_by_add_min_none! { u64, i64; i128 }
try_from_by_add_min_none! { usize, isize; i128 }

try_from_by_add_min_none! { i8, i8; i16, i32, i64, isize, i128 }
try_from_by_add_min_none! { i16, i16; i32, i64, isize, i128 }
try_from_by_add_min_none! { i32, i32; i64, isize, i128 }
try_from_by_add_min_none! { i64, i64; i128 }
try_from_by_add_min_none! { isize, isize; i128 }

try_from_by_add_max_none! { i8, i8; i16, i32, i64, isize, i128 }
try_from_by_add_max_none! { i16, i16; i32, i64, isize, i128 }
try_from_by_add_max_none! { i32, i32; i64, isize, i128 }
try_from_by_add_max_none! { i64, i64; i128 }
try_from_by_add_max_none! { isize, isize; i128 }

try_from_by_add_max_none! { i8, u8; u16, u32, u64, usize, u128 }
try_from_by_add_max_none! { i16, u16; u32, u64, usize, u128 }
try_from_by_add_max_none! { i32, u32; u64, usize, u128 }
try_from_by_add_max_none! { i64, u64; u128 }
try_from_by_add_max_none! { isize, usize; u128 }

try_from_by_add_max_none! { u8, u8; u16, u32, u64, usize, u128 }
try_from_by_add_max_none! { u16, u16; u32, u64, usize, u128 }
try_from_by_add_max_none! { u32, u32; u64, usize, u128 }
try_from_by_add_max_none! { u64, u64; u128 }
try_from_by_add_max_none! { usize, usize; u128 }

try_from_by_add_max_none! { u8, i8; i16, i32, i64, isize, i128 }
try_from_by_add_max_none! { u16, i16; i32, i64, isize, i128 }
try_from_by_add_max_none! { u32, i32; i64, isize, i128 }
try_from_by_add_max_none! { u64, i64; i128 }
try_from_by_add_max_none! { usize, isize; i128 }
