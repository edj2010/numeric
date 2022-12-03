use crate::traits::Ring;

pub fn pow<T: Ring + Clone>(x: T, e: usize) -> T {
    if e == 0 {
        T::one()
    } else if e & 1 > 0 {
        let p = pow(x.clone(), (e - 1) / 2);
        x.clone() * p.clone() * p
    } else {
        let p = pow(x, e / 2);
        p.clone() * p
    }
}
