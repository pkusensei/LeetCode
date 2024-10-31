mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn binary_gap(mut n: i32) -> i32 {
    let (mut res, mut streak) = (0, 0);
    let mut seen_one = false;
    while n > 0 {
        if n & 1 == 1 {
            seen_one = true;
            res = res.max(streak);
            streak = 1;
        } else if seen_one {
            streak += 1;
            res = res.max(streak);
        }
        n >>= 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(binary_gap(22), 2);
        debug_assert_eq!(binary_gap(8), 0);
        debug_assert_eq!(binary_gap(5), 2);
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
