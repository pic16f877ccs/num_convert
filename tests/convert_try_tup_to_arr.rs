use num_convert::TryTupToArr;

#[test]
fn tuple_to_arr_i8_8() {
    assert_eq!(
        TryTupToArr::<i8>::try_into_arr((
            i8::MIN,
            i8::MAX,
            i8::MAX as u8,
            i8::MIN as i16,
            i8::MAX as i16,
            i8::MAX as u16,
            i8::MIN as i32,
            i8::MAX as i32,
        )),
        Ok([
            i8::MIN,
            i8::MAX,
            i8::MAX,
            i8::MIN,
            i8::MAX,
            i8::MAX,
            i8::MIN,
            i8::MAX,
        ])
    );
}

#[test]
fn tuple_to_arr_i8_str_8() {
    assert_eq!(
        TryTupToArr::<i8>::try_into_arr((
            "0",
            "127",
            "-128",
            true,
            false,
            i8::MAX as u32,
            i8::MIN as i64,
            i8::MAX as i64
        )),
        Ok([
            0,
            i8::MAX,
            i8::MIN,
            1,
            0,
            i8::MAX,
            i8::MIN,
            i8::MAX
        ])
    );
}

#[test]
fn tuple_to_arr_u8_8() {
    assert_eq!(
        TryTupToArr::<u8>::try_into_arr((
            i8::MAX,
            u8::MIN as i16,
            u8::MAX as i16,
            u8::MAX as u16,
            u8::MIN as i32,
            u8::MAX as i32,
            u8::MAX as isize,
            u8::MIN as usize,
        )),
        Ok([
            i8::MAX as u8,
            u8::MIN,
            u8::MAX,
            u8::MAX,
            u8::MIN,
            u8::MAX,
            u8::MAX,
            u8::MIN,
        ])
    );
}

#[test]
fn tuple_to_arr_u16_8() {
    assert_eq!(
        TryTupToArr::<u16>::try_into_arr((
            u16::MIN as usize,
            u16::MAX as isize,
            i16::MAX as u16,
            u16::MIN as i16,
            i16::MAX,
            u8::MIN,
            u8::MAX,
            u8::MAX as i32,
        )),
        Ok([
            u16::MIN,
            u16::MAX,
            i16::MAX as u16,
            u16::MIN,
            i16::MAX as u16,
            u8::MIN as u16,
            u8::MAX as u16,
            u8::MAX as u16,
        ])
    );
}

#[test]
fn tuple_to_arr_u16_str_8() {
    assert_eq!(
        TryTupToArr::<u16>::try_into_arr((
            "16",
            true,
            false,
            1616_i32,
            "2025",
            6066_i16,
            u8::MIN as i16,
            u16::MAX as i32,
        )),
        Ok([16, 1, 0, 1616, 2025, 6066, u16::MIN, u16::MAX])
    );
}

// Create mi more tests for other types
#[test]
fn tuple_to_arr_i32_8() {
    assert_eq!(
        TryTupToArr::<i32>::try_into_arr((
            i8::MAX,
            i16::MIN,
            u16::MAX,
            i32::MAX as u32,
            i32::MIN as i64,
            i32::MAX as u64,
            i32::MIN as isize,
            i32::MAX as isize
        )),
        Ok([
            i8::MAX as i32,
            i16::MIN as i32,
            u16::MAX as i32,
            i32::MAX,
            i32::MIN,
            i32::MAX,
            i32::MIN,
            i32::MAX
        ])
    );
}
