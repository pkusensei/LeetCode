mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn arrange_words(text: &str) -> String {
    let mut words: Vec<_> = text.split_whitespace().collect();
    words.sort_by_key(|w| w.len());
    let mut res = words
        .into_iter()
        .map(|w| w.to_ascii_lowercase())
        .collect::<Vec<_>>()
        .join(" ")
        .into_bytes();
    res[0] = res[0].to_ascii_uppercase();
    String::from_utf8(res).unwrap()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(arrange_words("Leetcode is cool"), "Is cool leetcode");
        assert_eq!(
            arrange_words("Keep calm and code on"),
            "On and keep calm code"
        );
        assert_eq!(arrange_words("To be or not to be"), "To be or to be not");
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
