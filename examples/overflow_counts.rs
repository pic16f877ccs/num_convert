use std::ops::Shr;
use num_convert::{ FromAs, IntoAs, Tbits };
use num_convert::min_zero_max::Tzero;

fn overflow_cnt<T, O>(num: T) -> (O, T)
    where
        T: FromAs<O> + Copy + Shr<u32, Output = T> + Tzero<T>,
        O: FromAs<T> + TryFrom<T> + Tbits,
{
    match O::try_from(num) {
        Ok(ok) => (ok, T::tzero()),
        Err(_) => (O::from_as(num), num >> O::tbits())
    }
}

fn main() {
    assert_eq!(<u8 as FromAs<u16>>::from_as(258u16), 2u8);
    assert_eq!(<u16 as IntoAs<u8>>::into_as(258u16), 2u8);
    println!("Overflow value and counts => {:?}", overflow_cnt::<_, u16>(u32::MAX));
}
