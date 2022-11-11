# num_convert

Rust library for converting integers.

## Description

Type converting library.

### This library provide a way to convert from one type to another type.
### Trait ToByAdd and FromByAdd
- Convert to or from signed integers to unsigned in the full range of values.
- Convert to or from unsigned integers to signed in the full range of values.
- Supports generics types.
- Not all value types support conversions.

### Trait TryToByAdd or TryFromByAdd 
- Convert into or from signed integers to unsigned in the full range of values or possible values.
- Convert into or from unsigned integers to signed in the full range of values or possible values.
- Convert into or from signed integers to signed in the full range of values or possible values.
- Convert into or from unsigned integers to unsigned in the full range of values or possible values.
- Supports generics types.

## Usage

#### Add this to your Cargo.toml
```
[dependencies]
num_convert = { git = "https://github.com/pic16f877ccs/num_convert", version = "0.3.2" }
```
#### Or using cargo
```
cargo add num_convert --git "https://github.com/pic16f877ccs/num_convert"

```
#### Examples
```
 use num_convert::ToByAdd;

 fn convert_into_u8<T: ToByAdd>(min: T, max: T) -> (u8, u8) {
     (min.into_u8(), max.into_u8())
 }
 assert_eq!((u8::MIN, u8::MAX), convert_into_u8(i8::MIN, i8::MAX));

 assert_eq!(i128::MIN, ToByAdd::into_i128(u128::MIN));
 assert_eq!(u8::MAX as u64, ToByAdd::into_u64(i8::MAX));
```

```
 use num_convert::TryToByAdd;

 fn convert_into_u8<T>(min: T, max: T) -> (u8, u8)
 where
     T: TryToByAdd,
 {

     (min.try_into_u8().unwrap(), max.try_into_u8().unwrap())
 }
 assert_eq!((u8::MIN, u8::MAX), convert_into_u8(i8::MIN, i8::MAX));
 assert_eq!((u8::MIN, u8::MAX), convert_into_u8(i8::MIN as i64, i8::MAX as i64));

 assert_eq!(u8::MAX, TryToByAdd::try_into_u8(i8::MAX).unwrap());
 assert_eq!(i8::MIN, TryToByAdd::try_into_i8(u8::MIN).unwrap());
 assert_eq!(u8::MIN, TryToByAdd::try_into_u8(u8::MIN).unwrap());
```  

## License
GNU General Public License v3.0

