use num_convert::Tbits;
use paste::paste;

macro_rules! tbits_tests {
   ( $($value_type:ty),* ) => {
       $( paste! {

           #[test]
           fn [<test_tbits_$value_type>]() {
               assert_eq!($value_type::tbits(), <$value_type>::BITS);
           }
       })*
   }
}

tbits_tests! { i8,  u8,  i16,  u16,  i32,  u32,  i64,  u64,  isize,  usize,  i128,  u128 }
