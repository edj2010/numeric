use std::ops::{Add, Div, Mul, Sub};

/// Additive Identity
///
/// Satisfies
/// x + 0 = 0 + x = x
pub trait Zero: Sized {
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
}

macro_rules! impl_zero {
    ($($t:ty, $v:expr;)*) => ($(
        impl Zero for $t {
            #[inline]
            fn zero() -> Self {
                $v
            }
            #[inline]
            fn is_zero(&self) -> bool {
                *self == $v
            }
        }
    )*)
}

impl_zero!(u8, 0; u16, 0; u32, 0; u64, 0; usize, 0; i8, 0; i16, 0; i32, 0; i64, 0; isize, 0; f32, 0.; f64, 0.;);
#[cfg(has_i128)]
impl_zero!(u128, 0; i128, 0;);

pub trait One: Sized {
    fn one() -> Self;
    fn is_one(&self) -> bool;
}

macro_rules! impl_one {
    ($($t:ty, $v:expr;)*) => ($(
        impl One for $t {
            #[inline]
            fn one() -> Self {
                $v
            }
            #[inline]
            fn is_one(&self) -> bool {
                *self == $v
            }
        }
    )*)
}

impl_one!(u8, 1; u16, 1; u32, 1; u64, 1; usize, 1; i8, 1; i16, 1; i32, 1; i64, 1; isize, 1; f32, 1.; f64, 1.;);
#[cfg(has_i128)]
impl_one!(u128, 1; i128, 1;);

/////////
/// Trait describing a group
/// Promises notion of equality, additive identity, and inverses
/////////
pub trait Group: PartialEq + Zero + Add<Self, Output = Self> + Sub<Self, Output = Self> {}

impl<T> Group for T where T: PartialEq + Zero + Add<Self, Output = Self> + Sub<Self, Output = Self> {}
/////////
/// Trait describing number ring with
/// not-necessarily commutative/invertable multiplication
/////////
pub trait Ring: Group + One + Mul<Self, Output = Self> {}

impl<T> Ring for T where T: Group + One + Mul<Self, Output = Self> {}

/////////
/// Trait describing a generic number, with a notion of division and remainder
/////////
pub trait Number: Ring + Div<Self, Output = Self> {}

impl<T> Number for T where T: Ring + Div<Self, Output = Self> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero() {
        assert_eq!(usize::zero(), 0_usize);
        assert_eq!(u8::zero(), 0_u8);
        assert_eq!(f64::zero(), 0.0_f64);
    }

    #[test]
    fn one() {
        assert_eq!(isize::one(), 1_isize);
        assert_eq!(f64::one(), 1.0_f64);
    }
}
