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

//macro_rules! try_from_by_add_ok {
//    ( $( $from_type:ty, $($into_type:ty)* );* ) => {
//        $(
//            $( 
//                paste! {
//                    #[test]
//                    fn [<$into_type _try_from_$from_type _min>]() {
//                       assert_eq!(<$into_type>::try_from_by_add(<$into_type>::MIN as $from_type), Some(<$into_type>::MIN));
//                    }
//
//                    #[test]
//                    fn [<$into_type _try_from_$from_type _max>]() {
//                       assert_eq!(<$into_type>::try_from_by_add(<$into_type>::MAX as $from_type), Some(<$into_type>::MAX));
//                    }
//                }
//            )*
//        )*
//    };
//
//    ( $( $from_type:ty, $($into_type:ty),* );* ) => {
//        $( 
//            $(
//                paste! {
//                    #[test]
//                    fn [<$into_type _try_from_$from_type _min>]() {
//                       assert_eq!(<$into_type>::try_from_by_add(<$from_type>::MIN), Some(<$from_type>::MIN as $into_type));
//                    }
//
//                    #[test]
//                    fn [<$into_type _try_from_$from_type _max>]() {
//                       assert_eq!(<$into_type>::try_from_by_add(<$from_type>::MAX), Some(<$from_type>::MAX as $into_type));
//                    }
//                }
//            )*
//        )*
//    };
//}

//try_from_by_add_ok! { i16, i8; i32, i8 i16; i64, i8 i16 i32; isize, i8 i16 i32 }
//try_from_by_add_ok! { u16, u8; u32, u8 u16; u64, u8 u16 u32; usize, u8 u16 u32 }

//try_from_by_add_ok! { i8, i8, i16, i32, i64, isize, i128; i16, i16, i32, i64, isize, i128; i32, i32, i64, isize, i128 }
//try_from_by_add_ok! { i64, i64, isize, i128; isize, i64, isize, i128; i128, i128 }
//try_from_by_add_ok! { u8, u8, u16, u32, u64, usize, u128; u16, u16, u32, u64, usize, u128; u32, u32, u64, usize, u128 }
//try_from_by_add_ok! { u64, u64, usize, u128; usize, u64, usize, u128; u128, u128 }

//macro_rules! try_from_by_add_min_err {
//        ($($left_type:ty, $from:ty, $right_type:ty);+ ) => {
//            $(
//                paste! {
//                    #[test]
//                    #[should_panic]
//                    fn [<err_$left_type _try_from_$from _min>]() {
//                       assert_eq!(<$left_type>::MIN,
//                           paste! {<$left_type as TryFromByAdd>::[<try_from_$from>]((<$right_type>::MIN as $from) - 1).unwrap()});
//                    }
//                }
//            )*
//        }
//    }
//
//macro_rules! try_from_by_add_max_err {
//        ($($left_type:ty, $from:ty, $right_type:ty);+ ) => {
//            $(
//                paste! {
//                    #[test]
//                    #[should_panic]
//                    fn [<err_$left_type _try_from_$from _max>]() {
//                       assert_eq!(<$left_type>::MAX,
//                           paste! {<$left_type as TryFromByAdd>::[<try_from_$from>]((<$right_type>::MAX as $from) + 1).unwrap()});
//                    }
//                }
//            )*
//        }
//    }

//try_from_by_add_ok! {i8; i8, i8, i8; i8, i16, i8; i8, i32, i8; i8, i64, i8; i8, isize, i8; i8, i128, i8}
//try_from_by_add_ok! {i16; i8, i8, i8; i16, i16, i16; i16, i32, i16; i16, i64, i16; i16, isize, i16; i16, i128, i16}
//try_from_by_add_ok! {i32; i8, i8, i8; i16, i16, i16; i32, i32, i32; i32, i64, i32; i32, isize, i32; i32, i128, i32}
//try_from_by_add_ok! {i64; i8, i8, i8; i16, i16, i16; i32, i32, i32; i64, i64, i64; isize, isize, isize; i64, i128, i64}
//try_from_by_add_ok! {isize; i8, i8, i8; i16, i16, i16; i32, i32, i32; isize, i64, isize; isize, isize, isize; isize, i128, isize}
//try_from_by_add_ok! {i128; i8, i8, i8; i16, i16, i16; i32, i32, i32; i64, i64, i64; isize, isize, isize; i128, i128, i128}

//try_from_by_add_ok! {u8;    u8, i8, i8; u8,  i16, i8;  u8,  i32, i8;  u8,  i64, i8;  u8,    isize, i8;    u8,    i128, i8}
//try_from_by_add_ok! {u16;   u8, i8, i8; u16, i16, i16; u16, i32, i16; u16, i64, i16; u16,   isize, i16;   u16,   i128, i16}
//try_from_by_add_ok! {u32;   u8, i8, i8; u16, i16, i16; u32, i32, i32; u32, i64, i32; u32,   isize, i32;   u32,   i128, i32}
//try_from_by_add_ok! {u64;   u8, i8, i8; u16, i16, i16; u32, i32, i32; u64, i64, i64; usize, isize, isize; u64,   i128, i64}
//try_from_by_add_ok! {usize; u8, i8, i8; u16, i16, i16; u32, i32, i32; usize, i64, isize; usize, isize, isize; usize, i128, isize}
//try_from_by_add_ok! {u128;  u8, i8, i8; u16, i16, i16; u32, i32, i32; u64, i64, i64; usize, isize, isize; u128,  i128, i128}
//
//try_from_by_add_ok! {i8;    i8, u8, u8; i8,  u16, u8;  i8,  u32, u8;  i8,  u64, u8;  i8,    usize, u8;    i8,    u128, u8}
//try_from_by_add_ok! {i16;   i8, u8, u8; i16, u16, u16; i16, u32, u16; i16, u64, u16; i16,   usize, u16;   i16,   u128, u16}
//try_from_by_add_ok! {i32;   i8, u8, u8; i16, u16, u16; i32, u32, u32; i32, u64, u32; i32,   usize, u32;   i32,   u128, u32}
//try_from_by_add_ok! {i64;   i8, u8, u8; i16, u16, u16; i32, u32, u32; i64, u64, u64; isize, usize, usize; i64,   u128, u64}
//try_from_by_add_ok! {isize; i8, u8, u8; i16, u16, u16; i32, u32, u32; isize, u64, usize; isize, usize, usize; isize, u128, usize}
//try_from_by_add_ok! {i128;  i8, u8, u8; i16, u16, u16; i32, u32, u32; i64, u64, u64; isize, usize, usize; i128,  u128, u128}

//try_from_by_add_ok! {u8;    u8, u8, u8; u8,  u16, u8;  u8,  u32, u8;  u8,  u64, u8;  u8,    usize, u8;    u8,    u128, u8}
//try_from_by_add_ok! {u16;   u8, u8, u8; u16, u16, u16; u16, u32, u16; u16, u64, u16; u16,   usize, u16;   u16,   u128, u16}
//try_from_by_add_ok! {u32;   u8, u8, u8; u16, u16, u16; u32, u32, u32; u32, u64, u32; u32,   usize, u32;   u32,   u128, u32}
//try_from_by_add_ok! {u64;   u8, u8, u8; u16, u16, u16; u32, u32, u32; u64, u64, u64; usize, usize, usize; u64,   u128, u64}
//try_from_by_add_ok! {usize; u8, u8, u8; u16, u16, u16; u32, u32, u32; usize, u64, usize; usize, usize, usize; usize, u128, usize}
//try_from_by_add_ok! {u128;  u8, u8, u8; u16, u16, u16; u32, u32, u32; u64, u64, u64; usize, usize, usize; u128,  u128, u128}

//try_from_by_add_max_err! {i8, i16, i8; i8, i32, i8; i16, i32, i16; u8, i16, i8; u8, i32, u8; u16, i32, i16}
//try_from_by_add_max_err! {i8, i64, i8; i16, i64, i16; i32, i64, i32; u8, i64, i8; u16, i64, i16; u32, i64, i32}
//try_from_by_add_max_err! {i8, isize, i8; i16, isize, i16; i32, isize, i32; u8, isize, i8; u16, isize, i16; u32, isize, i32}
//try_from_by_add_max_err! {i8, i128, i8; i16, i128, i16; i32, i128, i32; i64, i128, i64; isize, i128, isize; u8, i128, i8; u16, i128, u16; u32, i128, i32; u64, i128, i64; usize, i128, isize}
//
//try_from_by_add_min_err! {i8, i16, i8; i8, i32, i8; i16, i32, i16}
//try_from_by_add_min_err! {i8, i64, i8; i16, i64, i16; i32, i64, i32}
//try_from_by_add_min_err! {i8, isize, i8; i16, isize, i16; i32, isize, i32}
//try_from_by_add_min_err! {i8, i128, i8; i16, i128, i16; i32, i128, i32; i64, i128, i64; isize, i128, isize}
//
//try_from_by_add_max_err! {i8, u16, u8; i8, u32, u8; i16, u32, u16; u8, u16, u8; u8, u32, u8; u16, u32, u16}
//try_from_by_add_max_err! {i8, u64, u8; i16, u64, u16; i32, u64, u32; u8, u64, u8; u16, u64, u16; u32, u64, u32}
//try_from_by_add_max_err! {i8, usize, u8; i16, usize, u16; i32, usize, u32; u8, usize, u8; u16, usize, u16; u32, usize, u32}
//try_from_by_add_max_err! {i8, u128, u8; i16, u128, u16; i32, u128, u32; i64, u128, u64; isize, u128, usize; u8, u128, u8; u16, u128, u16; u32, u128, u32; u64, u128, u64; usize, u128, usize}
