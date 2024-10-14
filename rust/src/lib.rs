mod helper;
mod trie;

use std::collections::HashSet;

#[allow(unused_imports)]
use helper::*;

pub fn longest_word(words: &[&str]) -> String {
    let prefix: HashSet<&str> = words.iter().map(|s| s.as_ref()).collect();
    let mut s = "";
    for &word in words.iter() {
        if check(&prefix, word) {
            if word.len() > s.len() || word.len() == s.len() && word < s {
                s = word
            }
        }
    }
    s.to_string()
}

fn check(prefix: &HashSet<&str>, word: &str) -> bool {
    let mut s = word.to_string();
    while s.len() > 1 {
        s.pop();
        if !prefix.contains(&s.as_str()) {
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
        debug_assert_eq!(longest_word(&["w", "wo", "wor", "worl", "world"]), "world");
        debug_assert_eq!(
            longest_word(&["a", "banana", "app", "appl", "ap", "apply", "apple"]),
            "apple"
        );
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
