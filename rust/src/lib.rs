mod helper;
mod trie;

use std::collections::{HashMap, HashSet};

#[allow(unused_imports)]
use helper::*;

pub fn most_common_word(paragraph: &str, banned: &[&str]) -> String {
    let banned: HashSet<_> = banned.iter().collect();
    paragraph
        .split(|c: char| !c.is_ascii_alphabetic())
        .fold(HashMap::new(), |mut acc, s| {
            let s = s.to_ascii_lowercase();
            if !banned.contains(&s.as_str()) && !s.is_empty() {
                *acc.entry(s).or_insert(0) += 1;
            }
            acc
        })
        .into_iter()
        .max_by_key(|(_k, v)| *v)
        .map(|(k, _v)| k)
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            most_common_word(
                "Bob hit a ball, the hit BALL flew far after it was hit.",
                &["hit"]
            ),
            "ball"
        );
        debug_assert_eq!(most_common_word("a.", &[]), "a");
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
