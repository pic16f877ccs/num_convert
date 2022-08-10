use num_convert::*;

#[test]
fn try_into_ok_i8() {
    assert_eq!(i8::MIN,             TryToByAdd::try_into_i8(&i8::MIN).unwrap());
    assert_eq!(i8::MAX,             TryToByAdd::try_into_i8(&i8::MAX).unwrap());
    assert_eq!(u8::MIN,             TryToByAdd::try_into_u8(&i8::MIN).unwrap());
    assert_eq!(u8::MAX,             TryToByAdd::try_into_u8(&i8::MAX).unwrap());
    assert_eq!(i8::MIN as i16,     TryToByAdd::try_into_i16(&i8::MIN).unwrap());
    assert_eq!(i8::MAX as i16,     TryToByAdd::try_into_i16(&i8::MAX).unwrap());
    assert_eq!(u8::MIN as u16,     TryToByAdd::try_into_u16(&i8::MIN).unwrap());
    assert_eq!(u8::MAX as u16,     TryToByAdd::try_into_u16(&i8::MAX).unwrap());
    assert_eq!(i8::MIN as i32,     TryToByAdd::try_into_i32(&i8::MIN).unwrap());
    assert_eq!(i8::MAX as i32,     TryToByAdd::try_into_i32(&i8::MAX).unwrap());
    assert_eq!(u8::MIN as u32,     TryToByAdd::try_into_u32(&i8::MIN).unwrap());
    assert_eq!(u8::MAX as u32,     TryToByAdd::try_into_u32(&i8::MAX).unwrap());
    assert_eq!(i8::MIN as i64,     TryToByAdd::try_into_i64(&i8::MIN).unwrap());
    assert_eq!(i8::MAX as i64,     TryToByAdd::try_into_i64(&i8::MAX).unwrap());
    assert_eq!(u8::MIN as u64,     TryToByAdd::try_into_u64(&i8::MIN).unwrap());
    assert_eq!(u8::MAX as u64,     TryToByAdd::try_into_u64(&i8::MAX).unwrap());
    assert_eq!(i8::MIN as isize, TryToByAdd::try_into_isize(&i8::MIN).unwrap());
    assert_eq!(i8::MAX as isize, TryToByAdd::try_into_isize(&i8::MAX).unwrap());
    assert_eq!(u8::MIN as usize, TryToByAdd::try_into_usize(&i8::MIN).unwrap());
    assert_eq!(u8::MAX as usize, TryToByAdd::try_into_usize(&i8::MAX).unwrap());
    assert_eq!(i8::MIN as i128,   TryToByAdd::try_into_i128(&i8::MIN).unwrap());
    assert_eq!(i8::MAX as i128,   TryToByAdd::try_into_i128(&i8::MAX).unwrap());
    assert_eq!(u8::MIN as u128,   TryToByAdd::try_into_u128(&i8::MIN).unwrap());
    assert_eq!(u8::MAX as u128,   TryToByAdd::try_into_u128(&i8::MAX).unwrap());
}

#[test]
fn try_into_ok_u8() {
    assert_eq!(i8::MIN,             TryToByAdd::try_into_i8(&u8::MIN).unwrap());
    assert_eq!(i8::MAX,             TryToByAdd::try_into_i8(&u8::MAX).unwrap());
    assert_eq!(u8::MIN,             TryToByAdd::try_into_u8(&u8::MIN).unwrap());
    assert_eq!(u8::MAX,             TryToByAdd::try_into_u8(&u8::MAX).unwrap());
    assert_eq!(i8::MIN as i16,     TryToByAdd::try_into_i16(&u8::MIN).unwrap());
    assert_eq!(i8::MAX as i16,     TryToByAdd::try_into_i16(&u8::MAX).unwrap());
    assert_eq!(u8::MIN as u16,     TryToByAdd::try_into_u16(&u8::MIN).unwrap());
    assert_eq!(u8::MAX as u16,     TryToByAdd::try_into_u16(&u8::MAX).unwrap());
    assert_eq!(i8::MIN as i32,     TryToByAdd::try_into_i32(&u8::MIN).unwrap());
    assert_eq!(i8::MAX as i32,     TryToByAdd::try_into_i32(&u8::MAX).unwrap());
    assert_eq!(u8::MIN as u32,     TryToByAdd::try_into_u32(&u8::MIN).unwrap());
    assert_eq!(u8::MAX as u32,     TryToByAdd::try_into_u32(&u8::MAX).unwrap());
    assert_eq!(i8::MIN as i64,     TryToByAdd::try_into_i64(&u8::MIN).unwrap());
    assert_eq!(i8::MAX as i64,     TryToByAdd::try_into_i64(&u8::MAX).unwrap());
    assert_eq!(u8::MIN as u64,     TryToByAdd::try_into_u64(&u8::MIN).unwrap());
    assert_eq!(u8::MAX as u64,     TryToByAdd::try_into_u64(&u8::MAX).unwrap());
    assert_eq!(i8::MIN as isize, TryToByAdd::try_into_isize(&u8::MIN).unwrap());
    assert_eq!(i8::MAX as isize, TryToByAdd::try_into_isize(&u8::MAX).unwrap());
    assert_eq!(u8::MIN as usize, TryToByAdd::try_into_usize(&u8::MIN).unwrap());
    assert_eq!(u8::MAX as usize, TryToByAdd::try_into_usize(&u8::MAX).unwrap());
    assert_eq!(i8::MIN as i128,   TryToByAdd::try_into_i128(&u8::MIN).unwrap());
    assert_eq!(i8::MAX as i128,   TryToByAdd::try_into_i128(&u8::MAX).unwrap());
    assert_eq!(u8::MIN as u128,   TryToByAdd::try_into_u128(&u8::MIN).unwrap());
    assert_eq!(u8::MAX as u128,   TryToByAdd::try_into_u128(&u8::MAX).unwrap());
}

#[test]
#[should_panic]
fn try_into_err_i16() {
    assert_eq!(i8::MIN, TryToByAdd::try_into_i8(&((i8::MIN as i16) - 1)).unwrap());
    assert_eq!(i8::MAX, TryToByAdd::try_into_i8(&((i8::MAX as i16) + 1)).unwrap());
    assert_eq!(u8::MIN, TryToByAdd::try_into_u8(&((i8::MIN as i16) - 1)).unwrap());
    assert_eq!(u8::MAX, TryToByAdd::try_into_u8(&((i8::MAX as i16) + 1)).unwrap());
}

#[test]
fn try_into_ok_i16() {
    assert_eq!(i8::MIN,  TryToByAdd::try_into_i8(&(i8::MIN as i16)).unwrap());
    assert_eq!(i8::MAX,  TryToByAdd::try_into_i8(&(i8::MAX as i16)).unwrap());
    assert_eq!(u8::MIN,  TryToByAdd::try_into_u8(&(i8::MIN as i16)).unwrap());
    assert_eq!(u8::MAX,  TryToByAdd::try_into_u8(&(i8::MAX as i16)).unwrap());
    assert_eq!(i16::MIN,            TryToByAdd::try_into_i16(&i16::MIN).unwrap());
    assert_eq!(i16::MAX,            TryToByAdd::try_into_i16(&i16::MAX).unwrap());
    assert_eq!(u16::MIN,            TryToByAdd::try_into_u16(&i16::MIN).unwrap());
    assert_eq!(u16::MAX,            TryToByAdd::try_into_u16(&i16::MAX).unwrap());
    assert_eq!(i16::MIN as i32,     TryToByAdd::try_into_i32(&i16::MIN).unwrap());
    assert_eq!(i16::MAX as i32,     TryToByAdd::try_into_i32(&i16::MAX).unwrap());
    assert_eq!(u16::MIN as u32,     TryToByAdd::try_into_u32(&i16::MIN).unwrap());
    assert_eq!(u16::MAX as u32,     TryToByAdd::try_into_u32(&i16::MAX).unwrap());
    assert_eq!(i16::MIN as i64,     TryToByAdd::try_into_i64(&i16::MIN).unwrap());
    assert_eq!(i16::MAX as i64,     TryToByAdd::try_into_i64(&i16::MAX).unwrap());
    assert_eq!(u16::MIN as u64,     TryToByAdd::try_into_u64(&i16::MIN).unwrap());
    assert_eq!(u16::MAX as u64,     TryToByAdd::try_into_u64(&i16::MAX).unwrap());
    assert_eq!(i16::MIN as isize, TryToByAdd::try_into_isize(&i16::MIN).unwrap());
    assert_eq!(i16::MAX as isize, TryToByAdd::try_into_isize(&i16::MAX).unwrap());
    assert_eq!(u16::MIN as usize, TryToByAdd::try_into_usize(&i16::MIN).unwrap());
    assert_eq!(u16::MAX as usize, TryToByAdd::try_into_usize(&i16::MAX).unwrap());
    assert_eq!(i16::MIN as i128,   TryToByAdd::try_into_i128(&i16::MIN).unwrap());
    assert_eq!(i16::MAX as i128,   TryToByAdd::try_into_i128(&i16::MAX).unwrap());
    assert_eq!(u16::MIN as u128,   TryToByAdd::try_into_u128(&i16::MIN).unwrap());
    assert_eq!(u16::MAX as u128,   TryToByAdd::try_into_u128(&i16::MAX).unwrap());
}

#[test]
#[should_panic]
fn try_into_err_u16() {
    assert_eq!(i8::MIN, TryToByAdd::try_into_i8(&((u8::MIN as u16) + 1)).unwrap());
    assert_eq!(i8::MAX, TryToByAdd::try_into_i8(&((u8::MAX as u16) + 1)).unwrap());
    assert_eq!(u8::MIN, TryToByAdd::try_into_u8(&((u8::MIN as u16) + 1)).unwrap());
    assert_eq!(u8::MAX, TryToByAdd::try_into_u8(&((u8::MAX as u16) + 1)).unwrap());
}

#[test]
fn try_into_ok_u16() {
    assert_eq!(i8::MIN,  TryToByAdd::try_into_i8(&(u8::MIN as u16)).unwrap());
    assert_eq!(i8::MAX,  TryToByAdd::try_into_i8(&(u8::MAX as u16)).unwrap());
    assert_eq!(u8::MIN,  TryToByAdd::try_into_u8(&(u8::MIN as u16)).unwrap());
    assert_eq!(u8::MAX,  TryToByAdd::try_into_u8(&(u8::MAX as u16)).unwrap());
    assert_eq!(i16::MIN,            TryToByAdd::try_into_i16(&u16::MIN).unwrap());
    assert_eq!(i16::MAX,            TryToByAdd::try_into_i16(&u16::MAX).unwrap());
    assert_eq!(u16::MIN,            TryToByAdd::try_into_u16(&u16::MIN).unwrap());
    assert_eq!(u16::MAX,            TryToByAdd::try_into_u16(&u16::MAX).unwrap());
    assert_eq!(i16::MIN as i32,     TryToByAdd::try_into_i32(&u16::MIN).unwrap());
    assert_eq!(i16::MAX as i32,     TryToByAdd::try_into_i32(&u16::MAX).unwrap());
    assert_eq!(u16::MIN as u32,     TryToByAdd::try_into_u32(&u16::MIN).unwrap());
    assert_eq!(u16::MAX as u32,     TryToByAdd::try_into_u32(&u16::MAX).unwrap());
    assert_eq!(i16::MIN as i64,     TryToByAdd::try_into_i64(&u16::MIN).unwrap());
    assert_eq!(i16::MAX as i64,     TryToByAdd::try_into_i64(&u16::MAX).unwrap());
    assert_eq!(u16::MIN as u64,     TryToByAdd::try_into_u64(&u16::MIN).unwrap());
    assert_eq!(u16::MAX as u64,     TryToByAdd::try_into_u64(&u16::MAX).unwrap());
    assert_eq!(i16::MIN as isize, TryToByAdd::try_into_isize(&u16::MIN).unwrap());
    assert_eq!(i16::MAX as isize, TryToByAdd::try_into_isize(&u16::MAX).unwrap());
    assert_eq!(u16::MIN as usize, TryToByAdd::try_into_usize(&u16::MIN).unwrap());
    assert_eq!(u16::MAX as usize, TryToByAdd::try_into_usize(&u16::MAX).unwrap());
    assert_eq!(i16::MIN as i128,   TryToByAdd::try_into_i128(&u16::MIN).unwrap());
    assert_eq!(i16::MAX as i128,   TryToByAdd::try_into_i128(&u16::MAX).unwrap());
    assert_eq!(u16::MIN as u128,   TryToByAdd::try_into_u128(&u16::MIN).unwrap());
    assert_eq!(u16::MAX as u128,   TryToByAdd::try_into_u128(&u16::MAX).unwrap());
}

#[test]
#[should_panic]
fn try_into_err_i32() {
    assert_eq!(i8::MIN,    TryToByAdd::try_into_i8(&((i8::MIN as i32) - 1)).unwrap());
    assert_eq!(i8::MAX,    TryToByAdd::try_into_i8(&((i8::MAX as i32) + 1)).unwrap());
    assert_eq!(u8::MIN,    TryToByAdd::try_into_u8(&((i8::MIN as i32) - 1)).unwrap());
    assert_eq!(u8::MAX,    TryToByAdd::try_into_u8(&((i8::MAX as i32) + 1)).unwrap());
    assert_eq!(i16::MIN, TryToByAdd::try_into_i16(&((i16::MIN as i32) - 1)).unwrap());
    assert_eq!(i16::MAX, TryToByAdd::try_into_i16(&((i16::MAX as i32) + 1)).unwrap());
    assert_eq!(u16::MIN, TryToByAdd::try_into_u16(&((i16::MIN as i32) - 1)).unwrap());
    assert_eq!(u16::MAX, TryToByAdd::try_into_u16(&((i16::MAX as i32) + 1)).unwrap());
}

#[test]
fn try_into_ok_i32() {
    assert_eq!(i8::MIN,    TryToByAdd::try_into_i8(&(i8::MIN as i32)).unwrap());
    assert_eq!(i8::MAX,    TryToByAdd::try_into_i8(&(i8::MAX as i32)).unwrap());
    assert_eq!(u8::MIN,    TryToByAdd::try_into_u8(&(i8::MIN as i32)).unwrap());
    assert_eq!(u8::MAX,    TryToByAdd::try_into_u8(&(i8::MAX as i32)).unwrap());
    assert_eq!(i16::MIN, TryToByAdd::try_into_i16(&(i16::MIN as i32)).unwrap());
    assert_eq!(i16::MAX, TryToByAdd::try_into_i16(&(i16::MAX as i32)).unwrap());
    assert_eq!(u16::MIN, TryToByAdd::try_into_u16(&(i16::MIN as i32)).unwrap());
    assert_eq!(u16::MAX, TryToByAdd::try_into_u16(&(i16::MAX as i32)).unwrap());
    assert_eq!(i32::MIN,              TryToByAdd::try_into_i32(&i32::MIN).unwrap());
    assert_eq!(i32::MAX,              TryToByAdd::try_into_i32(&i32::MAX).unwrap());
    assert_eq!(u32::MIN,              TryToByAdd::try_into_u32(&i32::MIN).unwrap());
    assert_eq!(u32::MAX,              TryToByAdd::try_into_u32(&i32::MAX).unwrap());
    assert_eq!(i32::MIN as i64,       TryToByAdd::try_into_i64(&i32::MIN).unwrap());
    assert_eq!(i32::MAX as i64,       TryToByAdd::try_into_i64(&i32::MAX).unwrap());
    assert_eq!(u32::MIN as u64,       TryToByAdd::try_into_u64(&i32::MIN).unwrap());
    assert_eq!(u32::MAX as u64,       TryToByAdd::try_into_u64(&i32::MAX).unwrap());
    assert_eq!(i32::MIN as isize,   TryToByAdd::try_into_isize(&i32::MIN).unwrap());
    assert_eq!(i32::MAX as isize,   TryToByAdd::try_into_isize(&i32::MAX).unwrap());
    assert_eq!(u32::MIN as usize,   TryToByAdd::try_into_usize(&i32::MIN).unwrap());
    assert_eq!(u32::MAX as usize,   TryToByAdd::try_into_usize(&i32::MAX).unwrap());
    assert_eq!(i32::MIN as i128,     TryToByAdd::try_into_i128(&i32::MIN).unwrap());
    assert_eq!(i32::MAX as i128,     TryToByAdd::try_into_i128(&i32::MAX).unwrap());
    assert_eq!(u32::MIN as u128,     TryToByAdd::try_into_u128(&i32::MIN).unwrap());
    assert_eq!(u32::MAX as u128,     TryToByAdd::try_into_u128(&i32::MAX).unwrap());
}

#[test]
#[should_panic]
fn try_into_err_u32() {
    assert_eq!(i8::MIN,   TryToByAdd::try_into_i8(&((u8::MIN  as u32) + 1)).unwrap());
    assert_eq!(i8::MAX,   TryToByAdd::try_into_i8(&((u8::MAX  as u32) + 1)).unwrap());
    assert_eq!(u8::MIN,   TryToByAdd::try_into_u8(&((u8::MIN  as u32) + 1)).unwrap());
    assert_eq!(u8::MAX,   TryToByAdd::try_into_u8(&((u8::MAX  as u32) + 1)).unwrap());
    assert_eq!(i16::MIN, TryToByAdd::try_into_i16(&((u16::MIN as u32) + 1)).unwrap());
    assert_eq!(i16::MAX, TryToByAdd::try_into_i16(&((u16::MAX as u32) + 1)).unwrap());
    assert_eq!(u16::MIN, TryToByAdd::try_into_u16(&((u16::MIN as u32) + 1)).unwrap());
    assert_eq!(u16::MAX, TryToByAdd::try_into_u16(&((u16::MAX as u32) + 1)).unwrap());
}

#[test]
fn try_into_ok_u32() {
    assert_eq!(i8::MIN,   TryToByAdd::try_into_i8(&(u8::MIN  as u32)).unwrap());
    assert_eq!(i8::MAX,   TryToByAdd::try_into_i8(&(u8::MAX  as u32)).unwrap());
    assert_eq!(u8::MIN,   TryToByAdd::try_into_u8(&(u8::MIN  as u32)).unwrap());
    assert_eq!(u8::MAX,   TryToByAdd::try_into_u8(&(u8::MAX  as u32)).unwrap());
    assert_eq!(i16::MIN, TryToByAdd::try_into_i16(&(u16::MIN as u32)).unwrap());
    assert_eq!(i16::MAX, TryToByAdd::try_into_i16(&(u16::MAX as u32)).unwrap());
    assert_eq!(u16::MIN, TryToByAdd::try_into_u16(&(u16::MIN as u32)).unwrap());
    assert_eq!(u16::MAX, TryToByAdd::try_into_u16(&(u16::MAX as u32)).unwrap());
    assert_eq!(i32::MIN,              TryToByAdd::try_into_i32(&u32::MIN).unwrap());
    assert_eq!(i32::MAX,              TryToByAdd::try_into_i32(&u32::MAX).unwrap());
    assert_eq!(u32::MIN,              TryToByAdd::try_into_u32(&u32::MIN).unwrap());
    assert_eq!(u32::MAX,              TryToByAdd::try_into_u32(&u32::MAX).unwrap());
    assert_eq!(i32::MIN as i64,       TryToByAdd::try_into_i64(&u32::MIN).unwrap());
    assert_eq!(i32::MAX as i64,       TryToByAdd::try_into_i64(&u32::MAX).unwrap());
    assert_eq!(u32::MIN as u64,       TryToByAdd::try_into_u64(&u32::MIN).unwrap());
    assert_eq!(u32::MAX as u64,       TryToByAdd::try_into_u64(&u32::MAX).unwrap());
    assert_eq!(i32::MIN as isize,   TryToByAdd::try_into_isize(&u32::MIN).unwrap());
    assert_eq!(i32::MAX as isize,   TryToByAdd::try_into_isize(&u32::MAX).unwrap());
    assert_eq!(u32::MIN as usize,   TryToByAdd::try_into_usize(&u32::MIN).unwrap());
    assert_eq!(u32::MAX as usize,   TryToByAdd::try_into_usize(&u32::MAX).unwrap());
    assert_eq!(i32::MIN as i128,     TryToByAdd::try_into_i128(&u32::MIN).unwrap());
    assert_eq!(i32::MAX as i128,     TryToByAdd::try_into_i128(&u32::MAX).unwrap());
    assert_eq!(u32::MIN as u128,     TryToByAdd::try_into_u128(&u32::MIN).unwrap());
    assert_eq!(u32::MAX as u128,     TryToByAdd::try_into_u128(&u32::MAX).unwrap());
}

#[test]
#[should_panic]
fn try_into_err_i64() {
    assert_eq!(i8::MIN,   TryToByAdd::try_into_i8(&((i8::MIN  as i64) - 1)).unwrap());
    assert_eq!(i8::MAX,   TryToByAdd::try_into_i8(&((i8::MAX  as i64) + 1)).unwrap());
    assert_eq!(u8::MIN,   TryToByAdd::try_into_u8(&((i8::MIN  as i64) - 1)).unwrap());
    assert_eq!(u8::MAX,   TryToByAdd::try_into_u8(&((i8::MAX  as i64) + 1)).unwrap());
    assert_eq!(i16::MIN, TryToByAdd::try_into_i16(&((i16::MIN as i64) - 1)).unwrap());
    assert_eq!(i16::MAX, TryToByAdd::try_into_i16(&((i16::MAX as i64) + 1)).unwrap());
    assert_eq!(u16::MIN, TryToByAdd::try_into_u16(&((i16::MIN as i64) - 1)).unwrap());
    assert_eq!(u16::MAX, TryToByAdd::try_into_u16(&((i16::MAX as i64) + 1)).unwrap());
    assert_eq!(i32::MIN, TryToByAdd::try_into_i32(&((i32::MIN as i64) - 1)).unwrap());
    assert_eq!(i32::MAX, TryToByAdd::try_into_i32(&((i32::MAX as i64) + 1)).unwrap());
    assert_eq!(u32::MIN, TryToByAdd::try_into_u32(&((i32::MIN as i64) - 1)).unwrap());
    assert_eq!(u32::MAX, TryToByAdd::try_into_u32(&((i32::MAX as i64) + 1)).unwrap());
}

#[test]
fn try_into_ok_i64() {
    assert_eq!(i8::MIN,   TryToByAdd::try_into_i8(&(i8::MIN  as i64)).unwrap());
    assert_eq!(i8::MAX,   TryToByAdd::try_into_i8(&(i8::MAX  as i64)).unwrap());
    assert_eq!(u8::MIN,   TryToByAdd::try_into_u8(&(i8::MIN  as i64)).unwrap());
    assert_eq!(u8::MAX,   TryToByAdd::try_into_u8(&(i8::MAX  as i64)).unwrap());
    assert_eq!(i16::MIN, TryToByAdd::try_into_i16(&(i16::MIN as i64)).unwrap());
    assert_eq!(i16::MAX, TryToByAdd::try_into_i16(&(i16::MAX as i64)).unwrap());
    assert_eq!(u16::MIN, TryToByAdd::try_into_u16(&(i16::MIN as i64)).unwrap());
    assert_eq!(u16::MAX, TryToByAdd::try_into_u16(&(i16::MAX as i64)).unwrap());
    assert_eq!(i32::MIN, TryToByAdd::try_into_i32(&(i32::MIN as i64)).unwrap());
    assert_eq!(i32::MAX, TryToByAdd::try_into_i32(&(i32::MAX as i64)).unwrap());
    assert_eq!(u32::MIN, TryToByAdd::try_into_u32(&(i32::MIN as i64)).unwrap());
    assert_eq!(u32::MAX, TryToByAdd::try_into_u32(&(i32::MAX as i64)).unwrap());
    assert_eq!(i64::MIN,              TryToByAdd::try_into_i64(&i64::MIN).unwrap());
    assert_eq!(i64::MAX,              TryToByAdd::try_into_i64(&i64::MAX).unwrap());
    assert_eq!(u64::MIN,              TryToByAdd::try_into_u64(&i64::MIN).unwrap());
    assert_eq!(u64::MAX,              TryToByAdd::try_into_u64(&i64::MAX).unwrap());
    assert_eq!(i64::MIN as isize,   TryToByAdd::try_into_isize(&i64::MIN).unwrap());
    assert_eq!(i64::MAX as isize,   TryToByAdd::try_into_isize(&i64::MAX).unwrap());
    assert_eq!(u64::MIN as usize,   TryToByAdd::try_into_usize(&i64::MIN).unwrap());
    assert_eq!(u64::MAX as usize,   TryToByAdd::try_into_usize(&i64::MAX).unwrap());
    assert_eq!(i64::MIN as i128,     TryToByAdd::try_into_i128(&i64::MIN).unwrap());
    assert_eq!(i64::MAX as i128,     TryToByAdd::try_into_i128(&i64::MAX).unwrap());
    assert_eq!(u64::MIN as u128,     TryToByAdd::try_into_u128(&i64::MIN).unwrap());
    assert_eq!(u64::MAX as u128,     TryToByAdd::try_into_u128(&i64::MAX).unwrap());
}

#[test]
#[should_panic]
fn try_into_err_u64() {
    assert_eq!(i8::MIN,   TryToByAdd::try_into_i8(&((u8::MIN  as u64) + 1)).unwrap());
    assert_eq!(i8::MAX,   TryToByAdd::try_into_i8(&((u8::MAX  as u64) + 1)).unwrap());
    assert_eq!(u8::MIN,   TryToByAdd::try_into_u8(&((u8::MIN  as u64) + 1)).unwrap());
    assert_eq!(u8::MAX,   TryToByAdd::try_into_u8(&((u8::MAX  as u64) + 1)).unwrap());
    assert_eq!(i16::MIN, TryToByAdd::try_into_i16(&((u16::MIN as u64) + 1)).unwrap());
    assert_eq!(i16::MAX, TryToByAdd::try_into_i16(&((u16::MAX as u64) + 1)).unwrap());
    assert_eq!(u16::MIN, TryToByAdd::try_into_u16(&((u16::MIN as u64) + 1)).unwrap());
    assert_eq!(u16::MAX, TryToByAdd::try_into_u16(&((u16::MAX as u64) + 1)).unwrap());
    assert_eq!(i32::MIN, TryToByAdd::try_into_i32(&((u32::MIN as u64) + 1)).unwrap());
    assert_eq!(i32::MAX, TryToByAdd::try_into_i32(&((u32::MAX as u64) + 1)).unwrap());
    assert_eq!(u32::MIN, TryToByAdd::try_into_u32(&((u32::MIN as u64) + 1)).unwrap());
    assert_eq!(u32::MAX, TryToByAdd::try_into_u32(&((u32::MAX as u64) + 1)).unwrap());
}

#[test]
fn try_into_ok_u64() {
    assert_eq!(i8::MIN,   TryToByAdd::try_into_i8(&(u8::MIN  as u64)).unwrap());
    assert_eq!(i8::MAX,   TryToByAdd::try_into_i8(&(u8::MAX  as u64)).unwrap());
    assert_eq!(u8::MIN,   TryToByAdd::try_into_u8(&(u8::MIN  as u64)).unwrap());
    assert_eq!(u8::MAX,   TryToByAdd::try_into_u8(&(u8::MAX  as u64)).unwrap());
    assert_eq!(i16::MIN, TryToByAdd::try_into_i16(&(u16::MIN as u64)).unwrap());
    assert_eq!(i16::MAX, TryToByAdd::try_into_i16(&(u16::MAX as u64)).unwrap());
    assert_eq!(u16::MIN, TryToByAdd::try_into_u16(&(u16::MIN as u64)).unwrap());
    assert_eq!(u16::MAX, TryToByAdd::try_into_u16(&(u16::MAX as u64)).unwrap());
    assert_eq!(i32::MIN, TryToByAdd::try_into_i32(&(u32::MIN as u64)).unwrap());
    assert_eq!(i32::MAX, TryToByAdd::try_into_i32(&(u32::MAX as u64)).unwrap());
    assert_eq!(u32::MIN, TryToByAdd::try_into_u32(&(u32::MIN as u64)).unwrap());
    assert_eq!(u32::MAX, TryToByAdd::try_into_u32(&(u32::MAX as u64)).unwrap());
    assert_eq!(i64::MIN,              TryToByAdd::try_into_i64(&u64::MIN).unwrap());
    assert_eq!(i64::MAX,              TryToByAdd::try_into_i64(&u64::MAX).unwrap());
    assert_eq!(u64::MIN,              TryToByAdd::try_into_u64(&u64::MIN).unwrap());
    assert_eq!(u64::MAX,              TryToByAdd::try_into_u64(&u64::MAX).unwrap());
    assert_eq!(i64::MIN as isize,   TryToByAdd::try_into_isize(&u64::MIN).unwrap());
    assert_eq!(i64::MAX as isize,   TryToByAdd::try_into_isize(&u64::MAX).unwrap());
    assert_eq!(u64::MIN as usize,   TryToByAdd::try_into_usize(&u64::MIN).unwrap());
    assert_eq!(u64::MAX as usize,   TryToByAdd::try_into_usize(&u64::MAX).unwrap());
    assert_eq!(i64::MIN as i128,     TryToByAdd::try_into_i128(&u64::MIN).unwrap());
    assert_eq!(i64::MAX as i128,     TryToByAdd::try_into_i128(&u64::MAX).unwrap());
    assert_eq!(u64::MIN as u128,     TryToByAdd::try_into_u128(&u64::MIN).unwrap());
    assert_eq!(u64::MAX as u128,     TryToByAdd::try_into_u128(&u64::MAX).unwrap());
}

#[test]
#[should_panic]
fn try_into_err_isize() {
    assert_eq!(i8::MIN,   TryToByAdd::try_into_i8(&((i8::MIN  as isize) - 1)).unwrap());
    assert_eq!(i8::MAX,   TryToByAdd::try_into_i8(&((i8::MAX  as isize) + 1)).unwrap());
    assert_eq!(u8::MIN,   TryToByAdd::try_into_u8(&((i8::MIN  as isize) - 1)).unwrap());
    assert_eq!(u8::MAX,   TryToByAdd::try_into_u8(&((i8::MAX  as isize) + 1)).unwrap());
    assert_eq!(i16::MIN, TryToByAdd::try_into_i16(&((i16::MIN as isize) - 1)).unwrap());
    assert_eq!(i16::MAX, TryToByAdd::try_into_i16(&((i16::MAX as isize) + 1)).unwrap());
    assert_eq!(u16::MIN, TryToByAdd::try_into_u16(&((i16::MIN as isize) - 1)).unwrap());
    assert_eq!(u16::MAX, TryToByAdd::try_into_u16(&((i16::MAX as isize) + 1)).unwrap());
    assert_eq!(i32::MIN, TryToByAdd::try_into_i32(&((i32::MIN as isize) - 1)).unwrap());
    assert_eq!(i32::MAX, TryToByAdd::try_into_i32(&((i32::MAX as isize) + 1)).unwrap());
    assert_eq!(u32::MIN, TryToByAdd::try_into_u32(&((i32::MIN as isize) - 1)).unwrap());
    assert_eq!(u32::MAX, TryToByAdd::try_into_u32(&((i32::MAX as isize) + 1)).unwrap());
}

#[test]
fn try_into_ok_isize() {
    assert_eq!(i8::MIN,   TryToByAdd::try_into_i8(&(i8::MIN  as isize)).unwrap());
    assert_eq!(i8::MAX,   TryToByAdd::try_into_i8(&(i8::MAX  as isize)).unwrap());
    assert_eq!(u8::MIN,   TryToByAdd::try_into_u8(&(i8::MIN  as isize)).unwrap());
    assert_eq!(u8::MAX,   TryToByAdd::try_into_u8(&(i8::MAX  as isize)).unwrap());
    assert_eq!(i16::MIN, TryToByAdd::try_into_i16(&(i16::MIN as isize)).unwrap());
    assert_eq!(i16::MAX, TryToByAdd::try_into_i16(&(i16::MAX as isize)).unwrap());
    assert_eq!(u16::MIN, TryToByAdd::try_into_u16(&(i16::MIN as isize)).unwrap());
    assert_eq!(u16::MAX, TryToByAdd::try_into_u16(&(i16::MAX as isize)).unwrap());
    assert_eq!(i32::MIN, TryToByAdd::try_into_i32(&(i32::MIN as isize)).unwrap());
    assert_eq!(i32::MAX, TryToByAdd::try_into_i32(&(i32::MAX as isize)).unwrap());
    assert_eq!(u32::MIN, TryToByAdd::try_into_u32(&(i32::MIN as isize)).unwrap());
    assert_eq!(u32::MAX, TryToByAdd::try_into_u32(&(i32::MAX as isize)).unwrap());
    assert_eq!(i64::MIN,              TryToByAdd::try_into_i64(&isize::MIN).unwrap());
    assert_eq!(i64::MAX,              TryToByAdd::try_into_i64(&isize::MAX).unwrap());
    assert_eq!(u64::MIN,              TryToByAdd::try_into_u64(&isize::MIN).unwrap());
    assert_eq!(u64::MAX,              TryToByAdd::try_into_u64(&isize::MAX).unwrap());
    assert_eq!(isize::MIN,          TryToByAdd::try_into_isize(&isize::MIN).unwrap());
    assert_eq!(isize::MAX,          TryToByAdd::try_into_isize(&isize::MAX).unwrap());
    assert_eq!(usize::MIN,          TryToByAdd::try_into_usize(&isize::MIN).unwrap());
    assert_eq!(usize::MAX,          TryToByAdd::try_into_usize(&isize::MAX).unwrap());
    assert_eq!(isize::MIN as i128,   TryToByAdd::try_into_i128(&isize::MIN).unwrap());
    assert_eq!(isize::MAX as i128,   TryToByAdd::try_into_i128(&isize::MAX).unwrap());
    assert_eq!(usize::MIN as u128,   TryToByAdd::try_into_u128(&isize::MIN).unwrap());
    assert_eq!(usize::MAX as u128,   TryToByAdd::try_into_u128(&isize::MAX).unwrap());
}

#[test]
#[should_panic]
fn try_into_err_usize() {
    assert_eq!(i8::MIN,   TryToByAdd::try_into_i8(&((u8::MIN  as usize) + 1)).unwrap());
    assert_eq!(i8::MAX,   TryToByAdd::try_into_i8(&((u8::MAX  as usize) + 1)).unwrap());
    assert_eq!(u8::MIN,   TryToByAdd::try_into_u8(&((u8::MIN  as usize) + 1)).unwrap());
    assert_eq!(u8::MAX,   TryToByAdd::try_into_u8(&((u8::MAX  as usize) + 1)).unwrap());
    assert_eq!(i16::MIN, TryToByAdd::try_into_i16(&((u16::MIN as usize) + 1)).unwrap());
    assert_eq!(i16::MAX, TryToByAdd::try_into_i16(&((u16::MAX as usize) + 1)).unwrap());
    assert_eq!(u16::MIN, TryToByAdd::try_into_u16(&((u16::MIN as usize) + 1)).unwrap());
    assert_eq!(u16::MAX, TryToByAdd::try_into_u16(&((u16::MAX as usize) + 1)).unwrap());
    assert_eq!(i32::MIN, TryToByAdd::try_into_i32(&((u32::MIN as usize) + 1)).unwrap());
    assert_eq!(i32::MAX, TryToByAdd::try_into_i32(&((u32::MAX as usize) + 1)).unwrap());
    assert_eq!(u32::MIN, TryToByAdd::try_into_u32(&((u32::MIN as usize) + 1)).unwrap());
    assert_eq!(u32::MAX, TryToByAdd::try_into_u32(&((u32::MAX as usize) + 1)).unwrap());
}

#[test]
fn try_into_ok_usize() {
    assert_eq!(i8::MIN,   TryToByAdd::try_into_i8(&(u8::MIN  as usize)).unwrap());
    assert_eq!(i8::MAX,   TryToByAdd::try_into_i8(&(u8::MAX  as usize)).unwrap());
    assert_eq!(u8::MIN,   TryToByAdd::try_into_u8(&(u8::MIN  as usize)).unwrap());
    assert_eq!(u8::MAX,   TryToByAdd::try_into_u8(&(u8::MAX  as usize)).unwrap());
    assert_eq!(i16::MIN, TryToByAdd::try_into_i16(&(u16::MIN as usize)).unwrap());
    assert_eq!(i16::MAX, TryToByAdd::try_into_i16(&(u16::MAX as usize)).unwrap());
    assert_eq!(u16::MIN, TryToByAdd::try_into_u16(&(u16::MIN as usize)).unwrap());
    assert_eq!(u16::MAX, TryToByAdd::try_into_u16(&(u16::MAX as usize)).unwrap());
    assert_eq!(i32::MIN, TryToByAdd::try_into_i32(&(u32::MIN as usize)).unwrap());
    assert_eq!(i32::MAX, TryToByAdd::try_into_i32(&(u32::MAX as usize)).unwrap());
    assert_eq!(u32::MIN, TryToByAdd::try_into_u32(&(u32::MIN as usize)).unwrap());
    assert_eq!(u32::MAX, TryToByAdd::try_into_u32(&(u32::MAX as usize)).unwrap());
    assert_eq!(i64::MIN,              TryToByAdd::try_into_i64(&usize::MIN).unwrap());
    assert_eq!(i64::MAX,              TryToByAdd::try_into_i64(&usize::MAX).unwrap());
    assert_eq!(u64::MIN,              TryToByAdd::try_into_u64(&usize::MIN).unwrap());
    assert_eq!(u64::MAX,              TryToByAdd::try_into_u64(&usize::MAX).unwrap());
    assert_eq!(isize::MIN,          TryToByAdd::try_into_isize(&usize::MIN).unwrap());
    assert_eq!(isize::MAX,          TryToByAdd::try_into_isize(&usize::MAX).unwrap());
    assert_eq!(usize::MIN,          TryToByAdd::try_into_usize(&usize::MIN).unwrap());
    assert_eq!(usize::MAX,          TryToByAdd::try_into_usize(&usize::MAX).unwrap());
    assert_eq!(isize::MIN as i128,   TryToByAdd::try_into_i128(&usize::MIN).unwrap());
    assert_eq!(isize::MAX as i128,   TryToByAdd::try_into_i128(&usize::MAX).unwrap());
    assert_eq!(usize::MIN as u128,   TryToByAdd::try_into_u128(&usize::MIN).unwrap());
    assert_eq!(usize::MAX as u128,   TryToByAdd::try_into_u128(&usize::MAX).unwrap());
}

#[test]
#[should_panic]
fn try_into_err_i128() {
    assert_eq!(i8::MIN,       TryToByAdd::try_into_i8(&((i8::MIN    as i128) - 1)).unwrap());
    assert_eq!(i8::MAX,       TryToByAdd::try_into_i8(&((i8::MAX    as i128) + 1)).unwrap());
    assert_eq!(u8::MIN,       TryToByAdd::try_into_u8(&((i8::MIN    as i128) - 1)).unwrap());
    assert_eq!(u8::MAX,       TryToByAdd::try_into_u8(&((i8::MAX    as i128) + 1)).unwrap());
    assert_eq!(i16::MIN,     TryToByAdd::try_into_i16(&((i16::MIN   as i128) - 1)).unwrap());
    assert_eq!(i16::MAX,     TryToByAdd::try_into_i16(&((i16::MAX   as i128) + 1)).unwrap());
    assert_eq!(u16::MIN,     TryToByAdd::try_into_u16(&((i16::MIN   as i128) - 1)).unwrap());
    assert_eq!(u16::MAX,     TryToByAdd::try_into_u16(&((i16::MAX   as i128) + 1)).unwrap());
    assert_eq!(i32::MIN,     TryToByAdd::try_into_i32(&((i32::MIN   as i128) - 1)).unwrap());
    assert_eq!(i32::MAX,     TryToByAdd::try_into_i32(&((i32::MAX   as i128) + 1)).unwrap());
    assert_eq!(u32::MIN,     TryToByAdd::try_into_u32(&((i32::MIN   as i128) - 1)).unwrap());
    assert_eq!(u32::MAX,     TryToByAdd::try_into_u32(&((i32::MAX   as i128) + 1)).unwrap());
    assert_eq!(i64::MIN,     TryToByAdd::try_into_i64(&((i64::MIN   as i128) - 1)).unwrap());
    assert_eq!(i64::MAX,     TryToByAdd::try_into_i64(&((i64::MAX   as i128) + 1)).unwrap());
    assert_eq!(u64::MIN,     TryToByAdd::try_into_u64(&((i64::MIN   as i128) - 1)).unwrap());
    assert_eq!(u64::MAX,     TryToByAdd::try_into_u64(&((i64::MAX   as i128) + 1)).unwrap());
    assert_eq!(isize::MIN, TryToByAdd::try_into_isize(&((isize::MIN as i128) - 1)).unwrap());
    assert_eq!(isize::MAX, TryToByAdd::try_into_isize(&((isize::MAX as i128) + 1)).unwrap());
    assert_eq!(usize::MIN, TryToByAdd::try_into_usize(&((isize::MIN as i128) - 1)).unwrap());
    assert_eq!(usize::MAX, TryToByAdd::try_into_usize(&((isize::MAX as i128) + 1)).unwrap());
}

#[test]
fn try_into_ok_i128() {
    assert_eq!(i8::MIN,       TryToByAdd::try_into_i8(&(i8::MIN    as i128)).unwrap());
    assert_eq!(i8::MAX,       TryToByAdd::try_into_i8(&(i8::MAX    as i128)).unwrap());
    assert_eq!(u8::MIN,       TryToByAdd::try_into_u8(&(i8::MIN    as i128)).unwrap());
    assert_eq!(u8::MAX,       TryToByAdd::try_into_u8(&(i8::MAX    as i128)).unwrap());
    assert_eq!(i16::MIN,     TryToByAdd::try_into_i16(&(i16::MIN   as i128)).unwrap());
    assert_eq!(i16::MAX,     TryToByAdd::try_into_i16(&(i16::MAX   as i128)).unwrap());
    assert_eq!(u16::MIN,     TryToByAdd::try_into_u16(&(i16::MIN   as i128)).unwrap());
    assert_eq!(u16::MAX,     TryToByAdd::try_into_u16(&(i16::MAX   as i128)).unwrap());
    assert_eq!(i32::MIN,     TryToByAdd::try_into_i32(&(i32::MIN   as i128)).unwrap());
    assert_eq!(i32::MAX,     TryToByAdd::try_into_i32(&(i32::MAX   as i128)).unwrap());
    assert_eq!(u32::MIN,     TryToByAdd::try_into_u32(&(i32::MIN   as i128)).unwrap());
    assert_eq!(u32::MAX,     TryToByAdd::try_into_u32(&(i32::MAX   as i128)).unwrap());
    assert_eq!(i64::MIN,     TryToByAdd::try_into_i64(&(i64::MIN   as i128)).unwrap());
    assert_eq!(i64::MAX,     TryToByAdd::try_into_i64(&(i64::MAX   as i128)).unwrap());
    assert_eq!(u64::MIN,     TryToByAdd::try_into_u64(&(i64::MIN   as i128)).unwrap());
    assert_eq!(u64::MAX,     TryToByAdd::try_into_u64(&(i64::MAX   as i128)).unwrap());
    assert_eq!(isize::MIN, TryToByAdd::try_into_isize(&(isize::MIN as i128)).unwrap());
    assert_eq!(isize::MAX, TryToByAdd::try_into_isize(&(isize::MAX as i128)).unwrap());
    assert_eq!(usize::MIN, TryToByAdd::try_into_usize(&(isize::MIN as i128)).unwrap());
    assert_eq!(usize::MAX, TryToByAdd::try_into_usize(&(isize::MAX as i128)).unwrap());
    assert_eq!(i128::MIN,                  TryToByAdd::try_into_i128(&i128::MIN).unwrap());
    assert_eq!(i128::MAX,                  TryToByAdd::try_into_i128(&i128::MAX).unwrap());
    assert_eq!(u128::MIN,                  TryToByAdd::try_into_u128(&i128::MIN).unwrap());
    assert_eq!(u128::MAX,                  TryToByAdd::try_into_u128(&i128::MAX).unwrap());
}

#[test]
#[should_panic]
fn try_into_err_u128() {
    assert_eq!(i8::MIN,       TryToByAdd::try_into_i8(&((u8::MIN    as u128) + 1)).unwrap());
    assert_eq!(i8::MAX,       TryToByAdd::try_into_i8(&((u8::MAX    as u128) + 1)).unwrap());
    assert_eq!(u8::MIN,       TryToByAdd::try_into_u8(&((u8::MIN    as u128) + 1)).unwrap());
    assert_eq!(u8::MAX,       TryToByAdd::try_into_u8(&((u8::MAX    as u128) + 1)).unwrap());
    assert_eq!(i16::MIN,     TryToByAdd::try_into_i16(&((u16::MIN   as u128) + 1)).unwrap());
    assert_eq!(i16::MAX,     TryToByAdd::try_into_i16(&((u16::MAX   as u128) + 1)).unwrap());
    assert_eq!(u16::MIN,     TryToByAdd::try_into_u16(&((u16::MIN   as u128) + 1)).unwrap());
    assert_eq!(u16::MAX,     TryToByAdd::try_into_u16(&((u16::MAX   as u128) + 1)).unwrap());
    assert_eq!(i32::MIN,     TryToByAdd::try_into_i32(&((u32::MIN   as u128) + 1)).unwrap());
    assert_eq!(i32::MAX,     TryToByAdd::try_into_i32(&((u32::MAX   as u128) + 1)).unwrap());
    assert_eq!(u32::MIN,     TryToByAdd::try_into_u32(&((u32::MIN   as u128) + 1)).unwrap());
    assert_eq!(u32::MAX,     TryToByAdd::try_into_u32(&((u32::MAX   as u128) + 1)).unwrap());
    assert_eq!(i64::MIN,     TryToByAdd::try_into_i64(&((u64::MIN   as u128) + 1)).unwrap());
    assert_eq!(i64::MAX,     TryToByAdd::try_into_i64(&((u64::MAX   as u128) + 1)).unwrap());
    assert_eq!(u64::MIN,     TryToByAdd::try_into_u64(&((u64::MIN   as u128) + 1)).unwrap());
    assert_eq!(u64::MAX,     TryToByAdd::try_into_u64(&((u64::MAX   as u128) + 1)).unwrap());
    assert_eq!(isize::MIN, TryToByAdd::try_into_isize(&((usize::MIN as u128) + 1)).unwrap());
    assert_eq!(isize::MAX, TryToByAdd::try_into_isize(&((usize::MAX as u128) + 1)).unwrap());
    assert_eq!(usize::MIN, TryToByAdd::try_into_usize(&((usize::MIN as u128) + 1)).unwrap());
    assert_eq!(usize::MAX, TryToByAdd::try_into_usize(&((usize::MAX as u128) + 1)).unwrap());
}

#[test]
fn try_into_ok_u128() {
    assert_eq!(i8::MIN,       TryToByAdd::try_into_i8(&(u8::MIN    as u128)).unwrap());
    assert_eq!(i8::MAX,       TryToByAdd::try_into_i8(&(u8::MAX    as u128)).unwrap());
    assert_eq!(u8::MIN,       TryToByAdd::try_into_u8(&(u8::MIN    as u128)).unwrap());
    assert_eq!(u8::MAX,       TryToByAdd::try_into_u8(&(u8::MAX    as u128)).unwrap());
    assert_eq!(i16::MIN,     TryToByAdd::try_into_i16(&(u16::MIN   as u128)).unwrap());
    assert_eq!(i16::MAX,     TryToByAdd::try_into_i16(&(u16::MAX   as u128)).unwrap());
    assert_eq!(u16::MIN,     TryToByAdd::try_into_u16(&(u16::MIN   as u128)).unwrap());
    assert_eq!(u16::MAX,     TryToByAdd::try_into_u16(&(u16::MAX   as u128)).unwrap());
    assert_eq!(i32::MIN,     TryToByAdd::try_into_i32(&(u32::MIN   as u128)).unwrap());
    assert_eq!(i32::MAX,     TryToByAdd::try_into_i32(&(u32::MAX   as u128)).unwrap());
    assert_eq!(u32::MIN,     TryToByAdd::try_into_u32(&(u32::MIN   as u128)).unwrap());
    assert_eq!(u32::MAX,     TryToByAdd::try_into_u32(&(u32::MAX   as u128)).unwrap());
    assert_eq!(i64::MIN,     TryToByAdd::try_into_i64(&(u64::MIN   as u128)).unwrap());
    assert_eq!(i64::MAX,     TryToByAdd::try_into_i64(&(u64::MAX   as u128)).unwrap());
    assert_eq!(u64::MIN,     TryToByAdd::try_into_u64(&(u64::MIN   as u128)).unwrap());
    assert_eq!(u64::MAX,     TryToByAdd::try_into_u64(&(u64::MAX   as u128)).unwrap());
    assert_eq!(isize::MIN, TryToByAdd::try_into_isize(&(usize::MIN as u128)).unwrap());
    assert_eq!(isize::MAX, TryToByAdd::try_into_isize(&(usize::MAX as u128)).unwrap());
    assert_eq!(usize::MIN, TryToByAdd::try_into_usize(&(usize::MIN as u128)).unwrap());
    assert_eq!(usize::MAX, TryToByAdd::try_into_usize(&(usize::MAX as u128)).unwrap());
    assert_eq!(i128::MIN,                  TryToByAdd::try_into_i128(&u128::MIN).unwrap());
    assert_eq!(i128::MAX,                  TryToByAdd::try_into_i128(&u128::MAX).unwrap());
    assert_eq!(u128::MIN,                  TryToByAdd::try_into_u128(&u128::MIN).unwrap());
    assert_eq!(u128::MAX,                  TryToByAdd::try_into_u128(&u128::MAX).unwrap());
}
