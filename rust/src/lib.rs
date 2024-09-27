mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn detect_capital_use(word: &str) -> bool {
    let s = word.as_bytes();
    s.iter().all(|b| b.is_ascii_uppercase())
        || s.iter().all(|b| b.is_ascii_lowercase())
        || (s[0].is_ascii_uppercase() && s[1..].iter().all(|b| b.is_ascii_lowercase()))
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(detect_capital_use("USA"));
        debug_assert!(!detect_capital_use("FlaG"));
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
}
