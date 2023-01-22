use num_convert::IntegerLen;
use paste::paste;

macro_rules! int_len_tests {
    (pos $type:ty) => {
        paste!{
            #[test]
            fn [<int_len_$type _pos>]() {
                let mut digits: $type = 1;
                while digits < $type::MAX {
                    assert_eq!(digits.len(), digits.to_string().trim_start_matches('-').len());
                    println!("test len -> {} string len {}", digits.len(), digits.to_string().trim_start_matches('-').len());
                    match digits.checked_mul(10) {
                        Some(val) => {digits = val;},
                        _ => break,
                    }
                }
            }
        }
    };

    (neg $type:ty) => {
        paste!{
            #[test]
            fn [<int_len_$type _neg>]() {
                let mut digits: $type = -1;
                while digits < $type::MAX {
                    assert_eq!(digits.len(), digits.to_string().trim_start_matches('-').len());
                    println!("test len -> {} string len {}", digits.len(), digits.to_string().trim_start_matches('-').len());
                    match digits.checked_mul(10) {
                        Some(val) => {digits = val;},
                        _ => break,
                    }
                }
            }
        }
    };

    (zero $type:ty) => {
        paste!{
            #[test]
            fn [<int_len_$type _zero>]() {
                let digits: $type = 0;
                assert_eq!(digits.len(), digits.to_string().trim_start_matches('-').len());
                println!("test len -> {} string len {}", digits.len(), digits.to_string().trim_start_matches('-').len());
            }
        }
    };
}

int_len_tests!{neg i8}
int_len_tests!{neg i16}
int_len_tests!{neg i32}
int_len_tests!{neg i64}
int_len_tests!{neg isize}
int_len_tests!{neg i128}

int_len_tests!{pos u8}
int_len_tests!{pos u16}
int_len_tests!{pos u32}
int_len_tests!{pos u64}
int_len_tests!{pos usize}
int_len_tests!{pos u128}
int_len_tests!{pos i8}
int_len_tests!{pos i16}
int_len_tests!{pos i32}
int_len_tests!{pos i64}
int_len_tests!{pos isize}
int_len_tests!{pos i128}

int_len_tests!{zero u8}
int_len_tests!{zero u16}
int_len_tests!{zero u32}
int_len_tests!{zero u64}
int_len_tests!{zero usize}
int_len_tests!{zero u128}
int_len_tests!{zero i8}
int_len_tests!{zero i16}
int_len_tests!{zero i32}
int_len_tests!{zero i64}
int_len_tests!{zero isize}
int_len_tests!{zero i128}


