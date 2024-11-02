mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn is_circular_sentence(sentence: &str) -> bool {
    if sentence.starts_with(' ') || sentence.ends_with(' ') {
        return false;
    }
    sentence.bytes().next() == sentence.bytes().last()
        && sentence
            .as_bytes()
            .windows(3)
            .filter(|w| w[1] == b' ')
            .all(|w| w[0] == w[2] && w[0] != b' ')
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(is_circular_sentence("leetcode exercises sound delightful"));
        debug_assert!(is_circular_sentence("eetcode"));
        debug_assert!(!is_circular_sentence("Leetcode is cool"));
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
