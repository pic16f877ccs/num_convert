use num_convert::{CheckRem, IntegerLen};
use paste::paste;

macro_rules! pos_len_tests {
    ( $type:ty ) => {
        paste! {
            #[test]
            fn [<int_len_$type _pos>]() {
                let mut digits: $type = 1;

                while digits < $type::MAX {
                    assert_eq!(digits.len(), digits.to_string().trim_start_matches('-').len());
                    match digits.checked_mul(10) {
                        Some(val) => {digits = val;},
                        _ => break,
                    }
                }
            }
        }
    };
}

macro_rules! neg_len_tests {
    ( $type:ty ) => {
        paste! {
            #[test]
            fn [<int_len_$type _neg>]() {
                let mut digits: $type = 1;

                while digits < $type::MAX {
                    assert_eq!(digits.len(), digits.to_string().trim_start_matches('-').len());
                    match digits.checked_mul(10) {
                        Some(val) => {digits = val;},
                        _ => break,
                    }
                }
            }
        }
    };
}

macro_rules! zero_len_tests {
    ( $type:ty ) => {
        paste! {
            #[test]
            fn [<int_len_$type _zero>]() {
                let digits: $type = 0;

                assert_eq!(digits.len(), digits.to_string().trim_start_matches('-').len());
            }
        }
    };
}

macro_rules! len_tests {
    ( pos $($type:ty),* ) => {
        $(
            pos_len_tests!{$type}
        )*
    };
    ( neg $($type:ty),* ) => {
        $(
            neg_len_tests!{$type}
        )*
    };
    ( zero $($type:ty),* ) => {
        $(
            zero_len_tests!{$type}
        )*
    };
}

len_tests! {zero i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, i128, u128}
len_tests! {pos i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, i128, u128}
len_tests! {neg i8, i16, i32, i64, isize, i128}

#[test]
fn test_no_rem_float() {
    assert_eq!(10.0_f32.no_rem(2.0_f32), true);
    assert_eq!(10.0_f32.no_rem(3.0_f32), false);
    //assert_eq!(0.0_32.no_rem(1.0_32), true);
}

#[test]
fn test_is_rem_float() {
    assert_eq!(10.0_f64.is_rem(2.0_f64), false);
    assert_eq!(10.0_f64.is_rem(3.0_f64), true);
    //assert_eq!(0.0_f64.is_rem(1.0_f64), false);
}

#[test]
fn test_no_rem() {
    assert_eq!(10_i8.no_rem(2_i8), true);
    assert_eq!(10_u8.no_rem(2_u8), true);
    assert_eq!(10_i16.no_rem(3_i16), false);
    assert_eq!(0_u16.no_rem(1_u16), true);
    assert_eq!(0_usize.no_rem(1_usize), true);
}

#[test]
fn test_is_rem() {
    assert_eq!(10_i32.is_rem(2_i32), false);
    assert_eq!(10_u32.is_rem(3_u32), true);
    assert_eq!(0_i64.is_rem(1_i64), false);
    assert_eq!(0_u64.is_rem(1_u64), false);
    assert_eq!(0_isize.is_rem(1_isize), false);
}
