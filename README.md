# num_convert

Rust library for converting integers.

## Description

Type converting library.

#### This library provide a way to convert from one type to another type.

- Convert from signed integers to unsigned in the full range of values.
- Supports generics types.
- Not all value types support conversions.

## Usage

#### Add this to your Cargo.toml
```
[dependencies]
num_convert = { git = "https://github.com/pic16f877ccs/num_convert", version = "0.1.0" }
```
#### Or using cargo
```
cargo add num_convert --git "https://github.com/pic16f877ccs/num_convert"

```
#### Examples
```
 use num_convert::ToByAdd;

 assert_eq!(i8::MIN, ToByAdd::to_i8(&i8::MIN));
 assert_eq!(i8::MAX, ToByAdd::to_i8(&i8::MAX));
 assert_eq!(u8::MIN, ToByAdd::to_u8(&i8::MIN));
 assert_eq!(u8::MAX, ToByAdd::to_u8(&i8::MAX));
```

## License
GNU General Public License v3.0

