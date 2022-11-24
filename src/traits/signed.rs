use crate::numbers::Sign;
use crate::traits::Number;
use std::cmp::Ord;
use std::cmp::Ordering;
use std::ops::Neg;

pub trait Signed: Number + Neg<Output = Self> {
    fn sign(&self) -> Sign;

    #[inline]
    fn of_sign(s: Sign) -> Self {
        match s {
            Sign::Positive => Self::one(),
            Sign::Zero => Self::zero(),
            Sign::Negative => Self::one().neg(),
        }
    }

    #[inline]
    fn abs(self) -> Self {
        match self.sign() {
            Sign::Positive | Sign::Zero => self,
            Sign::Negative => self.neg(),
        }
    }

    #[inline]
    fn sign_number(&self) -> Self {
        Self::of_sign(self.sign())
    }

    #[inline]
    fn is_positive(&self) -> bool {
        self.sign() == Sign::Positive
    }

    #[inline]
    fn is_negative(&self) -> bool {
        self.sign() == Sign::Negative
    }
}

macro_rules! impl_signed_int {
    ($($t:ty;)*) => ($(
        impl Signed for $t {
            #[inline]
            fn sign(&self) -> Sign {
                match self.cmp(&0) {
                    Ordering::Less => Sign::Negative,
                    Ordering::Equal => Sign::Zero,
                    Ordering::Greater => Sign::Positive,
                }
            }
        }
    )*)
}

impl_signed_int!(i8; i16; i32; i64; isize;);

#[cfg(has_i128)]
impl_signed_int!(i128);

macro_rules! impl_signed_float {
    ($($t:ty;)*) => ($(
        impl Signed for $t {
            #[inline]
            fn sign(&self) -> Sign {
                if self.is_sign_positive() {
                    Sign::Positive
                } else if *self == 0. {
                    Sign::Zero
                } else {
                    Sign::Negative
                }
            }
        }
    )*)
}

impl_signed_float!(f32; f64;);

pub trait Unsigned: Number {}

macro_rules! impl_unsigned {
    ($($t:ty;)*) => ($(
        impl Unsigned for $t {}
    )*)
}

impl_unsigned!(u8; u16; u32; u64; usize;);

#[cfg(has_i128)]
impl_unsigned!(u128);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signed_int() {
        assert_eq!(isize::of_sign(Sign::Negative), -1_isize);
        assert_eq!(i8::of_sign(Sign::Positive), 1_i8);
        assert_eq!(i64::of_sign(Sign::Zero), 0_i64);
    }

    #[test]
    fn signed_float() {
        assert_eq!(f64::of_sign(Sign::Negative), -1.0_f64);
        assert_eq!(f32::of_sign(Sign::Positive), 1.0_f32);
        assert_eq!(f64::of_sign(Sign::Zero), 0.0_f64);
    }

    #[test]
    fn sign() {
        assert_eq!(15_i16.sign(), Sign::Positive);
        assert_eq!((-0.5_f32).sign(), Sign::Negative);
        assert_eq!((-0.0_f32).sign(), Sign::Zero);
    }

    #[test]
    fn abs() {
        assert_eq!((-4_i16).abs(), 4_i16);
        assert_eq!(0_isize.abs(), 0_isize);
        assert_eq!((-4.0_f64).abs(), 4.0_f64.abs());
    }

    #[test]
    fn sign_number() {
        assert_eq!(15_i16.sign_number(), 1_i16);
        assert_eq!((-0.5_f32).sign_number(), -1.0_f32);
    }

    #[test]
    fn sign_test() {
        assert!(15_i16.is_positive());
        assert!(!15_i16.is_negative());
        assert!(!(-0.5_f32).is_positive());
        assert!((-0.5_f32).is_negative());
        assert!(!(0_i16).is_positive());
        assert!(!(0_i16).is_negative());
        assert!(!(-0.0_f32).is_positive());
        assert!(!(-0.0_f32).is_negative());
    }
}
