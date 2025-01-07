mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn number_of_substrings(s: &str) -> i32 {
    let (s, n) = (s.as_bytes(), s.len());
    let mut left = 0;
    let mut res = 0;
    let mut count = [0; 3];
    for (right, &b) in s.iter().enumerate() {
        match b {
            b'a' => count[0] += 1,
            b'b' => count[1] += 1,
            b'c' => count[2] += 1,
            _ => (),
        }
        while count.iter().all(|&v| v > 0) {
            res += n - right;
            match s[left] {
                b'a' => count[0] -= 1,
                b'b' => count[1] -= 1,
                b'c' => count[2] -= 1,
                _ => (),
            }
            left += 1;
        }
    }
    res as _
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(number_of_substrings("abcabc"), 10);
        assert_eq!(number_of_substrings("aaacb"), 3);
        assert_eq!(number_of_substrings("abc"), 1);
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
