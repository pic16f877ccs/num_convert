use num_convert::FromTuple;

#[test]
fn from_1_tup_to_i32() {
    assert_eq!(
        <i32 as FromTuple>::from_1((-60i8,)),
        [-60i32]
    );
}

#[test]
fn from_2_tup_to_i32() {
    assert_eq!(
        <i32 as FromTuple>::from_2((2023u16, -60i8,)),
        [2023i32, -60i32]
    );
}

#[test]
fn from_3_tup_to_i32() {
    assert_eq!(
        <i32 as FromTuple>::from_3((45u8, 2023i16, 60u8,)),
        [45i32, 2023i32, 60i32]
    );
}

#[test]
fn from_4_tup_to_i32() {
    assert_eq!(
        <i32 as FromTuple>::from_4((-2023i16, 45u8, 2023u16, -60i8,)),
        [-2023i32, 45i32, 2023i32, -60i32]
    );
}

#[test]
fn from_5_tup_to_i32() {
    assert_eq!(
        <i32 as FromTuple>::from_5((-128i8, -2023i16, 45u8, 2023u16, -60i8,)),
        [-128i32, -2023i32, 45i32, 2023i32, -60i32]
    );
}

