mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn is_alien_sorted(words: &[&str], order: &str) -> bool {
    let order = order
        .bytes()
        .enumerate()
        .fold(HashMap::new(), |mut acc, (i, b)| {
            acc.insert(b, i as u8);
            acc
        });
    words.windows(2).all(|w| {
        let [s1, s2] = [w[0], w[1]].map(|s| s.bytes().map(|b| order[&b]).collect::<Vec<_>>());
        s1 <= s2
    })
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(is_alien_sorted(
            &["hello", "leetcode"],
            "hlabcdefgijkmnopqrstuvwxyz"
        ));
        debug_assert!(!is_alien_sorted(
            &["word", "world", "row"],
            "worldabcefghijkmnpqstuvxyz"
        ));
        debug_assert!(!is_alien_sorted(
            &["apple", "app"],
            "abcdefghijklmnopqrstuvwxyz"
        ));
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
