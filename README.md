# num_convert

Rust library for converting integers.

## Description

Type converting library.

### This library provide a way to convert from one type to another type.
### Trait ToByAdd
- Convert from signed integers to unsigned in the full range of values.
- Supports generics types.
- Not all value types support conversions.

### Trait TryByAdd
- Convert from signed integers to unsigned in the full range of values or possible values.
- Convert from unsigned integers to signed in the full range of values or possible values.
- Convert from signed integers to signed in the full range of values or possible values.
- Convert from unsigned integers to unsigned in the full range of values or possible values.
- Supports generics types.

## Usage

#### Add this to your Cargo.toml
```
[dependencies]
num_convert = { git = "https://github.com/pic16f877ccs/num_convert", version = "0.2.0" }
```
#### Or using cargo
```
cargo add num_convert --git "https://github.com/pic16f877ccs/num_convert"

```
#### Examples
```
 use num_convert::ToByAdd;
 
 fn convert_i8_to_u8<T: ToByAdd>(min: T, max: T) -> (u8, u8) {
     (min.to_u8(), max.to_u8())
 }
 assert_eq!((u8::MIN, u8::MAX), convert_i8_to_u8(i8::MIN, i8::MAX));
```

```
 assert_eq!(i8::MIN, ToByAdd::to_i8(&i8::MIN));
 assert_eq!(i8::MAX, ToByAdd::to_i8(&i8::MAX));
 assert_eq!(u8::MIN, ToByAdd::to_u8(&i8::MIN));
 assert_eq!(u8::MAX, ToByAdd::to_u8(&i8::MAX));
```

```
use num_convert::TryToByAdd;
use std::fmt::Debug;

fn convert_i8_to_u8<T>(min: T, max: T) -> (u8, u8)
where
    T: TryToByAdd,
    <T as TryToByAdd>::Error: Debug, 
{
  
    (min.try_into_u8().unwrap(), max.try_into_u8().unwrap())
}   
assert_eq!((u8::MIN, u8::MAX), convert_i8_to_u8(i8::MIN, i8::MAX));
```  

```
assert_eq!(i8::MIN, TryToByAdd::try_into_i8(&i8::MIN).unwrap());
assert_eq!(i8::MAX, TryToByAdd::try_into_i8(&i8::MAX).unwrap());
assert_eq!(u8::MIN, TryToByAdd::try_into_u8(&i8::MIN).unwrap());
assert_eq!(u8::MAX, TryToByAdd::try_into_u8(&i8::MAX).unwrap());
assert_eq!(i8::MIN, TryToByAdd::try_into_i8(&u8::MIN).unwrap());
assert_eq!(i8::MAX, TryToByAdd::try_into_i8(&u8::MAX).unwrap());
assert_eq!(u8::MIN, TryToByAdd::try_into_u8(&u8::MIN).unwrap());
assert_eq!(u8::MAX, TryToByAdd::try_into_u8(&u8::MAX).unwrap());
```

## License
GNU General Public License v3.0

