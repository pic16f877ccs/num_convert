use num_convert::Tbits;
use paste::paste;

macro_rules! tbits_tests {
   ( $($value_type:ty, $value:expr);* ) => {
       $( paste! {

           #[test]
           fn [<get_tbits_$value_type>]() {
               assert_eq!($value_type::get_bits(), $value);
           }
       })*
   }
}

tbits_tests! { i8, 8; u8, 8; i16, 16; u16, 16; i32, 32; u32, 32; i64, 64; u64, 64; isize, 64; usize, 64; i128, 128; u128, 128 }
