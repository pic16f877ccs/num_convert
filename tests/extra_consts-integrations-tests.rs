use num_convert::Ebits;
use paste::paste;

macro_rules! const_tests {
   ( $($const_type:ty, $const_value:expr);* ) => {
       $( paste! {
           #[test]
           fn [<const_$const_type>]() {
               assert_eq!($const_type::EBITS, $const_value);
           }

           #[test]
           fn [<get_const_$const_type>]() {
               assert_eq!($const_type::get_bits(), $const_value);
           }
       })*
   }
}

const_tests! { i8, 8; u8, 8; i16, 16; u16, 16; i32, 32; u32, 32; i64, 64; u64, 64; isize, 64; usize, 64; i128, 128; u128, 128 }
