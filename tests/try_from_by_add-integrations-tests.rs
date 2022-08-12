use num_convert::*;

#[test]
fn from_out_i8() {
    assert_eq!(i8::MIN, <i8 as    TryFromByAdd>::try_from_i8(i8::MIN).unwrap());
    assert_eq!(i8::MIN, <i8 as    TryFromByAdd>::try_from_i8(i8::MIN).unwrap());
    assert_eq!(i8::MIN, <i8 as    TryFromByAdd>::try_from_u8(u8::MIN).unwrap());
    assert_eq!(i8::MAX, <i8 as    TryFromByAdd>::try_from_u8(u8::MAX).unwrap());
    assert_eq!(i8::MIN, <i8 as   TryFromByAdd>::try_from_i16(i8::MIN as i16).unwrap());
    assert_eq!(i8::MAX, <i8 as   TryFromByAdd>::try_from_i16(i8::MAX as i16).unwrap());
    assert_eq!(i8::MIN, <i8 as   TryFromByAdd>::try_from_u16(u8::MIN as u16).unwrap());
    assert_eq!(i8::MAX, <i8 as   TryFromByAdd>::try_from_u16(u8::MAX as u16).unwrap());
    assert_eq!(i8::MIN, <i8 as   TryFromByAdd>::try_from_i32(i8::MIN as i32).unwrap());
    assert_eq!(i8::MAX, <i8 as   TryFromByAdd>::try_from_i32(i8::MAX as i32).unwrap());
    assert_eq!(i8::MIN, <i8 as   TryFromByAdd>::try_from_u32(u8::MIN as u32).unwrap());
    assert_eq!(i8::MAX, <i8 as   TryFromByAdd>::try_from_u32(u8::MAX as u32).unwrap());
    assert_eq!(i8::MIN, <i8 as   TryFromByAdd>::try_from_i64(i8::MIN as i64).unwrap());
    assert_eq!(i8::MAX, <i8 as   TryFromByAdd>::try_from_i64(i8::MAX as i64).unwrap());
    assert_eq!(i8::MIN, <i8 as   TryFromByAdd>::try_from_u64(u8::MIN as u64).unwrap());
    assert_eq!(i8::MAX, <i8 as   TryFromByAdd>::try_from_u64(u8::MAX as u64).unwrap());
    assert_eq!(i8::MIN, <i8 as TryFromByAdd>::try_from_isize(i8::MIN as isize).unwrap());
    assert_eq!(i8::MAX, <i8 as TryFromByAdd>::try_from_isize(i8::MAX as isize).unwrap());
    assert_eq!(i8::MIN, <i8 as TryFromByAdd>::try_from_usize(u8::MIN as usize).unwrap());
    assert_eq!(i8::MAX, <i8 as TryFromByAdd>::try_from_usize(u8::MAX as usize).unwrap());
    assert_eq!(i8::MIN, <i8 as  TryFromByAdd>::try_from_i128(i8::MIN as i128).unwrap());
    assert_eq!(i8::MAX, <i8 as  TryFromByAdd>::try_from_i128(i8::MAX as i128).unwrap());
    assert_eq!(i8::MIN, <i8 as  TryFromByAdd>::try_from_u128(u8::MIN as u128).unwrap());
    assert_eq!(i8::MAX, <i8 as  TryFromByAdd>::try_from_u128(u8::MAX as u128).unwrap());
}

//#[test]
//fn from_out_i8() {
//    assert_eq!(u8::MIN, <u8 as    TryFromByAdd>::try_from_i8(i8::MIN).unwrap());
//    assert_eq!(u8::MIN, <u8 as    TryFromByAdd>::try_from_i8(i8::MIN).unwrap());
//    assert_eq!(u8::MIN, <u8 as    TryFromByAdd>::try_from_u8(u8::MIN).unwrap());
//    assert_eq!(u8::MAX, <u8 as    TryFromByAdd>::try_from_u8(u8::MAX).unwrap());
//    assert_eq!(u8::MIN, <u8 as   TryFromByAdd>::try_from_i16(i8::MIN as i16).unwrap());
//    assert_eq!(u8::MAX, <u8 as   TryFromByAdd>::try_from_i16(i8::MAX as i16).unwrap());
//    assert_eq!(u8::MIN, <u8 as   TryFromByAdd>::try_from_u16(u8::MIN as u16).unwrap());
//    assert_eq!(u8::MAX, <u8 as   TryFromByAdd>::try_from_u16(u8::MAX as u16).unwrap());
//    assert_eq!(u8::MIN, <u8 as   TryFromByAdd>::try_from_i32(i8::MIN as i32).unwrap());
//    assert_eq!(u8::MAX, <u8 as   TryFromByAdd>::try_from_i32(i8::MAX as i32).unwrap());
//    assert_eq!(u8::MIN, <u8 as   TryFromByAdd>::try_from_u32(u8::MIN as u32).unwrap());
//    assert_eq!(u8::MAX, <u8 as   TryFromByAdd>::try_from_u32(u8::MAX as u32).unwrap());
//    assert_eq!(u8::MIN, <u8 as   TryFromByAdd>::try_from_i64(i8::MIN as i64).unwrap());
//    assert_eq!(u8::MAX, <u8 as   TryFromByAdd>::try_from_i64(i8::MAX as i64).unwrap());
//    assert_eq!(u8::MIN, <u8 as   TryFromByAdd>::try_from_u64(u8::MIN as u64).unwrap());
//    assert_eq!(u8::MAX, <u8 as   TryFromByAdd>::try_from_u64(u8::MAX as u64).unwrap());
//    assert_eq!(u8::MIN, <u8 as TryFromByAdd>::try_from_isize(i8::MIN as isize).unwrap());
//    assert_eq!(u8::MAX, <u8 as TryFromByAdd>::try_from_isize(i8::MAX as isize).unwrap());
//    assert_eq!(u8::MIN, <u8 as TryFromByAdd>::try_from_usize(u8::MIN as usize).unwrap());
//    assert_eq!(u8::MAX, <u8 as TryFromByAdd>::try_from_usize(u8::MAX as usize).unwrap());
//    assert_eq!(u8::MIN, <u8 as  TryFromByAdd>::try_from_i128(i8::MIN as i128).unwrap());
//    assert_eq!(u8::MAX, <u8 as  TryFromByAdd>::try_from_i128(i8::MAX as i128).unwrap());
//    assert_eq!(u8::MIN, <u8 as  TryFromByAdd>::try_from_u128(u8::MIN as u128).unwrap());
//    assert_eq!(u8::MAX, <u8 as  TryFromByAdd>::try_from_u128(u8::MAX as u128).unwrap());
//}
//
//#[test]
//fn from_out_i16() {
//    assert_eq!(i8::MIN as i16, <i16 as    TryFromByAdd>::from_i8( i8::MIN).unwrap());
//    assert_eq!(i8::MAX as i16, <i16 as    TryFromByAdd>::from_i8( i8::MAX).unwrap());
//    assert_eq!(i8::MIN as i16, <i16 as    TryFromByAdd>::from_u8( u8::MIN).unwrap());
//    assert_eq!(i8::MAX as i16, <i16 as    TryFromByAdd>::from_u8( u8::MAX).unwrap());
//    assert_eq!(i16::MIN,       <i16 as   TryFromByAdd>::from_u16(u16::MIN).unwrap());
//    assert_eq!(i16::MAX,       <i16 as   TryFromByAdd>::from_u16(u16::MAX).unwrap());
//    assert_eq!(i16::MIN,       <i16 as   TryFromByAdd>::from_u32(u16::MIN as u32).unwrap());
//    assert_eq!(i16::MAX,       <i16 as   TryFromByAdd>::from_u32(u16::MAX as u32).unwrap());
//    assert_eq!(i16::MIN,       <i16 as   TryFromByAdd>::from_u64(u16::MIN as u64).unwrap());
//    assert_eq!(i16::MAX,       <i16 as   TryFromByAdd>::from_u64(u16::MAX as u64).unwrap());
//    assert_eq!(i16::MIN,       <i16 as TryFromByAdd>::from_usize(u16::MIN as usize).unwrap());
//    assert_eq!(i16::MAX,       <i16 as TryFromByAdd>::from_usize(u16::MAX as usize).unwrap());
//}
//
//#[test]
//fn from_out_u16() {
//    assert_eq!(u8::MIN as u16, <u16 as    TryFromByAdd>::from_i8( i8::MIN).unwrap());
//    assert_eq!(u8::MAX as u16, <u16 as    TryFromByAdd>::from_i8( i8::MAX).unwrap());
//    assert_eq!(u8::MIN as u16, <u16 as    TryFromByAdd>::from_u8( u8::MIN).unwrap());
//    assert_eq!(u8::MAX as u16, <u16 as    TryFromByAdd>::from_u8( u8::MAX).unwrap());
//    assert_eq!(u16::MIN,       <u16 as   TryFromByAdd>::from_u16(u16::MIN).unwrap());
//    assert_eq!(u16::MAX,       <u16 as   TryFromByAdd>::from_u16(u16::MAX).unwrap());
//    assert_eq!(u16::MIN,       <u16 as   TryFromByAdd>::from_u32(u16::MIN as u32).unwrap());
//    assert_eq!(u16::MAX,       <u16 as   TryFromByAdd>::from_u32(u16::MAX as u32).unwrap());
//    assert_eq!(u16::MIN,       <u16 as   TryFromByAdd>::from_u64(u16::MIN as u64).unwrap());
//    assert_eq!(u16::MAX,       <u16 as   TryFromByAdd>::from_u64(u16::MAX as u64).unwrap());
//    assert_eq!(u16::MIN,       <u16 as TryFromByAdd>::from_usize(u16::MIN as usize).unwrap());
//    assert_eq!(u16::MAX,       <u16 as TryFromByAdd>::from_usize(u16::MAX as usize).unwrap());
//}
//
//#[test]
//fn from_out_i32() {
//    assert_eq!(i8::MIN  as i32, <i32 as    TryFromByAdd>::from_i8( i8::MIN).unwrap());
//    assert_eq!(i8::MAX  as i32, <i32 as    TryFromByAdd>::from_i8( i8::MAX).unwrap());
//    assert_eq!(i8::MIN  as i32, <i32 as    TryFromByAdd>::from_u8( u8::MIN).unwrap());
//    assert_eq!(i8::MAX  as i32, <i32 as    TryFromByAdd>::from_u8( u8::MAX).unwrap());
//    assert_eq!(i16::MIN as i32, <i32 as   TryFromByAdd>::from_u16(u16::MIN).unwrap());
//    assert_eq!(i16::MAX as i32, <i32 as   TryFromByAdd>::from_u16(u16::MAX).unwrap());
//    assert_eq!(i32::MIN,        <i32 as   TryFromByAdd>::from_u32(u32::MIN).unwrap());
//    assert_eq!(i32::MAX,        <i32 as   TryFromByAdd>::from_u32(u32::MAX).unwrap());
//    assert_eq!(i32::MIN,        <i32 as   TryFromByAdd>::from_u64(u32::MIN as u64).unwrap());
//    assert_eq!(i32::MAX,        <i32 as   TryFromByAdd>::from_u64(u32::MAX as u64).unwrap());
//    assert_eq!(i32::MIN,        <i32 as TryFromByAdd>::from_usize(u32::MIN as usize).unwrap());
//    assert_eq!(i32::MAX,        <i32 as TryFromByAdd>::from_usize(u32::MAX as usize).unwrap());
//}
//
//#[test]
//fn from_out_u32() {
//    assert_eq!(u8::MIN  as u32, <u32 as    TryFromByAdd>::from_i8( i8::MIN).unwrap());
//    assert_eq!(u8::MAX  as u32, <u32 as    TryFromByAdd>::from_i8( i8::MAX).unwrap());
//    assert_eq!(u8::MIN  as u32, <u32 as    TryFromByAdd>::from_u8( u8::MIN).unwrap());
//    assert_eq!(u8::MAX  as u32, <u32 as    TryFromByAdd>::from_u8( u8::MAX).unwrap());
//    assert_eq!(u16::MIN as u32, <u32 as   TryFromByAdd>::from_u16(u16::MIN).unwrap());
//    assert_eq!(u16::MAX as u32, <u32 as   TryFromByAdd>::from_u16(u16::MAX).unwrap());
//    assert_eq!(u32::MIN,        <u32 as   TryFromByAdd>::from_u32(u32::MIN).unwrap());
//    assert_eq!(u32::MAX,        <u32 as   TryFromByAdd>::from_u32(u32::MAX).unwrap());
//    assert_eq!(u32::MIN,        <u32 as   TryFromByAdd>::from_u64(u32::MIN as u64).unwrap());
//    assert_eq!(u32::MAX,        <u32 as   TryFromByAdd>::from_u64(u32::MAX as u64).unwrap());
//    assert_eq!(u32::MIN,        <u32 as TryFromByAdd>::from_usize(u32::MIN as usize).unwrap());
//    assert_eq!(u32::MAX,        <u32 as TryFromByAdd>::from_usize(u32::MAX as usize).unwrap());
//}
//
//#[test]
//fn from_out_i64() {
//    assert_eq!(i8::MIN  as i64, <i64 as    TryFromByAdd>::from_i8( i8::MIN).unwrap());
//    assert_eq!(i8::MAX  as i64, <i64 as    TryFromByAdd>::from_i8( i8::MAX).unwrap());
//    assert_eq!(i8::MIN  as i64, <i64 as    TryFromByAdd>::from_u8( u8::MIN).unwrap());
//    assert_eq!(i8::MAX  as i64, <i64 as    TryFromByAdd>::from_u8( u8::MAX).unwrap());
//    assert_eq!(i16::MIN as i64, <i64 as   TryFromByAdd>::from_u16(u16::MIN).unwrap());
//    assert_eq!(i16::MAX as i64, <i64 as   TryFromByAdd>::from_u16(u16::MAX).unwrap());
//    assert_eq!(i32::MIN as i64, <i64 as   TryFromByAdd>::from_u32(u32::MIN).unwrap());
//    assert_eq!(i32::MAX as i64, <i64 as   TryFromByAdd>::from_u32(u32::MAX).unwrap());
//    assert_eq!(i64::MIN,        <i64 as   TryFromByAdd>::from_u64(u64::MIN).unwrap());
//    assert_eq!(i64::MAX,        <i64 as   TryFromByAdd>::from_u64(u64::MAX).unwrap());
//    assert_eq!(i64::MIN,        <i64 as TryFromByAdd>::from_usize(usize::MIN).unwrap());
//    assert_eq!(i64::MAX,        <i64 as TryFromByAdd>::from_usize(usize::MAX).unwrap());
//}
//
//#[test]
//fn from_out_u64() {
//    assert_eq!(u8::MIN  as u64, <u64 as    TryFromByAdd>::from_i8( i8::MIN).unwrap());
//    assert_eq!(u8::MAX  as u64, <u64 as    TryFromByAdd>::from_i8( i8::MAX).unwrap());
//    assert_eq!(u8::MIN  as u64, <u64 as    TryFromByAdd>::from_u8( u8::MIN).unwrap());
//    assert_eq!(u8::MAX  as u64, <u64 as    TryFromByAdd>::from_u8( u8::MAX).unwrap());
//    assert_eq!(u16::MIN as u64, <u64 as   TryFromByAdd>::from_u16(u16::MIN).unwrap());
//    assert_eq!(u16::MAX as u64, <u64 as   TryFromByAdd>::from_u16(u16::MAX).unwrap());
//    assert_eq!(u32::MIN as u64, <u64 as   TryFromByAdd>::from_u32(u32::MIN).unwrap());
//    assert_eq!(u32::MAX as u64, <u64 as   TryFromByAdd>::from_u32(u32::MAX).unwrap());
//    assert_eq!(u64::MIN,        <u64 as   TryFromByAdd>::from_u64(u64::MIN).unwrap());
//    assert_eq!(u64::MAX,        <u64 as   TryFromByAdd>::from_u64(u64::MAX).unwrap());
//    assert_eq!(u64::MIN,        <u64 as TryFromByAdd>::from_usize(usize::MIN).unwrap());
//    assert_eq!(u64::MAX,        <u64 as TryFromByAdd>::from_usize(usize::MAX).unwrap());
//}
//
//#[test]
//fn from_out_isize() {
//    assert_eq!(i8::MIN  as isize, <isize as    TryFromByAdd>::from_i8( i8::MIN).unwrap());
//    assert_eq!(i8::MAX  as isize, <isize as    TryFromByAdd>::from_i8( i8::MAX).unwrap());
//    assert_eq!(i8::MIN  as isize, <isize as    TryFromByAdd>::from_u8( u8::MIN).unwrap());
//    assert_eq!(i8::MAX  as isize, <isize as    TryFromByAdd>::from_u8( u8::MAX).unwrap());
//    assert_eq!(i16::MIN as isize, <isize as   TryFromByAdd>::from_u16(u16::MIN).unwrap());
//    assert_eq!(i16::MAX as isize, <isize as   TryFromByAdd>::from_u16(u16::MAX).unwrap());
//    assert_eq!(i32::MIN as isize, <isize as   TryFromByAdd>::from_u32(u32::MIN).unwrap());
//    assert_eq!(i32::MAX as isize, <isize as   TryFromByAdd>::from_u32(u32::MAX).unwrap());
//    assert_eq!(isize::MIN,        <isize as   TryFromByAdd>::from_u64(u64::MIN).unwrap());
//    assert_eq!(isize::MAX,        <isize as   TryFromByAdd>::from_u64(u64::MAX).unwrap());
//    assert_eq!(isize::MIN,        <isize as TryFromByAdd>::from_usize(usize::MIN).unwrap());
//    assert_eq!(isize::MAX,        <isize as TryFromByAdd>::from_usize(usize::MAX).unwrap());
//}
//
//#[test]
//fn from_out_usize() {
//    assert_eq!(u8::MIN  as usize, <usize as    TryFromByAdd>::from_i8( i8::MIN).unwrap());
//    assert_eq!(u8::MAX  as usize, <usize as    TryFromByAdd>::from_i8( i8::MAX).unwrap());
//    assert_eq!(u8::MIN  as usize, <usize as    TryFromByAdd>::from_u8( u8::MIN).unwrap());
//    assert_eq!(u8::MAX  as usize, <usize as    TryFromByAdd>::from_u8( u8::MAX).unwrap());
//    assert_eq!(u16::MIN as usize, <usize as   TryFromByAdd>::from_u16(u16::MIN).unwrap());
//    assert_eq!(u16::MAX as usize, <usize as   TryFromByAdd>::from_u16(u16::MAX).unwrap());
//    assert_eq!(u32::MIN as usize, <usize as   TryFromByAdd>::from_u32(u32::MIN).unwrap());
//    assert_eq!(u32::MAX as usize, <usize as   TryFromByAdd>::from_u32(u32::MAX).unwrap());
//    assert_eq!(usize::MIN,        <usize as   TryFromByAdd>::from_u64(u64::MIN).unwrap());
//    assert_eq!(usize::MAX,        <usize as   TryFromByAdd>::from_u64(u64::MAX).unwrap());
//    assert_eq!(usize::MIN,        <usize as TryFromByAdd>::from_usize(usize::MIN).unwrap());
//    assert_eq!(usize::MAX,        <usize as TryFromByAdd>::from_usize(usize::MAX).unwrap());
//}
