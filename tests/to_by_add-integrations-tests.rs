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
}

#[test]
fn to_by_add_isize() {
    assert_eq!(i64::MIN as i64, ToByAdd::to_i64(&isize::MIN));
    assert_eq!(i64::MAX as i64, ToByAdd::to_i64(&isize::MAX));
    assert_eq!(u64::MIN as u64, ToByAdd::to_u64(&isize::MIN));
    assert_eq!(u64::MAX as u64, ToByAdd::to_u64(&isize::MAX));
    assert_eq!(isize::MIN,    ToByAdd::to_isize(&isize::MIN));
    assert_eq!(isize::MAX,    ToByAdd::to_isize(&isize::MAX));
    assert_eq!(usize::MIN,    ToByAdd::to_usize(&isize::MIN));
    assert_eq!(usize::MAX,    ToByAdd::to_usize(&isize::MAX));
}
