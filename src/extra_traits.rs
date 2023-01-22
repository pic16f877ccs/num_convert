use crate::IntoAs;
use core::ops::Div;

/// A trait IntegerLen to determine the number of digits of integers.
pub trait IntegerLen {
    /// Returns the number of digits to self.
    fn len(self) -> usize;
}

impl<T> IntegerLen for T
where
    T: Eq + Copy + Div<Output = T> + IntoAs<T>,
{
    #[inline]
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
