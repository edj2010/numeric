use crate::traits::number::Number;

///////
/// Indicates a number behaves like an integer
/// in particular, has divisibility rules
/// like the natural numbers/integers or integers
/// we can look for common factors and (maybe) test primality
///////
pub trait Integer: Number {
    fn gcd(self, other: Self) -> Self;
    fn lcm(self, other: Self) -> Self;
}
