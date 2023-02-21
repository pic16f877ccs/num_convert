use num_convert::FromTuple;

#[test]
fn from_1_tup_to_i32() {
    assert_eq!(<i32 as FromTuple>::from_1((-60i8,)), [-60i32]);
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

#[test]
fn from_6_tup_to_i32() {
    assert_eq!(
        <i32 as FromTuple>::from_6((i32::MIN, -128i8, -2023i16, 45u8, 2023u16, -60i8,)),
        [i32::MIN, -128i32, -2023i32, 45i32, 2023i32, -60i32]
    );
}

#[test]
fn from_7_tup_to_i32() {
    assert_eq!(
        <i32 as FromTuple>::from_7((u16::MAX, i32::MIN, -128i8, -2023i16, 45u8, 2023u16, -60i8,)),
        [
            u16::MAX as i32,
            i32::MIN,
            -128i32,
            -2023i32,
            45i32,
            2023i32,
            -60i32
        ]
    );
}

#[test]
fn from_8_tup_to_i32() {
    assert_eq!(
        <i32 as FromTuple>::from_8((
            i16::MIN,
            u16::MAX,
            i32::MIN,
            -128i8,
            -2023i16,
            45u8,
            2023u16,
            -60i8,
        )),
        [
            i16::MIN as i32,
            u16::MAX as i32,
            i32::MIN,
            -128i32,
            -2023i32,
            45i32,
            2023i32,
            -60i32
        ]
    );
}

#[test]
fn from_1_tup_to_i128() {
    assert_eq!(<i128 as FromTuple>::from_1((-60i8,)), [-60i128]);
}

#[test]
fn from_2_tup_to_i128() {
    assert_eq!(
        <i128 as FromTuple>::from_2((2023u16, -60i8,)),
        [2023i128, -60i128]
    );
}

#[test]
fn from_3_tup_to_i128() {
    assert_eq!(
        <i128 as FromTuple>::from_3((45u8, 2023i16, 60u8,)),
        [45i128, 2023i128, 60i128]
    );
}

#[test]
fn from_4_tup_to_i128() {
    assert_eq!(
        <i128 as FromTuple>::from_4((-2023i16, 45u8, 2023u16, -60i8,)),
        [-2023i128, 45i128, 2023i128, -60i128]
    );
}

#[test]
fn from_5_tup_to_i128() {
    assert_eq!(
        <i128 as FromTuple>::from_5((-128i8, -2023i16, 45u8, 2023u16, -60i8,)),
        [-128i128, -2023i128, 45i128, 2023i128, -60i128]
    );
}

#[test]
fn from_6_tup_to_i128() {
    assert_eq!(
        <i128 as FromTuple>::from_6((i128::MIN, -128i8, -2023i16, 45u8, 2023u16, -60i8,)),
        [i128::MIN, -128i128, -2023i128, 45i128, 2023i128, -60i128]
    );
}

#[test]
fn from_7_tup_to_i128() {
    assert_eq!(
        <i128 as FromTuple>::from_7((u16::MAX, i128::MIN, -128i8, -2023i16, 45u8, 2023u16, -60i8,)),
        [
            u16::MAX as i128,
            i128::MIN,
            -128i128,
            -2023i128,
            45i128,
            2023i128,
            -60i128
        ]
    );
}

#[test]
fn from_8_tup_to_i128() {
    assert_eq!(
        <i128 as FromTuple>::from_8((
            i64::MIN,
            u64::MAX,
            true,
            i128::MAX,
            u64::MIN,
            45u8,
            2023u16,
            -60i8,
        )),
        [
            i64::MIN as i128,
            u64::MAX as i128,
            true as i128,
            i128::MAX,
            u64::MIN as i128,
            45i128,
            2023i128,
            -60i128
        ]
    );
}

#[test]
fn from_9_tup_to_i128() {
    assert_eq!(
        <i128 as FromTuple>::from_9((
            false,
            i64::MIN,
            u64::MAX,
            true,
            i128::MAX,
            u64::MIN,
            45u8,
            2023u16,
            -60i8,
        )),
        [
            false as i128,
            i64::MIN as i128,
            u64::MAX as i128,
            true as i128,
            i128::MAX,
            u64::MIN as i128,
            45i128,
            2023i128,
            -60i128
        ]
    );
}

#[test]
fn from_10_tup_to_i64() {
    assert_eq!(
        <i64 as FromTuple>::from_10((
            false,
            true,
            i8::MIN,
            i8::MAX,
            u8::MIN,
            u8::MAX,
            i16::MIN,
            i16::MAX,
            u16::MIN,
            u16::MAX
        )),
        [
            false as i64,
            true as i64,
            i8::MIN as i64,
            i8::MAX as i64,
            u8::MIN as i64,
            u8::MAX as i64,
            i16::MIN as i64,
            i16::MAX as i64,
            u16::MIN as i64,
            u16::MAX as i64
        ]
    );
}

#[test]
fn from_12_tup_to_i64() {
    assert_eq!(
        <i64 as FromTuple>::from_12((
            false,
            true,
            i8::MIN,
            i8::MAX,
            u8::MIN,
            u8::MAX,
            i16::MIN,
            i16::MAX,
            u16::MIN,
            u16::MAX,
            i32::MIN,
            i32::MAX
        )),
        [
            false as i64,
            true as i64,
            i8::MIN as i64,
            i8::MAX as i64,
            u8::MIN as i64,
            u8::MAX as i64,
            i16::MIN as i64,
            i16::MAX as i64,
            u16::MIN as i64,
            u16::MAX as i64,
            i32::MIN as i64,
            i32::MAX as i64
        ]
    );
}

#[test]
fn from_16_tup_to_i64() {
    assert_eq!(
        <i64 as FromTuple>::from_16((
            false,
            true,
            i8::MIN,
            i8::MAX,
            u8::MIN,
            u8::MAX,
            i16::MIN,
            i16::MAX,
            u16::MIN,
            u16::MAX,
            i32::MIN,
            i32::MAX,
            u32::MIN,
            u32::MAX,
            i64::MIN,
            i64::MAX
        )),
        [
            false as i64,
            true as i64,
            i8::MIN as i64,
            i8::MAX as i64,
            u8::MIN as i64,
            u8::MAX as i64,
            i16::MIN as i64,
            i16::MAX as i64,
            u16::MIN as i64,
            u16::MAX as i64,
            i32::MIN as i64,
            i32::MAX as i64,
            u32::MIN as i64,
            u32::MAX as i64,
            i64::MIN,
            i64::MAX
        ]
    );
}
