mod helper;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn palindrome_pairs(words: &[&str]) -> Vec<[i32; 2]> {
    let map = words
        .iter()
        .enumerate()
        .fold(HashMap::new(), |mut acc, (i, s)| {
            acc.insert(s.as_bytes(), i);
            acc
        });
    let mut res = vec![];
    for (i, word) in words.iter().enumerate() {
        for j in 0..=word.len() {
            let prefix = word[..j].as_bytes();
            let suffix = word[j..].as_bytes();
            if is_palindrome(prefix) {
                let mut reversed = suffix.to_vec();
                reversed.reverse();
                if let Some(&k) = map.get(&reversed.as_slice()) {
                    if k != i {
                        res.push([k as i32, i as i32]);
                    }
                }
            }
            if !suffix.is_empty() && is_palindrome(suffix) {
                let mut reversed = prefix.to_vec();
                reversed.reverse();
                if let Some(&k) = map.get(&reversed.as_slice()) {
                    if k != i {
                        res.push([i as i32, k as i32]);
                    }
                }
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        sort_eq(
            palindrome_pairs(&["abcd", "dcba", "lls", "s", "sssll"]),
            [[0, 1], [1, 0], [3, 2], [2, 4]],
        );
        sort_eq(palindrome_pairs(&["bat", "tab", "cat"]), [[0, 1], [1, 0]]);
        sort_eq(palindrome_pairs(&["a", ""]), [[0, 1], [1, 0]]);
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
