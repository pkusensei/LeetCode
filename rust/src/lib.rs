mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn string_matching(words: &[&str]) -> Vec<String> {
    let mut res = std::collections::HashSet::new();
    for (i, a) in words.iter().enumerate() {
        for b in words.iter().skip(1 + i) {
            if a.len() != b.len() {
                if a.contains(b) {
                    res.insert(b);
                }
                if b.contains(a) {
                    res.insert(a);
                }
            }
        }
    }
    res.into_iter().map(|s| s.to_string()).collect()
}

fn with_kmp(words: &[&str]) -> Vec<String> {
    fn compute_lps(s: &str) -> Vec<usize> {
        let (s, n) = (s.as_bytes(), s.len());
        let mut res = vec![0; n];
        let (mut idx, mut len) = (1, 0);
        while idx < n {
            if s[idx] == s[len] {
                len += 1;
                res[idx] = len;
                idx += 1;
            } else if len > 0 {
                len = res[len - 1];
            } else {
                idx += 1;
            }
        }
        res
    }
    fn is_sub_of(needle: &str, hay: &str, lps: &[usize]) -> bool {
        let [mut hay_idx, mut needle_idx] = [0; 2];
        let [hay, needle] = [hay, needle].map(|v| v.as_bytes());
        while hay_idx < hay.len() {
            if hay[hay_idx] == needle[needle_idx] {
                hay_idx += 1;
                needle_idx += 1;
                if needle_idx == needle.len() {
                    return true;
                }
            } else if needle_idx > 0 {
                needle_idx = lps[needle_idx - 1];
            } else {
                hay_idx += 1;
            }
        }
        false
    }

    let mut res = vec![];
    for a in words.iter() {
        let lps = compute_lps(a);
        for b in words.iter() {
            if a == b {
                continue;
            }
            if is_sub_of(a, b, &lps) {
                res.push(a.to_string());
                break;
            }
        }
    }
    res
}

fn with_suffix_trie(words: &[&str]) -> Vec<String> {
    #[derive(Debug, Clone, Default)]
    struct Trie {
        freq: i32,
        nodes: HashMap<u8, Trie>,
    }
    impl Trie {
        fn insert(&mut self, s: &str) {
            let mut curr = self;
            for b in s.bytes() {
                curr = curr.nodes.entry(b).or_default();
                curr.freq += 1;
            }
        }
        fn find(&self, s: &str) -> bool {
            let mut curr = self;
            for b in s.bytes() {
                curr = &curr.nodes[&b];
            }
            curr.freq > 1
        }
    }

    let mut root = Trie::default();
    for word in words.iter() {
        for i in 0..word.len() {
            root.insert(&word[i..]);
        }
    }
    let mut res = vec![];
    for word in words.iter() {
        if root.find(word) {
            res.push(word.to_string());
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
            string_matching(&["mass", "as", "hero", "superhero"]),
            ["as", "hero"],
        );
        sort_eq(string_matching(&["leetcode", "et", "code"]), ["et", "code"]);
        assert!(string_matching(&["blue", "green", "bu"]).is_empty());

        sort_eq(
            with_kmp(&["mass", "as", "hero", "superhero"]),
            ["as", "hero"],
        );
        sort_eq(with_kmp(&["leetcode", "et", "code"]), ["et", "code"]);
        assert!(with_kmp(&["blue", "green", "bu"]).is_empty());

        sort_eq(
            with_suffix_trie(&["mass", "as", "hero", "superhero"]),
            ["as", "hero"],
        );
        sort_eq(
            with_suffix_trie(&["leetcode", "et", "code"]),
            ["et", "code"],
        );
        assert!(with_suffix_trie(&["blue", "green", "bu"]).is_empty());
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
