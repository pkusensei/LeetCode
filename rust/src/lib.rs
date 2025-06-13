mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

use std::collections::BTreeMap;

#[allow(unused_imports)]
use helper::*;

pub fn longest_common_prefix(words: &[&str], k: i32) -> Vec<i32> {
    let mut trie = Trie::default();
    let mut freq = BTreeMap::new();
    for s in words.iter() {
        trie.insert(s.bytes(), k, &mut freq);
    }
    let mut res = vec![];
    for s in words.iter() {
        trie.remove(s.bytes(), k, &mut freq);
        res.push(*freq.keys().last().unwrap_or(&0));
        trie.insert(s.bytes(), k, &mut freq);
    }
    res
}

#[derive(Default)]
struct Trie {
    nodes: [Option<Box<Trie>>; 26],
    count: i32,
    depth: i32,
}

impl Trie {
    fn insert(&mut self, it: impl Iterator<Item = u8>, k: i32, freq: &mut BTreeMap<i32, i32>) {
        let mut curr = self;
        let mut depth = 0;
        for idx in it.map(|b| usize::from(b - b'a')) {
            curr = curr.nodes[idx].get_or_insert_default();
            depth += 1;
            curr.depth = depth;
            curr.count += 1;
            if curr.count >= k {
                *freq.entry(curr.depth).or_insert(0) += 1;
            }
        }
    }

    fn remove(&mut self, it: impl Iterator<Item = u8>, k: i32, freq: &mut BTreeMap<i32, i32>) {
        let mut curr = self;
        for idx in it.map(|b| usize::from(b - b'a')) {
            let Some(v) = curr.nodes[idx].as_mut() else {
                break;
            };
            curr = v;
            if curr.count >= k {
                let v = freq.entry(curr.depth).or_insert(0);
                *v -= 1;
                if *v <= 0 {
                    freq.remove(&curr.depth);
                }
            }
            curr.count -= 1;
        }
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
    fn basics() {
        assert_eq!(
            longest_common_prefix(&["jump", "run", "run", "jump", "run"], 2),
            [3, 4, 4, 3, 4]
        );
        assert_eq!(
            longest_common_prefix(&["dog", "racer", "car"], 2),
            [0, 0, 0]
        );
    }

    #[test]
    fn test() {}
}
