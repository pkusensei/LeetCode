mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
    for i in 1..=n / 2 {
        if check(i) && check(n - i) {
            return vec![i, n - i];
        }
    }
    vec![]
}

const fn check(mut n: i32) -> bool {
    while n > 0 {
        if n % 10 == 0 {
            return false;
        }
        n /= 10;
    }
    true
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(get_no_zero_integers(2), [1, 1]);
        assert_eq!(get_no_zero_integers(11), [2, 9]);
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
