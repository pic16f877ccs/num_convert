use num_convert::{TryFromDigits, TryFromByAdd, TryFromIntStr, TryTupToArr};
use num_convert::{MultiErrors, ConvertErrors, TryTupToArrErr};
use std::str::FromStr;

fn main() -> Result<(), ConvertErrors> {
    let val_str = <i8>::from_str("-156");
    let val_int = <u8>::try_from(522);
    let val_dig = <u8>::from_digits(321);
    let val_int_str = <u8>::try_from_int_str("333");
    let val_arr: Result<[u8; 2], TryTupToArrErr> = ("256", 225i32).try_into_arr();
    let val_by_add = i8::try_from_by_add(255u16).ok_or::<ConvertErrors>(MultiErrors::TryFromByAddErr.into());
    let tup_3 = (121u16, 203i16, 271u64);
    let tup_4 = (99u32, 2023u16, 17081975u64, 0u8);
    let tup_5 = (25u8, 99u32, 2023u16, 17081975i64, "1000999888");

    println!("{}", val_str.unwrap_err());
    println!("{}", val_int.unwrap_err());
    println!("{}", val_dig.unwrap_err());
    println!("{}", val_int_str.unwrap_err());
    println!("{}", val_arr.unwrap_err());
    println!("{}", TryTupToArr::<u8>::try_into_arr(tup_3).unwrap_err());
    println!("{:?}", TryTupToArr::try_into_arr(tup_4)?.iter().sum::<u64>());
    println!( "{:?}", TryTupToArr::try_into_arr(tup_5)?.iter().sum::<u64>());
    Ok(())
}
