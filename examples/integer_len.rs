use num_convert::IntoAs;
use std::ops::Div;

trait IntegerLen {
    fn len(self) -> usize;
}

impl<T> IntegerLen for T
where
    T: Eq + Copy + Div<Output = T> + IntoAs<T>,
{
    fn len(mut self) -> usize {
        let mut count = 0;
        let ten = 10.into_as();
        let zero = <T>::from_u8(0);
        while self != zero {
            self = self / ten;
            count += 1;
        }
        if count == 0 {
            1
        } else {
            count
        }
    }
}

fn main() {
    let val = u128::MAX;
    println!("Max value of u128 => {} digits", val.len());
    assert_eq!(val.len(), val.to_string().len());

    let val = i128::MIN;
    assert_eq!(val.len(), val.to_string().trim_start_matches('-').len());
    println!("Min value of i128 => {} digits", val.len());
}
