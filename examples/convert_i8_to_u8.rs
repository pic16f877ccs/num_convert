use num_convert::ToByAdd;

fn convert_i8_to_u8<T: ToByAdd>(value: T) -> u8 {
    value.into_u8()
}

fn main() {
    println!("{}", convert_i8_to_u8(-128i8));
    println!("{}", convert_i8_to_u8(127i8));
}
