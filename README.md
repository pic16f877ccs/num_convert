# num_convert

Rust library for converting integers.

## Description
This library provide a way to convert from one type to another type.
Supports generics types.

#### Generic traits for conversions between integer types.

- The FromByAdd trait for converting from negative integers to positive or vice versa.
- The IntoByAdd trait for converting into negative integers to positive or vice versa.
- The TryFromByAdd trait for converting from negative integers to positive or vice versa, that can fail.
- The TryIntoByAdd trait for converting into negative integers to positive or vice versa, that can fail.
- The FromAs generic trait for conversion from integers with possible overflow.
- The IntoAs generic trait for conversion into integers with possible overflow.
- The TryFromDigits trait for converting from digits as a number, with possible value types.

#### Other traits for integers.

- A trait IntegerLen to determine the number of digits of integers.
- The Sbits trait for define the size of integer value in bits.
- The Tbits trait for define the size of integer type in bits.
- The ToZero trait for implementing the null value of types.
- The ToMin trait for implement lower bounds on types.
- The ToMax trait for implement upper bounds on types.

### Usage

#### Add this to your Cargo.toml
```rust,ignore
[dependencies]
num_convert = { git = "https://github.com/pic16f877ccs/num_convert", version = "0.5.0" }
```
#### Or using cargo
```rust,ignore
cargo add num_convert --git "https://github.com/pic16f877ccs/num_convert"

```
#### Examples

Convert from negative into positive and positive into negative.
```
use num_convert::FromByAdd;

assert_eq!(<u8>::from_by_add(-28i8), 100u8);
assert_eq!(<i8>::from_by_add(10u8), -118i8);
```

Convert into between integer types.
```
use num_convert::IntoByAdd;

fn integer_to_integer<T1: IntoByAdd<U1>, T2: IntoByAdd<U2>, U1, U2>(min: T1, max: T2) -> (U1, U2) {
    (min.into_by_add(), max.into_by_add())
}
assert_eq!(integer_to_integer(i16::MIN, u16::MAX), (u16::MIN, i16::MAX));
assert_eq!(integer_to_integer(u16::MIN, u16::MAX), (i16::MIN, i16::MAX));
```

Convert from negative into positive without error and with error.
```
use num_convert::TryFromByAdd;

assert_eq!(<u8>::try_from_by_add(-128i16), Some(0u8));
assert_eq!(<u8>::try_from_by_add(-129i16), None);
```

Convert between 128 bit types lossless.
```
use num_convert::TryIntoByAdd;

assert_eq!(<i128 as TryIntoByAdd<u128>>::try_into_by_add(i128::MIN), Some(u128::MIN));
assert_eq!(<u128 as TryIntoByAdd<i128>>::try_into_by_add(u128::MIN), Some(i128::MIN));
```

Conversions type U16 in U8 without overflow and with overflow.
```
use num_convert::{IntoAs, FromAs};

assert_eq!(<u16 as IntoAs<u8>>::into_as(255u16), 255u8);
assert_eq!(<u16 as IntoAs<u8>>::into_as(258u16), 2u8);
assert_eq!(<u8>::from_as(261u16), 5u8);
```

Converting from digits as a number.
```
use num_convert::TryFromDigits;

let arr: [u32; 6] = [25_635_071, 25_634_091, 25_633_334, 25_636_309, 25_637_101, 25_636_243];
let val: Vec<u8> = arr.iter().map(|var| u8::from_digits(*var).unwrap_or(255u8) ).collect::<_>();
assert_eq!(val, [71, 91, 255, 255, 101, 243]);
```

The size of integer values in bits.
```
use num_convert::Sbits;

assert_eq!((-128i8).sbits(), 8u32);
assert_eq!(u64::MAX.sbits(), 64u32);
```

The size of integer type in bits.
```
use num_convert::Tbits;

assert_eq!(i8::tbits(), 8u32);
assert_eq!(u64::tbits(), 64u32);
```

Determining the number of digits of integers.
```
use num_convert::IntegerLen;

assert_eq!(0i8.len(), 1usize);
assert_eq!(i8::MAX.len(), 3usize);
assert_eq!(i128::MAX.len(), 39usize);
assert_eq!(u128::MAX.len(), 39usize);
```
## License
GNU General Public License v3.0

