use crate::traits::Zero;
use std::ops::{Mul, Neg};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub enum Sign {
    Negative,
    Zero,
    Positive,
}

impl Sign {
    pub fn abs(&self) -> Self {
        match self {
            Self::Zero => Self::Zero,
            _ => Self::Positive,
        }
    }
}

impl Mul for Sign {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Zero, _) | (_, Self::Zero) => Self::Zero,
            (Self::Negative, Self::Negative) | (Self::Positive, Self::Positive) => Self::Positive,
            _ => Self::Negative,
        }
    }
}

impl Zero for Sign {
    #[inline]
    fn zero() -> Self {
        Sign::Zero
    }

    #[inline]
    fn is_zero(&self) -> bool {
        *self == Sign::Zero
    }
}

impl Neg for Sign {
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            Self::Negative => Self::Positive,
            Self::Zero => Self::Zero,
            Self::Positive => Self::Negative,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero() {
        assert_eq!(Sign::zero(), Sign::Zero);
    }

    #[test]
    fn mul() {
        assert_eq!(Sign::Negative * Sign::Negative, Sign::Positive);
        assert_eq!(Sign::Negative * Sign::Zero, Sign::Zero);
        assert_eq!(Sign::Negative * Sign::Positive, Sign::Negative);
        assert_eq!(Sign::Zero * Sign::Negative, Sign::Zero);
        assert_eq!(Sign::Zero * Sign::Zero, Sign::Zero);
        assert_eq!(Sign::Zero * Sign::Positive, Sign::Zero);
        assert_eq!(Sign::Positive * Sign::Negative, Sign::Negative);
        assert_eq!(Sign::Positive * Sign::Zero, Sign::Zero);
        assert_eq!(Sign::Positive * Sign::Positive, Sign::Positive);
    }
}
