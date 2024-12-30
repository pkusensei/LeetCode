mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_flips(mut a: i32, mut b: i32, mut c: i32) -> i32 {
    let mut res = 0;
    while [a, b, c].iter().any(|&v| v > 0) {
        if c & 1 == 0 {
            res += (a & 1) + (b & 1);
        } else {
            res += match (a & 1, b & 1) {
                (1, 0) | (0, 1) => 0,
                (0, 0) => 1,
                _ => 0,
            };
        }
        [a, b, c] = [a, b, c].map(|v| v >> 1);
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(min_flips(2, 6, 5), 3);
        assert_eq!(min_flips(4, 2, 7), 1);
        assert_eq!(min_flips(1, 2, 3), 0);
    }

    #[test]
    fn test() {
        assert_eq!(min_flips(8, 3, 5), 3);
    }

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
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
