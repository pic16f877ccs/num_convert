# num_convert

Rust library for converting integers.

## Description
This library provides a way to convert from one type to another type.
Supports generic types.

#### Generic Traits for Conversions Between Integer Types

- The FromByAdd trait for converting from negative integers to positive or vice versa.
- The IntoByAdd trait for converting negative integers to positive or vice versa.
- The TryFromByAdd trait for converting from negative integers to positive or vice versa, which can fail.
- The TryIntoByAdd trait for converting negative integers to positive or vice versa, which can fail.
- The FromAs generic trait for conversion from integers with possible overflow.
- The IntoAs generic trait for conversion into integers with possible overflow.
- The TryFromDigits trait for converting from digits as a number, with possible value types.
- The TryFromIntStr trait for converting from str or integer to type integer.
- The FromTuple trait for converting a tuple of different types to an array of integers.
- The TryFromTuple trait for converting a tuple of different types to an array of integers, with possible conversion errors.

#### Other Traits for Integers

- The IntegerLen trait to determine the number of digits of integers.
- The Sbits trait for defining the size of an integer value in bits.
- The Tbits trait for defining the size of an integer type in bits.
- The ToZero trait for implementing the null value of types.
- The ToMin trait for implementing lower bounds on types.
- The ToMax trait for implementing upper bounds on types.

### Usage

#### Add this to your Cargo.toml
```toml
[dependencies]
num_convert = { git = "https://github.com/pic16f877ccs/num_convert", version = "0.7.3" }
```

#### Or using cargo
```sh
cargo add num_convert --git "https://github.com/pic16f877ccs/num_convert"
```
#### Examples

Convert from negative to positive and vice versa.
```rust
use num_convert::FromByAdd;

assert_eq!(<u8>::from_by_add(-28i8), 100u8);
assert_eq!(<i8>::from_by_add(10u8), -118i8);
```

Convert between integer types.
```rust
use num_convert::IntoByAdd;

fn integer_to_integer<T1: IntoByAdd<U1>, T2: IntoByAdd<U2>, U1, U2>(min: T1, max: T2) -> (U1, U2) {
    (min.into_by_add(), max.into_by_add())
}
assert_eq!(integer_to_integer(i16::MIN, u16::MAX), (u16::MIN, i16::MAX));
assert_eq!(integer_to_integer(u16::MIN, u16::MAX), (i16::MIN, i16::MAX));
```

Convert from negative to positive without error and with error.
```rust
use num_convert::TryFromByAdd;

assert_eq!(<u8>::try_from_by_add(-128i16), Some(0u8));
assert_eq!(<u8>::try_from_by_add(-129i16), None);
```

Convert between 128-bit types losslessly.
```rust
use num_convert::TryIntoByAdd;

assert_eq!(<i128 as TryIntoByAdd<u128>>::try_into_by_add(i128::MIN), Some(u128::MIN));
assert_eq!(<u128 as TryIntoByAdd<i128>>::try_into_by_add(u128::MIN), Some(i128::MIN));
```

Convert between U16 and U8 without overflow and with overflow.
```rust
use num_convert::{IntoAs, FromAs};

assert_eq!(<u16 as IntoAs<u8>>::into_as(255u16), 255u8);
assert_eq!(<u16 as IntoAs<u8>>::into_as(258u16), 2u8);
assert_eq!(<u8>::from_as(261u16), 5u8);
```

Convert from digits as a number.
```rust
use num_convert::TryFromDigits;

let arr: [u32; 6] = [25_635_071, 25_634_091, 25_633_334, 25_636_309, 25_637_101, 25_636_243];
let val: Vec<u8> = arr.iter().map(|var| u8::from_digits(*var).unwrap_or(255u8) ).collect::<_>();
assert_eq!(val, [71, 91, 255, 255, 101, 243]);
```

Determine the size of integer values in bits.
```rust
use num_convert::Sbits;

assert_eq!((-128i8).sbits(), 8u32);
assert_eq!(u64::MAX.sbits(), 64u32);
```

Determine the size of integer types in bits.
```rust
use num_convert::Tbits;

assert_eq!(i8::tbits(), 8u32);
assert_eq!(u64::tbits(), 64u32);
```

Determine the number of digits of integers.
```rust
use num_convert::IntegerLen;

assert_eq!(0i8.len(), 1usize);
assert_eq!(i8::MAX.len(), 3usize);
assert_eq!(i128::MAX.len(), 39usize);
assert_eq!(u128::MAX.len(), 39usize);
```

Convert from tuple to array of integers.
```rust
use num_convert::FromTuple;

assert_eq!(<i32 as FromTuple>::from_5((true, false, 45u8, 2023u16, -60i8,)), [1i32, 0i32, 45i32, 2023i32, -60i32]);
assert_eq!(<i32>::from_3((45u8, 2023u16, -53i8,)).iter().sum::<i32>(), 2015i32);
```

Convert from str or integer to type integer.
```rust
use num_convert::TryFromIntStr;

assert_eq!(<u16 as TryFromIntStr<&str>>::try_from_int_str("2023"), Ok(2023u16));
assert!(<u16 as TryFromIntStr<&str>>::try_from_int_str("20-2-2023").is_err());
assert_eq!(<u16 as TryFromIntStr<u128>>::try_from_int_str(22023), Ok(22023));
assert!(<u16 as TryFromIntStr<u128>>::try_from_int_str(222022).is_err());
assert_eq!(<i32 as TryFromIntStr<bool>>::try_from_int_str(true), Ok(1i32));
assert_eq!(<i32 as TryFromIntStr<bool>>::try_from_int_str(false), Ok(0i32));
```

Convert tuple to array of integers.
```rust
use num_convert::TryTupToArr;

assert_eq!(TryTupToArr::<i32>::try_into_arr((45u8, 2023u16, -60i8,)),
Ok([45i32, 2023i32, -60i32]));

let arr: [i16; 3] = ("45", 2023u16, true,).try_into_arr().unwrap();
assert_eq!(arr, [45i16, 2023i16, 1i16]);
```

## License
MIT

