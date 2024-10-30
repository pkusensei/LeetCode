mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn mirror_reflection(p: i32, q: i32) -> i32 {
    let gcd = gcd(p, q);
    let lcm = p * q / gcd;
    // the first corner that ray hits
    let (a, b) = (lcm / p, lcm / q);
    match (a & 1, b & 1) {
        (1, 1) => 1,
        (0, 1) => 0,
        _ => 2,
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(mirror_reflection(2, 1), 2);
        debug_assert_eq!(mirror_reflection(3, 1), 1);
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: AsMut<[T1]>,
        I2: AsMut<[T2]>,
    {
        i1.as_mut().sort_unstable();
        i2.as_mut().sort_unstable();
        debug_assert_eq!(i1.as_mut(), i2.as_mut());
    }

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64>,
        T2: Into<f64>,
    {
        const EP: f64 = 1e-5;
        debug_assert!((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP);
    }
}
