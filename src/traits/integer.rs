use crate::traits::number::{Number, Zero};
use std::ops::Rem;
///////
/// Indicates a number behaves like an integer (integral domain)
/// in particular, has divisibility rules and primes
/// like the natural numbers/integers or integers
///////
pub trait Integer: Number {
    fn is_factor_of(self, other: Self) -> bool;

    #[inline]
    fn is_divisible_by(self, other: Self) -> bool {
        other.is_factor_of(self)
    }

    #[inline]
    fn is_unit(self) -> bool {
        self.is_factor_of(Self::one())
    }
}

macro_rules! impl_integer_int {
    ($($t:ty;)*) => ($(
        impl Integer for $t {
            #[inline]
            fn is_factor_of(self, other: Self) -> bool {
                other%self == Self::zero()
            }
        }
    )*)
}

impl_integer_int!(u8; u16; u32; u64; usize; i8; i16; i32; i64; isize;);

#[cfg(has_i128)]
impl_integer_int!(u128; i128);

///////
/// Indicates a number is a "euclidean domain"
/// in particular, has a "euclidean function"
/// like the natural numbers/integers or integers
/// This also promises the idea of a "remainder"
///////
pub trait Euclidean: Integer + Rem<Self, Output = Self> {
    fn gcd(self, other: Self) -> Self;
    fn lcm(self, other: Self) -> Self;
}

//////
/// A default implementation of the euclidean algorithm
/// Required the underlying type to implement clone
//////
#[macro_export]
macro_rules! impl_euclidean_default {
    ($t:ty) => {
        impl Euclidean for $t {
            fn gcd(self, other: Self) -> Self {
                if other == Self::zero() {
                    self
                } else {
                    other.clone().gcd(self % other)
                }
            }

            fn lcm(self, other: Self) -> Self {
                (self.clone() * other.clone()) / self.gcd(other)
            }
        }
    };
}

macro_rules! impl_euclidean_unsigned_int {
    ($($t:ty;)*) => ($(
        impl Euclidean for $t {
            fn gcd(mut self, mut other: Self) -> Self {
                if self == 0 { return other; }
                else if other == 0 { return self; }

                let i = self.trailing_zeros(); self >>= i;
                let j = other.trailing_zeros(); other >>= j;
                let k = std::cmp::min(i, j);

                while self != other {
                    if self < other { std::mem::swap(&mut self, &mut other); }
                    self -= other;
                    self >>= self.trailing_zeros();
                }
                self << k
            }

            fn lcm(self, other: Self) -> Self {
                self * other / self.gcd(other)
            }
        }
    )*)
}

impl_euclidean_unsigned_int!(u8; u16; u32; u64; usize;);

#[cfg(has_i128)]
impl_euclidean_unsigned_int!(u128;);

macro_rules! impl_euclidean_signed_int {
    ($($t:ty;)*) => ($(
        impl Euclidean for $t {
            fn gcd(mut self, mut other: Self) -> Self {
                self = self.abs();
                other = other.abs();

                if self == 0 { return other; }
                else if other == 0 { return self; }

                let i = self.trailing_zeros(); self >>= i;
                let j = other.trailing_zeros(); other >>= j;
                let k = std::cmp::min(i, j);

                while self != other {
                    if self < other { std::mem::swap(&mut self, &mut other); }
                    self -= other;
                    self >>= self.trailing_zeros();
                }
                self << k
            }

            fn lcm(self, other: Self) -> Self {
                self * other / self.gcd(other)
            }
        }
    )*)
}

impl_euclidean_signed_int!(i8; i16; i32; i64; isize;);

#[cfg(has_i128)]
impl_euclidean_signed_int!(i128;);
