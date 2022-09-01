 use num_convert::FromByAdd;
 use paste::paste;

#[test]
fn from_out_i8() {
    assert_eq!(i8::MIN, <i8 as    FromByAdd>::from_i8(i8::MIN));
    assert_eq!(i8::MAX, <i8 as    FromByAdd>::from_i8(i8::MAX));
    assert_eq!(i8::MIN, <i8 as    FromByAdd>::from_u8(u8::MIN));
    assert_eq!(i8::MAX, <i8 as    FromByAdd>::from_u8(u8::MAX));
}

#[test]
fn from_out_u8() {
    assert_eq!(u8::MIN, <u8 as    FromByAdd>::from_i8(i8::MIN));
    assert_eq!(u8::MAX, <u8 as    FromByAdd>::from_i8(i8::MAX));
    assert_eq!(u8::MIN, <u8 as    FromByAdd>::from_u8(u8::MIN));
    assert_eq!(u8::MAX, <u8 as    FromByAdd>::from_u8(u8::MAX));
}


#[test]
fn from_out_i16() {
    assert_eq!(i8::MIN  as i16, <i16 as    FromByAdd>::from_i8(i8::MIN));
    assert_eq!(i8::MAX  as i16, <i16 as    FromByAdd>::from_i8(i8::MAX));
    assert_eq!(i8::MIN  as i16, <i16 as    FromByAdd>::from_u8(u8::MIN));
    assert_eq!(i8::MAX  as i16, <i16 as    FromByAdd>::from_u8(u8::MAX));
    assert_eq!(i16::MIN as i16, <i16 as   FromByAdd>::from_i16(i16::MIN));
    assert_eq!(i16::MAX as i16, <i16 as   FromByAdd>::from_i16(i16::MAX));
    assert_eq!(i16::MIN as i16, <i16 as   FromByAdd>::from_u16(u16::MIN));
    assert_eq!(i16::MAX as i16, <i16 as   FromByAdd>::from_u16(u16::MAX));
}

#[test]
fn from_out_u16() {
    assert_eq!(u8::MIN as u16, <u16 as    FromByAdd>::from_i8(i8::MIN));
    assert_eq!(u8::MAX as u16, <u16 as    FromByAdd>::from_i8(i8::MAX));
    assert_eq!(u8::MIN as u16, <u16 as    FromByAdd>::from_u8(u8::MIN));
    assert_eq!(u8::MAX as u16, <u16 as    FromByAdd>::from_u8(u8::MAX));
    assert_eq!(u16::MIN,       <u16 as   FromByAdd>::from_i16(i16::MIN));
    assert_eq!(u16::MAX,       <u16 as   FromByAdd>::from_i16(i16::MAX));
    assert_eq!(u16::MIN,       <u16 as   FromByAdd>::from_u16(u16::MIN));
    assert_eq!(u16::MAX,       <u16 as   FromByAdd>::from_u16(u16::MAX));
}

#[test]
fn from_out_i32() {
    assert_eq!(i8::MIN  as i32, <i32 as    FromByAdd>::from_i8(i8::MIN));
    assert_eq!(i8::MAX  as i32, <i32 as    FromByAdd>::from_i8(i8::MAX));
    assert_eq!(i8::MIN  as i32, <i32 as    FromByAdd>::from_u8(u8::MIN));
    assert_eq!(i8::MAX  as i32, <i32 as    FromByAdd>::from_u8(u8::MAX));
    assert_eq!(i16::MIN as i32, <i32 as   FromByAdd>::from_i16(i16::MIN));
    assert_eq!(i16::MAX as i32, <i32 as   FromByAdd>::from_i16(i16::MAX));
    assert_eq!(i16::MIN as i32, <i32 as   FromByAdd>::from_u16(u16::MIN));
    assert_eq!(i16::MAX as i32, <i32 as   FromByAdd>::from_u16(u16::MAX));
    assert_eq!(i32::MIN,        <i32 as   FromByAdd>::from_i32(i32::MIN));
    assert_eq!(i32::MAX,        <i32 as   FromByAdd>::from_i32(i32::MAX));
    assert_eq!(i32::MIN,        <i32 as   FromByAdd>::from_u32(u32::MIN));
    assert_eq!(i32::MAX,        <i32 as   FromByAdd>::from_u32(u32::MAX));
}

// For adding 32 bit arch.
#[cfg(target_pointer_width = "32")]
#[test]
fn from_out_i32_32bit() {
    assert_eq!(i32::MIN,        <i32 as   FromByAdd>::from_isize(isize::MIN));
    assert_eq!(i32::MAX,        <i32 as   FromByAdd>::from_isize(isize::MAX));
    assert_eq!(i32::MIN,        <i32 as   FromByAdd>::from_usize(usize::MIN));
    assert_eq!(i32::MAX,        <i32 as   FromByAdd>::from_usize(usize::MAX));
}

#[test]
fn from_out_u32() {
    assert_eq!(u8::MIN  as u32, <u32 as    FromByAdd>::from_i8(i8::MIN));
    assert_eq!(u8::MAX  as u32, <u32 as    FromByAdd>::from_i8(i8::MAX));
    assert_eq!(u8::MIN  as u32, <u32 as    FromByAdd>::from_u8(u8::MIN));
    assert_eq!(u8::MAX  as u32, <u32 as    FromByAdd>::from_u8(u8::MAX));
    assert_eq!(u16::MIN as u32, <u32 as   FromByAdd>::from_i16(i16::MIN));
    assert_eq!(u16::MAX as u32, <u32 as   FromByAdd>::from_i16(i16::MAX));
    assert_eq!(u16::MIN as u32, <u32 as   FromByAdd>::from_u16(u16::MIN));
    assert_eq!(u16::MAX as u32, <u32 as   FromByAdd>::from_u16(u16::MAX));
    assert_eq!(u32::MIN,        <u32 as   FromByAdd>::from_i32(i32::MIN));
    assert_eq!(u32::MAX,        <u32 as   FromByAdd>::from_i32(i32::MAX));
    assert_eq!(u32::MIN,        <u32 as   FromByAdd>::from_u32(u32::MIN));
    assert_eq!(u32::MAX,        <u32 as   FromByAdd>::from_u32(u32::MAX));
}

// For adding 32 bit arch.
#[cfg(target_pointer_width = "32")]
#[test]
fn from_out_u32_32bit() {
    assert_eq!(u32::MIN,        <u32 as   FromByAdd>::from_isize(isize::MIN));
    assert_eq!(u32::MAX,        <u32 as   FromByAdd>::from_isize(isize::MAX));
    assert_eq!(u32::MIN,        <u32 as   FromByAdd>::from_usize(usize::MIN));
    assert_eq!(u32::MAX,        <u32 as   FromByAdd>::from_usize(usize::MAX));
}


#[test]
fn from_out_i64() {
    assert_eq!(i8::MIN  as i64,   <i64 as    FromByAdd>::from_i8(i8::MIN));
    assert_eq!(i8::MAX  as i64,   <i64 as    FromByAdd>::from_i8(i8::MAX));
    assert_eq!(i8::MIN  as i64,   <i64 as    FromByAdd>::from_u8(u8::MIN));
    assert_eq!(i8::MAX  as i64,   <i64 as    FromByAdd>::from_u8(u8::MAX));
    assert_eq!(i16::MIN as i64,   <i64 as   FromByAdd>::from_i16(i16::MIN));
    assert_eq!(i16::MAX as i64,   <i64 as   FromByAdd>::from_i16(i16::MAX));
    assert_eq!(i16::MIN as i64,   <i64 as   FromByAdd>::from_u16(u16::MIN));
    assert_eq!(i16::MAX as i64,   <i64 as   FromByAdd>::from_u16(u16::MAX));
    assert_eq!(i32::MIN as i64,   <i64 as   FromByAdd>::from_i32(i32::MIN));
    assert_eq!(i32::MAX as i64,   <i64 as   FromByAdd>::from_i32(i32::MAX));
    assert_eq!(i32::MIN as i64,   <i64 as   FromByAdd>::from_u32(u32::MIN));
    assert_eq!(i32::MAX as i64,   <i64 as   FromByAdd>::from_u32(u32::MAX));
    assert_eq!(i64::MIN,          <i64 as   FromByAdd>::from_i64(i64::MIN));
    assert_eq!(i64::MAX,          <i64 as   FromByAdd>::from_i64(i64::MAX));
    assert_eq!(i64::MIN,          <i64 as   FromByAdd>::from_u64(u64::MIN));
    assert_eq!(i64::MAX,          <i64 as   FromByAdd>::from_u64(u64::MAX));
    assert_eq!(isize::MIN as i64, <i64 as FromByAdd>::from_isize(isize::MIN));
    assert_eq!(isize::MAX as i64, <i64 as FromByAdd>::from_isize(isize::MAX));
    assert_eq!(isize::MIN as i64, <i64 as FromByAdd>::from_usize(usize::MIN));
    assert_eq!(isize::MAX as i64, <i64 as FromByAdd>::from_usize(usize::MAX));
}

#[test]
fn from_out_u64() {
    assert_eq!(u8::MIN  as u64,   <u64 as    FromByAdd>::from_i8(i8::MIN));
    assert_eq!(u8::MAX  as u64,   <u64 as    FromByAdd>::from_i8(i8::MAX));
    assert_eq!(u8::MIN  as u64,   <u64 as    FromByAdd>::from_u8(u8::MIN));
    assert_eq!(u8::MAX  as u64,   <u64 as    FromByAdd>::from_u8(u8::MAX));
    assert_eq!(u16::MIN as u64,   <u64 as   FromByAdd>::from_i16(i16::MIN));
    assert_eq!(u16::MAX as u64,   <u64 as   FromByAdd>::from_i16(i16::MAX));
    assert_eq!(u16::MIN as u64,   <u64 as   FromByAdd>::from_u16(u16::MIN));
    assert_eq!(u16::MAX as u64,   <u64 as   FromByAdd>::from_u16(u16::MAX));
    assert_eq!(u32::MIN as u64,   <u64 as   FromByAdd>::from_i32(i32::MIN));
    assert_eq!(u32::MAX as u64,   <u64 as   FromByAdd>::from_i32(i32::MAX));
    assert_eq!(u32::MIN as u64,   <u64 as   FromByAdd>::from_u32(u32::MIN));
    assert_eq!(u32::MAX as u64,   <u64 as   FromByAdd>::from_u32(u32::MAX));
    assert_eq!(u64::MIN,          <u64 as   FromByAdd>::from_i64(i64::MIN));
    assert_eq!(u64::MAX,          <u64 as   FromByAdd>::from_i64(i64::MAX));
    assert_eq!(u64::MIN,          <u64 as   FromByAdd>::from_u64(u64::MIN));
    assert_eq!(u64::MAX,          <u64 as   FromByAdd>::from_u64(u64::MAX));
    assert_eq!(usize::MIN as u64, <u64 as FromByAdd>::from_isize(isize::MIN));
    assert_eq!(usize::MAX as u64, <u64 as FromByAdd>::from_isize(isize::MAX));
    assert_eq!(usize::MIN as u64, <u64 as FromByAdd>::from_usize(usize::MIN));
    assert_eq!(usize::MAX as u64, <u64 as FromByAdd>::from_usize(usize::MAX));
}

#[test]
fn from_out_isize() {
    assert_eq!(i8::MIN  as isize, <isize as    FromByAdd>::from_i8(i8::MIN));
    assert_eq!(i8::MAX  as isize, <isize as    FromByAdd>::from_i8(i8::MAX));
    assert_eq!(i8::MIN  as isize, <isize as    FromByAdd>::from_u8(u8::MIN));
    assert_eq!(i8::MAX  as isize, <isize as    FromByAdd>::from_u8(u8::MAX));
    assert_eq!(i16::MIN as isize, <isize as   FromByAdd>::from_i16(i16::MIN));
    assert_eq!(i16::MAX as isize, <isize as   FromByAdd>::from_i16(i16::MAX));
    assert_eq!(i16::MIN as isize, <isize as   FromByAdd>::from_u16(u16::MIN));
    assert_eq!(i16::MAX as isize, <isize as   FromByAdd>::from_u16(u16::MAX));
    assert_eq!(i32::MIN as isize, <isize as   FromByAdd>::from_i32(i32::MIN));
    assert_eq!(i32::MAX as isize, <isize as   FromByAdd>::from_i32(i32::MAX));
    assert_eq!(i32::MIN as isize, <isize as   FromByAdd>::from_u32(u32::MIN));
    assert_eq!(i32::MAX as isize, <isize as   FromByAdd>::from_u32(u32::MAX));
    assert_eq!(isize::MIN,        <isize as FromByAdd>::from_isize(isize::MIN));
    assert_eq!(isize::MAX,        <isize as FromByAdd>::from_isize(isize::MAX));
    assert_eq!(isize::MIN,        <isize as FromByAdd>::from_usize(usize::MIN));
    assert_eq!(isize::MAX,        <isize as FromByAdd>::from_usize(usize::MAX));
}

// For adding 64 bit arch.
#[cfg(target_pointer_width = "64")]
#[test]
fn from_out_isize_64bit() {
    assert_eq!(isize::MIN,        <isize as   FromByAdd>::from_i64(i64::MIN));
    assert_eq!(isize::MAX,        <isize as   FromByAdd>::from_i64(i64::MAX));
    assert_eq!(isize::MIN,        <isize as   FromByAdd>::from_u64(u64::MIN));
    assert_eq!(isize::MAX,        <isize as   FromByAdd>::from_u64(u64::MAX));
}

#[test]
fn from_out_usize() {
    assert_eq!(u8::MIN  as usize, <usize as    FromByAdd>::from_i8(i8::MIN));
    assert_eq!(u8::MAX  as usize, <usize as    FromByAdd>::from_i8(i8::MAX));
    assert_eq!(u8::MIN  as usize, <usize as    FromByAdd>::from_u8(u8::MIN));
    assert_eq!(u8::MAX  as usize, <usize as    FromByAdd>::from_u8(u8::MAX));
    assert_eq!(u16::MIN as usize, <usize as   FromByAdd>::from_i16(i16::MIN));
    assert_eq!(u16::MAX as usize, <usize as   FromByAdd>::from_i16(i16::MAX));
    assert_eq!(u16::MIN as usize, <usize as   FromByAdd>::from_u16(u16::MIN));
    assert_eq!(u16::MAX as usize, <usize as   FromByAdd>::from_u16(u16::MAX));
    assert_eq!(u32::MIN as usize, <usize as   FromByAdd>::from_i32(i32::MIN));
    assert_eq!(u32::MAX as usize, <usize as   FromByAdd>::from_i32(i32::MAX));
    assert_eq!(u32::MIN as usize, <usize as   FromByAdd>::from_u32(u32::MIN));
    assert_eq!(u32::MAX as usize, <usize as   FromByAdd>::from_u32(u32::MAX));
    assert_eq!(usize::MIN,        <usize as FromByAdd>::from_isize(isize::MIN));
    assert_eq!(usize::MAX,        <usize as FromByAdd>::from_isize(isize::MAX));
    assert_eq!(usize::MIN,        <usize as FromByAdd>::from_usize(usize::MIN));
    assert_eq!(usize::MAX,        <usize as FromByAdd>::from_usize(usize::MAX));
}

// For adding 64 bit arch.
#[cfg(target_pointer_width = "64")]
#[test]
fn from_out_usize_64bit() {
    assert_eq!(usize::MIN,        <usize as   FromByAdd>::from_i64(i64::MIN));
    assert_eq!(usize::MAX,        <usize as   FromByAdd>::from_i64(i64::MAX));
    assert_eq!(usize::MIN,        <usize as   FromByAdd>::from_u64(u64::MIN));
    assert_eq!(usize::MAX,        <usize as   FromByAdd>::from_u64(u64::MAX));
}

#[test]
fn from_out_i128() {
    assert_eq!(i8::MIN    as i128, <i128 as    FromByAdd>::from_i8(i8::MIN));
    assert_eq!(i8::MAX    as i128, <i128 as    FromByAdd>::from_i8(i8::MAX));
    assert_eq!(i8::MIN    as i128, <i128 as    FromByAdd>::from_u8(u8::MIN));
    assert_eq!(i8::MAX    as i128, <i128 as    FromByAdd>::from_u8(u8::MAX));
    assert_eq!(i16::MIN   as i128, <i128 as   FromByAdd>::from_i16(i16::MIN));
    assert_eq!(i16::MAX   as i128, <i128 as   FromByAdd>::from_i16(i16::MAX));
    assert_eq!(i16::MIN   as i128, <i128 as   FromByAdd>::from_u16(u16::MIN));
    assert_eq!(i16::MAX   as i128, <i128 as   FromByAdd>::from_u16(u16::MAX));
    assert_eq!(i32::MIN   as i128, <i128 as   FromByAdd>::from_i32(i32::MIN));
    assert_eq!(i32::MAX   as i128, <i128 as   FromByAdd>::from_i32(i32::MAX));
    assert_eq!(i32::MIN   as i128, <i128 as   FromByAdd>::from_u32(u32::MIN));
    assert_eq!(i32::MAX   as i128, <i128 as   FromByAdd>::from_u32(u32::MAX));
    assert_eq!(i64::MIN   as i128, <i128 as   FromByAdd>::from_i64(i64::MIN));
    assert_eq!(i64::MAX   as i128, <i128 as   FromByAdd>::from_i64(i64::MAX));
    assert_eq!(i64::MIN   as i128, <i128 as   FromByAdd>::from_u64(u64::MIN));
    assert_eq!(i64::MAX   as i128, <i128 as   FromByAdd>::from_u64(u64::MAX));
    assert_eq!(isize::MIN as i128, <i128 as FromByAdd>::from_isize(isize::MIN));
    assert_eq!(isize::MAX as i128, <i128 as FromByAdd>::from_isize(isize::MAX));
    assert_eq!(isize::MIN as i128, <i128 as FromByAdd>::from_usize(usize::MIN));
    assert_eq!(isize::MAX as i128, <i128 as FromByAdd>::from_usize(usize::MAX));
    assert_eq!(i128::MIN,          <i128 as  FromByAdd>::from_i128(i128::MIN));
    assert_eq!(i128::MAX,          <i128 as  FromByAdd>::from_i128(i128::MAX));
    assert_eq!(i128::MIN,          <i128 as  FromByAdd>::from_u128(u128::MIN));
    assert_eq!(i128::MAX,          <i128 as  FromByAdd>::from_u128(u128::MAX));
}

#[test]
fn from_out_u128() {
    assert_eq!(u8::MIN    as u128, <u128 as    FromByAdd>::from_i8(i8::MIN));
    assert_eq!(u8::MAX    as u128, <u128 as    FromByAdd>::from_i8(i8::MAX));
    assert_eq!(u8::MIN    as u128, <u128 as    FromByAdd>::from_u8(u8::MIN));
    assert_eq!(u8::MAX    as u128, <u128 as    FromByAdd>::from_u8(u8::MAX));
    assert_eq!(u16::MIN   as u128, <u128 as   FromByAdd>::from_i16(i16::MIN));
    assert_eq!(u16::MAX   as u128, <u128 as   FromByAdd>::from_i16(i16::MAX));
    assert_eq!(u16::MIN   as u128, <u128 as   FromByAdd>::from_u16(u16::MIN));
    assert_eq!(u16::MAX   as u128, <u128 as   FromByAdd>::from_u16(u16::MAX));
    assert_eq!(u32::MIN   as u128, <u128 as   FromByAdd>::from_i32(i32::MIN));
    assert_eq!(u32::MAX   as u128, <u128 as   FromByAdd>::from_i32(i32::MAX));
    assert_eq!(u32::MIN   as u128, <u128 as   FromByAdd>::from_u32(u32::MIN));
    assert_eq!(u32::MAX   as u128, <u128 as   FromByAdd>::from_u32(u32::MAX));
    assert_eq!(u64::MIN   as u128, <u128 as   FromByAdd>::from_i64(i64::MIN));
    assert_eq!(u64::MAX   as u128, <u128 as   FromByAdd>::from_i64(i64::MAX));
    assert_eq!(u64::MIN   as u128, <u128 as   FromByAdd>::from_u64(u64::MIN));
    assert_eq!(u64::MAX   as u128, <u128 as   FromByAdd>::from_u64(u64::MAX));
    assert_eq!(usize::MIN as u128, <u128 as FromByAdd>::from_isize(isize::MIN));
    assert_eq!(usize::MAX as u128, <u128 as FromByAdd>::from_isize(isize::MAX));
    assert_eq!(usize::MIN as u128, <u128 as FromByAdd>::from_usize(usize::MIN));
    assert_eq!(usize::MAX as u128, <u128 as FromByAdd>::from_usize(usize::MAX));
    assert_eq!(u128::MIN,          <u128 as  FromByAdd>::from_i128(i128::MIN));
    assert_eq!(u128::MAX,          <u128 as  FromByAdd>::from_i128(i128::MAX));
    assert_eq!(u128::MIN,          <u128 as  FromByAdd>::from_u128(u128::MIN));
    assert_eq!(u128::MAX,          <u128 as  FromByAdd>::from_u128(u128::MAX));
}
