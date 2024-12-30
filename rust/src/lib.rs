mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn distinct_echo_substrings(text: &str) -> i32 {
    let (s, n) = (text.as_bytes(), text.len());
    let mut set = std::collections::HashSet::new();
    for len in 1..=n / 2 {
        let (mut left, mut right) = (0, len);
        let mut count = 0;
        while left < n - len {
            if s[left] == s[right] {
                count += 1;
            } else {
                count = 0;
            }
            if count == len {
                set.insert(&s[left + 1 - len..1 + left]);
                count -= 1;
            }
            left += 1;
            right += 1;
        }
    }
    set.len() as _
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(distinct_echo_substrings("abcabcabc"), 3);
        assert_eq!(distinct_echo_substrings("leetcodeleetcode"), 2);
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
