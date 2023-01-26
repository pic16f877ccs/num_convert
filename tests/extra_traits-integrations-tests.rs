use num_convert::IntegerLen;
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
