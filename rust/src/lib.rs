mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn count_prefix_suffix_pairs(words: &[&str]) -> i64 {
    let mut res = 0;
    let mut trie = Trie::default();
    for s in words {
        res += trie.insert(s.bytes().zip(s.bytes().rev()));
    }
    res
}

#[derive(Default)]
struct Trie {
    nodes: HashMap<[u8; 2], Trie>,
    count: i64,
}

impl Trie {
    fn insert(&mut self, it: impl Iterator<Item = (u8, u8)>) -> i64 {
        let mut curr = self;
        let mut res = 0;
        for (b1, b2) in it {
            curr = curr.nodes.entry([b1, b2]).or_default();
            res += curr.count;
        }
        curr.count += 1;
        res
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {{
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        }};
    }

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        }};
    }

    #[test]
    fn basics() {
        assert_eq!(count_prefix_suffix_pairs(&["a", "aba", "ababa", "aa"]), 4);
        assert_eq!(count_prefix_suffix_pairs(&["pa", "papa", "ma", "mama"]), 2);
        assert_eq!(count_prefix_suffix_pairs(&["abab", "ab"]), 0);
    }

    #[test]
    fn test() {}
}
