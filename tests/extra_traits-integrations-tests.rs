use num_convert::IntegerLen;
use paste::paste;

macro_rules! int_len_tests {
    ($type:ty) => {
        paste!{
            #[test]
            fn [<int_len_$type>]() {
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
    }
}

int_len_tests!{u8}
int_len_tests!{u16}
int_len_tests!{u32}
int_len_tests!{u64}
int_len_tests!{usize}
int_len_tests!{u128}
int_len_tests!{i8}
int_len_tests!{i16}
int_len_tests!{i32}
int_len_tests!{i64}
int_len_tests!{isize}
int_len_tests!{i128}


