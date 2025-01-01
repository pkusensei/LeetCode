mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_score(s: &str) -> i32 {
    let n = s.len();
    let [mut prefix, mut suffix] = [0, 1].map(|_| Vec::with_capacity(n - 1));
    for b in s.bytes().take(n - 1) {
        prefix.push(i32::from(b == b'0') + prefix.last().unwrap_or(&0));
    }
    for b in s.bytes().rev().take(n - 1) {
        suffix.push(i32::from(b == b'1') + suffix.last().unwrap_or(&0));
    }
    prefix
        .into_iter()
        .zip(suffix.into_iter().rev())
        .map(|(a, b)| a + b)
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(max_score("011101"), 5);
        assert_eq!(max_score("00111"), 5);
        assert_eq!(max_score("1111"), 3);
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
