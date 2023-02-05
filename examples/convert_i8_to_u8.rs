use num_convert::IntoByAdd;

fn convert_i8_to_u8<T: IntoByAdd<u8>>(value: T) -> u8 {
    value.into_by_add()
}

fn main() {
    println!("{}", convert_i8_to_u8(-128i8));
    println!("{}", convert_i8_to_u8(127i8));
}
