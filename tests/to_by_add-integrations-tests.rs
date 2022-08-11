use num_convert::*;

#[test]
fn to_by_add_i8() {
    assert_eq!(i8::MIN,             ToByAdd::to_i8(&i8::MIN));
    assert_eq!(i8::MAX,             ToByAdd::to_i8(&i8::MAX));
    assert_eq!(u8::MIN,             ToByAdd::to_u8(&i8::MIN));
    assert_eq!(u8::MAX,             ToByAdd::to_u8(&i8::MAX));
    assert_eq!(i8::MIN as i16,     ToByAdd::to_i16(&i8::MIN));
    assert_eq!(i8::MAX as i16,     ToByAdd::to_i16(&i8::MAX));
    assert_eq!(u8::MIN as u16,     ToByAdd::to_u16(&i8::MIN));
    assert_eq!(u8::MAX as u16,     ToByAdd::to_u16(&i8::MAX));
    assert_eq!(i8::MIN as i32,     ToByAdd::to_i32(&i8::MIN));
    assert_eq!(i8::MAX as i32,     ToByAdd::to_i32(&i8::MAX));
    assert_eq!(u8::MIN as u32,     ToByAdd::to_u32(&i8::MIN));
    assert_eq!(u8::MAX as u32,     ToByAdd::to_u32(&i8::MAX));
    assert_eq!(i8::MIN as i64,     ToByAdd::to_i64(&i8::MIN));
    assert_eq!(i8::MAX as i64,     ToByAdd::to_i64(&i8::MAX));
    assert_eq!(u8::MIN as u64,     ToByAdd::to_u64(&i8::MIN));
    assert_eq!(u8::MAX as u64,     ToByAdd::to_u64(&i8::MAX));
    assert_eq!(i8::MIN as isize, ToByAdd::to_isize(&i8::MIN));
    assert_eq!(i8::MAX as isize, ToByAdd::to_isize(&i8::MAX));
    assert_eq!(u8::MIN as usize, ToByAdd::to_usize(&i8::MIN));
    assert_eq!(u8::MAX as usize, ToByAdd::to_usize(&i8::MAX));
    assert_eq!(i8::MIN as i128,   ToByAdd::to_i128(&i8::MIN));
    assert_eq!(i8::MAX as i128,   ToByAdd::to_i128(&i8::MAX));
    assert_eq!(u8::MIN as u128,   ToByAdd::to_u128(&i8::MIN));
    assert_eq!(u8::MAX as u128,   ToByAdd::to_u128(&i8::MAX));
}

#[test]
fn to_by_add_u8() {
    assert_eq!(i8::MIN,             ToByAdd::to_i8(&u8::MIN));
    assert_eq!(i8::MAX,             ToByAdd::to_i8(&u8::MAX));
    assert_eq!(u8::MIN,             ToByAdd::to_u8(&u8::MIN));
    assert_eq!(u8::MAX,             ToByAdd::to_u8(&u8::MAX));
    assert_eq!(i8::MIN as i16,     ToByAdd::to_i16(&u8::MIN));
    assert_eq!(i8::MAX as i16,     ToByAdd::to_i16(&u8::MAX));
    assert_eq!(u8::MIN as u16,     ToByAdd::to_u16(&u8::MIN));
    assert_eq!(u8::MAX as u16,     ToByAdd::to_u16(&u8::MAX));
    assert_eq!(i8::MIN as i32,     ToByAdd::to_i32(&u8::MIN));
    assert_eq!(i8::MAX as i32,     ToByAdd::to_i32(&u8::MAX));
    assert_eq!(u8::MIN as u32,     ToByAdd::to_u32(&u8::MIN));
    assert_eq!(u8::MAX as u32,     ToByAdd::to_u32(&u8::MAX));
    assert_eq!(i8::MIN as i64,     ToByAdd::to_i64(&u8::MIN));
    assert_eq!(i8::MAX as i64,     ToByAdd::to_i64(&u8::MAX));
    assert_eq!(u8::MIN as u64,     ToByAdd::to_u64(&u8::MIN));
    assert_eq!(u8::MAX as u64,     ToByAdd::to_u64(&u8::MAX));
    assert_eq!(i8::MIN as isize, ToByAdd::to_isize(&u8::MIN));
    assert_eq!(i8::MAX as isize, ToByAdd::to_isize(&u8::MAX));
    assert_eq!(u8::MIN as usize, ToByAdd::to_usize(&u8::MIN));
    assert_eq!(u8::MAX as usize, ToByAdd::to_usize(&u8::MAX));
    assert_eq!(i8::MIN as i128,   ToByAdd::to_i128(&u8::MIN));
    assert_eq!(i8::MAX as i128,   ToByAdd::to_i128(&u8::MAX));
    assert_eq!(u8::MIN as u128,   ToByAdd::to_u128(&u8::MIN));
    assert_eq!(u8::MAX as u128,   ToByAdd::to_u128(&u8::MAX));
}

#[test]
fn to_by_add_i16() {
    assert_eq!(i16::MIN,            ToByAdd::to_i16(&i16::MIN));
    assert_eq!(i16::MAX,            ToByAdd::to_i16(&i16::MAX));
    assert_eq!(u16::MIN,            ToByAdd::to_u16(&i16::MIN));
    assert_eq!(u16::MAX,            ToByAdd::to_u16(&i16::MAX));
    assert_eq!(i16::MIN as i32,     ToByAdd::to_i32(&i16::MIN));
    assert_eq!(i16::MAX as i32,     ToByAdd::to_i32(&i16::MAX));
    assert_eq!(u16::MIN as u32,     ToByAdd::to_u32(&i16::MIN));
    assert_eq!(u16::MAX as u32,     ToByAdd::to_u32(&i16::MAX));
    assert_eq!(i16::MIN as i64,     ToByAdd::to_i64(&i16::MIN));
    assert_eq!(i16::MAX as i64,     ToByAdd::to_i64(&i16::MAX));
    assert_eq!(u16::MIN as u64,     ToByAdd::to_u64(&i16::MIN));
    assert_eq!(u16::MAX as u64,     ToByAdd::to_u64(&i16::MAX));
    assert_eq!(i16::MIN as isize, ToByAdd::to_isize(&i16::MIN));
    assert_eq!(i16::MAX as isize, ToByAdd::to_isize(&i16::MAX));
    assert_eq!(u16::MIN as usize, ToByAdd::to_usize(&i16::MIN));
    assert_eq!(u16::MAX as usize, ToByAdd::to_usize(&i16::MAX));
    assert_eq!(i16::MIN as i128,   ToByAdd::to_i128(&i16::MIN));
    assert_eq!(i16::MAX as i128,   ToByAdd::to_i128(&i16::MAX));
    assert_eq!(u16::MIN as u128,   ToByAdd::to_u128(&i16::MIN));
    assert_eq!(u16::MAX as u128,   ToByAdd::to_u128(&i16::MAX));
}

#[test]
fn to_by_add_u16() {
    assert_eq!(i16::MIN,            ToByAdd::to_i16(&u16::MIN));
    assert_eq!(i16::MAX,            ToByAdd::to_i16(&u16::MAX));
    assert_eq!(u16::MIN,            ToByAdd::to_u16(&u16::MIN));
    assert_eq!(u16::MAX,            ToByAdd::to_u16(&u16::MAX));
    assert_eq!(i16::MIN as i32,     ToByAdd::to_i32(&u16::MIN));
    assert_eq!(i16::MAX as i32,     ToByAdd::to_i32(&u16::MAX));
    assert_eq!(u16::MIN as u32,     ToByAdd::to_u32(&u16::MIN));
    assert_eq!(u16::MAX as u32,     ToByAdd::to_u32(&u16::MAX));
    assert_eq!(i16::MIN as i64,     ToByAdd::to_i64(&u16::MIN));
    assert_eq!(i16::MAX as i64,     ToByAdd::to_i64(&u16::MAX));
    assert_eq!(u16::MIN as u64,     ToByAdd::to_u64(&u16::MIN));
    assert_eq!(u16::MAX as u64,     ToByAdd::to_u64(&u16::MAX));
    assert_eq!(i16::MIN as isize, ToByAdd::to_isize(&u16::MIN));
    assert_eq!(i16::MAX as isize, ToByAdd::to_isize(&u16::MAX));
    assert_eq!(u16::MIN as usize, ToByAdd::to_usize(&u16::MIN));
    assert_eq!(u16::MAX as usize, ToByAdd::to_usize(&u16::MAX));
    assert_eq!(i16::MIN as i128,   ToByAdd::to_i128(&u16::MIN));
    assert_eq!(i16::MAX as i128,   ToByAdd::to_i128(&u16::MAX));
    assert_eq!(u16::MIN as u128,   ToByAdd::to_u128(&u16::MIN));
    assert_eq!(u16::MAX as u128,   ToByAdd::to_u128(&u16::MAX));
}

#[test]
fn to_by_add_i32() {
    assert_eq!(i32::MIN,            ToByAdd::to_i32(&i32::MIN));
    assert_eq!(i32::MAX,            ToByAdd::to_i32(&i32::MAX));
    assert_eq!(u32::MIN,            ToByAdd::to_u32(&i32::MIN));
    assert_eq!(u32::MAX,            ToByAdd::to_u32(&i32::MAX));
    assert_eq!(i32::MIN as i64,     ToByAdd::to_i64(&i32::MIN));
    assert_eq!(i32::MAX as i64,     ToByAdd::to_i64(&i32::MAX));
    assert_eq!(u32::MIN as u64,     ToByAdd::to_u64(&i32::MIN));
    assert_eq!(u32::MAX as u64,     ToByAdd::to_u64(&i32::MAX));
    assert_eq!(i32::MIN as isize, ToByAdd::to_isize(&i32::MIN));
    assert_eq!(i32::MAX as isize, ToByAdd::to_isize(&i32::MAX));
    assert_eq!(u32::MIN as usize, ToByAdd::to_usize(&i32::MIN));
    assert_eq!(u32::MAX as usize, ToByAdd::to_usize(&i32::MAX));
    assert_eq!(i32::MIN as i128,   ToByAdd::to_i128(&i32::MIN));
    assert_eq!(i32::MAX as i128,   ToByAdd::to_i128(&i32::MAX));
    assert_eq!(u32::MIN as u128,   ToByAdd::to_u128(&i32::MIN));
    assert_eq!(u32::MAX as u128,   ToByAdd::to_u128(&i32::MAX));
}

#[test]
fn to_by_add_u32() {
    assert_eq!(i32::MIN,            ToByAdd::to_i32(&u32::MIN));
    assert_eq!(i32::MAX,            ToByAdd::to_i32(&u32::MAX));
    assert_eq!(u32::MIN,            ToByAdd::to_u32(&u32::MIN));
    assert_eq!(u32::MAX,            ToByAdd::to_u32(&u32::MAX));
    assert_eq!(i32::MIN as i64,     ToByAdd::to_i64(&u32::MIN));
    assert_eq!(i32::MAX as i64,     ToByAdd::to_i64(&u32::MAX));
    assert_eq!(u32::MIN as u64,     ToByAdd::to_u64(&u32::MIN));
    assert_eq!(u32::MAX as u64,     ToByAdd::to_u64(&u32::MAX));
    assert_eq!(i32::MIN as isize, ToByAdd::to_isize(&u32::MIN));
    assert_eq!(i32::MAX as isize, ToByAdd::to_isize(&u32::MAX));
    assert_eq!(u32::MIN as usize, ToByAdd::to_usize(&u32::MIN));
    assert_eq!(u32::MAX as usize, ToByAdd::to_usize(&u32::MAX));
    assert_eq!(i32::MIN as i128,   ToByAdd::to_i128(&u32::MIN));
    assert_eq!(i32::MAX as i128,   ToByAdd::to_i128(&u32::MAX));
    assert_eq!(u32::MIN as u128,   ToByAdd::to_u128(&u32::MIN));
    assert_eq!(u32::MAX as u128,   ToByAdd::to_u128(&u32::MAX));
}

#[test]
fn to_by_add_i64() {
    assert_eq!(i64::MIN,            ToByAdd::to_i64(&i64::MIN));
    assert_eq!(i64::MAX,            ToByAdd::to_i64(&i64::MAX));
    assert_eq!(u64::MIN,            ToByAdd::to_u64(&i64::MIN));
    assert_eq!(u64::MAX,            ToByAdd::to_u64(&i64::MAX));
    assert_eq!(i64::MIN as isize, ToByAdd::to_isize(&i64::MIN));
    assert_eq!(i64::MAX as isize, ToByAdd::to_isize(&i64::MAX));
    assert_eq!(u64::MIN as usize, ToByAdd::to_usize(&i64::MIN));
    assert_eq!(u64::MAX as usize, ToByAdd::to_usize(&i64::MAX));
    assert_eq!(i64::MIN as i128,   ToByAdd::to_i128(&i64::MIN));
    assert_eq!(i64::MAX as i128,   ToByAdd::to_i128(&i64::MAX));
    assert_eq!(u64::MIN as u128,   ToByAdd::to_u128(&i64::MIN));
    assert_eq!(u64::MAX as u128,   ToByAdd::to_u128(&i64::MAX));
}

#[test]
fn to_by_add_u64() {
    assert_eq!(i64::MIN,            ToByAdd::to_i64(&u64::MIN));
    assert_eq!(i64::MAX,            ToByAdd::to_i64(&u64::MAX));
    assert_eq!(u64::MIN,            ToByAdd::to_u64(&u64::MIN));
    assert_eq!(u64::MAX,            ToByAdd::to_u64(&u64::MAX));
    assert_eq!(i64::MIN as isize, ToByAdd::to_isize(&u64::MIN));
    assert_eq!(i64::MAX as isize, ToByAdd::to_isize(&u64::MAX));
    assert_eq!(u64::MIN as usize, ToByAdd::to_usize(&u64::MIN));
    assert_eq!(u64::MAX as usize, ToByAdd::to_usize(&u64::MAX));
    assert_eq!(i64::MIN as i128,   ToByAdd::to_i128(&u64::MIN));
    assert_eq!(i64::MAX as i128,   ToByAdd::to_i128(&u64::MAX));
    assert_eq!(u64::MIN as u128,   ToByAdd::to_u128(&u64::MIN));
    assert_eq!(u64::MAX as u128,   ToByAdd::to_u128(&u64::MAX));
}

#[test]
fn to_by_add_isize() {
    assert_eq!(i64::MIN,            ToByAdd::to_i64(&isize::MIN));
    assert_eq!(i64::MAX,            ToByAdd::to_i64(&isize::MAX));
    assert_eq!(u64::MIN,            ToByAdd::to_u64(&isize::MIN));
    assert_eq!(u64::MAX,            ToByAdd::to_u64(&isize::MAX));
    assert_eq!(isize::MIN,        ToByAdd::to_isize(&isize::MIN));
    assert_eq!(isize::MAX,        ToByAdd::to_isize(&isize::MAX));
    assert_eq!(usize::MIN,        ToByAdd::to_usize(&isize::MIN));
    assert_eq!(usize::MAX,        ToByAdd::to_usize(&isize::MAX));
    assert_eq!(isize::MIN as i128, ToByAdd::to_i128(&isize::MIN));
    assert_eq!(isize::MAX as i128, ToByAdd::to_i128(&isize::MAX));
    assert_eq!(usize::MIN as u128, ToByAdd::to_u128(&isize::MIN));
    assert_eq!(usize::MAX as u128, ToByAdd::to_u128(&isize::MAX));
}

#[test]
fn to_by_add_usize() {
    assert_eq!(i64::MIN,            ToByAdd::to_i64(&usize::MIN));
    assert_eq!(i64::MAX,            ToByAdd::to_i64(&usize::MAX));
    assert_eq!(u64::MIN,            ToByAdd::to_u64(&usize::MIN));
    assert_eq!(u64::MAX,            ToByAdd::to_u64(&usize::MAX));
    assert_eq!(isize::MIN,        ToByAdd::to_isize(&usize::MIN));
    assert_eq!(isize::MAX,        ToByAdd::to_isize(&usize::MAX));
    assert_eq!(usize::MIN,        ToByAdd::to_usize(&usize::MIN));
    assert_eq!(usize::MAX,        ToByAdd::to_usize(&usize::MAX));
    assert_eq!(isize::MIN as i128, ToByAdd::to_i128(&usize::MIN));
    assert_eq!(isize::MAX as i128, ToByAdd::to_i128(&usize::MAX));
    assert_eq!(usize::MIN as u128, ToByAdd::to_u128(&usize::MIN));
    assert_eq!(usize::MAX as u128, ToByAdd::to_u128(&usize::MAX));
}

#[test]
fn to_by_add_i128() {
    assert_eq!(i128::MIN, ToByAdd::to_i128(&i128::MIN));
    assert_eq!(i128::MAX, ToByAdd::to_i128(&i128::MAX));
    assert_eq!(u128::MIN, ToByAdd::to_u128(&i128::MIN));
    assert_eq!(u128::MAX, ToByAdd::to_u128(&i128::MAX));
}

#[test]
fn to_by_add_u128() {
    assert_eq!(i128::MIN, ToByAdd::to_i128(&u128::MIN));
    assert_eq!(i128::MAX, ToByAdd::to_i128(&u128::MAX));
    assert_eq!(u128::MIN, ToByAdd::to_u128(&u128::MIN));
    assert_eq!(u128::MAX, ToByAdd::to_u128(&u128::MAX));
}
