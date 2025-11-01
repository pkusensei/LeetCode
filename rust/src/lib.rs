mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

use itertools::chain;
use std::iter::once;

struct WordFilter {
    trie: Trie,
}

impl WordFilter {
    fn new(words: Vec<String>) -> Self {
        let mut trie = Trie::default();
        for (idx, s) in words.iter().enumerate() {
            let n = s.len();
            for i in 0..n {
                trie.add(chain!(s.bytes().skip(i), once(1 + b'z'), s.bytes()), idx);
            }
        }
        Self { trie }
    }

    fn f(&self, pref: String, suff: String) -> i32 {
        self.trie
            .find(chain!(suff.bytes(), once(1 + b'z'), pref.bytes()))
    }
}

struct Trie {
    nodes: [Option<Box<Trie>>; 27],
    idx: i32,
}

impl Default for Trie {
    fn default() -> Self {
        Self {
            nodes: Default::default(),
            idx: -1,
        }
    }
}

impl Trie {
    fn add(&mut self, s: impl Iterator<Item = u8>, idx: usize) {
        let mut curr = self;
        for b in s {
            let i = usize::from(b - b'a');
            curr = curr.nodes[i].get_or_insert_default();
            curr.idx = idx as i32
        }
    }

    fn find(&self, s: impl Iterator<Item = u8>) -> i32 {
        let mut curr = self;
        for b in s {
            let i = usize::from(b - b'a');
            let Some(v) = curr.nodes[i].as_ref() else {
                return -1;
            };
            curr = v;
        }
        curr.idx
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

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
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
    fn basics() {}

    #[test]
    fn test() {}
}
