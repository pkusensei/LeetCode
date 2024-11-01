mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn make_fancy_string(s: &str) -> String {
    s.as_bytes()
        .chunk_by(|a, b| a == b)
        .flat_map(|ch| ch.iter().take(2).copied().map(char::from))
        .collect()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(make_fancy_string("leeetcode"), "leetcode");
        debug_assert_eq!(make_fancy_string("aaabaaaa"), "aabaa");
        debug_assert_eq!(make_fancy_string("aab"), "aab");
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
