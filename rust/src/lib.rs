mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn count_consistent_strings(allowed: &str, words: &[&str]) -> i32 {
    let a = as_bits(allowed);
    words.iter().filter(|s| (as_bits(s) | a) == a).count() as _
}

fn as_bits(s: &str) -> i32 {
    s.bytes().fold(0, |acc, b| acc | (1 << (b - b'a')))
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            count_consistent_strings("ab", &["ad", "bd", "aaab", "baa", "badab"]),
            2
        );
        debug_assert_eq!(
            count_consistent_strings("abc", &["a", "b", "c", "ab", "ac", "bc", "abc"]),
            7
        );
        debug_assert_eq!(
            count_consistent_strings("cad", &["cc", "acd", "b", "ba", "bac", "bad", "ac", "d"]),
            4
        );
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(i1: I1, i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: IntoIterator<Item = T1>,
        I2: IntoIterator<Item = T2>,
    {
        let mut v1: Vec<_> = i1.into_iter().collect();
        let mut v2: Vec<_> = i2.into_iter().collect();
        v1.sort_unstable();
        v2.sort_unstable();
        debug_assert_eq!(v1, v2);
    }
}
