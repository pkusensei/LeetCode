mod helper;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn word_pattern(pattern: &str, s: &str) -> bool {
    let plen = pattern.len();
    let slen = s.split_whitespace().count();
    if plen != slen {
        return false;
    }
    let mut m1 = HashMap::new();
    let mut m2 = HashMap::new();
    for (b, s) in pattern.bytes().zip(s.split_whitespace()) {
        if m1.insert(b, s).is_some_and(|v| v != s) {
            return false;
        }
        if m2.insert(s, b).is_some_and(|v| v != b) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(word_pattern("abba", "dog cat cat dog"));
        debug_assert!(!word_pattern("abba", "dog cat cat fish"));
        debug_assert!(!word_pattern("aaaa", "dog cat cat dog"));
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
