mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn balanced_string(s: &str) -> i32 {
    let n = s.len();
    let mut count = s.bytes().fold([0; 26], |mut acc, b| {
        acc[usize::from(b - b'A')] += 1;
        acc
    });
    let mut res = n;
    let mut left = 0;
    // Find the window whose removal leads to all counts <= n/4
    for (right, b) in s.bytes().enumerate() {
        count[usize::from(b - b'A')] -= 1;
        while left < n && count.iter().all(|&v| v <= n / 4) {
            res = res.min(right + 1 - left);
            count[usize::from(s.as_bytes()[left] - b'A')] += 1;
            left += 1
        }
    }
    res as i32
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(balanced_string("QWER"), 0);
        assert_eq!(balanced_string("QQWE"), 1);
        assert_eq!(balanced_string("QQQW"), 2);
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
        if (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() > EP {
            dbg!(a, b);
        }
    }
}
