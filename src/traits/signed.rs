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

impl_signed_int!(i8; i16; i32; i64;);

#[cfg(has_i128)]
impl_signed_int!(i128);

macro_rules! impl_signed_float {
    ($($t:ty;)*) => ($(
        impl Signed for $t {
            #[inline]
            fn sign(&self) -> Sign {
                if self.is_sign_positive() {
                    Sign::Positive
                } else if self.is_sign_negative() {
                    Sign::Negative
                } else {
                    Sign::Zero
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

impl_unsigned!(u8; u16; u32; u64;);

#[cfg(has_i128)]
impl_unsigned!(u128);
